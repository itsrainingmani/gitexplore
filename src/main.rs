use std::env;
use std::process;
use structopt::StructOpt;

use git_explore::{Data, Cli};

fn main() {
  let cli = Cli::from_args();
  let d = Data::construct();

  println!("{:?}", cli.search_terms);
  println!("{:?}", d.primary);
}
