// The structures and parsing in this module are mainly based off of the
// following: http://www.dragonwins.com/domains/GetTechEd/bmp/bmpfileformat.htm

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
    unimplemented!()
}
