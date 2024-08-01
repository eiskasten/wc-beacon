// SPDX-License-Identifier: GPL-3.0-only

use crate::pokestrmap::{CHARACTER_MAP_BY_GENIV, CHARACTER_MAP_BY_UTF16};
use std::string::String;
use utf16::Utf16Grapheme;

pub const STRING_TERMINATOR: u16 = 0xffff;
pub const ESCAPE_CHAR: &str = "\\";
pub const ESCAPE_CODEPOINT: &str = "\\x";

#[derive(Debug)]
pub struct Gen4Str {
    pub vec: Vec<u16>,
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

#[derive(Debug, PartialEq)]
pub struct DecodeError {
    pub escaped: String,
    pub idx: usize,
    pub char: u16,
}

impl TryFrom<&Gen4Str> for String {
    type Error = DecodeError;

    fn try_from(value: &Gen4Str) -> Result<Self, Self::Error> {
        let utf16str: Vec<Result<&'static Utf16Grapheme, &u16>> = value.vec.iter().map(|c| if let Some(g) = to_utf16(*c) { Ok(g) } else { Err(c) }).collect();
        let invalid_grapheme = utf16str.iter().enumerate().find(|(_, g)| g.is_err());
        if let Some((i, _)) = invalid_grapheme {
            let escaped = String::from_utf16(&*utf16str.iter().flat_map(|go|
            {
                match go {
                    Ok(go) => {
                        match go {
                            Utf16Grapheme::Bmp(bmp) => vec![*bmp],
                            Utf16Grapheme::Comp(c0, c1) => vec![*c0, *c1]
                        }
                    }
                    Err(c) => { format!("{}{:04x}", ESCAPE_CODEPOINT, c).encode_utf16().collect() }
                }
            }
            ).collect::<Vec<u16>>()).expect("Invalid UTF16 character, check the character mapping and recompile");
            Err(DecodeError { escaped, idx: i, char: value.vec[i] })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen4_deserialize_hello() {
        let hello = Gen4Str { vec: "Hello!".encode_utf16().map(|c| to_geniv_char(&Utf16Grapheme::Bmp(c)).unwrap()).collect() };

        let parsed = String::try_from(&hello);
        assert_eq!(parsed, Ok("Hello!".to_string()));
    }

    #[test]
    fn gen4_deserialize_hello_unknown() {
        let mut hello = Gen4Str { vec: "Hello!".encode_utf16().map(|c| to_geniv_char(&Utf16Grapheme::Bmp(c)).unwrap()).collect() };
        let unknown_char0 = 0x08e0;
        let unknown_char1 = 0xa0a1;

        hello.vec.insert(1, unknown_char0);
        hello.vec.insert(2, unknown_char1);
        hello.vec.insert(6, unknown_char1);


        let parsed = String::try_from(&hello);

        let out_str = format!("H{}{:04x}{}{:04x}ell{}{:04x}o!", ESCAPE_CODEPOINT, unknown_char0, ESCAPE_CODEPOINT, unknown_char1, ESCAPE_CODEPOINT, unknown_char1);

        assert_eq!(parsed, Err(DecodeError {
            escaped: out_str,
            idx: 1,
            char: unknown_char0,
        }));
    }
}