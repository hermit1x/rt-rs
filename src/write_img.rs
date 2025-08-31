use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use image::codecs::jpeg::JpegEncoder;
use image::ColorType;
pub fn write_jpg<P: AsRef<Path>>(
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