use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use image::{codecs::jpeg::JpegEncoder, ColorType};

fn write_ppm<P: AsRef<std::path::Path>>(path: P, width: usize, height: usize, buffer: &[u8]) -> std::io::Result<()> {
    let expected = width
        .checked_mul(height)
        .and_then(|px| px.checked_mul(3))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "width*height*3 overflow"))?;

    if buffer.len() != expected {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("buffer length {} != expected {}", buffer.len(), expected),
        ));
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P6")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    writer.write_all(buffer)?;
    writer.flush()?;
    Ok(())
}

// New: write the RGB buffer to a JPEG file with configurable quality (1..=100).
fn write_jpg<P: AsRef<Path>>(
    path: P,
    width: usize,
    height: usize,
    buffer: &[u8],
    quality: u8,
) -> std::io::Result<()> {
    // Validate buffer length = width * height * 3 (RGB)
    let expected = width
        .checked_mul(height)
        .and_then(|px| px.checked_mul(3))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "width*height*3 overflow"))?;

    if buffer.len() != expected {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("buffer length {} != expected {}", buffer.len(), expected),
        ));
    }

    // JPEG encoder expects u32 dimensions
    let w = u32::try_from(width)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "width does not fit into u32"))?;
    let h = u32::try_from(height)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "height does not fit into u32"))?;

    // Create file and encoder
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(&mut writer, quality);

    // Encode as RGB8
    encoder
        .encode(buffer, w, h, ColorType::Rgb8.into())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    writer.flush()?;
    Ok(())
}

fn main() {
    let width: usize = 1920;
    let height: usize = 1080;

    // Create an array (Vec) of size width * height * 3 (e.g., RGB buffer)
    let size = width
        .checked_mul(height)
        .and_then(|px| px.checked_mul(3))
        .expect("width*height*3 overflowed");
    let mut buffer: Vec<u8> = vec![0u8; size];

    // Fill buffer per the C++ gradient: r = i/(w-1), g = j/(h-1), b = 0
    for j in 0..height {
        for i in 0..width {
            let p = (j * width + i) * 3;
            let r = if width > 1 { (255.999 * (i as f64) / ((width - 1) as f64)).floor() as u8 } else { 0 };
            let g = if height > 1 { (255.999 * (j as f64) / ((height - 1) as f64)).floor() as u8 } else { 0 };
            let b = 0u8;
            buffer[p] = r;
            buffer[p + 1] = g;
            buffer[p + 2] = b;
        }
    }

    // Print the buffer as (r, g, b) with width pixels per row and height rows
    // for j in 0..height {
    //     for i in 0..width {
    //         let p = (j * width + i) * 3;
    //         let r = buffer[p];
    //         let g = buffer[p + 1];
    //         let b = buffer[p + 2];
    //         if i + 1 == width {
    //             print!("({r}, {g}, {b})");
    //         } else {
    //             print!("({r}, {g}, {b}) ");
    //         }
    //     }
    //     println!();
    // }

    // match write_ppm("output/rgb_test.ppm", width, height, &buffer) {
    //     Ok(()) => println!("Wrote output.ppm ({}x{} pixels)", width, height),
    //     Err(e) => eprintln!("Failed to write output.ppm: {}", e),
    // }

    match write_jpg("output/rgb_test.jpg", width, height, &buffer, 100) {
        Ok(()) => println!("Wrote output.jpg ({}x{} pixels)", width, height),
        Err(e) => eprintln!("Failed to write output.jpg: {}", e),
    }
}
