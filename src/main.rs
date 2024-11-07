use std::time::SystemTime;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

fn main() {
    let start = SystemTime::now();
    let raw = std::fs::read("/dev/sdc1").unwrap();

    let mut num_png_header = 0;

    for i in 0..raw.len() - 8 {
        // If check short circuits if the beggining of the header is not found
        if raw[i] == 0x89 && raw[i + 1] == 0x50 && raw[i..i + 8] == PNG_HEADER {
            println!("found PNG header at offset {}", i);
            num_png_header += 1;
        }
    }

    let end = SystemTime::now().duration_since(start).unwrap_or_default();

    println!("\nfound {} PNG headers", num_png_header);
    println!("megabytes scraped: {}", raw.len() / (1028 * 1028));
    println!("time: {}", end.as_secs_f32());
}
