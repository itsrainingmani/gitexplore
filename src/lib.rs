use std::collections::HashMap;
use std::process;
use std::error::Error;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use exitcode;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_args_length() {
    let search_terms = vec![
      "add".to_string(), 
      "a".to_string(), 
      "commit".to_string()
    ];
    let cfg = Config::new(Cli {debug: false, search_terms}).unwrap();

    assert_eq!(
      3,
      cfg.search.len()
    )
  }

  #[test]
  fn first_pass_search_match() {
    let search_terms = vec![
      "add".to_string(), 
      "a".to_string(), 
      "commit".to_string()
    ];
    let cfg = Config::new(Cli {debug: false, search_terms}).unwrap();
    let result = first_pass(&cfg.search[0], &cfg.data.primary);

    assert_eq!(
      result.is_some(),
      true
    )
  }

  #[test]
  fn first_pass_search_no_match() {
    let search_terms = vec![
      "weird".to_string(), 
      "a".to_string(), 
      "commit".to_string()
    ];
    let cfg = Config::new(Cli {debug: false, search_terms}).unwrap();
    let result = first_pass(&cfg.search[0], &cfg.data.primary);

    assert_eq!(
      result.is_some(),
      false
    )
  }
}

#[derive(Debug)]
pub struct Config {
  pub search: Vec<String>,
  pub data: Data
}

impl Config {
  pub fn new(cli: Cli) -> Result<Config, &'static str> {

    // Data Construction
    let options_str = include_str!("options.json");
    let data: Data = serde_json::from_str(options_str).unwrap_or_else(|err| {
      // If internal data is corrupted there's no point trying to continue execution or passing the error up the chain.
      // Write the error msg to stderr
      eprintln!("Internal Data corrupted: {}\nExiting...", err);
      
      // Use the SOFTWARE exit code which indicates that an internal software error has been detected
      process::exit(exitcode::SOFTWARE);
    });

    if cli.search_terms.len() < 1 {
      return Err("No search terms used");
    }

    // We don't worry about the debug field in the Cli struct
    Ok(Config {search: cli.search_terms, data})
  }
}

pub fn run(cfg: Config) -> Result<String, Box<dyn Error>> {
  Ok(String::from("Hello"))
}

fn first_pass<'a>(term: &String, options: &'a Vec<OptionValue>) -> Option<&'a OptionValue> {
  for option in options.iter() {
    match option {
      OptionValue::TierOne { label, value: _ } => {
        if label.contains(term) {
          return Some(option)
        }
      },
      _ => (),
    }
  }

  None
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub primary: Vec<OptionValue>,
  pub secondary: HashMap<String, Vec<OptionValue>>, // dynamic keys
  pub tertiary: HashMap<String, Vec<OptionValue>> // dynamic keys
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Use an enum to represent the kinds of option values since it's optional for usage and nb fields to be present in the data
pub enum OptionValue {
  TierOne {label: String, value: String},
  TierTwo {label: String, value: String, usage: String},
  TierThree {label: String, value: String, usage: String, nb: String}
}

#[derive(Debug, StructOpt)]
/// Welcome to the Git Explore CLI,
/// where you can search for git commands with natural language
pub struct Cli {
  /// Activate debug mode
  // short and long flags (-d, --debug) will be deduced from the field's name
  #[structopt(short, long)]
  pub debug: bool,

  /// The action or command you're looking for
  pub search_terms: Vec<String>
}