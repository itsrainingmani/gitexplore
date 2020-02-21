use std::process;
use structopt::StructOpt;
use ctrlc;
use exitcode;

use gitexplore::{Cli, Config};

fn main() {
  ctrlc::set_handler(move || {
    // Exit the program with 130 code to
    // signify ctrl-c
    process::exit(130);
  }).expect("Error setting Ctrl-C handler");
  
  let config = Config::new(Cli::from_args()).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(exitcode::USAGE);
  });

  println!("{:?}", config.search);
  println!("{:?}", config.data.primary);
}

fn check_primary_options(cfg: &Config) -> Result<String, Box<dyn std::error::Error>> {
  let first_term = &cfg.search[0];
  Ok(String::from("Hello"))
}