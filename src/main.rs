use std::{fs, thread};
use std::error::Error;
use std::io::Read;
use std::ops::Add;
use std::result::Result;
use std::time::Duration;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::GGID::{English, French, German, Italian, Japanese, Korean, Spanish};

type MacAddress = [u8; 6];

const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);


fn main() -> Result<(), Box<dyn Error>> {
    let dev_name = "wlp0s20f3";
    let dev_addr: MacAddress = [0x94, 0xe6, 0xf7, 0x06, 0xcf, 0x6b];
    let broadcast_addr: MacAddress = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let r = fs::read("/")?;
    let p = PCD::try_from(r.as_slice())?;
    let p2: PCD<Partitioned> = p.into();
    let region = (German as u32).to_le_bytes();

    let f_00 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x00, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x03, 0xfc, 0x9f, 0xa4, 0x77, 0xa3, 0x6f, 0x56, 0x7c, 0x17, 0xb7, 0x56, 0x0e, 0x87, 0x38, 0xd8, 0xbc, 0x37, 0x71, 0x29, 0x9c, 0x0c, 0x4c, 0xd1, 0xba, 0x4d, 0xc0, 0x01, 0xd4, 0xbc, 0x81, 0x5b, 0xdd, 0xe6, 0x46, 0xd1, 0x57, 0x66, 0x95, 0x58, 0x81, 0x08, 0x1e, 0x69, 0x06, 0xe4, 0x93, 0x9b, 0xa8, 0x5f, 0xb7, 0x3a, 0x4f, 0x9a, 0xaa, 0x9b, 0x76, 0x86, 0xa7, 0xe8, 0x7f, 0xfd, 0x48, 0x60, 0xdf, 0x45, 0x2a, 0x6d, 0x30, 0x85, 0xa1, 0x8f, 0x1d, 0x3c, 0xb0, 0x70, 0x1e, 0xe1, 0x22, 0x7e, 0x70, 0x8e, 0x64, 0x67, 0xa4, 0xfd, 0xbf, 0xb1, 0xdb, 0x5d, 0xd2, 0x51, 0x02, 0xb0, 0x12, 0x6f, 0x88, 0xb9, 0x72, 0xb0, 0x77, 0xec, 0x84, 0x5e];
    let f_01 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x01, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x74, 0x15, 0x7c, 0x14, 0xdc, 0xa9, 0x7f, 0xe8, 0x2c, 0xe2, 0x0e, 0x21, 0x20, 0xa8, 0xe3, 0x13, 0xd1, 0x11, 0x5f, 0x55, 0x40, 0xd6, 0x8d, 0x80, 0xf3, 0x90, 0xe8, 0xcd, 0x6c, 0x26, 0x90, 0x95, 0x66, 0xb0, 0x05, 0xa9, 0x93, 0xde, 0xde, 0xc8, 0x7b, 0xe7, 0x3b, 0x70, 0xfd, 0x8a, 0x1b, 0x5b, 0x97, 0x9f, 0xf1, 0xec, 0xcc, 0xb7, 0x5f, 0xd2, 0x3d, 0x17, 0x6e, 0x2d, 0xf4, 0x22, 0x2e, 0x5f, 0xd4, 0x52, 0x33, 0x52, 0x13, 0x5b, 0x15, 0xf3, 0x92, 0x58, 0x64, 0xb9, 0x50, 0x85, 0x31, 0x46, 0x6b, 0xc3, 0xc3, 0x2c, 0xfa, 0xbd, 0x74, 0x8f, 0x8f, 0xfe, 0xe6, 0xf8, 0xf1, 0xfd, 0xdd, 0x13, 0x5a, 0xc8, 0xb6, 0xbf, 0x9e, 0xb7, 0xc7, 0xb4];
    let f_02 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x02, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x65, 0x42, 0x7a, 0xd0, 0xea, 0xba, 0x66, 0x84, 0xb6, 0x10, 0x5a, 0xe2, 0x3a, 0xdf, 0x27, 0x54, 0x08, 0xe8, 0xba, 0xac, 0x97, 0x78, 0x49, 0xc5, 0x55, 0xe2, 0xf4, 0x23, 0xf5, 0xee, 0x59, 0xa7, 0x36, 0x2d, 0xa4, 0xcb, 0x18, 0x63, 0x2e, 0x93, 0x19, 0x41, 0x90, 0xb2, 0xe9, 0x28, 0x4e, 0x47, 0x3d, 0x89, 0x11, 0xfc, 0x1f, 0x06, 0xa5, 0xbd, 0x71, 0x17, 0x36, 0xf0, 0x64, 0xf9, 0x5e, 0x39, 0x26, 0x6c, 0x9b, 0x4c, 0xcd, 0x2e, 0x6d, 0x4e, 0x40, 0xe6, 0x1f, 0x26, 0x97, 0x84, 0x53, 0x83, 0x10, 0x53, 0xe1, 0x1a, 0x34, 0x18, 0x65, 0x88, 0x70, 0xf0, 0x7d, 0xa7, 0xd2, 0xf7, 0xa6, 0x7b, 0x30, 0x1d, 0x42, 0x86, 0x96, 0x57, 0x49, 0x8f];
    let f_03 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x03, 0x00, 0xa8, 0x03, 0x00, 0x00, 0xdc, 0x72, 0xe0, 0x17, 0x78, 0xc2, 0x41, 0x67, 0x40, 0x18, 0x02, 0x25, 0xc8, 0xe9, 0x9b, 0xa5, 0x64, 0xbe, 0x13, 0x9a, 0x7f, 0xaa, 0x85, 0xe2, 0xe3, 0xf6, 0x87, 0x0a, 0x54, 0x7b, 0x23, 0x74, 0x70, 0xd3, 0x7e, 0x01, 0x39, 0x1b, 0xce, 0x53, 0xba, 0xb1, 0x8a, 0x8d, 0xf4, 0x9f, 0x0e, 0x71, 0xad, 0x0c, 0x87, 0xd7, 0x88, 0x48, 0x6d, 0x43, 0x4f, 0x22, 0x24, 0xb7, 0xeb, 0x72, 0xd9, 0xde, 0xfe, 0x94, 0x7b, 0x8f, 0xb0, 0x84, 0x03, 0x78, 0x1f, 0x71, 0x35, 0x83, 0xa8, 0x51, 0xd9, 0x68, 0x06, 0x08, 0xb3, 0x78, 0xe9, 0xae, 0xaa, 0x74, 0x0b, 0x8b, 0xee, 0x80, 0xc1, 0xd5, 0xfc, 0xe3, 0xf0, 0x95, 0x3a, 0xe6, 0x6a, 0x2d, 0x33, 0x9a];
    let f_04 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x04, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x93, 0xa5, 0x4a, 0xa3, 0xf3, 0xdd, 0x53, 0x23, 0xe4, 0x56, 0x8c, 0x67, 0x3b, 0x10, 0x83, 0x32, 0xc0, 0x5e, 0x6c, 0xd1, 0x9d, 0x88, 0x1d, 0x91, 0xb0, 0x6c, 0xf2, 0x33, 0x97, 0xb0, 0x95, 0x54, 0x6f, 0x7d, 0x3d, 0x94, 0x4d, 0xa9, 0x2a, 0x64, 0x72, 0x7b, 0x8e, 0x50, 0x8c, 0x59, 0x16, 0xfe, 0x9d, 0x51, 0x28, 0x0e, 0xb7, 0x65, 0xf9, 0xca, 0x3a, 0xf9, 0x48, 0xa4, 0x48, 0xfb, 0x80, 0x13, 0x37, 0x95, 0x87, 0x40, 0x3a, 0x3d, 0xc7, 0x3a, 0x55, 0x11, 0x1a, 0x2c, 0xbb, 0x67, 0x75, 0x84, 0xd3, 0x11, 0xae, 0x44, 0xb6, 0x8b, 0xbc, 0xb9, 0x03, 0xa6, 0x33, 0x3a, 0x98, 0x5f, 0x79, 0x20, 0x25, 0xbb, 0x55, 0xd0, 0xc7, 0x07, 0x1d, 0x10];
    let f_05 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x05, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x14, 0x9e, 0x85, 0x0e, 0x8d, 0xf8, 0x58, 0xf0, 0x6d, 0xa7, 0xf0, 0x0e, 0xce, 0xe9, 0xad, 0x68, 0xe9, 0x79, 0xc7, 0xd1, 0x9e, 0x8a, 0xef, 0x28, 0x94, 0xe4, 0xb7, 0x8a, 0x24, 0xae, 0xeb, 0x4f, 0xc4, 0xb9, 0x00, 0xa5, 0x99, 0x5d, 0xd0, 0x56, 0xba, 0x73, 0xb8, 0x8d, 0xfa, 0x25, 0x45, 0x1e, 0x99, 0xee, 0x41, 0x85, 0x70, 0x09, 0x64, 0xb3, 0x2d, 0x52, 0x6a, 0x3a, 0x05, 0xc2, 0x65, 0xbb, 0xc2, 0x01, 0x83, 0x1a, 0x52, 0xb9, 0xed, 0xed, 0xba, 0xf7, 0xc0, 0xdd, 0xb6, 0xbd, 0xf0, 0x99, 0x84, 0x11, 0x21, 0x3e, 0x3a, 0x9f, 0x8d, 0xe7, 0x64, 0xbe, 0x2d, 0x4a, 0xf7, 0xd0, 0x64, 0xd7, 0xd8, 0x9a, 0x40, 0xde, 0xad, 0x95, 0x03, 0xfb];
    let f_06 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x06, 0x00, 0xa8, 0x03, 0x00, 0x00, 0xec, 0xbb, 0xe9, 0x05, 0x4d, 0x54, 0x91, 0xf5, 0x96, 0x6a, 0x35, 0xdf, 0xb1, 0x4e, 0xd9, 0x1c, 0xca, 0x62, 0xe1, 0x15, 0x3f, 0xac, 0x3d, 0x2f, 0x64, 0xb3, 0xdb, 0x49, 0xf6, 0x3b, 0x19, 0x31, 0xf3, 0xe2, 0x67, 0xc1, 0x91, 0x50, 0xae, 0x39, 0xeb, 0x5b, 0xf6, 0xd0, 0xf1, 0x28, 0x14, 0x4b, 0x67, 0x06, 0x39, 0x82, 0x5f, 0x18, 0x48, 0xba, 0x89, 0x82, 0x10, 0xab, 0x49, 0xf7, 0x6c, 0x03, 0x67, 0x48, 0x3f, 0x3f, 0x00, 0x33, 0xa1, 0xed, 0x5d, 0xcb, 0xb2, 0x83, 0xd5, 0x35, 0x29, 0x37, 0x78, 0x49, 0x25, 0xc9, 0xdc, 0x92, 0x8a, 0x9f, 0xdb, 0x17, 0x25, 0x7c, 0xbe, 0x13, 0x22, 0x19, 0x18, 0x32, 0x6c, 0x4b, 0x2f, 0x67, 0x4a, 0xc9];
    let f_07 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x07, 0x00, 0xa8, 0x03, 0x00, 0x00, 0xb3, 0xec, 0x20, 0x8f, 0xa4, 0x9f, 0xbb, 0x76, 0xb1, 0xbd, 0x1e, 0x04, 0x50, 0x2c, 0x0c, 0xa7, 0xab, 0xab, 0x54, 0xa2, 0x68, 0x5a, 0xd0, 0xcf, 0x3c, 0x32, 0xd4, 0x3e, 0x52, 0x9b, 0xbd, 0x95, 0xb7, 0xd1, 0xe0, 0xd7, 0xfb, 0xb3, 0x6b, 0xa0, 0x58, 0x74, 0x59, 0x1c, 0xad, 0xba, 0x12, 0xa9, 0x37, 0x4f, 0xe6, 0xf7, 0xa0, 0x21, 0xa4, 0xad, 0x5a, 0x52, 0x8f, 0x02, 0xc1, 0xc0, 0x21, 0xfd, 0x9c, 0x02, 0xce, 0x62, 0xe2, 0x00, 0x12, 0x76, 0xe4, 0x52, 0x14, 0x3d, 0x7f, 0x06, 0x31, 0xd6, 0xc7, 0xce, 0x68, 0x79, 0x44, 0x87, 0xe1, 0x88, 0x78, 0xc6, 0xd0, 0x01, 0x99, 0x57, 0x68, 0x63, 0xe7, 0x71, 0xa1, 0x99, 0x4b, 0x35, 0xb8, 0x4c];
    let f_08 = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0x08, 0x00, 0xa8, 0x03, 0x00, 0x00, 0x3d, 0x76, 0xe8, 0x1a, 0xaf, 0x25, 0xaf, 0x26, 0x84, 0x2a, 0x45, 0x76, 0xe8, 0x2f, 0xb9, 0xe6, 0x01, 0x0e, 0x4b, 0x0d, 0x3c, 0x21, 0x9e, 0x70, 0x58, 0xf8, 0xcd, 0x12, 0x80, 0x6d, 0x5d, 0x65, 0x85, 0x91, 0xf8, 0x30, 0x6f, 0xc2, 0x2e, 0x34, 0xd5, 0x0c, 0x5e, 0x0c, 0x07, 0x88, 0x4c, 0x4c, 0x41, 0xd8, 0xe8, 0x12, 0x72, 0xe4, 0xaa, 0x27, 0x29, 0xc5, 0xc0, 0x87, 0x15, 0x67, 0xce, 0xc6, 0x65, 0x41, 0xbe, 0xfd, 0xc7, 0x23, 0xa9, 0x37, 0xa5, 0xc3, 0x08, 0xb8, 0x25, 0xd9, 0x51, 0xd0, 0x07, 0x4b, 0xbe, 0xb0, 0xb1, 0xc6, 0x7e, 0xc0, 0x44, 0x68, 0x20, 0x59, 0xc8, 0xcd, 0xaf, 0x85, 0x3c, 0xf2, 0x44, 0x82, 0xa8, 0x22, 0x52, 0x52];
    let f_ff = [0x0a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, region[0], region[1], region[2], region[3], 0x00, 0x00, 0x70, 0x00, 0x28, 0x00, 0x0c, 0x00, 0xc5, 0xbd, 0xff, 0xff, 0xa8, 0x03, 0x00, 0x00, 0x37, 0x01, 0x5d, 0x01, 0x57, 0x01, 0x58, 0x01, 0x49, 0x01, 0x56, 0x01, 0x5d, 0x01, 0xde, 0x01, 0x4a, 0x01, 0x56, 0x01, 0x53, 0x01, 0x51, 0x01, 0xde, 0x01, 0x3d, 0x01, 0x54, 0x01, 0x45, 0x01, 0x47, 0x01, 0x49, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x0c, 0x00, 0x00, 0x08, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let payloads: Vec<&[u8]> = vec![
        &f_00,
        &f_01,
        &f_02,
        &f_03,
        &f_04,
        &f_05,
        &f_06,
        &f_07,
        &f_08,
        &f_ff,
    ];
    eprintln!("Use device '{}' with ethernet address '{:02x?}' and broadcast address '{:02x?}'", dev_name, dev_addr, broadcast_addr);
    let mut cap = pcap::Capture::from_device(dev_name)?.open()?;
    let mut counter: usize = 0;
    loop {
        let dumped_radio: &[u8] = &[
            0x00, 0x00, // rev, pad
            0x38, 0x00, // header length
            0x2f, 0x40, 0x40, 0xa0, 0x20, 0x08, 0x00, 0xa0, 0x20, 0x08, 0x00, 0x00, // present flags
            0x4d, 0x6c, 0xb8, 0x06, 0x00, 0x00, 0x00, 0x00, // MAC timestamp, update
            0x12, // flags
            0x04, // data rate
            0x8a, 0x09, // channel frequency
            0xa0, 0x00, // channel flags
            0xbd, // antenna signal
            0x00, // ?
            0x00, 0x00, // rx flags
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ?
            0xee, 0x6b, 0xb8, 0x06, 0x00, 0x00, 0x00, 0x00, 0x16, 0x00, 0x11, 0x03, // timestamp information, update
            0xbc, // antenna signal
            0x00, // antenna
            0xbd, // antenna signal
            0x01 // antenna
        ];
        let sequence = (counter << 4).to_le_bytes();
        let beacon_frame: &[u8] = &[
            0x80, 0x00, // frame control field (type, subtype)
            0x00, 0x00, // duration
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // destination address
            0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // source address
            0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // bssid
            sequence[0], sequence[1] // sequence number, fragment number, update increment by 0x10
        ];
        let wireless_management: &[u8] = &[
            // fixed parameters
            0xcc, 0xc8, 0x08, 0x2f, 0x00, 0x00, 0x00, 0x00, // timestamp, update
            0x0a, 0x00, // beacon interval
            0x21, 0x00, // capabilities information
            // tagged parameters
            0x01, 0x02, 0x82, 0x84, // tag supported rates
            0x03, 0x01, 0x07, // ds parameter set, current channel
            0x05, 0x05, 0x01, 0x02, 0x00, 0x00, 0x00, // traffic indication map, update
            // vendor specific
            0xdd, // tag number
            0x88, // tag length
            0x00, 0x09, 0xbf, // OUI
            0x00, // OUI type
        ];
        let mut header_plus_body = [
            beacon_frame,
            wireless_management,
            &payloads.get(counter % 0xa).expect("Out of bounds"),
        ].concat();
        let recalc_crc = CRC_32.checksum(header_plus_body.as_slice());

        let packet = [
            dumped_radio,
            header_plus_body.as_slice(),
            &recalc_crc.to_le_bytes()
        ].concat();
        cap.sendpacket(packet.as_slice())?;

        counter += 1;
        thread::sleep(Duration::from_micros(10240));
    }
}

