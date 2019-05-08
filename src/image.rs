use crate::sdl2::render::{Texture, UpdateTextureError};

/// Represents a Color in RGBA format
///
/// Each component ranges from 0 to 255, with 0 representing no color
/// at all in that component, and 255 representing the most color possible.
/// With the alpha component, however, 0 represents complete transparency,
/// and 255 represents
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    /// Construct a new RGBA pixel from the components
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA { r, g, b, a }
    }
}

pub const RGBA_BYTES: usize = 4;

pub struct Image {
    // The raw data stored with 4 bytes per color.
    //
    // We use this raw representation instead of storing RGBA, because
    // it's the preferred format for passing to renderers like SDL,
    // which is one of the more common uses of this type.
    data: Vec<u8>,
    // How many pixels are in a row
    row_width: usize,
    /// How many pixels are in a row of the image
    pub width: u32,
    /// How many rows of pixels there are
    pub height: u32,
}

impl Image {
    /// Construct a new image of certain dimensions
    ///
    /// The image will be completely filled with black, transparent pixels.
    pub fn new(width: u32, height: u32) -> Image {
        let row_width = width as usize;
        let row_height = height as usize;
        let data = vec![0; RGBA_BYTES * row_width * row_height];
        Image {
            data,
            row_width,
            width,
            height,
        }
    }

    /// Check whether or not x and y are in the bounds of this image
    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// Read a pixel at a specific spot in the image
    ///
    /// This function doesn't check whether or not the pixel is in the
    /// bounds of the image.
    pub fn read(&self, x: u32, y: u32) -> RGBA {
        let i = RGBA_BYTES * (self.row_width * (y as usize) + (x as usize));
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
    pub fn write(&mut self, x: u32, y: u32, pixel: RGBA) {
        let i = RGBA_BYTES * (self.row_width * (y as usize) + (x as usize));
        self.data[i] = pixel.r;
        self.data[i + 1] = pixel.g;
        self.data[i + 2] = pixel.b;
        self.data[i + 3] = pixel.a;
    }

    /// Fill a texture with the pixels in this image
    pub fn fill(&self, texture: &mut Texture) -> Result<(), UpdateTextureError> {
        texture.update(None, &self.data, RGBA_BYTES * self.row_width)
    }
}

/// Represents an iterator over the pixels of an image
pub struct ImageIterator<'a> {
    image: &'a Image,
    index: usize,
}

impl<'a> ImageIterator<'a> {
    fn new(image: &'a Image) -> Self {
        ImageIterator { image, index: 0 }
    }
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = RGBA;

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = self.index + 4;
        if next_index > self.image.data.len() {
            return None;
        }
        let r = self.image.data[self.index];
        self.index += 1;
        let g = self.image.data[self.index];
        self.index += 1;
        let b = self.image.data[self.index];
        self.index += 1;
        let a = self.image.data[self.index];
        self.index += 1;
        Some(RGBA { r, g, b, a })
    }
}

impl<'a> IntoIterator for &'a Image {
    type Item = RGBA;
    type IntoIter = ImageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ImageIterator::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image() {
        let mut image = Image::new(4, 4);
        let red = RGBA::new(0xFF, 0, 0, 0xFF);
        image.write(0, 0, red);
        image.write(1, 0, red);
        assert_eq!(image.read(0, 0), red);
        assert_eq!(image.read(1, 0), red);
    }
}
