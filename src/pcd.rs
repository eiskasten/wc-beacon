// SPDX-License-Identifier: GPL-3.0-only

use std::cmp::min;
use std::fmt::{Display, Formatter};
use rc4::{KeyInit, Rc4, StreamCipher};

use crate::MacAddress;
use crate::pcd::CardType::{Accessory, AzureFlute, Item, ManaphyEgg, MemberCard, OaksLetter, Pokemon, PokemonEgg, PoketchApp, PokewalkerArea, Rule, Seal, Secretkey, Unknown};
use crate::pcd::Game::{Diamond, HeartGold, Pearl, Platinum, SoulSilver};
use crate::pokestr::{Gen4Str, STRING_TERMINATOR};

pub const PCD_LENGTH: usize = PCD_PGT_LENGTH + PCD_HEADER_LENGTH + PCD_CARD_DATA_LENGTH;
// = (856)10
pub const PCD_EXTENDED_LENGTH: usize = PCD_LENGTH + PCD_HEADER_LENGTH;
pub const PCD_PGT_LENGTH: usize = 0x104;
pub const PCD_HEADER_LENGTH: usize = 0x50;
pub const PCD_CARD_DATA_LENGTH: usize = 0x204;
pub const PCD_FRAGMENTS: usize = 0x0a;
pub const PCD_FRAGMENT_LENGTH: usize = PCD_EXTENDED_LENGTH / (PCD_FRAGMENTS - 1);

/// Attribute const offsets are absolute to pcd raw data
pub const PCD_TITLE_OFFSET: usize = 0x104;

pub const PCD_CARD_ID_OFFSET: usize = 0x150;
pub const PCD_GAMES_OFFSET: usize = 0x14c;

/// Length in u16 units inclusive termination
pub const PCD_TITLE_MAX_LENGTH: usize = (PCD_GAMES_OFFSET - PCD_TITLE_OFFSET) / 2;


pub const PCD_COMMENT_OFFSET: usize = 0x154;
/// Length in u16 units inclusive termination
pub const PCD_COMMENT_MAX_LENGTH: usize = (PCD_REDISTRIBUTION_OFFSET - PCD_COMMENT_OFFSET) / 2;

pub const PCD_ICONS_OFFSET: usize = 0x34b;

pub const PCD_RECEIVED_OFFSET: usize = 0x354;

pub const PCD_REDISTRIBUTION_OFFSET: usize = 0x356;

pub type PCDFragment = [u8; PCD_FRAGMENT_LENGTH];
pub type PCDHeader = [u8; PCD_HEADER_LENGTH];

pub struct PCD<State> {
    state: State,
}

pub struct Raw {
    data: [u8; PCD_LENGTH],
}

pub struct Encrypted {
    data: [u8; PCD_EXTENDED_LENGTH],
}

pub struct Partitioned {
    pgt: [u8; PCD_PGT_LENGTH],
    header: PCDHeader,
    card_data: [u8; PCD_CARD_DATA_LENGTH],
}

pub struct Extended {
    header: PCDHeader,
    pgt: [u8; PCD_PGT_LENGTH],
    header_duplicate: PCDHeader,
    card_data: [u8; PCD_CARD_DATA_LENGTH],
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CardType {
    None = 0x0,
    Pokemon = 0x1,
    PokemonEgg = 0x2,
    Item = 0x3,
    Rule = 0x4,
    Seal = 0x5,
    Accessory = 0x6,
    ManaphyEgg = 0x7,
    MemberCard = 0x8,
    OaksLetter = 0x9,
    AzureFlute = 0xa,
    PoketchApp = 0xb,
    Secretkey = 0xc,
    Unknown = 0xd,
    PokewalkerArea = 0xe,
}

impl TryFrom<u8> for CardType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        static CARD_TYPES: [CardType; 15] = [CardType::None,
            Pokemon,
            PokemonEgg,
            Item,
            Rule,
            Seal,
            Accessory,
            ManaphyEgg,
            MemberCard,
            OaksLetter,
            AzureFlute,
            PoketchApp,
            Secretkey,
            Unknown,
            PokewalkerArea];
        CARD_TYPES.iter().find(|&&t| t as u8 == value).map(|&t| t).ok_or(())
    }
}

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Game {
    Diamond = 1 << 2,
    Pearl = 1 << 3,
    Platinum = 1 << 4,
    HeartGold = 1 << 15,
    SoulSilver = 1 << 0,
}


