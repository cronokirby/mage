use std::io;

extern crate structopt;
use structopt::StructOpt;
extern crate sdl2;

mod bmp;
mod cli;
mod display;
mod image;

fn main() -> io::Result<()> {
    let opt = cli::Opt::from_args();
    opt.dispatch()
}
