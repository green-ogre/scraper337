use crate::FileScraper;

const JPG_HEADER_1: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xDB];
const JPG_HEADER_2: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xEE];
const JPG_HEADER_3: [u8; 12] = [
    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
];

pub struct JpgScraper;

impl FileScraper for JpgScraper {
    fn extension(&self) -> &'static str {
        "jpeg"
    }

    fn file_detected(&self, raw: &[u8]) -> bool {
        JPG_HEADER_1 == raw[..JPG_HEADER_1.len()]
            || JPG_HEADER_2 == raw[..JPG_HEADER_2.len()]
            || JPG_HEADER_3 == raw[..JPG_HEADER_3.len()]
            || (raw[..4] == [0xFF, 0xD8, 0xFF, 0xE1]
                && raw[6..12] == [0x45, 0x78, 0x69, 0x66, 0x00, 0x00])
    }

    fn file_bytes<'a>(&self, raw: &'a [u8]) -> Option<&'a [u8]> {
        for i in 0..raw.len() - 8 {
            if raw[i] == 0xFF && raw[i + 1] == 0xD9 {
                return Some(&raw[..i]);
            }
        }

        None
    }
}
