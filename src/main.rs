use std::env;
use std::process;
use structopt::StructOpt;

use git_explore::{Data, Cli};

fn main() {
  let cli = Cli::from_args();
  println!("{:?}", cli.search_terms);
  
  let d = Data::construct();
  println!("{:?}", d.primary);
}
