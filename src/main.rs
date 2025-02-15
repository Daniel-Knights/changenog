use std::{
    env::{args, current_dir},
    fs,
    io::{stdout, Write},
    path::Path,
    process,
};

use changelog::{generate_changelog, get_prev_entry_tag};
use chrono::DateTime;
use git::{GitCommit, GitRoot, GitTag};
use log::log_exit;
use options::ChangenogOptions;

mod changelog;
mod constant;
mod git;
mod log;
mod options;

fn main() {
    let cwd = current_dir().unwrap();
    let cwd = Path::new(cwd.to_str().unwrap());
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

    let git_root = GitRoot::get(cwd, 0);
    let all_tags = GitTag::get_tags(&opts.tag_filters);

    if all_tags.is_empty() {
        log_exit("no tags found");

        process::exit(0)
    }

    let abs_input_path = cwd.join(&opts.input_path);

    let existing_changelog = if opts.overwrite {
        ""
    } else {
        &fs::read_to_string(cwd.join(&abs_input_path)).unwrap_or("".to_string())
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
    let remote_url = GitRoot::get_remote_url(&opts);

    let new_changelog = generate_changelog(
        existing_changelog,
        &all_tags,
        &tags_since,
        commits_since,
        remote_url,
    );

    if opts.output == "file" {
        fs::write(abs_input_path, new_changelog).expect("unable to write changelog");
    } else if opts.output == "stdout" {
        stdout()
            .write_all(new_changelog.as_bytes())
            .expect("unable to write changelog to stdout");
    }
}
