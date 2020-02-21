use std::process;
use structopt::StructOpt;
use ctrlc;

use gitexplore::{Cli, Config, OptionValue};

fn main() {
  ctrlc::set_handler(move || {
    // Exit the program with 130 code to
    // signify ctrl-c
    process::exit(130);
  }).expect("Error setting Ctrl-C handler");
  
  let config = Config::new(Cli::from_args());

  println!("{:?}", config.search);
  println!("{:?}", config.data.primary);
}

fn check_primary_options(terms: &Vec<String>, primary: &Vec<OptionValue>) -> Result<String, Box<dyn std::error::Error>> {
  let first_term = terms.get(0);
  Ok(String::from("Hello"))
}