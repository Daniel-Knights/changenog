use fancy_regex::Regex;

use crate::utils::run;

#[derive(Debug, Clone)]
pub struct GitTag {
    pub name: String,
    pub date: String,
    pub target_commit: String,
}

impl GitTag {
    /// Gets all tags since the previous entry
    pub fn get_all_since(prev_entry_tag: &Option<String>, tag_filters: &[Regex]) -> Vec<Self> {
        let raw_tags = Self::get_raw(prev_entry_tag);

        raw_tags
            .iter()
            .filter_map(|t| {
                let tag = Self::from_raw(t);

                if tag_filters.iter().any(|r| !r.is_match(&tag.name).unwrap()) {
                    return None;
                }

                Some(tag)
            })
            .collect::<Vec<Self>>()
    }

    /// Returns raw tags since previous entry in a parsable format
    fn get_raw(prev_entry_tag: &Option<String>) -> Vec<String> {
        // Tags since and including `prev_entry_tag`
        let no_merged_arg = if prev_entry_tag.is_some() {
            &format!("--no-merged={}", prev_entry_tag.clone().unwrap())
        } else {
            ""
        };

        let tag_args = vec![
            "tag",
            no_merged_arg,
            "--sort=-creatordate", // Newest to oldest
            // `%(object)`: target commit if tag is annotated
            // `%(objectname)`: target commit if tag is lightweight
            "--format=%(refname:short)////%(creatordate:iso-strict)////%(object)////%(objectname)",
        ];

        run("git", &tag_args)
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    }

    /// Parses raw tag into GitTag
    fn from_raw(raw_tag: &str) -> Self {
        let tag_parts: Vec<&str> = raw_tag.split("////").collect();

        let (tag_name, tag_date, object, objectname) = (
            tag_parts[0].to_string(),
            tag_parts[1].to_string(),
            tag_parts[2].to_string(),
            tag_parts[3].to_string(),
        );

        // `object`: target commit if tag is annotated
        // `objectname`: target commit if tag is lightweight
        let target_commit = if object.is_empty() {
            objectname
        } else {
            object
        };

        Self {
            name: tag_name.clone(),
            date: tag_date,
            target_commit,
        }
    }
}
