// SPDX-License-Identifier: GPL-3.0-only

use crate::pokestrmap::{CHARACTER_MAP_BY_GENIV, CHARACTER_MAP_BY_UTF16};
use std::string::String;
use utf16::Utf16Grapheme;

pub const STRING_TERMINATOR: u16 = 0xffff;
#[derive(Debug)]
pub struct Gen4Str {
    vec: Vec<u16>,
}


impl Gen4Str {
    pub fn new(data: Vec<u16>) -> Self {
        Self {
            vec: data
        }
    }
}

impl TryFrom<&String> for Gen4Str {
    type Error = Utf16Grapheme;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let graphemes = utf16::str_to_utf16_graphemes(value);
        let pokestr: Vec<Option<u16>> = graphemes.iter().map(to_geniv_char).collect();
        let invalid_grapheme = pokestr.iter().enumerate().find(|(_, g)| g.is_none());
        if let Some((i, _)) = invalid_grapheme {
            Err(graphemes[i])
        } else {
            Ok(Gen4Str {
                vec: pokestr.iter().map(|i| i.unwrap()).collect()
            })
        }
    }
}

impl TryFrom<&Gen4Str> for String {
    type Error = u16;

    fn try_from(value: &Gen4Str) -> Result<Self, Self::Error> {
        let utf16str: Vec<Option<&'static Utf16Grapheme>> = value.vec.iter().map(|c| to_utf16(*c)).collect();
        let invalid_grapheme = utf16str.iter().enumerate().find(|(_, g)| g.is_none());
        if let Some((i, _)) = invalid_grapheme {
            Err(value.vec[i])
        } else {
            Ok(String::from_utf16(&*utf16str.iter().flat_map(|go|
            {
                let &g = go.unwrap();
                match g {
                    Utf16Grapheme::Bmp(bmp) => vec![bmp],
                    Utf16Grapheme::Comp(c0, c1) => vec![c0, c1]
                }
            }
            ).collect::<Vec<u16>>()).expect("Invalid UTF16 character, check the character mapping and recompile"))
        }
    }
}

const HARD_CODED_MAPPINGS: [(u16, Utf16Grapheme); 1] = [(0xe000, Utf16Grapheme::Bmp(0x0a))];

/// Look up the corresponding UTF16 grapheme to a pokémon gen iv character.
/// Returns [None] when the character map does not contain such a character.
fn to_utf16(geniv_char: u16) -> Option<&'static Utf16Grapheme> {
    for m in &HARD_CODED_MAPPINGS {
        if m.0 == geniv_char {
            return Some(&m.1);
        }
    }
    CHARACTER_MAP_BY_GENIV.get(usize::from(geniv_char))
}

/// Look up the corresponding pokémon gen iv character to a UTF16 grapheme.
/// Returns [None] when the character map does not contain such a character.
fn to_geniv_char(grapheme: &Utf16Grapheme) -> Option<u16> {
    for m in &HARD_CODED_MAPPINGS {
        if m.1 == *grapheme {
            return Some(m.0);
        }
    }
    CHARACTER_MAP_BY_UTF16.binary_search_by(|(u, _)| u.cmp(grapheme)).map(|i| CHARACTER_MAP_BY_UTF16[i].1).ok()
}