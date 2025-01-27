use std::collections::HashMap;

use fancy_regex::Regex;

use crate::log::log_warn;

/// Structs

struct RawOptions {
    overwrite: bool,
    no_links: bool,
    max_commits: Vec<String>,
    remote_url: Vec<String>,
    tag_filter_regex: Vec<String>,
    commit_filter_regex: Vec<String>,
    commit_filter_preset: Vec<String>,
}

pub struct Options {
    pub overwrite: bool,
    pub no_links: bool,
    pub max_commits: i32,
    pub remote_url: Option<String>,
    pub tag_filter_regex: Vec<Regex>,
    pub filter: Vec<Regex>,
}

/// Implementations

impl Options {
    /// Gets formatted options from CLI args
    pub fn from_args(cli_args: Vec<String>) -> Options {
        let raw_opts = RawOptions {
            overwrite: cli_args.contains(&"--overwrite".to_string()),
            no_links: cli_args.contains(&"--no-links".to_string()),
            max_commits: Options::get_arg(&cli_args, "--max-commits"),
            remote_url: Options::get_arg(&cli_args, "--remote-url"),
            tag_filter_regex: Options::get_arg(&cli_args, "--tag-filter-regex"),
            commit_filter_regex: Options::get_arg(&cli_args, "--commit-filter-regex"),
            commit_filter_preset: Options::get_arg(&cli_args, "--commit-filter-preset"),
        };

        let commit_filter_presets =
            Options::get_commit_filter_presets(raw_opts.commit_filter_preset);

        Options {
            overwrite: raw_opts.overwrite,
            no_links: raw_opts.no_links,
            max_commits: raw_opts
                .max_commits
                .get(0)
                .unwrap_or(&"1000".to_string())
                .parse::<i32>()
                .expect("invalid max-commits arg"),
            remote_url: raw_opts.remote_url.get(0).cloned(),
            tag_filter_regex: raw_opts
                .tag_filter_regex
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid tag-filter-regex: {}", r)))
                .collect::<Vec<Regex>>(),
            filter: raw_opts
                .commit_filter_regex
                .iter()
                .map(|r| Regex::new(r).expect(&format!("invalid commit-filter-regex: {}", r)))
                .chain(commit_filter_presets)
                .collect::<Vec<Regex>>(),
        }
    }

    /// Returns presets matching those passed
    fn get_commit_filter_presets(presets: Vec<String>) -> Vec<Regex> {
        let mut presets_map: HashMap<String, Regex> = HashMap::new();

        presets_map.insert(
            "angular".to_string(),
            Regex::new(
                r"(?x)^
                    (?:feat|fix|perf|docs) # Type
                    (?:\(.+?\))?!?: # Scope
                    .* # Message
                $",
            )
            .unwrap(),
        );
        presets_map.insert(
            "angular-readme-only-docs".to_string(),
            Regex::new(
                r"(?x)^
                    (?!docs(?!\()|docs\((?!readme\)))
                    .*
                $",
            )
            .unwrap(),
        );
        presets_map.insert(
            "no-changelog".to_string(),
            Regex::new(
                r"(?x)^
                    (?!.*[^a-zA-Z]changelog[^a-zA-Z])
                    .*
                $",
            )
            .unwrap(),
        );

        presets
            .iter()
            .filter_map(|p| {
                let found_preset = presets_map.get(p.as_str()).cloned();

                if found_preset.is_none() {
                    log_warn(&format!("unknown preset: {}", p));
                }

                found_preset
            })
            .collect::<Vec<Regex>>()
    }

    /// Finds arg and returns the parsed value(s)
    fn get_arg(cli_args: &Vec<String>, arg: &str) -> Vec<String> {
        cli_args
            .iter()
            .filter_map(|a| {
                if a.starts_with(&format!("{}=", arg)) {
                    return Some(a.split("=").nth(1).unwrap().to_string());
                }

                None
            })
            .collect::<Vec<String>>()
    }
}
