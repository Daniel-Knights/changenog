use std::{
    env::args,
    fs,
    io::{stdout, Write},
    process,
};

use changelog::Changelog;
use git::{commit::GitCommit, root::GitRoot, tag::GitTag};
use log::log_exit;
use options::ChangenogOptions;
use release::ReleaseCollection;

mod changelog;
mod constant;
mod git;
mod log;
mod options;
mod release;
mod utils;

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

    let output_path = opts.root.join("CHANGELOG.md");

    let existing_changelog = if opts.overwrite || !output_path.exists() {
        ""
    } else {
        &fs::read_to_string(&output_path).unwrap()
    };

    let prev_entry_tag = Changelog::get_prev_entry_tag(existing_changelog);
    let commits_since = GitCommit::get_all_since(&prev_entry_tag, &opts);
    let tags_since = GitTag::get_all_since(&prev_entry_tag);

    let releases = ReleaseCollection::from_tags(&tags_since)
        .populate_commits(&commits_since)
        .apply_filters(&opts.tag_filters, &opts.commit_filters);

    if !opts.overwrite && releases.0.is_empty() {
        log_exit("no new version(s)");

        process::exit(0)
    }

    let remote_url = GitRoot::get_remote_url(&opts);
    let new_changelog =
        Changelog::generate(&releases, existing_changelog, remote_url, prev_entry_tag);

    if opts.output == "file" {
        fs::write(output_path, new_changelog).expect("unable to write changelog");
    } else if opts.output == "stdout" {
        stdout()
            .write_all(new_changelog.as_bytes())
            .expect("unable to write changelog to stdout");
    }
}
