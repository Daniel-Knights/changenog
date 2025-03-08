use chrono::DateTime;
use fancy_regex::Regex;

use crate::release::ReleaseCollection;

pub struct Changelog;

impl Changelog {
    /// Generates the new changelog
    pub fn generate(
        entries: &ReleaseCollection,
        existing_changelog: &str,
        remote_url: Option<String>,
        prev_tag: Option<String>,
    ) -> String {
        let mut new_changelog = String::new();

        entries.0.iter().enumerate().for_each(|(i, entry)| {
            if entry.commits.is_empty() {
                return;
            }

            let prev_entry = entries.0.get(i + 1);
            let prev_tag = if prev_entry.is_some() {
                Some(prev_entry.unwrap().tags[0].name.clone())
            } else if prev_tag.is_some() {
                prev_tag.clone()
            } else {
                None
            };

            // Format compare URL
            let compare_url = match (&remote_url, prev_tag) {
                (Some(remote_url), Some(prev_tag)) => {
                    // Use first tag from both entries
                    let curr_tag = entry.tags[0].name.clone();
                    let url = format!("{}/compare/{}...{}", remote_url, prev_tag, curr_tag);

                    Some(url)
                }
                (Some(remote_url), None) => {
                    let url = format!("{}/tags", remote_url);

                    Some(url)
                }
                _ => None,
            };

            // Format release heading
            let tag_date = DateTime::parse_from_rfc3339(&entry.date).unwrap();
            let formatted_date = tag_date.format("%d/%m/%Y");

            let joined_tags = entry
                .tags
                .iter()
                .map(|t| t.name.clone())
                .collect::<Vec<String>>()
                .join(", ");

            let release_heading = if compare_url.is_some() {
                format!(
                    "## [{}]({}) ({})",
                    joined_tags,
                    compare_url.unwrap(),
                    formatted_date
                )
            } else {
                format!("## {} ({})", joined_tags, formatted_date)
            };

            // Format commits
            let formatted_commits = entry
                .commits
                .iter()
                .map(|c| {
                    let abbrev_hash = c.hash[..7].to_string();

                    if let Some(remote_url) = &remote_url {
                        format!(
                            "- {} ([{}]({}/commit/{}))",
                            c.subject, abbrev_hash, remote_url, c.hash
                        )
                    } else {
                        format!("- {} ({})", c.subject, abbrev_hash)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            new_changelog = format!(
                "{}\n\n{}\n\n{}",
                new_changelog, release_heading, formatted_commits
            );
        });

        if !existing_changelog.is_empty() {
            new_changelog += &format!("\n\n{}", existing_changelog.trim());
        }

        new_changelog.trim().to_string() + "\n"
    }

    /// Parses existing changelog for the tag heading of the latest entry
    pub fn get_prev_entry_tag(existing_changelog: &str) -> Option<String> {
        Regex::new(r"## *\[?([^\]\s\(]+)")
            .ok()
            .and_then(|re| re.captures(existing_changelog).ok().flatten())
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }
}
