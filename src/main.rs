use std::{
    env::args,
    fs,
    io::{stdout, Write},
    process,
};

use changelog::Changelog;
use chrono::DateTime;
use git::{commit::GitCommit, root::GitRoot, tag::GitTag};
use log::log_exit;
use options::ChangenogOptions;

mod changelog;
mod constant;
mod git;
mod log;
mod options;

fn main() {
    let cli_args = args().skip(1).collect::<Vec<String>>();

    // Print version
    if cli_args.contains(&"--version".to_string()) || cli_args.contains(&"-v".to_string()) {
        println!("{}", env!("CARGO_PKG_VERSION"));

        process::exit(0)
    }

    // Print help
    if cli_args.contains(&"--help".to_string()) || cli_args.contains(&"-h".to_string()) {
        ChangenogOptions::help();

        process::exit(0)
    }

    let opts = ChangenogOptions::from_args(&cli_args);

    let all_tags = GitTag::get_tags(&opts.tag_filters);

    if all_tags.is_empty() {
        log_exit("no tags found");

        process::exit(0)
    }

    let output_path = opts.root.join("CHANGELOG.md");

    let existing_changelog = if opts.overwrite || !output_path.exists() {
        ""
    } else {
        &fs::read_to_string(&output_path).unwrap()
    };

    let prev_entry_tag = Changelog::get_prev_entry_tag(existing_changelog, &all_tags);

    let prev_entry_date = if prev_entry_tag.is_some() {
        Some(DateTime::parse_from_rfc3339(prev_entry_tag.unwrap().date.as_str()).unwrap())
    } else {
        None
    };

    let commits_since = GitCommit::get_commits_since(prev_entry_date, &opts);

    if commits_since.is_empty() {
        log_exit("no commits since previous version");

        process::exit(0)
    }

    let tags_since = GitTag::get_tags_since(&all_tags, prev_entry_date);

    if !opts.overwrite && tags_since.is_empty() {
        log_exit("no new version(s)");

        process::exit(0)
    }

    let remote_url = GitRoot::get_remote_url(&opts);

    let new_changelog = Changelog::generate(
        existing_changelog,
        &all_tags,
        &tags_since,
        commits_since,
        remote_url,
    );

    if opts.output == "file" {
        fs::write(output_path, new_changelog).expect("unable to write changelog");
    } else if opts.output == "stdout" {
        stdout()
            .write_all(new_changelog.as_bytes())
            .expect("unable to write changelog to stdout");
    }
}
