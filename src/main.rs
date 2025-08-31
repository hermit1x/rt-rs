mod ray;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use image::{codecs::jpeg::JpegEncoder, ColorType};
use ray::*;

type Color = nalgebra::Vector3<f64>;

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

fn write_color(pixel_buffer: &mut [u8], color: Color) -> () {
    pixel_buffer[0] = (color[0] * 255.999) as u8;
    pixel_buffer[1] = (color[1] * 255.999) as u8;
    pixel_buffer[2] = (color[2] * 255.999) as u8;
}


fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = ((width as f64) / aspect_ratio) as usize;
    let height: usize = height.max(1);

    // Camera
    let focal_length = 1.0f64;
    let viewport_height = 2.0f64;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = nalgebra::Vector3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = nalgebra::Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = nalgebra::Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (width as f64);
    let pixel_delta_v = viewport_v / (height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - nalgebra::Vector3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    

    // Create an array (Vec) of size width * height * 3 (e.g., RGB buffer)
    let size = width
        .checked_mul(height)
        .and_then(|px| px.checked_mul(3))
        .expect("width*height*3 overflowed");
    let mut buffer: Vec<u8> = vec![0u8; size];

    // Fill buffer per the C++ gradient: r = i/(w-1), g = j/(h-1), b = 0
    for j in 0..height {
        for i in 0..width {
            let pixel_center = pixel00_loc + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let p = (j * width + i) * 3;
            write_color(
                &mut buffer[p..p+3],
                ray_color(&ray)
            );
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
