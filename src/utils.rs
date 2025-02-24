use std::{io::Error, process::Command};

pub fn run(cmd: &str, args: &[&str]) -> Result<String, Error> {
    let args = args
        .iter()
        .filter_map(|a| if !a.is_empty() { Some(*a) } else { None })
        .collect::<Vec<&str>>();

    let output = Command::new(cmd).args(args).output();

    if let Ok(output) = output {
        return Ok(String::from_utf8(output.stdout).unwrap());
    }

    Err(output.err().unwrap())
}
