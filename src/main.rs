use ctrlc;
use exitcode;
use std::process;
use structopt::StructOpt;

use gitexplore::{Cli, Config};

fn main() {
  ctrlc::set_handler(move || {
    // Exit the program with 130 code to
    // signify ctrl-c
    process::exit(130);
  })
  .expect("Error setting Ctrl-C handler");

  let config = Config::new(Cli::from_args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(exitcode::USAGE);
  });

  println!("Gitexplore CLI");

  if let Err(e) = gitexplore::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(exitcode::SOFTWARE);
  }
}