#[repr(u32)]
enum GGID {
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

struct PCD<State> {
    state: State,
}

struct Raw<'a> {
    data: &'a [u8; PCD_LENGTH],
}

struct Partitioned<'a> {
    pgt: &'a [u8; PCD_PGT_LENGTH],
    header: &'a [u8; PCD_HEADER_LENGTH],
    card_data: &'a [u8; PCD_CARD_DATA_LENGTH],
}

struct Extended<'a> {
    pgt: &'a [u8; PCD_PGT_LENGTH],
    header: &'a [u8; PCD_HEADER_LENGTH],
    card_data: &'a [u8; PCD_CARD_DATA_LENGTH],
    header_duplicate: &'a [u8; PCD_HEADER_LENGTH],
}

const PCD_LENGTH: usize = 0x358;
// = (856)10
const PCD_PGT_LENGTH: usize = 0x104;
const PCD_HEADER_LENGTH: usize = 0x50;
const PCD_CARD_DATA_LENGTH: usize = 0x204;

impl<'a> TryFrom<&'a [u8]> for PCD<Raw<'a>> {
    type Error = String;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let sized_value: &[u8; PCD_LENGTH] = <&[u8; PCD_LENGTH]>::try_from(value).map_err(|_| format!("PCD size needs to be {}, but was: {}", PCD_LENGTH, value.len()))?;
        Ok(PCD { state: Raw { data: sized_value } })
    }
}

