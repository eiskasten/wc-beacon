use crate::{GGID, MacAddress};
use crate::pcd::{Encrypted, PCD, PCDFragment};

pub struct BeaconFrameGenerator {
    counter: u64,
}

impl BeaconFrameGenerator {
    pub fn new(address: &MacAddress, region: GGID, pcd: &PCD<Encrypted>) -> Self {
        Self {
            counter: 0
        }
    }
}

impl Iterator for BeaconFrameGenerator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        None
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

// sequence number, u16

const BEACON_FRAME: [u8; 10] = [
    0x80, 0x00, // frame control field (type, subtype)
    0x00, 0x00, // duration
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff // destination address
];

//    0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // source address
//    0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80, // bssid

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

fn packet(frames_count: u32, fragment_index: u8, checksum: u16, payload_length: u32, packet_payload: PCDFragment, ggid: GGID) -> Vec<u8> {
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
        &fragment_index.to_le_bytes(),
        &payload_length.to_le_bytes(),
        &packet_payload
    ].concat()
}

// Length  Value/Meaning
// 1  0xdd (tag ID)
// 1  0x88 (tag length)
// 3  0x00 0x09 0xbf (OUI, Nintendo)
// 1  0x00 (OUI subtype)
//
// 132  --- actual packet ---
// 28  --- packet header ---
// 4  0xa (frames count?)
// 2  0x1
// 2  0x1
// 4  GGID (language code)
// 2  0x0
// 2  0x70
// 2  0x28
// 2  0xc
// 2  checksum
// 2  fragment index
// 4  0x3a8 (payload length)
//
// 104  --- packet payload ---