impl Game {
    pub fn parse(n: u16) -> Vec<Self> {
        let games = vec![Diamond, Pearl, Platinum, HeartGold, SoulSilver];
        games.iter().filter(|&g| *g as u16 & n > 0).map(|g| *g).collect()
    }
}

pub struct Deserialized {
    pub title: String,
    pub card_type: CardType,
    pub card_id: u16,
    pub games: Vec<Game>,
    pub comment: String,
    pub redistribution: u8,
    pub icons: (u16, u16, u16),
    pub pgt: [u8; PCD_PGT_LENGTH],
    pub received: u16,
}

impl<'a> TryFrom<&'a [u8]> for PCD<Raw> {
    type Error = String;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let sized_value: [u8; PCD_LENGTH] = <[u8; PCD_LENGTH]>::try_from(value).map_err(|_| format!("PCD size needs to be {}, but was: {}", PCD_LENGTH, value.len()))?;
        Ok(PCD { state: Raw { data: sized_value } })
    }
}

impl<'a> TryFrom<&'a [u8]> for PCD<Encrypted> {
    type Error = String;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let sized_value: [u8; PCD_EXTENDED_LENGTH] = <[u8; PCD_EXTENDED_LENGTH]>::try_from(value).map_err(|_| format!("PCD size needs to be {}, but was: {}", PCD_EXTENDED_LENGTH, value.len()))?;
        Ok(PCD { state: Encrypted { data: sized_value } })
    }
}

impl PCD<Partitioned> {
    pub fn header(&self) -> PCDHeader {
        self.state.header
    }
    pub fn data(&self) -> Vec<u8> {
        [
            self.state.pgt.as_slice(),
            &self.state.header,
            &self.state.card_data,
        ].concat()
    }
}

impl From<PCD<Raw>> for PCD<Partitioned> {
    fn from(value: PCD<Raw>) -> Self {
        let sized_value = value.state.data;
        PCD {
            state: Partitioned {
                pgt: <[u8; PCD_PGT_LENGTH]>::try_from(&sized_value[..PCD_PGT_LENGTH]).unwrap(),
                header: <[u8; PCD_HEADER_LENGTH]>::try_from(&sized_value[PCD_PGT_LENGTH..PCD_HEADER_LENGTH + PCD_PGT_LENGTH]).unwrap(),
                card_data: <[u8; PCD_CARD_DATA_LENGTH]>::try_from(&sized_value[PCD_HEADER_LENGTH + PCD_PGT_LENGTH..PCD_CARD_DATA_LENGTH + PCD_HEADER_LENGTH + PCD_PGT_LENGTH]).unwrap(),
            }
        }
    }
}

impl PCD<Partitioned> {
    pub fn deserialize(self) -> PCD<Deserialized> {

        // self.state.header
        let header: Vec<u16> = self.state.header.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let card_data: Vec<u16> = self.state.card_data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();

        let icons_offset_rela = (PCD_ICONS_OFFSET - PCD_COMMENT_OFFSET) / 2;

        let des = Deserialized {
            title: (&first_str(&self.state.header, PCD_TITLE_MAX_LENGTH)).try_into().unwrap(),
            card_type: CardType::try_from(self.state.pgt[0]).unwrap_or(Unknown),
            card_id: header[(PCD_CARD_ID_OFFSET - PCD_TITLE_OFFSET) / 2],
            games: Game::parse(header[(PCD_GAMES_OFFSET - PCD_TITLE_OFFSET) / 2].rotate_left(8)),
            comment: (&first_str(&self.state.card_data, PCD_COMMENT_MAX_LENGTH)).try_into().unwrap(),
            redistribution: self.state.card_data[PCD_REDISTRIBUTION_OFFSET - PCD_COMMENT_OFFSET],
            icons: (card_data[icons_offset_rela], card_data[icons_offset_rela + 1], card_data[icons_offset_rela + 2]),
            pgt: self.state.pgt,
            received: card_data[(PCD_RECEIVED_OFFSET - PCD_COMMENT_OFFSET) / 2],
        };
        PCD { state: des }
    }
}

