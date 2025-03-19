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
    pub fn get_all_since(prev_entry_tag: &Option<String>) -> Vec<Self> {
        Self::get_raw(prev_entry_tag)
            .iter()
            .map(|t| Self::from_raw(t))
            .collect::<Vec<Self>>()
    }

    /// Applies each filter in `tag_filters` to each tag in `tags` and returns the result.
    /// All filters must match for a tag to be included.
    pub fn apply_filters(tags: &[GitTag], tag_filters: &[Regex]) -> Vec<GitTag> {
        tags.iter()
            .filter_map(|t| {
                let all_filters_matched = tag_filters.iter().all(|r| r.is_match(&t.name).unwrap());

                if all_filters_matched {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<GitTag>>()
    }

    //// Private

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
