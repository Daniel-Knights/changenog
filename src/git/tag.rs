use fancy_regex::Regex;

use crate::utils::run;

use super::commit::GitCommit;

#[derive(Debug, Clone)]
pub struct GitTag {
    pub name: String,
    pub date: String,
    pub target_commit: String,
    pub commits: Vec<GitCommit>,
    pub prev_tag: Option<String>,
}

impl GitTag {
    /// Gets all tags in the repo
    pub fn get_all_since(prev_entry_tag: &Option<String>, tag_filters: &[Regex]) -> Vec<Self> {
        let raw_tags = Self::get_raw(prev_entry_tag);

        let mut prev_tag = prev_entry_tag.clone();

        let mut tags = raw_tags
            .iter()
            .filter_map(|t| {
                let tag = Self::from_raw(t, &prev_tag);

                if tag_filters.iter().any(|r| !r.is_match(&tag.name).unwrap()) {
                    return None;
                }

                prev_tag = Some(tag.name.clone());

                Some(tag)
            })
            .collect::<Vec<Self>>();

        tags.reverse(); // Newest to oldest

        tags
    }

    /// Populates each tag's `commit` field with the relevant commits.
    /// `tags` and `commits` must be sorted newest to oldest.
    pub fn populate_commits(tags: &mut [GitTag], commits: &[GitCommit]) {
        // Skip any untagged commits
        let skip_i = commits
            .iter()
            .position(|c| c.hash == tags[0].target_commit)
            .unwrap();

        let mut tag_i = 0;

        for c in commits.iter().skip(skip_i) {
            let next_tag = tags.get(tag_i + 1);

            if next_tag.is_some() && c.hash == next_tag.unwrap().target_commit {
                tag_i += 1;
            }

            tags[tag_i].commits.push(c.clone());
        }
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
            "--sort=creatordate",
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
    fn from_raw(raw_tag: &str, prev_tag: &Option<String>) -> Self {
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
            commits: vec![],
            prev_tag: prev_tag.clone(),
        }
    }
}
