use exitcode;
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};
use std::process;
use structopt::StructOpt;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn verify_args_length() {
    let search_terms = vec!["add".to_string(), "a".to_string(), "commit".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();

    assert_eq!(2, cfg.search.len())
  }

  #[test]
  fn verify_lowercase() {
    let search_terms = vec!["AdD".to_string(), "ComMit".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();

    assert_eq!(vec!["add".to_string(), "commit".to_string(),], cfg.search)
  }

  #[test]
  fn verify_stripping_articles() {
    let search_terms = vec![
      "add".to_string(),
      "a".to_string(),
      "commit".to_string(),
      "to".to_string(),
      "the".to_string(),
      "repo".to_string(),
    ];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();

    assert_eq!(
      vec![
        "add".to_string(),
        "commit".to_string(),
        "to".to_string(),
        "repo".to_string()
      ],
      cfg.search
    )
  }

  #[test]
  fn first_pass_search_match() {
    let search_terms = vec!["add".to_string(), "a".to_string(), "commit".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();
    let result = first_pass(&cfg);

    assert_eq!(result.is_some(), true)
  }

  #[test]
  fn first_pass_search_no_match() {
    let search_terms = vec!["weird".to_string(), "a".to_string(), "commit".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();
    let result = first_pass(&cfg);

    assert_eq!(result.is_some(), false)
  }

  #[test]
  fn second_pass_test() {
    let search_terms = vec!["add".to_string(), "new".to_string(), "branch".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();
    if let Some(fp_result) = first_pass(&cfg) {
      second_pass(&cfg, fp_result);
    }
  }

  #[test]
  fn combine_test() {
    let search_terms = vec!["add".to_string(), "new".to_string(), "branch".to_string()];
    let cfg = Config::new(Cli {
      debug: false,
      search_terms,
    })
    .unwrap();

    println!("\nAdd Test");
    let add_terms = combine_secondary_tertiary(&cfg, &cfg.search[0]);
    println!("{:?}", add_terms);

    println!("\nShow Test");
    let show_terms = combine_secondary_tertiary(&cfg, &String::from("show"));
    println!("{:?}", show_terms);
  }
}

#[derive(Debug)]
pub struct Config {
  pub search: Vec<String>,
  pub data: Data,
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

    // Transform all search terms into lowercase
    // Strip search term vector of articles - a, an, the
    let new_search_terms: Vec<String> = cli
      .search_terms
      .clone()
      .iter()
      .map(|x| x.to_lowercase())
      .filter(|x| is_article(x))
      .collect();

    // We don't worry about the debug field in the Cli struct
    Ok(Config {
      search: new_search_terms,
      data,
    })
  }
}

fn is_article(x: &String) -> bool {
  return *x != "a" && *x != "an" && *x != "the";
}

pub fn run(cfg: Config) -> Result<String, &'static str> {
  let fp_res = first_pass(&cfg);

  match fp_res {
    Some(fp) => {
      let _sp_res = second_pass(&cfg, &fp);
    }
    None => return Err("Invalid search term"),
  }

  Ok(String::from("Hello"))
}

fn first_pass<'a>(cfg: &'a Config) -> Option<&'a OptionValue> {
  let term = &cfg.search[0];
  let options = &cfg.data.primary;
  for option in options.iter() {
    let label = option.get_label();
    if label.contains(term) {
      return Some(option);
    }
  }

  None
}

fn second_pass<'a>(cfg: &'a Config, fp_res: &'a OptionValue) -> Option<&'a Vec<OptionValue>> {
  let sp_results: Vec<OptionValue> = Vec::new();
  let terms = &cfg.search[1..];
  let sec_options = &cfg.data.secondary;

  // Use a Binary Heap to hold the secondary options in order of most word matches
  let mut maxheap: BinaryHeap<i8> = BinaryHeap::new();

  // Have to match Enum variants
  match fp_res {
    OptionValue::TierOne { label: _, value } => {
      // Match on HashMap Option result
      match sec_options.get(value) {
        Some(sec) => {
          // Key is available
          println!("{:?}", sec);
        }
        None => {
          // No key found in HashMap
          println!("Not found");
        }
      }
    }
    _ => (),
  }

  None
}

fn combine_secondary_tertiary<'a>(cfg: &'a Config, term: &String) -> Vec<String> {
  let mut combined_search_terms: Vec<String> = Vec::new();

  // The search term exists in the secondary options data
  if let Some(secondary) = &cfg.data.secondary.get(term) {
    for s in secondary.iter() {
      // Match on possible enum variants
      match s {
        // This means there is a tertiary option
        OptionValue::TierOne { label, value } => {
          match &cfg.data.tertiary.get(value) {
            Some(tertiary_data) => {
              // Loop through the tertiary items for the key
              // and append the label to the corresponding secondary item label
              // Add this concatenated label to the combined_search_terms vec
              for t in tertiary_data.iter() {
                let t_label = t.get_label();
                let combined_label = [term.clone(), label.clone(), t_label.clone()].join(" ");
                combined_search_terms.push(combined_label);
              }
            }
            None => (),
          }
        }
        _ => {
          let s_label = s.get_label();
          combined_search_terms.push([term.clone(), s_label.clone()].join(" "));
        }
      }
    }
  }

  combined_search_terms
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub primary: Vec<OptionValue>,
  pub secondary: HashMap<String, Vec<OptionValue>>, // dynamic keys
  pub tertiary: HashMap<String, Vec<OptionValue>>,  // dynamic keys
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Use an enum to represent the kinds of option values since it's optional for usage and nb fields to be present in the data
pub enum OptionValue {
  TierThree {
    label: String,
    value: String,
    usage: String,
    nb: String,
  },
  TierTwo {
    label: String,
    value: String,
    usage: String,
  },
  TierOne {
    label: String,
    value: String,
  },
}

// Impl block for getter methods
impl OptionValue {
  fn get_label(&self) -> &String {
    match self {
      OptionValue::TierOne { label, .. }
      | OptionValue::TierTwo { label, .. }
      | OptionValue::TierThree { label, .. } => {
        return &label;
      }
    }
  }

  fn get_value(&self) -> &String {
    match self {
      OptionValue::TierOne { value, .. }
      | OptionValue::TierTwo { value, .. }
      | OptionValue::TierThree { value, .. } => {
        return &value;
      }
    }
  }
}

#[derive(Debug, StructOpt)]
/// Welcome to the Git Explore CLI,
/// where you can search for git commands with natural language
/// Example usage: $gitexplore add new commit
pub struct Cli {
  /// Activate debug mode
  // short and long flags (-d, --debug) will be deduced from the field's name
  #[structopt(short, long)]
  pub debug: bool,

  /// The action or command you're looking for
  pub search_terms: Vec<String>,
}