impl<'a> From<PCD<Raw<'a>>> for PCD<Partitioned<'a>> {
    fn from(value: PCD<Raw<'a>>) -> Self {
        let sized_value = value.state.data;
        PCD {
            state: Partitioned {
                pgt: <&[u8; PCD_PGT_LENGTH]>::try_from(&sized_value[..PCD_PGT_LENGTH]).unwrap(),
                header: <&[u8; PCD_HEADER_LENGTH]>::try_from(&sized_value[PCD_PGT_LENGTH..PCD_HEADER_LENGTH + PCD_PGT_LENGTH]).unwrap(),
                card_data: <&[u8; PCD_CARD_DATA_LENGTH]>::try_from(&sized_value[PCD_HEADER_LENGTH + PCD_PGT_LENGTH..PCD_CARD_DATA_LENGTH + PCD_HEADER_LENGTH + PCD_PGT_LENGTH]).unwrap(),
            }
        }
    }
}

impl<'a> From<PCD<Partitioned<'a>>> for PCD<Extended<'a>> {
    fn from(value: PCD<Partitioned<'a>>) -> PCD<Extended<'a>> {
        let part = value.state;
        PCD {
            state: Extended {
                pgt: part.pgt,
                header: part.header,
                card_data: part.card_data,
                header_duplicate: part.header,
            }
        }
    }
}