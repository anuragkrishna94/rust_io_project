# rust_io_project
Chapter 12 I/O Project code from Rust book

# Commands to run the project.
- Run a case sensitive search <br />
`cargo run -- <INPUT_WORD> hello.txt > output.txt` <br />
example
> `cargo run -- body hello.txt > output.txt`

- Run a case in sensitive search <br />**Windows**<br />
`$env:IGNORE_CASE=1; cargo run -- <INPUT_WORD> hello.txt > output.txt` <br />
example-Windows
> `$env:IGNORE_CASE=1; cargo run -- then hello.txt > output.txt