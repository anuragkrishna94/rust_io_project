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
     println!("With text: \n {file_data}");
     Ok(())
 }