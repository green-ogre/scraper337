pub trait FileScraper {
    /// File type extension.
    fn extension(&self) -> &'static str;

    /// Is a file detectable at the begining of the `raw` slice.
    ///
    /// https://en.wikipedia.org/wiki/List_of_file_signatures
    fn file_detected(&self, raw: &[u8]) -> bool;

    /// Returns the entire file as a byte slice if the data is uncorrupted.
    ///
    /// This does NOT guarantee that a file is valid. If it is impractical to verify a file's
    /// validity, then specify this is [`FileScraper::requires_validation`]
    fn file_bytes<'a>(&self, raw: &'a [u8]) -> Option<&'a [u8]>;

    fn requires_validation(&self) -> bool {
        true
    }
}

#[derive(Debug, Default)]
pub struct FileScraperReport {
    pub valid_files: usize,
    pub invalid_files: usize,
}

macro_rules! file_stub {
    ($ty:ident, $ext:expr, $starts_with:tt, $extra:expr) => {
        pub struct $ty;

        impl FileScraper for $ty {
            fn extension(&self) -> &'static str {
                $ext
            }

            fn file_detected(&self, raw: &[u8]) -> bool {
                (0..$starts_with.len())
                    .map(|i| raw.starts_with(&$starts_with[i]))
                    .any(|b| b)
                    && $extra(raw)
            }

            fn file_bytes<'a>(&self, _raw: &'a [u8]) -> Option<&'a [u8]> {
                // This never fires :(
                // if let Ok(_) = image::ImageReader::new(Cursor::new(raw)).decode() {
                //     panic!("handle this case!");
                // }

                None
            }

            fn requires_validation(&self) -> bool {
                false
            }
        }
    };
}

file_stub!(
    WavScraper,
    "wav",
    [[0x52, 0x49, 0x46, 0x46]],
    |raw: &[u8]| {
        if let Ok(str) = String::from_utf8(raw[8..12].to_vec()) {
            str == "WAVE"
        } else {
            false
        }
    }
);
file_stub!(
    AiffScraper,
    "aiff",
    [[0x46, 0x4F, 0x52, 0x4D,]],
    |raw: &[u8]| {
        if let Ok(str) = String::from_utf8(raw[8..12].to_vec()) {
            str == "AIFF"
        } else {
            false
        }
    }
);
file_stub!(PdfScraper, "pdf", [[0x25, 0x50, 0x44, 0x46, 0x2D,]], |_| {
    true
});
file_stub!(MidiScraper, "midi", [[0x4D, 0x54, 0x68, 0x64,]], |_| {
    true
});
file_stub!(
    RtfScraper,
    "rtf",
    [[0x7B, 0x5C, 0x72, 0x74, 0x66, 0x31,]],
    |_| { true }
);
file_stub!(
    Mpeg4Scraper,
    "mp4",
    [[0x66, 0x74, 0x79, 0x70, 0x4D, 0x53, 0x4E, 0x56,]],
    |_| { true }
);

file_stub!(
    X509CertScraper,
    "crt",
    [[
        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x43, 0x45, 0x52, 0x54,
        0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x45, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D,
    ]],
    |_| { true }
);
