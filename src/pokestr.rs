// SPDX-License-Identifier: GPL-3.0-only

use crate::pokestrmap::{CHARACTER_MAP_BY_GENIV, CHARACTER_MAP_BY_UTF16};
use std::string::String;
use utf16::Utf16Grapheme;

pub const STRING_TERMINATOR: u16 = 0xffff;
pub const ESCAPE_CHAR: char = '\\';
pub const ESCAPE_CODEPOINT_CHAR: char = 'x';

#[derive(Debug)]
pub struct Gen4Str {
    pub vec: Vec<u16>,
}


impl TryFrom<&String> for Gen4Str {
    type Error = (usize, Utf16Grapheme);

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let graphemes_escaped = utf16::str_to_utf16_graphemes(value);
        let mut graphemes = Vec::with_capacity(graphemes_escaped.len());

        let esc_graph = Utf16Grapheme::Bmp(ESCAPE_CHAR.encode_utf16(&mut [0, 0])[0]);
        let cod_graph = Utf16Grapheme::Bmp(ESCAPE_CODEPOINT_CHAR.encode_utf16(&mut [0, 0])[0]);

        let mut last_esc = 0;
        let mut escaped: Vec<(usize, u16)> = vec![];

        for i in 0..graphemes_escaped.len() {
            let g = graphemes_escaped[i];
            if g == esc_graph {
                match last_esc {
                    0 => last_esc = 1,
                    1 => {
                        graphemes.push(g);
                        last_esc = 0;
                    }
                    _ => return Err((i, g))
                }
            } else {
                if last_esc > 0 {
                    if last_esc == 1 && g != cod_graph { // only one possibility at the moment
                        return Err((i, g));
                    }

                    if last_esc == 5 { // skipped all 5 characters (including code escape) now collect and parse them

                        let mut digits = [0x0u16; 4];

                        for j in 0..4 {
                            let g = graphemes_escaped[i - j];
                            match g {
                                Utf16Grapheme::Bmp(c) => digits[j] = c,
                                _ => return Err((i - j, g))
                            }
                        }

                        let utf16_str = String::from_utf16(&digits).map_err(|_| (i, g))?;
                        let gen4_code = u16::from_str_radix(&*utf16_str, 16).map_err(|_| (i, g))?;

                        escaped.push((graphemes.len(), gen4_code));

                        graphemes.push(CHARACTER_MAP_BY_UTF16[0].0); // just push the first available grapheme and replace later

                        last_esc = 0;
                    } else { // just continue until 5 characters
                        last_esc += 1;
                    }
                } else { // ordinary character in non-escape mode
                    graphemes.push(g);
                }
            }
        }

        let mut pokestr: Vec<Option<u16>> = graphemes.iter().map(to_geniv_char).collect();

        for (i, c) in escaped {
            pokestr[i] = Some(c);
        }

        let invalid_grapheme = pokestr.iter().enumerate().find(|(_, g)| g.is_none());
        if let Some((i, _)) = invalid_grapheme {
            Err((i, graphemes[i]))
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
                    Err(c) => { format!("{}{:04x}", ESCAPE_CODEPOINT_CHAR, c).encode_utf16().collect() }
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

        let out_str = format!("H{}{:04x}{}{:04x}ell{}{:04x}o!", ESCAPE_CODEPOINT_CHAR, unknown_char0, ESCAPE_CODEPOINT_CHAR, unknown_char1, ESCAPE_CODEPOINT_CHAR, unknown_char1);

        assert_eq!(parsed, Err(DecodeError {
            escaped: out_str,
            idx: 1,
            char: unknown_char0,
        }));
    }
}