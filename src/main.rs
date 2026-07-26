

use std::process;

use codecrafters_shell::{run, utilities::exit};


fn main() {
    match run() { 
    Ok(()) => exit(0),
    Err(error) => {
        eprintln!("Error: ${error}");
        process::exit(1);
    }
    }
}
