use std::{process::Command, time::SystemTime};

use jpg::JpgScraper;
use png::PngScraper;

mod jpg;
mod png;

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

fn main() {
    Command::new("rm")
        .arg("-rf")
        .arg("extract")
        .output()
        .unwrap();
    Command::new("mkdir")
        .arg("-p")
        .arg("extract")
        .output()
        .unwrap();

    let start = SystemTime::now();

    let raw = std::fs::read("/dev/sdc1").unwrap();
    let file_scrapers: Vec<Box<dyn FileScraper>> = vec![Box::new(JpgScraper), Box::new(PngScraper)];

    for i in 0..raw.len() - 12 {
        for scraper in file_scrapers.iter() {
            if scraper.file_detected(&raw[i..]) {
                let extension = scraper.extension();
                println!("found {} header at offset {}", extension, i);
                Command::new("mkdir")
                    .arg("-p")
                    .arg(format!("extract/{}", extension))
                    .output()
                    .unwrap();

                let name = format!("extract/{}/{}.{}", extension, i, extension);
                if let Some(bytes) = scraper.file_bytes(&raw[i..]) {
                    std::fs::write(&name, bytes)
                        .unwrap_or_else(|_| panic!("could not write file to {}", name));
                    if scraper.requires_validation()
                        && image::ImageReader::open(&name).unwrap().decode().is_err()
                    {
                        println!("invalid {} generated, deleting...", extension);
                        std::fs::remove_file(&name).unwrap();
                    }
                } else {
                    println!("could not retrieve file bytes for {}", &name);
                }
            }
        }
    }

    let end = SystemTime::now().duration_since(start).unwrap_or_default();
    println!("\nmegabytes scraped: {}", raw.len() / (1028 * 1028));
    println!("time: {:#.4}s\n", end.as_secs_f32());
}
