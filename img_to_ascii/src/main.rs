use image::{DynamicImage, GenericImageView, Pixel, io::Reader};
use std::{fs::File, io::Write};

// const ASCII_CHARS: [char; 12] = [' ', '.', ':', ';', '+', '*', '?', '%', 'S', '#', '@', '$'];
// const ASCII_CHARS: [char; 12] = [' ', ' ', '.', ':', ';', '+', '*', '?', '%', 'S', '#', '@'];
// const ASCII_CHARS: [char; 4] = [' ', '░', '▒', '▓'];
const ASCII_CHARS: [char; 12] = ['$', '@', '#', 'S', '%', '?', '*', '+', ';', ':', ' ', ' '];
const FRAMES_CNT: u32 = 6571;
const OUTPUT_PATH: &str = "F:\\Develop\\rust\\bad_apple\\resources\\bad_apple_frames.txt";

pub fn img_to_ascii(img: &DynamicImage, width: u32, height: u32) -> String {
    let img = img.resize_exact(width, height, image::imageops::FilterType::Nearest);
    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            // 计算灰度值 (0-255)
            let brightness =
                0.2126 * rgb[0] as f32 + 0.7152 * rgb[1] as f32 + 0.0722 * rgb[2] as f32;
            let index = (brightness / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;
            ascii_art.push(ASCII_CHARS[index]);
        }
        ascii_art.push('\n');
    }

    ascii_art
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = File::create(OUTPUT_PATH)?;
    let mut writer = std::io::BufWriter::new(output);

    for i in 1..=FRAMES_CNT {
        let path = format!("frames/{:04}.png", i);
        let img = Reader::open(&path)?.decode()?;
        let ascii_frame = img_to_ascii(&img, 84, 42);
        writeln!(writer, "---FRAME---\n{}", ascii_frame)?;
    }

    Ok(())
}
