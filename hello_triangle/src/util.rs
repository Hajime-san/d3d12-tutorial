use std::str;

pub fn utf16_to_vec(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}
