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
        GitTag::get_raw(prev_entry_tag)
            .iter()
            .map(|t| GitTag::from_raw(t))
            .collect::<Vec<GitTag>>()
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

    /// Returns raw tags since previous tag in a parsable format
    fn get_raw(prev_tag_name: &Option<String>) -> Vec<String> {
        // Tags since and including `prev_tag_name`
        let no_merged_arg = if prev_tag_name.is_some() {
            &format!("--no-merged={}", prev_tag_name.as_ref().unwrap())
        } else {
            ""
        };

        let tag_args = vec![
            "tag",
            no_merged_arg,
            "--sort=-creatordate", // Newest to oldest
            "--format=%(creatordate:iso-strict) %(refname:short)",
        ];

        run("git", &tag_args)
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    }

    /// Parses raw tag into GitTag
    fn from_raw(raw_tag: &str) -> GitTag {
        let (date, name) = raw_tag.split_once(" ").unwrap();

        GitTag {
            name: name.to_string(),
            date: DateTime::parse_from_rfc3339(date).unwrap(),
        }
    }
}
