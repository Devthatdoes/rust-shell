

use std::{
    fmt::Display,
    io::{self, stdin, Write},
};

use std::process;
use anyhow::{Context, Result};

use crate::commands::Command;


pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).context("reading the user input");
    Ok(user_input.trim().to_owned())
}

pub fn print_error(message: impl Display) {
    eprintln!("{message}",);
}

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn get_command() -> Result<Command> {
    let user_input = get_user_input()?;
    let command_and_arguments = parse_input(user_input);
    let command = Command::from(command_and_arguments);
    Ok(command)
}

pub fn exit(code: i32) {
    process::exit(code);
}

pub fn parse_input(user_input: String) -> (String, String) {
    let mut split_user_input = user_input.split_whitespace();
    let command_input = split_user_input.next().unwrap_or(" ").to_owned();
    let arguments = split_user_input.collect::<String>();

    (command_input, arguments)
}
