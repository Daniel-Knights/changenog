use std::process::Command;

use crate::options::ChangenogOptions;

#[derive(Debug)]
pub struct GitRoot;

impl GitRoot {
    pub fn get() -> String {
        let cmd_output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .unwrap();

        String::from_utf8(cmd_output.stdout).unwrap()
    }

    pub fn get_remote_url(opts: &ChangenogOptions) -> Option<String> {
        if opts.no_links {
            return None;
        }

        if opts.remote_url.is_some() {
            let mut url = opts.remote_url.clone().unwrap();

            if url.ends_with("/") {
                url.pop();
            }

            return Some(url);
        }

        let cmd_output = Command::new("git")
            .args(["config", "--get", "remote.origin.url"])
            .output();

        if let Ok(cmd_output) = cmd_output {
            let url = String::from_utf8(cmd_output.stdout)
                .expect("unable to parse stdout")
                .replace(".git", "")
                .trim()
                .to_string();

            if !url.is_empty() {
                return Some(url);
            }
        }

        None
    }
}
