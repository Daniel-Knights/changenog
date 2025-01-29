use fancy_regex::Regex;

use crate::git::GitTag;

pub fn get_prev_entry_tag(existing_changelog: &str, all_tags: &[GitTag]) -> Option<GitTag> {
    let prev_entry_captures = Regex::new(r"## *\[?([^\]\s\(]+)")
        .unwrap()
        .captures(existing_changelog);

    let mut prev_entry_tag = None;

    if let Ok(Some(unwrapped_captures)) = prev_entry_captures {
        let prev_entry_heading = unwrapped_captures.get(1).unwrap().as_str();

        prev_entry_tag = all_tags.iter().find(|t| t.name == prev_entry_heading);
    };

    prev_entry_tag.cloned()
}
