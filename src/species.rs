// SPDX-License-Identifier: GPL-3.0-only

use crate::speciesmap::SPECIES_MAP;

pub fn species_by_pokedex(idx: usize) -> Option<&'static str> {
    if idx > 0 && idx <= SPECIES_MAP.len() {
        Some(SPECIES_MAP[idx - 1])
    } else {
        None
    }
}