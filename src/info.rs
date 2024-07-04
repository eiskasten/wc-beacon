// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::pcd::{Partitioned, PCD, Raw};

pub fn info(pcd: PathBuf) -> Result<(), Box<dyn Error>> {
    let pcd: PCD<Raw> = PCD::try_from(fs::read(pcd)?.as_slice())?;
    let partitioned: PCD<Partitioned> = pcd.into();
    let des = partitioned.deserialize();
    eprintln!("{}", des);
    Ok(())
}