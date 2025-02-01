use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
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

#[derive(PartialEq)]
enum CliArgKind {
    Boolean,
    String,
    Number,
    Regex,
}

struct CliArg {
    name: &'static str,
    kind: CliArgKind,
    description: &'static str,
}

//// Implementations

impl ChangenogOptions {
    const DEFINITIONS: [CliArg; 7] = [
        CliArg {
            name: "--overwrite",
            kind: CliArgKind::Boolean,
            description: "overwrite existing changelog",
        },
        CliArg {
            name: "--no-links",
            kind: CliArgKind::Boolean,
            description: "disable links",
        },
        CliArg {
            name: "--remote-url",
            kind: CliArgKind::String,
            description: "remote URL to use for links (default: origin)",
        },
        CliArg {
            name: "--max-commits",
            kind: CliArgKind::Number,
            description: "maximum number of commits to process (default: 1000)",
        },
        CliArg {
            name: "--tag-filter-regex",
            kind: CliArgKind::Regex,
            description: "regex pattern(s) that each tag must match to be included",
        },
        CliArg {
            name: "--commit-filter-regex",
            kind: CliArgKind::Regex,
            description: "regex pattern(s) that each commit must match to be included",
        },
        CliArg {
            name: "--commit-filter-preset",
            kind: CliArgKind::String,
            description: "filter preset(s) to use",
        },
    ];

    const OVERWRITE: &'static CliArg = &Self::DEFINITIONS[0];
    const NO_LINKS: &'static CliArg = &Self::DEFINITIONS[1];
    const MAX_COMMITS: &'static CliArg = &Self::DEFINITIONS[2];
    const REMOTE_URL: &'static CliArg = &Self::DEFINITIONS[3];
    const TAG_FILTER_REGEX: &'static CliArg = &Self::DEFINITIONS[4];
    const COMMIT_FILTER_REGEX: &'static CliArg = &Self::DEFINITIONS[5];
    const COMMIT_FILTER_PRESET: &'static CliArg = &Self::DEFINITIONS[6];

    /// Gets formatted options from CLI args
    pub fn from_args(cli_args: &[String]) -> Self {
        let processed_args = Self::process_args(cli_args);

        let commit_filter_presets = Self::get_commit_filter_presets(
            processed_args
                .get(Self::COMMIT_FILTER_PRESET.name)
                .unwrap_or(&HashSet::new()),
        );

        Self {
            overwrite: processed_args.contains_key(Self::OVERWRITE.name),
            no_links: processed_args.contains_key(Self::NO_LINKS.name),
            max_commits: processed_args
                .get(Self::MAX_COMMITS.name)
                .unwrap_or(&HashSet::from(["1000".to_string()]))
                .iter()
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .expect("invalid max-commits arg"),
            remote_url: processed_args
                .get(Self::REMOTE_URL.name)
                .unwrap_or(&HashSet::new())
                .iter()
                .nth(0)
                .map(|s| s.to_string()),
            tag_filters: processed_args
                .get(Self::TAG_FILTER_REGEX.name)
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid tag-filter-regex: {}", r)))
                .collect::<Vec<Regex>>(),
            commit_filters: processed_args
                .get(Self::COMMIT_FILTER_REGEX.name)
                .unwrap_or(&HashSet::new())
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid commit-filter-regex: {}", r)))
                .chain(commit_filter_presets)
                .collect::<Vec<Regex>>(),
        }
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
            let found_opt = Self::DEFINITIONS.iter().find(|f| f.name == key);

            // Exit on unknown arg
            if found_opt.is_none() {
                log_exit(&format!("unknown arg: {}", key));

                process::exit(1)
            }

            let opt = found_opt.unwrap();

            let entry = processed_args
                .entry(opt.name.to_string())
                .or_insert(HashSet::new());

            // Insert val from `<key>=<val>` format
            if !val.is_empty() {
                Self::assert_not_boolean(opt, arg);

                entry.insert(val.to_string());

                return;
            }

            let next_arg = &cli_args.get(i + 1);

            // If next arg starts with `--`, assume bool and insert empty string
            if next_arg.is_none() || next_arg.unwrap().starts_with("--") {
                entry.insert("".to_string());

                return;
            }

            Self::assert_not_boolean(opt, arg);

            // Insert next arg
            entry.insert(next_arg.unwrap().to_string());

            skip_next = true;
        });

        processed_args
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

    pub fn help() {
        let (longest_name_len, longest_kind_len) =
            Self::DEFINITIONS.iter().fold((0, 0), |acc, d| {
                (acc.0.max(d.name.len()), acc.1.max(d.kind.to_string().len()))
            });

        println!("Changenog options:");

        Self::DEFINITIONS.iter().for_each(|d| {
            println!(
                "  {}{} | {}{} | {}",
                d.name,
                " ".repeat(longest_name_len - d.name.len()),
                d.kind,
                " ".repeat(longest_kind_len - d.kind.to_string().len()),
                d.description
            );
        });
    }

    /// If `opt` is boolean, logs an error and exits the program
    fn assert_not_boolean(opt: &CliArg, arg: &str) {
        if opt.kind != CliArgKind::Boolean {
            return;
        }

        log_exit(&format!("unexpected value for boolean flag: {}", arg));

        process::exit(1)
    }
}

impl Display for CliArgKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Boolean => "boolean",
            Self::String => "string",
            Self::Number => "number",
            Self::Regex => "regex",
        };

        write!(f, "{}", kind)
    }
}
