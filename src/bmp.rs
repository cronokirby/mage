use std::io;
use crate::image::{Image, RGBA_BYTES};
// The structures and parsing in this module are mainly based off of the
// following: http://www.dragonwins.com/domains/GetTechEd/bmp/bmpfileformat.htm

/// Parse a little endian integer from a slice of bytes
///
/// This function doesn't check size at all, so this should be done
/// before calling it.
fn u32_le(data: &[u8]) -> u32 {
    (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24)
}

fn write_u32_le<W: io::Write>(writer: &mut W, num: u32) -> io::Result<()> {
    let mut buf = [0; 4];
    buf[0] = num as u8;
    buf[1] = (num >> 8) as u8;
    buf[2] = (num >> 16) as u8;
    buf[3] = (num >> 24) as u8;
    writer.write_all(&buf)
}

fn write_u16_le<W: io::Write>(writer: &mut W, num: u16) -> io::Result<()> {
    writer.write_all(&[num as u8, (num >> 8) as u8])
}


/// Represents the errors we can encounter when reading a bmp file
pub enum BMPError {
    /// The format of the file doesn't match the specification
    InvalidFormat(String),
    /// The format of the file is valid, but we don't support it
    ///
    /// This is necessary because we don't support esoteric formats
    /// like 8 bit or 24 bit pixels, even though they aren't invalid.
    UnsupportedFormat(String),
}

pub type BMPResult<T> = Result<T, BMPError>;

fn invalid_format<T, S: Into<String>>(s: S) -> BMPResult<T> {
    Err(BMPError::InvalidFormat(s.into()))
}

fn unsupported_format<T, S: Into<String>>(s: S) -> BMPResult<T> {
    Err(BMPError::UnsupportedFormat(s.into()))
}

/// This contains the data in the file header for BMP
struct FileHeader {
    /// How big this file is (including this header)
    size: u32,
    /// At what index does the pixel data start (including this header)
    offset: u32,
}

/// This contains information about the image
struct ImageHeader {
    /// How large this header is
    size: u32,
    /// How wide this image is, i.e. how many pixels are in a scanline
    width: u32,
    /// How high this image is
    height: u32,
    /// How many bits are assigned to each pixel
    bit_count: u16,
    /// What type of compression is used
    compression: CompressionType,
    /// How many bytes are dedicated to the image data
    image_bytes: u32,
    /// How many x pixels per meter
    x_pixels_per_meter: u32,
    /// How many y pixels per meter
    y_pixels_per_meter: u32,
    /// How many colors are used in the color map
    color_used: u32,
    /// How many colors are important in the color map
    color_important: u32,
}

enum CompressionType {
    /// No compression at all
    Uncompressed,
    /// This is only usable with 4 bit pixels
    RLE4,
    /// This is only usable with 4 bit pixels
    RLE8,
    /// This compression type is requires for 16 or 32 bit pixels
    Bitfields,
    /// We use this to capture any unknown compression type
    Unknown,
}

impl Into<u32> for CompressionType {
    fn into(self) -> u32 {
        match self {
            CompressionType::Uncompressed => 0,
            CompressionType::RLE8 => 1,
            CompressionType::RLE4 => 2,
            CompressionType::Bitfields => 3,
            CompressionType::Unknown => 69
        }
    }
}

/// This contains a mask for each color component of a 32 bit pixel
struct ColorMasks {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

/// This holds all the header information for a bitmap image
struct Header {
    file_header: FileHeader,
    image_header: ImageHeader,
    masks: ColorMasks,
}

// This assumes we're parsing the header from the start of the slice
fn parse_file_header(data: &[u8]) -> BMPResult<FileHeader> {
    if data.len() < 14 {
        return invalid_format("insufficient length");
    }
    if data[0] != 66 || data[1] != 77 {
        return invalid_format("header didn't start with 'BM'");
    }
    let size = u32_le(&data[2..]);
    if data[6] != 0 || data[7] != 0 || data[8] != 0 || data[9] != 0 {
        return invalid_format("reserved bytes not 0");
    }
    let offset = u32_le(&data[10..]);
    Ok(FileHeader { size, offset })
}


fn write_file_header<W: io::Write>(writer: &mut W, header: &FileHeader) -> io::Result<()> {
    writer.write_all(&[66, 77])?;
    write_u32_le(writer, header.size)?;
    writer.write_all(&[0, 0, 0, 0])?;
    write_u32_le(writer, header.offset)
}

fn write_image_header<W: io::Write>(writer: &mut W, header: &ImageHeader) -> io::Result<()> {
    write_u32_le(writer, header.size)?;
    write_u32_le(writer, header.width)?;
    write_u32_le(writer, header.height)?;
    writer.write_all(&[1, 0])?;
    write_u16_le(writer, header.bit_count)?;
    write_u32_le(writer, header.compression.into())?;
    write_u32_le(writer, header.image_bytes)?;
    write_u32_le(writer, header.x_pixels_per_meter)?;
    write_u32_le(writer, header.y_pixels_per_meter)?;
    write_u32_le(writer, header.color_used)?;
    write_u32_le(writer, header.color_important)
}

pub fn write_image<W: io::Write>(writer: &mut W, image: &Image) -> io::Result<()> {
    let pixel_count = image.width * image.height;
    let file_header = FileHeader {
        size: 122 + (RGBA_BYTES * pixel_count) as u32,
        offset: 122
    };
    write_file_header(writer, &file_header)
}