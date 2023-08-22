// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::MacAddress;
use crate::pcd::{Encrypted, PCD};

pub fn decrypt(epcd_file: PathBuf, checksum: u16, address: MacAddress, pcd_file: PathBuf) -> Result<(), Box<dyn Error>> {
    let data = fs::read(epcd_file)?;
    let pcd: PCD<Encrypted> = data.as_slice().try_into()?;
    let decrypted_data = pcd.decrypt(&address, checksum).simplify().data();
    fs::write(pcd_file, decrypted_data)?;
    Ok(())
}