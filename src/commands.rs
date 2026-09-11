pub enum Command {
    Exit,
    Echo(String),
    NotFound(String),
}

impl From<(String, String)> for Command {
    fn from((command, arguments): (String, String)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
           _ => Self::NotFound(command),
        }
    }
}
