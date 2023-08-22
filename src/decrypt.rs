// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::MacAddress;
use crate::pcd::{Encrypted, PCD};


/// Decrypts an encrypted PCD file and saves the decrypted data to a new file.
///
/// This function reads an encrypted PCD file, decrypts its content using the provided
/// checksum and Ethernet address, and then saves the decrypted data to a new file.
///
/// # Arguments
///
/// * `epcd_file` - A [PathBuf] representing the path to the encrypted PCD file.
/// * `checksum` - The checksum value used for decryption.
/// * `address` - A [MacAddress] representing the Ethernet address.
/// * `pcd_file` - A [PathBuf] representing the path to the output decrypted PCD file.
///
/// # Returns
///
/// Returns [Ok(())] if the decryption and file write process runs successfully,
/// otherwise returns an error wrapped in a [Box<dyn Error>].
///
pub fn decrypt(epcd_file: PathBuf, checksum: u16, address: MacAddress, pcd_file: PathBuf) -> Result<(), Box<dyn Error>> {
    let data = fs::read(epcd_file)?;
    let pcd: PCD<Encrypted> = data.as_slice().try_into()?;
    let decrypted_data = pcd.decrypt(&address, checksum).simplify().data();
    fs::write(pcd_file, decrypted_data)?;
    Ok(())
}