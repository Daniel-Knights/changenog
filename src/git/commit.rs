use fancy_regex::Regex;

use crate::{options::ChangenogOptions, utils::run};

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub subject: String,
}

impl GitCommit {
    /// Gets all commits since the previous entry
    pub fn get_all_since(prev_entry_tag: &Option<String>, opts: &ChangenogOptions) -> Vec<Self> {
        Self::get_raw(prev_entry_tag, opts)
            .iter()
            .map(|c| Self::from_raw(c))
            .collect()
    }

    /// Applies each filter in `commit_filters` to each commit in `commits` and returns the result.
    /// All filters must match for a commit to be included.
    pub fn apply_filters(commits: &[GitCommit], commit_filters: &[Regex]) -> Vec<GitCommit> {
        commits
            .iter()
            .filter_map(|c| {
                let all_filters_matched = commit_filters
                    .iter()
                    .all(|f| f.is_match(&c.subject).unwrap());

                if all_filters_matched {
                    Some(c.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<GitCommit>>()
    }

    //// Private

    /// Returns raw commits since previous entry in a parsable format
    fn get_raw(prev_entry_tag: &Option<String>, opts: &ChangenogOptions) -> Vec<String> {
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
    fn from_raw(raw_commit: &str) -> Self {
        let parts = raw_commit.split("////").collect::<Vec<&str>>();

        Self {
            hash: parts[0].to_string(),
            subject: parts[1].to_string(),
        }
    }
}
