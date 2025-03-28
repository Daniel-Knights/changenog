use crate::{cli::options::ChangenogOptions, utils::run};

#[derive(Debug)]
pub struct GitRoot;

impl GitRoot {
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

        let cmd_output = run("git", &["config", "--get", "remote.origin.url"]);

        if let Ok(cmd_output) = cmd_output {
            let url = cmd_output.replace(".git", "").trim().to_string();

            if !url.is_empty() {
                return Some(url);
            }
        }

        None
    }
}
