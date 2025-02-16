use std::process::Command;

use chrono::{DateTime, FixedOffset, TimeDelta};

use crate::{git::root::GitRoot, options::ChangenogOptions};

#[derive(Debug)]
pub struct GitCommit {
    pub hash: String,
    pub abbrev_hash: String,
    pub author_date: String,
    pub subject: String,
    pub files: Vec<String>,
}

impl GitCommit {
    /// Gets commits since the previous entry
    pub fn get_commits_since(
        prev_entry_date: Option<DateTime<FixedOffset>>,
        opts: &ChangenogOptions,
    ) -> Vec<Self> {
        let raw_commits = Self::get_raw_commits(opts, prev_entry_date);
        let git_root = GitRoot::get();

        let formatted_root = opts
            .root
            .strip_prefix(git_root)
            .expect("root arg must be within nearest git root")
            .to_str()
            .unwrap();

        raw_commits
            .iter()
            .filter_map(|commit| {
                let parsed_commit = Self::from_pretty(commit);

                if parsed_commit.files.len() == 0 {
                    return None;
                }

                // Restrict to current package
                if parsed_commit
                    .files
                    .iter()
                    .all(|f| !f.starts_with(formatted_root))
                {
                    return None;
                }

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
            .rev() // Oldest to newest
            .collect::<Vec<Self>>()
    }

    /// Returns raw commits since previous entry in a parsable format
    fn get_raw_commits(
        opts: &ChangenogOptions,
        prev_entry_date: Option<DateTime<FixedOffset>>,
    ) -> Vec<String> {
        let max_commits_arg = format!("--max-count={}", opts.max_commits.to_string());

        let since_arg = format!(
            "--since={}",
            prev_entry_date
                .unwrap_or(DateTime::UNIX_EPOCH.fixed_offset())
                // Add 1s to exclude commits made at the same time as previous entry
                .checked_add_signed(TimeDelta::new(1, 0).unwrap())
                .unwrap()
        );

        // Log in a parsable format
        let log_args = vec![
            "log",
            "--name-status",
            "--pretty=format:<<<<<<<%nhash: %H%nabbrev_hash: %h%nauthor_date: %aI%nsubject: %s%n>>>>>>>",
            max_commits_arg.as_str(),
            since_arg.as_str(),
        ];

        let cmd_output = Command::new("git").args(log_args).output().unwrap();
        let full_log = String::from_utf8(cmd_output.stdout).unwrap();

        full_log
            .trim()
            .split("<<<<<<<")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1..]
            .to_vec()
    }

    /// Parses raw commit into GitCommit
    fn from_pretty(commit_str: &str) -> Self {
        let parts = commit_str.split(">>>>>>>").collect::<Vec<&str>>();
        let (metadata, files) = (parts[0], parts[1]);
        let metadata_lines = metadata.lines().filter(|l| *l != "").collect::<Vec<&str>>();
        let files = files
            .lines()
            .filter_map(|f| {
                if f == "" {
                    return None;
                }

                let filename = f
                    .splitn(2, "\t")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[1]
                    .clone();

                Some(filename)
            })
            .collect::<Vec<String>>();

        let [hash, abbrev_hash, author_date, subject] = [
            metadata_lines[0].replace("hash: ", "").to_string(),
            metadata_lines[1].replace("abbrev_hash: ", "").to_string(),
            metadata_lines[2].replace("author_date: ", "").to_string(),
            metadata_lines[3].replace("subject: ", "").to_string(),
        ];

        Self {
            hash,
            abbrev_hash,
            author_date,
            subject,
            files,
        }
    }
}
