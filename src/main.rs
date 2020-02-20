use std::env;
use std::process;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
  pub primary: Vec<OptionValue>,
  pub secondary: HashMap<String, Vec<OptionValue>>,
  pub tertiary: HashMap<String, Vec<OptionValue>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum OptionValue {
  TierOne {label: String, value: String},
  TierTwo {label: String, value: String, usage: String},
  TierThree {label: String, value: String, usage: String, nb: String}
}

fn main() {
  println!("Hello, world!");

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let options_str = include_str!("options.json");
  println!("{:?}", options_str.len());

  let v: Data = serde_json::from_str(options_str).unwrap();
  println!("{:?}", v.primary);
}
