use std::process::Command;

use chrono::{DateTime, FixedOffset, TimeDelta};
use fancy_regex::{Captures, Regex};

use crate::options::ChangenogOptions;

//// Structs

#[derive(Debug)]
pub struct GitRoot;

#[derive(Debug, Clone)]
pub struct GitTag {
    pub name: String,
    pub date: String,
}

#[derive(Debug)]
pub struct GitCommit {
    pub hash: String,
    pub abbrev_hash: String,
    pub author_date: String,
    pub subject: String,
    pub files: Vec<String>,
}

//// Implementations

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

impl GitTag {
    /// Gets all tags in the repo
    pub fn get_tags(tag_filters: &[Regex]) -> Vec<Self> {
        // Log in parsable format
        let cmd_output = Command::new("git")
            .args([
                "tag",
                "-l",
                "--sort=-creatordate",
                "--format=%(creatordate:iso-strict)//%(refname:short)",
            ])
            .output()
            .unwrap();

        let tags_log = String::from_utf8(cmd_output.stdout).expect("unable to parse stdout");
        let tag_regex = Regex::new(r"(?<date>.+?)\/\/(?<name>.+)").unwrap();

        tags_log
            .lines()
            .filter_map(|t| {
                if t == "" {
                    return None;
                }

                let raw_tag = tag_regex.captures(t).unwrap().unwrap();
                let tag = Self::from(raw_tag);

                if tag_filters.iter().any(|r| !r.is_match(&tag.name).unwrap()) {
                    return None;
                }

                Some(tag)
            })
            .collect()
    }

    /// Gets tags since the previous entry
    pub fn get_tags_since(
        all_tags: &[Self],
        prev_entry_date: Option<DateTime<FixedOffset>>,
    ) -> Vec<Self> {
        all_tags
            .iter()
            .filter(|t| {
                prev_entry_date.is_none()
                    || DateTime::parse_from_rfc3339(&t.date).unwrap() > prev_entry_date.unwrap()
            })
            .rev() // Oldest to newest
            .cloned()
            .collect()
    }
}

impl From<Captures<'_>> for GitTag {
    fn from(captures: Captures) -> Self {
        Self {
            name: captures.name("name").unwrap().as_str().to_string(),
            date: captures.name("date").unwrap().as_str().to_string(),
        }
    }
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
