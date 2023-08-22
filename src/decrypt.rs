use std::error::Error;
use std::fs;

use crate::MacAddress;
use crate::pcd::{Encrypted, PCD};

pub fn decrypt() -> Result<(), Box<dyn Error>> {
    let in_filename = "";
    let out_filename = "";
    let address: MacAddress = [0xa4, 0xc0, 0xe1, 0x6e, 0x76, 0x80];
    let checksum: u16 = 0;

    let data = fs::read(in_filename)?;
    let pcd: PCD<Encrypted> = data.as_slice().try_into()?;
    let decrypted_data = pcd.decrypt(&address, checksum).simplify().data();
    fs::write(out_filename, decrypted_data)?;
    Ok(())
}