const UTF16_HIGH_SURROGATE_MASK: u16 = 0b1101100000000000;
const UTF16_LOW_SURROGATE_MASK: u16 = 0b1101110000000000;
const UTF16_SURROGATE_REMAINDER: u16 = 0b00000011111111111;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
/// A single UTF16 grapheme
pub enum Utf16Grapheme {
    /// Graphemes which are covered by BMP and therefore can be represented within 16bit.
    Bmp(u16),
    /// Graphemes which require to split them into two u16.
    /// First is the high surrogate, second is the low surreogate.
    Comp(u16, u16),
}

/// Creates a packet for the given beacon frame parameters.
///
/// Split a string to its UTF16 graphemes as [Vec<Utf16Grapheme>].
///
/// # Arguments
///
/// * `str` - The string to split.
///
/// # Returns
///
/// Returns a [Vec<Utf16Grapheme>] containing the graphemes.
///
pub fn str_to_utf16_graphemes(str: &str) -> Vec<Utf16Grapheme> {
    let characters: Vec<u16> = str.encode_utf16().collect();
    let mut graphemes: Vec<Utf16Grapheme> = Vec::with_capacity(characters.len());
    for i in 1..characters.len() {
        if characters[i - 1] ^ UTF16_HIGH_SURROGATE_MASK <= UTF16_SURROGATE_REMAINDER {
            graphemes.push(Utf16Grapheme::Comp(characters[i - 1], characters[i]));
        } else {
            graphemes.push(Utf16Grapheme::Bmp(characters[i - 1]));
        }
    }
    if let Some(&c) = characters.last() {
        if c ^ UTF16_LOW_SURROGATE_MASK > UTF16_SURROGATE_REMAINDER {
            graphemes.push(Utf16Grapheme::Bmp(c));
        }
    }
    graphemes
}