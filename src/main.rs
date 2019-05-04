extern crate structopt;
use structopt::StructOpt;
extern crate sdl2;

mod cli;
mod display;
mod image;


fn main() {
    let opt = cli::Opt::from_args();
    opt.dispatch();
}
