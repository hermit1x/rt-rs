use std::fs::File;
use std::io::{BufWriter, Write};

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

    match write_ppm("output/rgb_test.ppm", width, height, &buffer) {
        Ok(()) => println!("Wrote output.ppm ({}x{} pixels)", width, height),
        Err(e) => eprintln!("Failed to write output.ppm: {}", e),
    }
}
