// SPDX-License-Identifier: GPL-3.0-only

use rc4::{KeyInit, Rc4, StreamCipher};

use crate::MacAddress;

pub const PCD_LENGTH: usize = PCD_PGT_LENGTH + PCD_HEADER_LENGTH + PCD_CARD_DATA_LENGTH;
// = (856)10
pub const PCD_EXTENDED_LENGTH: usize = PCD_LENGTH + PCD_HEADER_LENGTH;
pub const PCD_PGT_LENGTH: usize = 0x104;
pub const PCD_HEADER_LENGTH: usize = 0x50;
pub const PCD_CARD_DATA_LENGTH: usize = 0x204;
pub const PCD_FRAGMENTS: usize = 0x0a;
pub const PCD_FRAGMENT_LENGTH: usize = PCD_EXTENDED_LENGTH / (PCD_FRAGMENTS - 1);

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