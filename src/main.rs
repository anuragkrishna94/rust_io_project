use std::env;
use std::process;
use rust_io_project::Config;
use rust_io_project::run;

// Remember to group configuration variables viz., commandline arguments here,
// to achieve separation of concerns.
fn main_OLD() {
    // `args()` function will panic if there are any arguments containing invalid Unicode.
    // `args_os()` produces `OsString` but that is platform DEPENDENT
    // let env_args: Vec<String> = env::args().collect();
    
    // Sample output
    // dbg!(env_args);
    // [src\main.rs:5] env_args = [
    //     "target\\debug\\rust_io_project.exe",
    //     "searchstring",
    //     "hello.txt",
    // ]

    // let query = &env_args[1];
    // let file_path = &env_args[2];

    // println!("Searching for : {}", query);
    // println!("In File : {}", file_path);

    // let file_data = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");
    // println!("With text: \n {file_data}");
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    let config: Config = Config::build(&env_args, ignore_case).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for : {}", config.query);
    println!("In File : {}", config.file_path);

    // `unwrap_or_else()` is not required here as `Ok(())` is of no use.
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }

    // value already moved hence will throw an error
    // println!("Searching for : {}", config.query);
    // println!("In File : {}", config.file_path);
}

