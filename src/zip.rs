use crate::FileScraper;

pub struct ZipScraper;

impl FileScraper for ZipScraper {
    fn extension(&self) -> &'static str {
        "zip"
    }

    fn file_detected(&self, raw: &[u8]) -> bool {
        raw.starts_with(&[0x50, 0x4B, 0x03, 0x04])
            || raw.starts_with(&[0x50, 0x4B, 0x05, 0x06])
            || raw.starts_with(&[0x50, 0x4B, 0x07, 0x08])
    }

    fn file_bytes<'a>(&self, _raw: &'a [u8]) -> Option<&'a [u8]> {
        unimplemented!()
    }
}
