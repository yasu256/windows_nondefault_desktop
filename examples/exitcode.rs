use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

fn main() {
    windows_nondefault_desktop::assume_active_desktop();

    let mut input = String::new();

    print!("Enter exit code> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let exitcode = input.trim_end().parse().unwrap();

    exit(exitcode);
}
