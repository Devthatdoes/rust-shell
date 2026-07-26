mod commands;
mod errors;
pub mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

use anyhow::{Context, Result};

use crate::{
    commands::Command,
    errors::CustomError,
    utilities::{get_user_input, get_command, print_error, print_prompt, exit},
};

pub fn run() -> Result<()> {
    loop {
    print_prompt(); 
    
    let command = get_command().context("getting command")?;

    match command {
            Command::Exit => break,
            Command::NotFound(command_string) => {
                let error = CustomError::CommandNotFound(command_string);
                print_error(error);
            }
        }

    }

    #[allow(unreachable_code)]

    Ok(())
}
