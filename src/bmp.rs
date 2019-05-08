use crate::image::{Image, RGBA_BYTES};
use std::convert::TryFrom;
use std::io;
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

fn i32_le(data: &[u8]) -> i32 {
    (data[0] as i32) | ((data[1] as i32) << 8) | ((data[2] as i32) << 16) | ((data[3] as i32) << 24)
}

fn write_i32_le<W: io::Write>(writer: &mut W, num: i32) -> io::Result<()> {
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
    ///
    /// This can be negative to signal that y goes down the image.
    height: i32,
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

#[derive(Clone, Copy)]
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

impl From<CompressionType> for u32 {
    fn from(compression: CompressionType) -> Self {
        match compression {
            CompressionType::Uncompressed => 0,
            CompressionType::RLE8 => 1,
            CompressionType::RLE4 => 2,
            CompressionType::Bitfields => 3,
            CompressionType::Unknown => 69,
        }
    }
}

/// This holds the color masks representing a given color format
///
/// The BMP format uses these color masks to represent different color
/// formats.
#[derive(PartialEq)]
struct ColorMasks {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

/// This contains the color formats that we can handle.
#[derive(Clone, Copy)]
enum ColorFormat {
    RGBA,
}

impl TryFrom<ColorMasks> for ColorFormat {
    type Error = BMPError;

    fn try_from(mask: ColorMasks) -> Result<Self, Self::Error> {
        let formats = [ColorFormat::RGBA];
        for &f in &formats {
            if mask == f.into() {
                return Ok(f);
            }
        }
        unsupported_format("Unknown color format")
    }
}

impl From<ColorFormat> for ColorMasks {
    fn from(format: ColorFormat) -> Self {
        match format {
            ColorFormat::RGBA => ColorMasks {
                r: 0xFF_00_00_00,
                g: 0x00_FF_00_00,
                b: 0x00_00_FF_00,
                a: 0x00_00_00_FF,
            },
        }
    }
}

/// This holds all the header information for a bitmap image
struct Header {
    file_header: FileHeader,
    image_header: ImageHeader,
    format: ColorFormat,
}

// This assumes we're parsing the header from the start of the slice
fn parse_file_header(data: &[u8]) -> BMPResult<FileHeader> {
    if data.len() < 14 {
        return invalid_format("insufficient file header length");
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

// This assumes we're parsing the header from the start of the slice
fn parse_image_header(data: &[u8]) -> BMPResult<ImageHeader> {
    if data.len() < 44 {
        return invalid_format("insufficient image header length")
    }
    let size = u32_le(data);
    let width = u32_le(&data[4..]);
    let height = u32_le(&data[8..]) as i32;
    unimplemented!()
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
    write_i32_le(writer, header.height)?;
    writer.write_all(&[1, 0])?;
    write_u16_le(writer, header.bit_count)?;
    write_u32_le(writer, header.compression.into())?;
    write_u32_le(writer, header.image_bytes)?;
    write_u32_le(writer, header.x_pixels_per_meter)?;
    write_u32_le(writer, header.y_pixels_per_meter)?;
    write_u32_le(writer, header.color_used)?;
    write_u32_le(writer, header.color_important)
}

fn write_format<W: io::Write>(writer: &mut W, format: ColorFormat) -> io::Result<()> {
    let mask = ColorMasks::from(format);
    write_u32_le(writer, mask.r)?;
    write_u32_le(writer, mask.g)?;
    write_u32_le(writer, mask.b)?;
    write_u32_le(writer, mask.a)?;
    write_u32_le(writer, 0x57696E20)?;
    writer.write_all(&[0; 48])
}

pub fn write_image<W: io::Write>(writer: &mut W, image: &Image) -> io::Result<()> {
    let pixel_count = image.width * image.height;
    let file_header = FileHeader {
        size: 122 + (RGBA_BYTES as u32 * pixel_count),
        offset: 122,
    };
    let image_header = ImageHeader {
        size: 108,
        width: image.width as u32,
        height: -(image.height as i32),
        bit_count: 32,
        compression: CompressionType::Bitfields,
        image_bytes: (image.width * image.height * 4) as u32,
        x_pixels_per_meter: 2835,
        y_pixels_per_meter: 2835,
        color_used: 0,
        color_important: 0,
    };
    write_file_header(writer, &file_header)?;
    write_image_header(writer, &image_header)?;
    write_format(writer, ColorFormat::RGBA)?;
    for pixel in image {
        writer.write_all(&[pixel.a, pixel.b, pixel.g, pixel.r])?;
    }
    Ok(())
}
