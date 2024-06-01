use std::env;
use std::error;
use std::fs;
use std::fmt;
use std::io;

mod catfile;

#[derive(Debug, PartialEq, Eq)]
enum Error {
    UnknownCommand(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::UnknownCommand(command) => write!(f, "unknown command: {command}"),
        }
    }
}

impl error::Error for Error {}

fn init() -> Result<(), io::Error> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");
    Ok(())
}

fn unknown_command(command: &str) -> Result<(), Error> {
    Err(Error::UnknownCommand(command.to_string()))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err)  = match args[1].as_str() {
        "init" => init().map_err(|err|Box::new(err) as Box<dyn error::Error>),
        "cat-file" => catfile::cat_file(&args[3]).map_err(|err|Box::new(err) as Box<dyn error::Error>),
        _ => unknown_command(&args[1]).map_err(|err|Box::new(err) as Box<dyn error::Error>)
    } {
        println!("{err}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unknown_command() {
        let command = "unknown-command";
        let expected_error = Err(Error::UnknownCommand(command.to_string()));

        let actual_result = unknown_command(command);
        assert_eq!(actual_result, expected_error);
    }
}

