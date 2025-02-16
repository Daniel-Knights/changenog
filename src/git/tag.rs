use std::process::Command;

use chrono::{DateTime, FixedOffset};
use fancy_regex::{Captures, Regex};

#[derive(Debug, Clone)]
pub struct GitTag {
    pub name: String,
    pub date: String,
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
