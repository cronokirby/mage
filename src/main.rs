extern crate structopt;
use structopt::StructOpt;

mod cli;


fn main() {
    let opt = cli::Opt::from_args();
    opt.dispatch();
}
