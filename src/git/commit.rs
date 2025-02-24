use crate::{options::ChangenogOptions, utils::run};

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub subject: String,
}

impl GitCommit {
    /// Gets all commits since `prev_entry_tag`
    pub fn get_all_since(prev_entry_tag: &Option<String>, opts: &ChangenogOptions) -> Vec<Self> {
        let raw_commits = Self::get_raw(opts, prev_entry_tag);

        raw_commits
            .iter()
            .filter_map(|c| {
                let parsed_commit = Self::from_pretty(c);

                // Apply CLI arg filters
                if opts
                    .commit_filters
                    .iter()
                    .any(|f| !f.is_match(&parsed_commit.subject).unwrap())
                {
                    return None;
                }

                Some(parsed_commit)
            })
            .collect()
    }

    /// Returns raw commits since previous entry in a parsable format
    fn get_raw(opts: &ChangenogOptions, prev_entry_tag: &Option<String>) -> Vec<String> {
        let max_commits_arg = format!("--max-count={}", opts.max_commits.to_string());

        let since_arg = if prev_entry_tag.is_some() {
            &format!("{}..", prev_entry_tag.clone().unwrap())
        } else {
            ""
        };

        // Log in a parsable format
        let log_args = vec![
            "log",
            max_commits_arg.as_str(),
            "--pretty=%H////%s",
            since_arg,
            "--",
            opts.root.to_str().unwrap(), // Only show commits with file changes in root
        ];

        run("git", &log_args)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    /// Parses raw commit into GitCommit
    fn from_pretty(commit_str: &str) -> Self {
        let parts = commit_str.split("////").collect::<Vec<&str>>();

        Self {
            hash: parts[0].to_string(),
            subject: parts[1].to_string(),
        }
    }
}
