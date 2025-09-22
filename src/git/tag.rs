use chrono::{DateTime, FixedOffset};
use fancy_regex::Regex;

use crate::utils::run;

#[derive(Debug, Clone)]
pub struct GitTag {
    pub name: String,
    pub date: DateTime<FixedOffset>,
}

impl GitTag {
    /// Gets all tags since the previous entry
    pub fn get_all_since(prev_entry_tag: &Option<String>) -> Vec<GitTag> {
        let tag_args = vec![
            "tag",
            "--sort=-creatordate", // Newest to oldest
            "--format=%(creatordate:iso-strict) %(refname:short)",
        ];

        let cmd_output = run("git", &tag_args).unwrap();
        let tag_lines = cmd_output.lines();

        let mut tags = vec![];

        for line in tag_lines {
            let line_str = line.to_string();
            let tag = GitTag::from_raw(&line_str);

            // Break once we reach the previous tag name
            if prev_entry_tag.as_ref().is_some_and(|t| &tag.name == t) {
                break;
            }

            tags.push(tag)
        }

        tags
    }

    /// Validates `tag.name` against all filters.
    pub fn passes_filters(&self, tag_filters: &[Regex]) -> bool {
        tag_filters.iter().all(|r| r.is_match(&self.name).unwrap())
    }

    /// Returns bool indicating if the given tag exists in the repo
    pub fn exists(tag_name: &str) -> bool {
        !run("git", &["tag", "-l", tag_name]).unwrap().is_empty()
    }

    //// Private

    /// Parses raw tag into GitTag
    fn from_raw(raw_tag: &str) -> GitTag {
        let (date, name) = raw_tag.split_once(" ").unwrap();

        GitTag {
            name: name.to_string(),
            date: DateTime::parse_from_rfc3339(date).unwrap(),
        }
    }
}
