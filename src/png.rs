use crate::FileScraper;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const PNG_CHUNK_TYPES: [&str; 19] = [
    "IHDR", "PLTE", "IDAT", "IEND", "tRNS", "cHRM", "gAMA", "iCCP", "sBIT", "sRGB", "tEXt", "iTXt",
    "zTXt", "bKGD", "hIST", "pHYs", "sPLT", "tIME", "eXIf",
];

pub struct PngScraper;

impl FileScraper for PngScraper {
    fn extension(&self) -> &'static str {
        "png"
    }

    fn file_detected(&self, raw: &[u8]) -> bool {
        raw[0] == 0x89 && raw[1] == 0x50 && raw[..8] == PNG_HEADER
    }

    fn file_bytes<'a>(&self, raw: &'a [u8]) -> Option<&'a [u8]> {
        let mut offset = 8;

        loop {
            let len = u32::from_le_bytes([
                raw[offset + 3],
                raw[offset + 2],
                raw[offset + 1],
                raw[offset],
            ]);
            let chunk_type = String::from_utf8(vec![
                raw[offset + 4],
                raw[offset + 5],
                raw[offset + 6],
                raw[offset + 7],
            ])
            .unwrap();

            if !PNG_CHUNK_TYPES.contains(&&*chunk_type) {
                println!("unknown chunk type: {}", chunk_type);
                return None;
            }

            if &*chunk_type == "IEND" {
                break;
            }

            offset += len as usize + 12;
        }

        Some(&raw[..offset])
    }

    fn requires_validation(&self) -> bool {
        false
    }
}
