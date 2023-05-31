use std::fs;
use std::error::Error;

pub struct Config {
     pub query: String,
     pub file_path: String
}

impl Config {
     // In general as programmers we expect `new()` to never fail.
     fn new(args: &[String]) -> Config {
          if args.len() < 3 {
               panic!("Not enough arguments");
          }
          Config {
               query: args[1].clone(),
               file_path: args[2].clone()
          }
     }

     pub fn build(args: &[String]) -> Result<Config, &'static str> {
          if args.len() < 3 {
               Err("Not enough arguments")
          }
          else {
               Ok(Config {
                    query: args[1].clone(),
                    file_path: args[2].clone()
               })
          }
     }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
     let file_data = fs::read_to_string(config.file_path)?;
     for line in search(&config.query, &file_data) {
          println!("{line}");
     }
     Ok(())
}

// REturned vector should contain string slices that reference slices of the arument `contents`
// Data returned will live as long as data passed into `contents`
pub fn search<'a>(query: & str, contents: &'a str) -> Vec<&'a str> {
     let mut results = Vec::new();
     for line in contents.lines() {
          if line.contains(query) {
               results.push(line);
          }
     }
     results
}

#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn one_result() {
          let query = "duct";
          let contents = "\nRust:\nsafe, fast, productive.\nPick three.";
          assert_eq!(vec!["safe, fast, productive."], search(query, contents));
     }
}