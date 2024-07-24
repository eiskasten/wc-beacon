// SPDX-License-Identifier: GPL-3.0-only

extern crate core;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::result::Result;

use clap::{Parser, Subcommand, ValueEnum};

use crate::beacon::distribute;
use crate::decrypt::decrypt;
use crate::info::{info, set};
use crate::pcd::{CardType, Game};

mod pcd;
mod beacon;
mod decrypt;
mod pokestr;
mod info;

pub mod pokestrmap {
    include!(concat!(env!("OUT_DIR"), "/pokestrmap.rs"));
}

/// The main entry point of the CLI application.
///
/// Parses command-line arguments using the `Cli` struct, and then executes
/// the appropriate command based on the parsed input.
///
/// # Returns
///
/// Returns `Ok(())` if the program runs successfully, otherwise returns an
/// error wrapped in a `Box<dyn Error>`.
///
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Distribute { pcd, region, device, address, interval } =>
            distribute(pcd, region, device, address.unwrap_or([0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80]), interval),
        Command::Decrypt { epcd, checksum, address, pcd } => decrypt(epcd, checksum, address, pcd),
        Command::Info { pcd } => info(pcd),
        Command::Set { title, kind: card_type, card_id, games, description: comment, redistribution, icons, pgt, date: received, pcd, output } => set(title, card_type, card_id, games, comment, redistribution, icons, pgt, received, pcd, output)
    }
}

#[derive(Parser)]
#[command(version, author, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Distribute a given PCD file to the Pokemon games using a WiFi device
    #[command(name = "dist")]
    Distribute {
        /// The PCD file to distribute
        #[arg(short, long, value_name = "PCD_FILE")]
        pcd: PathBuf,
        /// The region to distribute the wondercard in
        #[arg(short, long, value_enum)]
        region: GGID,
        /// The WiFi device to use for the distribution
        #[arg(short, long)]
        device: String,
        /// MAC Address to spoof [default: a4:c0:e1:6e:76:80]
        #[arg(short, long, value_parser = mac_address_parser)]
        address: Option<MacAddress>,
        /// The interval used for the beacon frames in µs
        #[arg(short, long, default_value_t = 10240)]
        interval: u64,
    },
    /// Decrypt a PCD file which was distributed over the network
    #[command(name = "dec")]
    Decrypt {
        /// The encrypted PCD file to decrypt
        #[arg(short, long, value_name = "ENCRYPTED_PCD_FILE")]
        epcd: PathBuf,
        /// The original checksum of the PCD
        #[arg(short, long)]
        checksum: u16,
        /// The MAC Address used for the original distribution
        #[arg(short, long, value_parser = mac_address_parser)]
        address: MacAddress,
        /// The location of the decrypted output, will be overwritten if the file already exists
        #[arg(short, long, value_name = "PCD_FILE")]
        pcd: PathBuf,
    },
    /// Show information about a given pcd file
    #[command(name = "info")]
    Info {
        /// The PCD file to show the information about
        #[arg(short, long, value_name = "PCD_FILE")]
        pcd: PathBuf
    },
    /// Create a new PCD file or edit an existing one
    #[command(name = "set")]
    Set {
        /// PCD file to edit, non-destructive only for input, leave empty to create from scratch
        #[arg(short, long, value_name = "PCD_FILE")]
        pcd: Option<PathBuf>,
        /// Wonder Card title
        #[arg(short, long, value_name = "TITLE")]
        title: Option<String>,
        /// Wonder Card Type
        #[arg(short, long, value_name = "KIND")]
        kind: Option<CardType>,
        /// Wonder Card ID
        #[arg(short, long, value_name = "ID")]
        card_id: Option<u16>,
        /// Games to distribute to
        #[arg(short, long, value_name = "GAMES")]
        games: Option<Vec<Game>>,
        /// Wonder Card comment/description
        #[arg(short, long, value_name = "DESCRIPTION")]
        description: Option<String>,
        /// How often players can redistribute, 255 for unlimited
        #[arg(short, long, value_name = "REDISTRIBUTION")]
        redistribution: Option<u8>,
        /// Exactly 3 Wonder Card Icons, use Pokédex index and 0 for none
        #[arg(short, long, value_name = "ICONS")]
        icons: Option<Vec<u16>>,
        /// PGT File
        #[arg(long, value_name = "PGT")]
        pgt: Option<PathBuf>,
        /// Wonder Card received date
        #[arg(long, value_name = "received")]
        date: Option<u16>,
        /// Output
        #[arg(long, value_name = "FILE")]
        output: PathBuf,
    },
}

/// Region codes.
/// Represent languages not regions themself, e.g. English is for UK and US.
#[repr(u32)]
#[derive(Copy, Clone, ValueEnum)]
pub enum GGID {
    /// Japanese
    #[value(name = "ja")]
    Japanese = 0x345,
    /// English
    #[value(name = "en")]
    English = 0x400318,
    /// French
    #[value(name = "fr")]
    French = 0x8000cd,
    /// German
    #[value(name = "de")]
    German = 0x8000ce,
    /// Italian
    #[value(name = "it")]
    Italian = 0x8000cf,
    /// Spanish
    #[value(name = "es")]
    Spanish = 0x8000d0,
    /// Korean
    #[value(name = "ko")]
    Korean = 0xc00018,
}

impl Display for GGID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                GGID::Japanese => "jp",
                GGID::English => "en",
                GGID::French => "fr",
                GGID::German => "de",
                GGID::Italian => "it",
                GGID::Spanish => "es",
                GGID::Korean => "ko",
            })
    }
}

/// A simple MAC Address representation.
type MacAddress = [u8; 6];

/// Parses a MAC address string and returns a [MacAddress].
///
/// A MAC address is a unique identifier assigned to network interfaces.
/// The format is six pairs of hexadecimal digits separated by colons.
///
/// # Arguments
///
/// * `value` - A string representing a MAC address.
///
/// # Returns
///
/// Returns a [Result] containing a [MacAddress] if the parsing is successful,
/// or a [String] containing an error message if parsing fails.
///
/// # Examples
///
/// ```
/// let mac_address_str = "00:1A:2B:3C:4D:5E";
/// let result = mac_address_parser(mac_address_str);
///
/// match result {
///     Ok(mac_address) => {
///         println!("Parsed MAC address: {:?}", mac_address);
///     }
///     Err(error) => {
///         eprintln!("Error parsing MAC address: {}", error);
///     }
/// }
/// ```
///
fn mac_address_parser(value: &str) -> Result<MacAddress, String> {
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 6 {
        return Err(format!("A MAC address requires 6 blocks, but {} were specified", parts.len()));
    }
    let mut address: MacAddress = [0; 6];
    for i in 0..6 {
        let part = parts.get(i).unwrap();
        address[i] = u8::from_str_radix(part, 16).map_err(|_| format!("MAC address blocks must consist of hexadecimal block, but provided: '{}'", part))?;
    }
    Ok(address)
}
