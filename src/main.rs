use std::{
    env::{args, current_dir},
    fs,
    path::Path,
    process,
};

use chrono::DateTime;
use git::{GitCommit, GitRoot, GitTag, get_remote_url};
use log::log_exit;
use options::Options;
use parse::get_prev_entry_tag;

mod git;
mod log;
mod options;
mod parse;

fn main() {
    let cwd = current_dir().unwrap();
    let cwd = Path::new(cwd.to_str().unwrap());
    let cli_args = args().skip(1).collect::<Vec<String>>();
    let opts = Options::from_args(cli_args);

    let git_root = GitRoot::get(cwd, 0);
    let all_tags = GitTag::get_tags(&opts.tag_filter_regex);

    if all_tags.is_empty() {
        log_exit("no tags found");

        process::exit(0)
    }

    let dest = cwd.join("CHANGELOG.md");

    let existing_changelog = if opts.overwrite {
        ""
    } else {
        &fs::read_to_string(cwd.join(&dest)).unwrap_or("".to_string())
    };

    let prev_entry_tag = get_prev_entry_tag(existing_changelog, &all_tags);

    let prev_entry_date = if prev_entry_tag.is_some() {
        Some(DateTime::parse_from_rfc3339(prev_entry_tag.unwrap().date.as_str()).unwrap())
    } else {
        None
    };

    let tags_since = GitTag::get_tags_since(&all_tags, prev_entry_date);

    if !opts.overwrite && tags_since.len() == 0 {
        log_exit("no new version(s)");

        process::exit(0)
    }

    let commits_since = GitCommit::get_commits_since(git_root.dir, cwd, prev_entry_date, &opts);

    let new_changelog = generate_changelog(
        opts,
        existing_changelog,
        all_tags,
        tags_since,
        commits_since,
    );

    fs::write(dest, new_changelog).expect("unable to write changelog");
}

/// Generates the new changelog
fn generate_changelog(
    opts: Options,
    existing_changelog: &str,
    all_tags: Vec<GitTag>,
    tags_since: Vec<GitTag>,
    mut commits_since: Vec<GitCommit>,
) -> String {
    let remote_url = get_remote_url(opts);

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

    new_changelog.trim().to_string()
}
