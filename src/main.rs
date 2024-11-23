use file_scraper::FileScraper;
use scraper::Scraper;

pub mod file_scraper;
mod jpg;
mod png;
mod scraper;
mod zip;

// TODO:
// 1. Fix chunking bounds, what if a file needs the data accross a chunk boundary?
// 2. Add more common file types.

fn main() {
    Scraper::default()
        .register_scrapers(vec![
            Box::new(jpg::JpgScraper),
            Box::new(png::PngScraper),
            Box::new(file_scraper::WavScraper),
            Box::new(file_scraper::AiffScraper),
            Box::new(file_scraper::PdfScraper),
            Box::new(file_scraper::MidiScraper),
            Box::new(file_scraper::RtfScraper),
            Box::new(file_scraper::Mpeg4Scraper),
            Box::new(file_scraper::X509CertScraper),
        ])
        .run();
}
