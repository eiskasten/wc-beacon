// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, thread};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{GGID, MacAddress};
use crate::pcd::{Encrypted, Extended, Partitioned, PCD, PCD_EXTENDED_LENGTH, PCDFragment, PCDHeader, Raw, zero_pad};

/// A beacon frame generator which can generate an indefinite number of beacon frames.
pub struct BeaconFrameGenerator {
    /// The pre-constructed packets without the radio head.
    beacon_frames: Vec<Vec<u8>>,
    /// The radio head and the mac addresses.
    head: [u8; HEAD_LENGTH],
    /// A counter used for sequences.
    counter: u64,
}

const HEAD_LENGTH: usize = RADIO_HEAD.len() + BEACON_FRAME.len() + 2 * 6;
const ADDRESS_OFFSET: usize = RADIO_HEAD.len() + BEACON_FRAME.len();
const CRC_32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

/// Distributes encrypted packets using the provided parameters.
///
/// This function sends encrypted packets to a network device at a specified interval,
/// generating beacon frames with appropriate headers.
///
/// # Arguments
///
/// * `pcd` - A [PathBuf] representing the path to the PCD file.
/// * `region` - A [GGID] indicating the region.
/// * `device` - A [String] representing the network device name.
/// * `address` - A [MacAddress] representing the Ethernet address.
/// * `interval` - A [u64] specifying the time interval between sending packets, in microseconds.
///
/// # Returns
///
/// Returns `Ok(())` if the distribution process runs successfully,
/// otherwise returns an error wrapped in a `Box<dyn Error>`.
/// However, this function will actually never terminate on success and will run forever.
///
pub fn distribute(pcd: PathBuf, region: GGID, device: String, address: MacAddress, interval: u64) -> Result<(), Box<dyn Error>> {
    let broadcast_addr: MacAddress = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    eprintln!("Use device '{}' with ethernet address '{:02x?}' and broadcast address '{:02x?}'", device, address, broadcast_addr);
    let mut cap = pcap::Capture::from_device(device.as_str())?.open()?;
    eprintln!("Open wondercard file '{}'", pcd.as_path().display());
    let pcd: PCD<Raw> = PCD::try_from(fs::read(pcd)?.as_slice())?;
    let partitioned: PCD<Partitioned> = pcd.into();
    let header = partitioned.header();
    let extended: PCD<Extended> = partitioned.into();
    let checksum = extended.checksum()?;
    eprintln!("Wondercard has checksum {:04x}", checksum);
    let encrypted = extended.encrypt(&address)?;
    let generator = BeaconFrameGenerator::new(address, region, &encrypted, header, checksum);
    eprintln!("Distributing in {} µs intervals for region '{}'...", interval, region);
    for packet in generator {
        cap.sendpacket(packet.as_slice())?;
        thread::sleep(Duration::from_micros(interval));
    }
    Ok(())
}

impl BeaconFrameGenerator {
    /// Creates a new [BeaconFrameGenerator] instance with the provided parameters.
    ///
    /// This function initializes a [BeaconFrameGenerator] instance, generating beacon
    /// frames for encrypted fragments of the PCD data.
    ///
    /// # Arguments
    ///
    /// * `address` - A [MacAddress] representing the Ethernet address.
    /// * `region` - A [GGID] indicating the region.
    /// * `pcd` - A reference to the encrypted PCD data.
    /// * `header` - A [PCDHeader] representing the PCD header.
    /// * `checksum` - A [u16] representing the checksum of the extended PCD.
    ///
    /// # Returns
    ///
    /// Returns a new [BeaconFrameGenerator] instance.
    ///
    pub fn new(address: MacAddress, region: GGID, pcd: &PCD<Encrypted>, header: PCDHeader, checksum: u16) -> Self {
        let mut fragments = pcd.fragments();
        fragments.push(zero_pad(header));
        let beacon_frames = (0..fragments.len()).map(|f| wireless_management(packet(fragments.len() as u32, f as u16, checksum, PCD_EXTENDED_LENGTH as u32, *fragments.get(f).unwrap(), region))).collect();
        let mut head: [u8; HEAD_LENGTH] = [0; HEAD_LENGTH];
        head[..RADIO_HEAD.len()].copy_from_slice(&RADIO_HEAD);
        head[RADIO_HEAD.len()..ADDRESS_OFFSET].copy_from_slice(&BEACON_FRAME);
        head[ADDRESS_OFFSET..ADDRESS_OFFSET + 6].copy_from_slice(&address);
        head[ADDRESS_OFFSET + 6..ADDRESS_OFFSET + 12].copy_from_slice(&address);
        Self {
            beacon_frames,
            head,
            counter: 0,
        }
    }
}

