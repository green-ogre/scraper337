use file_scraper::FileScraper;
use jpg::JpgScraper;
use png::PngScraper;
use scraper::Scraper;

pub mod file_scraper;
mod jpg;
mod png;
mod scraper;
mod zip;

fn main() {
    Scraper::default()
        .register_scrapers(vec![
            Box::new(JpgScraper),
            Box::new(PngScraper),
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