fn first_str(data: &[u8], max_len: usize) -> Gen4Str {
    let chunks = data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]]));
    let mut chunks_clone = chunks.clone();
    let len = min(max_len, chunks_clone.position(|e| e == STRING_TERMINATOR).unwrap_or(max_len)).checked_sub(1).unwrap_or(0);

    let str_vec = chunks.take(len).collect();

    Gen4Str::new(str_vec)
}

impl Display for PCD<Deserialized> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {}\ticons: {:?}\n\
        type: {:?}\tcard ID: {}\n\n\
        {}\n\n\
        games: {:?}\n\
        redistribution limit: {}{}\n\
        received: {}\n", self.state.title, self.state.icons, self.state.card_type, self.state.card_id, self.state.comment, self.state.games, self.state.redistribution, if self.state.redistribution == 0xff { "(unlimited)" } else { "" }, self.state.received)
    }
}

impl From<PCD<Partitioned>> for PCD<Extended> {
    fn from(value: PCD<Partitioned>) -> PCD<Extended> {
        let part = value.state;
        PCD {
            state: Extended {
                header: part.header,
                pgt: part.pgt,
                header_duplicate: part.header,
                card_data: part.card_data,
            }
        }
    }
}

impl PCD<Extended> {
    /// Calculates the checksum of the binding data.
    ///
    /// This method calculates the checksum of the concatenated binding data, which includes
    /// the header, PGT, header duplicate, and card data.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing the calculated checksum as a [u16] if the calculation
    /// is successful, otherwise returns a [String] containing an error message.
    ///
    pub fn checksum(&self) -> Result<u16, String> {
        let state = &self.state;
        let mut checksum: u16 = 0;
        let binding = [state.header.as_slice(), &state.pgt, &state.header_duplicate, &state.card_data].concat();
        let data = binding.as_slice();
        if data.len() % 2 != 0 {
            return Err(format!("The data length {} is not dividable by 2", &data.len()));
        }
        for chunk in data.chunks_exact(2) {
            let c0 = chunk[0];
            let c1 = chunk[1];
            checksum = checksum.wrapping_add(u16::from_le_bytes([c0, c1]));
            checksum = checksum.rotate_left(1);
        }
        Ok(checksum)
    }

    /// Encrypts the PCD data using the provided address.
    ///
    /// This method encrypts the PCD data using the provided Ethernet address and checksum,
    /// transforming it into encrypted data.
    ///
    /// # Arguments
    ///
    /// * `address` - A reference to a [MacAddress] representing the Ethernet address.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing a new encrypted [PCD<Encrypted>] if the encryption
    /// process is successful, otherwise returns a [String] containing an error message.
    ///
    pub fn encrypt(self, address: &MacAddress) -> Result<PCD<Encrypted>, String> {
        let checksum = self.checksum()?;
        let state = self.state;
        let data: &mut [u8] = &mut [state.header.as_slice(), &state.pgt, &state.header_duplicate, &state.card_data].concat();
        let key = key(address, checksum);
        let mut rc4 = Rc4::new(&key.into());
        rc4.apply_keystream(data.as_mut());
        let imm_data: &[u8] = data;
        let sized_data: [u8; PCD_EXTENDED_LENGTH] = <[u8; PCD_EXTENDED_LENGTH]>::try_from(imm_data).map_err(|_| format!("Encrypted data length is {} instead of {}", data.len(), PCD_LENGTH))?;
        Ok(PCD::new(sized_data))
    }

