use std::env;
use std::fs;

fn main() {
    // `args()` function will panic if there are any arguments containing invalid Unicode.
    // `args_os()` produces `OsString` but that is platform DEPENDENT
    let env_args: Vec<String> = env::args().collect();
    
    // Sample output
    // dbg!(env_args);
    // [src\main.rs:5] env_args = [
    //     "target\\debug\\rust_io_project.exe",
    //     "searchstring",
    //     "hello.txt",
    // ]

    let query = &env_args[1];
    let file_path = &env_args[2];

    println!("Searching for : {}", query);
    println!("In File : {}", file_path);

    let file_data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text: \n {file_data}");
}
