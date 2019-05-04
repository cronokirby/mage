/// Represents a Color in RGBA format
/// 
/// Each component ranges from 0 to 255, with 0 representing no color
/// at all in that component, and 255 representing the most color possible.
/// With the alpha component, however, 0 represents complete transparency,
/// and 255 represents
#[derive(Clone, Copy, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl RGBA {
    /// Construct a new RGBA pixel from the components
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA { r, g, b, a}
    }
}

const RGBA_BYTES: usize = 4;


pub struct Image {
    // The raw data stored with 4 bytes per color.
    //
    // We use this raw representation instead of storing RGBA, because
    // it's the preferred format for passing to renderers like SDL,
    // which is one of the more common uses of this type.
    data: Vec<u8>,
    // How many pixels are in a row of the image
    width: usize,
    // How many rows of pixels there are
    height: usize
}

impl Image {
    /// Construct a new image of certain dimensions
    /// 
    /// The image will be completely filled with black, transparent pixels.
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![0; RGBA_BYTES * width * height];
        Image { data, width, height }
    }

    /// Check whether or not x and y are in the bounds of this image
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Read a pixel at a specific spot in the image
    /// 
    /// This function doesn't check whether or not the pixel is in the
    /// bounds of the image.
    pub fn read(&self, x: usize, y: usize) -> RGBA {
        let i = self.width * y + x;
        let r = self.data[i];
        let g = self.data[i + 1];
        let b = self.data[i + 2];
        let a = self.data[i + 3];
        RGBA { r, g, b, a }
    }

    /// Write a pixel at a specific spot in the iamge
    /// 
    /// This function doesn't check whether or not the pixel is in the bounds
    /// of the image.
    pub fn write(&mut self, x: usize, y: usize, pixel: RGBA) {
        let i = self.width * y + x;
        self.data[i] = pixel.r;
        self.data[i + 1] = pixel.g;
        self.data[i + 2] = pixel.b;
        self.data[i + 3] = pixel.a;
    }
}