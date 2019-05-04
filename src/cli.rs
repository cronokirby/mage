use crate::structopt::StructOpt;


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
        println!("{:?}", self);
    }
}