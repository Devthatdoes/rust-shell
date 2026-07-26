#[allow(unused_imports)]
use std::io::{self, Write};

pub fn run() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
