use crate::file_scraper::{FileScraper, FileScraperReport};
use clap::Parser;
use std::{
    collections::HashMap, fs::File, io::Read, os::fd::AsRawFd, process::Command, time::SystemTime,
};

/// Drive Data Scraper for ECE 337
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    drive_path: String,

    /// Size of a chunk in megabytes
    #[arg(short, long, default_value_t = 512)]
    chunk_size: usize,
}

#[derive(Default)]
pub struct Scraper {
    scrapers: Vec<Box<dyn FileScraper>>,
    reports: HashMap<&'static str, FileScraperReport>,
}

impl Scraper {
    pub fn register_scrapers(&mut self, scrapers: Vec<Box<dyn FileScraper>>) -> &mut Self {
        for scraper in scrapers.into_iter() {
            self.reports
                .insert(scraper.extension(), FileScraperReport::default());
            self.scrapers.push(scraper);
        }

        self
    }

    pub fn run(&mut self) {
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

        let args = Args::parse();

        // 512M
        let chunk_size = 1024 * 1024 * args.chunk_size;
        let mut raw = vec![0; chunk_size];
        let mut file = std::fs::File::open(&args.drive_path).unwrap();
        let total_size = get_drive_size(&file).unwrap_or_default() as usize;
        let total_chunks = total_size / chunk_size;

        let mut total_megabytes_scraped = 0.;
        let mut total_invalid_files = 0;
        let mut total_valid_files = 0;

        let mut chunk = 0;
        loop {
            // raw.clear();
            if file.read_exact(&mut raw).is_err() {
                break;
            }

            // println!("\nchunk entry: {:?}", &raw[0..16]);

            for i in 0..raw.len() - 12 {
                for scraper in self.scrapers.iter() {
                    if scraper.file_detected(&raw[i..]) {
                        let extension = scraper.extension();
                        // println!("found {} header at offset {}", extension, i);
                        let name = format!("extract/{}/{}.{}", extension, i, extension);
                        if let Some(bytes) = scraper.file_bytes(&raw[i..]) {
                            Command::new("mkdir")
                                .arg("-p")
                                .arg(format!("extract/{}", extension))
                                .output()
                                .unwrap();

                            std::fs::write(&name, bytes)
                                .unwrap_or_else(|_| panic!("could not write file to {}", name));
                            if scraper.requires_validation()
                                && image::ImageReader::open(&name).unwrap().decode().is_err()
                            {
                                self.reports.get_mut(&extension).unwrap().invalid_files += 1;
                                total_invalid_files += 1;
                                // println!("invalid {} generated, deleting...", extension);
                                std::fs::remove_file(&name).unwrap();
                            } else {
                                self.reports.get_mut(&extension).unwrap().valid_files += 1;
                                total_valid_files += 1;
                            }
                        } else {
                            // println!("could not retrieve file bytes for {}", &name);
                        }
                    }
                }
            }

            let end = SystemTime::now().duration_since(start).unwrap_or_default();
            let megabytes_scraped = raw.len() as f32 / (1028. * 1028.);
            total_megabytes_scraped += megabytes_scraped;

            println!("\n------------------- Chunk {chunk}/{total_chunks} -------------------");
            println!("chunk size:\t\t\t{megabytes_scraped:.2}");
            println!("total megabytes scraped:\t{total_megabytes_scraped:.2}");
            println!("total invalid files:\t\t{total_invalid_files}");
            println!("total valid files:\t\t{total_valid_files}");
            println!("total time:\t\t\t{:#.2}s", end.as_secs_f32());
            println!("Scraper reports:");
            for (ext, report) in self.reports.iter() {
                println!("\t{ext}\t::\t{report:?}");
            }

            chunk += 1;
        }
    }
}

fn get_drive_size(file: &File) -> Result<u64, std::io::Error> {
    const BLKGETSIZE64: u64 = 0x80081272;

    let fd = file.as_raw_fd();
    let mut size: u64 = 0;

    unsafe {
        if libc::ioctl(fd, BLKGETSIZE64 as _, &mut size as *mut u64) == -1 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(size)
}
