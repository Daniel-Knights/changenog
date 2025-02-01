use std::{
    collections::{HashMap, HashSet},
    process,
};

use fancy_regex::Regex;

use crate::{
    constant::{
        ANGULAR_README_ONLY_DOCS_REGEX, ANGULAR_REGEX, NO_CHANGELOG_REGEX, NO_SEMVER_REGEX,
    },
    log::log_exit,
};

//// Structs

pub struct ChangenogOptions {
    pub overwrite: bool,
    pub no_links: bool,
    pub max_commits: i32,
    pub remote_url: Option<String>,
    pub tag_filters: Vec<Regex>,
    pub commit_filters: Vec<Regex>,
}

pub struct CliFlags;

//// Implementations

impl ChangenogOptions {
    /// Gets formatted options from CLI args
    pub fn from_args(cli_args: &[String]) -> Self {
        let processed_args = Self::process_args(cli_args);

        let commit_filter_presets = Self::get_commit_filter_presets(
            processed_args
                .get(CliFlags::COMMIT_FILTER_PRESET)
                .unwrap_or(&HashSet::new()),
        );

        Self {
            overwrite: processed_args.contains_key(CliFlags::OVERWRITE),
            no_links: processed_args.contains_key(CliFlags::NO_LINKS),
            max_commits: processed_args
                .get(CliFlags::MAX_COMMITS)
                .unwrap_or(&HashSet::from(["1000".to_string()]))
                .iter()
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .expect("invalid max-commits arg"),
            remote_url: processed_args
                .get(CliFlags::REMOTE_URL)
                .unwrap_or(&HashSet::new())
                .iter()
                .nth(0)
                .map(|s| s.to_string()),
            tag_filters: processed_args
                .get(CliFlags::TAG_FILTER_REGEX)
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid tag-filter-regex: {}", r)))
                .collect::<Vec<Regex>>(),
            commit_filters: processed_args
                .get(CliFlags::COMMIT_FILTER_REGEX)
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid commit-filter-regex: {}", r)))
                .chain(commit_filter_presets)
                .collect::<Vec<Regex>>(),
        }
    }

    /// Returns presets matching those passed
    fn get_commit_filter_presets(presets: &HashSet<String>) -> Vec<Regex> {
        let mut presets_map: HashMap<&str, Regex> = HashMap::new();

        presets_map.insert("angular", Regex::new(ANGULAR_REGEX).unwrap());
        presets_map.insert(
            "angular-readme-only-docs",
            Regex::new(ANGULAR_README_ONLY_DOCS_REGEX).unwrap(),
        );
        presets_map.insert("no-changelog", Regex::new(NO_CHANGELOG_REGEX).unwrap());
        presets_map.insert("no-semver", Regex::new(NO_SEMVER_REGEX).unwrap());

        presets
            .iter()
            .filter_map(|p| {
                let found_preset = presets_map.get(p.as_str()).cloned();

                if found_preset.is_none() {
                    log_exit(&format!("unknown preset: {}", p));

                    process::exit(1)
                }

                found_preset
            })
            .collect::<Vec<Regex>>()
    }

    /// Returns hash map of processed args
    fn process_args(cli_args: &[String]) -> HashMap<String, HashSet<String>> {
        let mut processed_args: HashMap<String, HashSet<String>> = HashMap::new();

        let mut skip_next = false;

        cli_args.iter().enumerate().for_each(|(i, arg)| {
            if skip_next {
                skip_next = false;

                return;
            }

            let (key, val) = arg.split_once('=').unwrap_or((arg, ""));

            // Exit on unknown arg
            if !CliFlags::ALL.contains(&key) {
                log_exit(&format!("unknown arg: {}", key));

                process::exit(1)
            }

            let entry = processed_args
                .entry(key.to_string())
                .or_insert(HashSet::new());

            // Insert val from `<key>=<val>` format
            if !val.is_empty() {
                entry.insert(val.to_string());

                return;
            }

            let next_arg = &cli_args.get(i + 1);

            // If next arg starts with `--`, assume bool and insert empty string
            if next_arg.is_none() || next_arg.unwrap().starts_with("--") {
                entry.insert("".to_string());

                return;
            }

            // Insert next arg
            entry.insert(next_arg.unwrap().to_string());

            skip_next = true;
        });

        processed_args
    }
}

impl CliFlags {
    const ALL: [&'static str; 7] = [
        "--overwrite",
        "--no-links",
        "--max-commits",
        "--remote-url",
        "--tag-filter-regex",
        "--commit-filter-regex",
        "--commit-filter-preset",
    ];

    const OVERWRITE: &str = Self::ALL[0];
    const NO_LINKS: &str = Self::ALL[1];
    const MAX_COMMITS: &str = Self::ALL[2];
    const REMOTE_URL: &str = Self::ALL[3];
    const TAG_FILTER_REGEX: &str = Self::ALL[4];
    const COMMIT_FILTER_REGEX: &str = Self::ALL[5];
    const COMMIT_FILTER_PRESET: &str = Self::ALL[6];
}
