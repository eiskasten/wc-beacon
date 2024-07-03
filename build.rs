use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use utf16::{str_to_utf16_graphemes, Utf16Grapheme};

const MAP_PATH: &str = "gen-iv-character-map.txt";
const CHARACTERS: u16 = 0x1fe;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", MAP_PATH);

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("pokestrmap.rs");

    let reader = BufReader::new(File::open(MAP_PATH).expect("Character map for generation iv"));
    let mapping: HashMap<u16, Utf16Grapheme> = reader.lines().filter_map(|r| {
        if let Ok(s) = r { if s.starts_with("0x") { Some(s) } else { None } } else { None }
    }).map(|s| (u16::from_str_radix(&s[2..6], 16).expect(&format!("Invalid number: 0x{}", &s[2..6])), str_to_utf16_graphemes(&s[7..]).remove(0))).collect();

    let mut writer = LineWriter::new(File::create(path).unwrap());
    write!(writer, "use utf16::Utf16Grapheme; pub const CHARACTER_MAP_BY_GENIV: [Utf16Grapheme; {}] = [\n", CHARACTERS).unwrap();
    for i in 0..CHARACTERS {
        let g = mapping.get(&i).unwrap_or(&Utf16Grapheme::Bmp(0));
        if let &Utf16Grapheme::Bmp(bmp) = g {
            write!(writer, "Utf16Grapheme::Bmp({}),\n", bmp).unwrap();
        }
        if let &Utf16Grapheme::Comp(c0, c1) = g {
            write!(writer, "Utf16Grapheme::Comp({},{}),\n", c0, c1).unwrap();
        }
    }
    write!(writer, "];").unwrap();
    write!(writer, "pub const CHARACTER_MAP_BY_UTF16: [(Utf16Grapheme, u16); {}] = [\n", mapping.len()).unwrap();
    let mut mapping_vec: Vec<(&u16, &Utf16Grapheme)> = mapping.iter().collect();
    mapping_vec.sort_by(|(_, a), (_, b)| { a.cmp(b) });
    for (pc, uc) in mapping_vec {
        if let &Utf16Grapheme::Bmp(bmp) = uc {
            write!(writer, "(Utf16Grapheme::Bmp({}),{}),\n", bmp, pc).unwrap();
        }
        if let &Utf16Grapheme::Comp(c0, c1) = uc {
            write!(writer, "(Utf16Grapheme::Comp({},{}),{}),\n", c0, c1, pc).unwrap();
        }
    }
    write!(writer, "];").unwrap();
}
