// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::pcd::{CardType, Deserialized, Game, Partitioned, PCD, PCD_LENGTH, Raw};

pub fn info(pcd: PathBuf) -> Result<(), Box<dyn Error>> {
    let pcd: PCD<Raw> = PCD::try_from(fs::read(pcd)?.as_slice())?;
    let partitioned: PCD<Partitioned> = pcd.into();
    let des = partitioned.deserialize();
    eprintln!("{}", des);
    Ok(())
}

pub fn set(title: Option<String>, card_type: Option<CardType>, card_id: Option<u16>, games: Option<Vec<Game>>, comment: Option<String>, redistribution: Option<u8>, icons: Option<Vec<u16>>, pgt: Option<PathBuf>, received: Option<u16>, pcd: Option<PathBuf>, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut pcd = if let Some(f) = pcd {
        let raw: PCD<Raw> = PCD::try_from(fs::read(f)?.as_slice())?;
        let parts: PCD<Partitioned> = raw.try_into().unwrap();
        parts.deserialize()
    } else {
        PCD::<Deserialized>::new()
    };

    if let Some(t) = title {
        pcd.state.title = t;
    }

    if let Some(c) = card_type {
        pcd.state.card_type = c;
    }

    if let Some(c) = card_id {
        pcd.state.card_id = c;
    }

    if let Some(g) = games {
        pcd.state.games = g;
    }

    if let Some(c) = comment {
        pcd.state.comment = c;
    }

    if let Some(r) = redistribution {
        pcd.state.redistribution = r;
    }

    if let Some(i) = icons {
        pcd.state.icons = (i[0], i[1], i[2]);
    }

    if let Some(r) = received {
        pcd.state.received = r;
    }

    if let Some(p) = pgt {
        let mut f = File::open(p).expect("Unable to open pgt file");
        f.read_exact(&mut pcd.state.pgt).expect("Unable to read pgt");
    }

    let pcd: PCD<Raw> = (&pcd.serialize()).try_into()?;
    let pcd_data: [u8; PCD_LENGTH] = pcd.into();

    let mut f = File::create(output)?;
    f.write(&pcd_data).expect("Unable to write pcd file");

    Ok(())
}