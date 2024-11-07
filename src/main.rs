use std::{process::Command, time::SystemTime};

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

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

    let mut png_headers = Vec::new();

    for i in 0..raw.len() - 8 {
        // If check short circuits if the beggining of the header is not found
        if raw[i] == 0x89 && raw[i + 1] == 0x50 && raw[i..i + 8] == PNG_HEADER {
            println!("found PNG header at offset {}", i);
            png_headers.push(i);
        }
    }

    let end = SystemTime::now().duration_since(start).unwrap_or_default();
    println!("\nfound {} PNG headers", png_headers.len());
    println!("megabytes scraped: {}", raw.len() / (1028 * 1028));
    println!("time: {:#.4}s\n", end.as_secs_f32());

    let start = SystemTime::now();
    let mut extracted_pngs = 0;
    for (which_png, start) in png_headers.iter().enumerate() {
        let mut len = 0;
        let mut chunk_type = String::new();
        let mut offset = *start + 8;

        let parse_chunk = |offset: usize, len: &mut u32, chunk_type: &mut String| {
            *len = u32::from_le_bytes([
                raw[offset + 3],
                raw[offset + 2],
                raw[offset + 1],
                raw[offset],
            ]);
            *chunk_type = format!(
                "{}{}{}{}",
                raw[offset + 4] as char,
                raw[offset + 5] as char,
                raw[offset + 6] as char,
                raw[offset + 7] as char
            );
            // println!("chunk type: {chunk_type}");
            // println!("len: {len}");
        };

        loop {
            parse_chunk(offset, &mut len, &mut chunk_type);
            offset += len as usize + 12;
            if &*chunk_type == "IEND" {
                break;
            }
        }

        println!("png {}", which_png + 1);
        println!("start of png: {start}");
        println!("end of png:   {offset}");
        println!("total len:    {}", offset - start);

        std::fs::write(
            format!("extract/png_{}.png", which_png),
            &raw[*start..offset],
        )
        .expect("could not extract parsed png");
        extracted_pngs += 1;
    }
    let end = SystemTime::now().duration_since(start).unwrap_or_default();

    println!("\nextracted {} PNGs", extracted_pngs);
    println!("time: {:#.4}s\n", end.as_secs_f32());
}
