use crate::bmp;
use crate::display::display;
use crate::image::{Image, RGBA};
use crate::structopt::StructOpt;
use std::fs::File;
use std::io;

#[derive(Debug, StructOpt)]
#[structopt(name = "mage")]
pub enum Opt {
    #[structopt(name = "show")]
    /// Show the image in a file
    Show {
        /// The input file to show
        input: String,
    },
    #[structopt(name = "convert")]
    /// Convert an image from one format to another
    Convert {
        /// The image file to convert
        input: String,
        #[structopt(short = "o")]
        /// The output file for the image
        output: String,
    },
}

impl Opt {
    /// Handle all cases of the command line options, running
    /// the right sub-programs
    pub fn dispatch(self) -> io::Result<()> {
        match self {
            Opt::Show { .. } => show(),
            Opt::Convert { .. } => {
                let image = make_image();
                let file = File::create("foo.bmp")?;
                let mut writer = io::BufWriter::new(file);
                bmp::write_image(&mut writer, &image)
            }
        }
    }
}

fn show() -> io::Result<()> {
    let image = make_image();
    display(image);
    Ok(())
}

fn make_image() -> Image {
    let mut image = Image::new(255, 200);
    for x in 0..255 {
        for y in 0..200 {
            image.write(x, y, RGBA::new(0xFF, x as u8, y as u8, 0xFF));
        }
    }
    image
}
