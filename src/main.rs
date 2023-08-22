// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::path::PathBuf;
use std::result::Result;

use clap::{Parser, Subcommand, ValueEnum};

use crate::beacon::distribute;
use crate::decrypt::decrypt;

mod pcd;
mod beacon;
mod decrypt;

type MacAddress = [u8; 6];

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Distribute { pcd, region, device, address, interval } =>
            distribute(pcd, region, device, address.unwrap_or([0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80]), interval),
        Command::Decrypt { epcd, checksum, address, pcd } => decrypt(epcd, checksum, address, pcd)
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
