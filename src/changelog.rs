use chrono::DateTime;
use fancy_regex::Regex;

use crate::git::{commit::GitCommit, tag::GitTag};

/// Generates the new changelog
pub fn generate_changelog(
    existing_changelog: &str,
    all_tags: &[GitTag],
    tags_since: &[GitTag],
    mut commits_since: Vec<GitCommit>,
    remote_url: Option<String>,
) -> String {
    let mut new_changelog = existing_changelog.to_string();

    tags_since.iter().for_each(|tag| {
        let tag_date = DateTime::parse_from_rfc3339(&tag.date).unwrap();

        let splice_index = commits_since
            .iter()
            .position(|c| DateTime::parse_from_rfc3339(&c.author_date).unwrap() > tag_date)
            .unwrap_or(commits_since.len());

        let entry_commits = commits_since.splice(0..splice_index, None).rev();

        if entry_commits.len() == 0 {
            return;
        }

        // Format compare URL
        let prev_tag_index = all_tags.iter().position(|t| t.name == tag.name).unwrap() + 1;
        let prev_tag = all_tags.get(prev_tag_index);
        let compare_url = if remote_url.is_none() {
            None
        } else {
            if let Some(prev_tag) = prev_tag {
                let url = format!(
                    "{}/compare/{}...{}",
                    remote_url.clone().unwrap(),
                    prev_tag.name,
                    tag.name
                );

                Some(url)
            } else {
                let url = format!("{}/tags", remote_url.clone().unwrap());

                Some(url)
            }
        };

        // Format version heading
        let formatted_date = tag_date.format("%d/%m/%Y");
        let version_heading = if compare_url.is_some() {
            format!(
                "## [{}]({}) ({})",
                tag.name,
                compare_url.unwrap(),
                formatted_date
            )
        } else {
            format!("## {} ({})", tag.name, formatted_date)
        };

        // Format commits
        let formatted_commits = entry_commits
            .map(|c| {
                if let Some(remote_url) = &remote_url {
                    format!(
                        "- {} ([{}]({}/commit/{}))",
                        c.subject, c.abbrev_hash, remote_url, c.hash
                    )
                } else {
                    format!("- {} ({})", c.subject, c.abbrev_hash)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        new_changelog = format!(
            "{}\n\n{}\n\n{}",
            version_heading, formatted_commits, new_changelog
        );
    });

    new_changelog.trim().to_string() + "\n"
}

/// Parses existing changelog for the tag heading of the latest entry
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
