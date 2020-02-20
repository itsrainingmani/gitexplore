use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub primary: Vec<OptionValue>,
  pub secondary: HashMap<String, Vec<OptionValue>>, // dynamic keys
  pub tertiary: HashMap<String, Vec<OptionValue>> // dynamic keys
}

impl Data {
  pub fn construct() -> Data {
    let options_str = include_str!("options.json");
    let v: Data = serde_json::from_str(options_str).unwrap();
    v
  }
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

  /// Natural Language Search
  pub search_terms: Vec<String>
}