impl Iterator for BeaconFrameGenerator {
    type Item = Vec<u8>;

    /// Generates the next beacon frame packet.
    ///
    /// This method generates the next beacon frame packet in the sequence,
    /// combining the beacon frame header, sequence information, fragment data,
    /// and CRC checksum.
    ///
    /// # Returns
    ///
    /// Returns an [Option] containing the next beacon frame packet as a [Vec<u8>].
    /// If there are no more packets to generate, it returns [None].
    /// However, this should actually never happen.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        let sequence = (self.counter << 4).to_le_bytes();
        let next = self.beacon_frames.get(self.counter as usize % self.beacon_frames.len()).map(|fragment| {
            let packet: Vec<u8> = [
                &self.head,
                &sequence[..2],
                fragment
            ].concat();
            let crc_checksum = CRC_32.checksum(&packet[RADIO_HEAD.len()..]);
            [
                packet.as_slice(),
                &crc_checksum.to_le_bytes()
            ].concat()
        });
        self.counter += 1;
        next
    }
}

const RADIO_HEAD: [u8; 56] = [
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

const BEACON_FRAME: [u8; 10] = [
    0x80, 0x00, // frame control field (type, subtype)
    0x00, 0x00, // duration
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff // destination address
];

//    0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // source address
//    0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // bssid
// sequence number, u16

const WIRELESS_MANAGEMENT: [u8; 32] = [
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

// packet

/// Creates a packet for the given beacon frame parameters.
///
/// This function constructs a packet using the provided parameters for the
/// beacon frame, including frames count, fragment index, checksum, payload
/// length, packet payload, and GGID.
///
/// # Arguments
///
/// * `frames_count` - The total number of frames in the beacon frame sequence.
/// * `fragment_index` - The index of the current fragment within the sequence.
/// * `checksum` - The checksum value of the packet.
/// * `payload_length` - The length of the packet payload.
/// * `packet_payload` - The payload data of the packet.
/// * `ggid` - A [GGID] indicating the region.
///
/// # Returns
///
/// Returns a [Vec<u8>] containing the constructed packet.
///
fn packet(frames_count: u32, fragment_index: u16, checksum: u16, payload_length: u32, packet_payload: PCDFragment, ggid: GGID) -> Vec<u8> {
    [
        frames_count.to_le_bytes().as_slice(),
        &0x1u16.to_le_bytes(),
        &0x1u16.to_le_bytes(),
        &(ggid as u32).to_le_bytes(),
        &0x0u16.to_le_bytes(),
        &0x70u16.to_le_bytes(),
        &0x28u16.to_le_bytes(),
        &0xcu16.to_le_bytes(),
        &checksum.to_le_bytes(),
        &(if fragment_index == (frames_count - 1) as u16 { 0xffff } else { fragment_index }).to_le_bytes(),
        &payload_length.to_le_bytes(),
        &packet_payload
    ].concat()
}

fn wireless_management(packet: Vec<u8>) -> Vec<u8> {
    [
        WIRELESS_MANAGEMENT.as_slice(),
        &packet
    ].concat()
}
