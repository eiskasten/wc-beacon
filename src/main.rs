use std::{fs, thread};
use std::error::Error;
use std::ops::Add;
use std::result::Result;
use std::time::Duration;

use crate::beacon::BeaconFrameGenerator;
use crate::decrypt::decrypt;
use crate::GGID::{English, French, German, Italian, Japanese, Korean, Spanish};
use crate::pcd::{Extended, Partitioned, PCD, Raw};

mod pcd;
mod beacon;
mod decrypt;

type MacAddress = [u8; 6];

fn main() -> Result<(), Box<dyn Error>> {
    decrypt()
}

fn main2() -> Result<(), Box<dyn Error>> {
    let dev_name = "wlp0s20f3";
    let dev_addr: MacAddress = [0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80];
    let src_addr: MacAddress = [0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80];
    let broadcast_addr: MacAddress = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    eprintln!("Use device '{}' with ethernet address '{:02x?}' and broadcast address '{:02x?}'", dev_name, dev_addr, broadcast_addr);
    let mut cap = pcap::Capture::from_device(dev_name)?.open()?;
    let pcd: PCD<Raw> = PCD::try_from(fs::read("008-dp-deoxys.pcd")?.as_slice())?;
    let partitioned: PCD<Partitioned> = pcd.into();
    let header = partitioned.header();
    let extended: PCD<Extended> = partitioned.into();
    let checksum = extended.checksum()?;
    eprintln!("Wondercard has checksum {:04x}", checksum);
    let encrypted = extended.encrypt(&dev_addr)?;
    let generator = BeaconFrameGenerator::new(dev_addr, German, &encrypted, header, checksum);
    for packet in generator {
        cap.sendpacket(packet.as_slice())?;
        thread::sleep(Duration::from_micros(10240));
    }
    Ok(())
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum GGID {
    Japanese = 0x345,
    English = 0x400318,
    French = 0x8000cd,
    German = 0x8000ce,
    Italian = 0x8000cf,
    Spanish = 0x8000d0,
    Korean = 0xc00018,
}

impl TryFrom<&str> for GGID {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "jp" => Ok(Japanese),
            "en" => Ok(English),
            "fr" => Ok(French),
            "de" => Ok(German),
            "it" => Ok(Italian),
            "es" => Ok(Spanish),
            "ko" => Ok(Korean),
            _ => Err(String::from("Unknown language code: ").add(value))
        }
    }
}
