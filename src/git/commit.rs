use fancy_regex::Regex;

use crate::{cli::options::ChangenogOptions, utils::run};

use super::tag::GitTag;

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub subject: String,
}

impl GitCommit {
    /// Returns all commits between `prev_tag` and `tag`.
    /// If `prev_tag` is `None`, returns all commits from the beginning of the repo
    /// history up to `tag`.
    pub fn get_all_for_tag(
        tag: &GitTag,
        prev_tag: &Option<String>,
        opts: &ChangenogOptions,
    ) -> Vec<GitCommit> {
        let tag_range = if prev_tag.is_some() {
            let prev_tag_name = prev_tag.clone().unwrap_or("".to_string());

            &format!("{}..{}", prev_tag_name, tag.name)
        } else {
            &tag.name
        };

        let log_args = &vec![
            "log",
            tag_range,
            "--pretty=%H %s",
            "--",
            opts.root.to_str().unwrap(), // Only show commits with file changes in root
        ];

        let cmd_output = run("git", log_args).unwrap();
        let raw_commits = cmd_output.lines().collect::<Vec<&str>>();

        raw_commits
            .iter()
            .map(|l| GitCommit::from_raw(l))
            .collect::<Vec<GitCommit>>()
    }

    /// Parses raw commit into GitCommit
    pub fn from_raw(raw_commit: &str) -> Self {
        let (hash, subject) = raw_commit.split_once(" ").unwrap();

        Self {
            hash: hash.to_string(),
            subject: subject.to_string(),
        }
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
}
