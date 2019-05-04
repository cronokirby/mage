use crate::structopt::StructOpt;
use crate::display::display;
use crate::image::{Image, RGBA};


#[derive(Debug, StructOpt)]
#[structopt(name = "mage")]
pub enum Opt {
    #[structopt(name = "show")]
    /// Show the image in a file
    Show {
        /// The input file to show
        input: String
    },
    #[structopt(name = "convert")]
    /// Convert an image from one format to another
    Convert {
        /// The image file to convert
        input: String,
        #[structopt(short = "o")]
        /// The output file for the image
        output: String
    }
}

impl Opt {
    /// Handle all cases of the command line options, running
    /// the right sub-programs
    pub fn dispatch(self) {
        match self {
            Opt::Show{..} => show(),
            Opt::Convert{..} => println!("convert")
        }
    }
}

fn show() {
    let mut image = Image::new(255, 200);
    for x in 0..255 {
        for y in 0..200 {
            image.write(x, y, RGBA::new(0xFF, x as u8, y as u8, 0xFF));
        }
    }
    display(image);
}