    /// Go back to the [PCD<Partitioned>] state.
    ///
    pub fn simplify(self) -> PCD<Partitioned> {
        PCD {
            state: Partitioned {
                pgt: self.state.pgt,
                header: self.state.header,
                card_data: self.state.card_data,
            }
        }
    }
}

impl PCD<Encrypted> {
    /// Construct from raw data.
    ///
    /// # Arguments
    ///
    /// * `data`: the data which later should be distributed or decrypted
    ///
    /// returns: [PCD<Encrypted>]
    ///
    fn new(data: [u8; PCD_EXTENDED_LENGTH]) -> Self {
        PCD {
            state: Encrypted {
                data
            }
        }
    }

    /// Fragment the encrypted data.
    ///
    /// Split the payload into the desired fragment sizes without the header.
    ///
    /// returns: The [PCD<Fragment>] state.
    ///
    pub fn fragments(&self) -> Vec<PCDFragment> {
        self.state.data.chunks_exact(PCD_EXTENDED_LENGTH / (PCD_FRAGMENTS - 1)).map(|f| <[u8; PCD_FRAGMENT_LENGTH]>::try_from(f).unwrap()).collect()
    }

    /// Decrypts the encrypted PCD data using the provided address and checksum.
    ///
    /// This method decrypts the encrypted PCD data using the provided Ethernet address
    /// and checksum, returning the decrypted data as a [PCD<Extended>] instance.
    ///
    /// # Arguments
    ///
    /// * `address` - A reference to a [MacAddress] representing the Ethernet address.
    /// * `checksum` - The checksum value used for decryption.
    ///
    /// # Returns
    ///
    /// Returns a [PCD<Extended>] instance containing the decrypted PCD data.
    ///
    pub fn decrypt(self, address: &MacAddress, checksum: u16) -> PCD<Extended> {
        let key = key(address, checksum);
        let mut rc4 = Rc4::new(&key.into());
        let mut data = self.state.data;
        rc4.apply_keystream(data.as_mut());
        let mut header: [u8; PCD_HEADER_LENGTH] = [0; PCD_HEADER_LENGTH];
        let mut pgt: [u8; PCD_PGT_LENGTH] = [0; PCD_PGT_LENGTH];
        let mut card_data: [u8; PCD_CARD_DATA_LENGTH] = [0; PCD_CARD_DATA_LENGTH];
        header.copy_from_slice(&data[0..PCD_HEADER_LENGTH]);
        pgt.copy_from_slice(&data[PCD_HEADER_LENGTH..PCD_PGT_LENGTH + PCD_HEADER_LENGTH]);
        card_data.copy_from_slice(&data[2 * PCD_HEADER_LENGTH + PCD_PGT_LENGTH..PCD_CARD_DATA_LENGTH + PCD_PGT_LENGTH + 2 * PCD_HEADER_LENGTH]);
        PCD {
            state: Extended {
                header,
                pgt,
                header_duplicate: header,
                card_data,
            }
        }
    }
}

fn key(address: &MacAddress, checksum: u16) -> [u8; 8] {
    let checksum_parts = checksum.to_le_bytes();
    let mut key = [address[0], address[1], checksum_parts[0], checksum_parts[1], address[4], address[5], address[2], address[3]];
    let mut hw_low = 0xa2u8;
    let mut hw_high = 0x3fu8;
    for i in 0..4 {
        let i_low = 2 * i;
        let i_high = i_low + 1;
        key[i_low] ^= hw_low;
        key[i_high] ^= hw_high;
        hw_low = key[i_low];
        hw_high = key[i_high];
    }
    key
}

pub fn zero_pad(header: PCDHeader) -> PCDFragment {
    let mut padded_header: PCDFragment = [0; PCD_FRAGMENT_LENGTH];
    padded_header[..PCD_HEADER_LENGTH].copy_from_slice(&header);
    padded_header
}