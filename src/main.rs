extern crate structopt;
use structopt::StructOpt;

mod cli;
mod image;


fn main() {
    let opt = cli::Opt::from_args();
    opt.dispatch();
}
