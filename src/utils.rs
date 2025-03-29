use std::{error::Error, process::Command};

pub fn run(cmd: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let args = args
        .iter()
        .filter(|a| !a.is_empty())
        .copied()
        .collect::<Vec<&str>>();

    let output = Command::new(cmd).args(args).output()?;

    Ok(String::from_utf8(output.stdout)?)
}
