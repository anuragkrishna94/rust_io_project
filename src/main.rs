use std::env;

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
}
