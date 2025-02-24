use chrono::DateTime;
use fancy_regex::Regex;

use crate::git::tag::GitTag;

pub struct Changelog;

impl Changelog {
    /// Generates the new changelog
    pub fn generate(
        tags_since: &[GitTag],
        existing_changelog: &str,
        remote_url: Option<String>,
    ) -> String {
        let mut new_changelog = String::new();

        tags_since.iter().for_each(|tag| {
            if tag.commits.is_empty() {
                return;
            }

            // Format compare URL
            let compare_url = match (&remote_url, tag.prev_tag.clone()) {
                (Some(remote_url), Some(prev_tag)) => {
                    let url = format!("{}/compare/{}...{}", remote_url, prev_tag, tag.name);

                    Some(url)
                }
                (Some(remote_url), None) => {
                    let url = format!("{}/tags", remote_url);

                    Some(url)
                }
                _ => None,
            };

            // Format version heading
            let tag_date = DateTime::parse_from_rfc3339(&tag.date).unwrap();
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
            let formatted_commits = tag
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
                new_changelog, version_heading, formatted_commits
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
