use std::{
    env::args,
    fs,
    io::{stdout, Write},
    process::ExitCode,
};

use changelog::Changelog;
use cli::{options::ChangenogOptions, subcommand::Subcommand};
use git::{root::GitRoot, tag::GitTag};
use log::log_exit;
use release::ReleaseCollection;

mod changelog;
mod cli;
mod constant;
mod git;
mod log;
mod release;
mod utils;

fn main() -> ExitCode {
    let cli_args = args().skip(1).collect::<Vec<String>>();

    // Print version
    if cli_args.contains(&"--version".to_string()) || cli_args.contains(&"-v".to_string()) {
        Subcommand::version();

        return ExitCode::SUCCESS;
    }

    // Print help
    if cli_args.contains(&"--help".to_string()) || cli_args.contains(&"-h".to_string()) {
        Subcommand::help();

        return ExitCode::SUCCESS;
    }

    // Get options from args
    let opts = match ChangenogOptions::from_args(&cli_args) {
        Ok(opts) => opts,
        Err(err) => {
            log_exit(&err.to_string(), false);

            return ExitCode::FAILURE;
        }
    };

    let output_path = opts.root.join("CHANGELOG.md");

    let existing_changelog = if opts.overwrite || !output_path.exists() {
        ""
    } else {
        &fs::read_to_string(&output_path).unwrap()
    };

    // Build entries
    let prev_entry_tag = Changelog::get_prev_entry_tag(existing_changelog);
    let tags_since = GitTag::get_all_since(&prev_entry_tag);
    let releases = ReleaseCollection::from(&tags_since, &prev_entry_tag, &opts);

    if !opts.overwrite && releases.0.is_empty() {
        log_exit("no new version(s)", true);

        return ExitCode::SUCCESS;
    }

    // Generate changelog
    let new_changelog = Changelog::generate(
        &releases,
        existing_changelog,
        GitRoot::get_remote_url(&opts),
        prev_entry_tag,
    );

    if new_changelog.trim().is_empty() {
        log_exit("no entries generated", true);

        return ExitCode::SUCCESS;
    }

    // Write changelog
    let write_result = match opts.output.as_str() {
        "file" => fs::write(&output_path, &new_changelog),
        "stdout" => stdout().write_all(new_changelog.as_bytes()),
        _ => unreachable!(),
    };

    match write_result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            log_exit(&format!("Failed to write changelog: {}", err), false);

            return ExitCode::FAILURE;
        }
    }
}
