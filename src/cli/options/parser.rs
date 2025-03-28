use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    process,
};

use fancy_regex::Regex;

use crate::{
    cli::validator::Validator,
    constant::{
        ANGULAR_README_ONLY_DOCS_REGEX, ANGULAR_REGEX, NO_CHANGELOG_REGEX, NO_SEMVER_REGEX,
    },
    log::log_exit,
};

use super::options::ChangenogOptions;

type ParsedArgs = HashMap<String, HashSet<String>>;

//// Implementations

impl ChangenogOptions {
    /// Gets parsed options from CLI args
    pub fn from_args(cli_args: &[String]) -> ChangenogOptions {
        let parsed_args = ChangenogOptions::parse_args(cli_args);

        ChangenogOptions {
            overwrite: Self::parse_overwrite(&parsed_args),
            root: Self::parse_root(&parsed_args),
            output: Self::parse_output(&parsed_args),
            no_links: Self::parse_no_links(&parsed_args),
            max_entries: Self::parse_max_entries(&parsed_args),
            remote_url: Self::parse_remote_url(&parsed_args),
            tag_filters: Self::parse_tag_filters(&parsed_args),
            commit_filters: Self::parse_commit_filters(&parsed_args),
        }
    }

    //// Private

    fn parse_args(cli_args: &[String]) -> ParsedArgs {
        let mut parsed_args: ParsedArgs = HashMap::new();

        // Next may be the value for current, so skip it
        let mut skip_next = false;

        cli_args.iter().enumerate().for_each(|(i, arg)| {
            if skip_next {
                skip_next = false;

                return;
            }

            let (key, val) = arg.split_once('=').unwrap_or((arg, ""));
            let found_opt = ChangenogOptions::DEFINITIONS.iter().find(|f| f.name == key);

            // Exit on unknown arg
            if found_opt.is_none() {
                log_exit(&format!("unknown arg: {}", key));

                process::exit(1)
            }

            let opt = found_opt.unwrap();

            // Validate arg
            if !Validator::validate_arg(opt, val) {
                // TODO: defer exits to main?
                process::exit(1);
            }

            let entry = parsed_args
                .entry(opt.name.to_string())
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
            let val = next_arg.unwrap();

            entry.insert(val.clone());

            skip_next = true;
        });

        parsed_args
    }

    fn parse_overwrite(parsed_args: &ParsedArgs) -> bool {
        parsed_args.contains_key(ChangenogOptions::OVERWRITE.name)
    }

    fn parse_root(parsed_args: &ParsedArgs) -> PathBuf {
        let default_val = HashSet::from([".".to_string()]);

        let root_string = parsed_args
            .get(ChangenogOptions::ROOT.name)
            .unwrap_or(&default_val)
            .iter()
            .nth(0)
            .unwrap();

        PathBuf::from(root_string).canonicalize().unwrap()
    }

    fn parse_output(parsed_args: &ParsedArgs) -> String {
        let default_val = HashSet::from([ChangenogOptions::OUTPUT.default.unwrap().to_string()]);

        parsed_args
            .get(ChangenogOptions::OUTPUT.name)
            .unwrap_or(&default_val)
            .iter()
            .nth(0)
            .unwrap()
            .clone()
    }

    fn parse_no_links(parsed_args: &ParsedArgs) -> bool {
        parsed_args.contains_key(ChangenogOptions::NO_LINKS.name)
    }

    fn parse_max_entries(parsed_args: &ParsedArgs) -> usize {
        let default_val =
            HashSet::from([ChangenogOptions::MAX_ENTRIES.default.unwrap().to_string()]);

        parsed_args
            .get(ChangenogOptions::MAX_ENTRIES.name)
            .unwrap_or(&default_val)
            .iter()
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .expect("invalid max-entries arg")
    }

    fn parse_remote_url(parsed_args: &ParsedArgs) -> Option<String> {
        parsed_args
            .get(ChangenogOptions::REMOTE_URL.name)
            .unwrap_or(&HashSet::new())
            .iter()
            .nth(0)
            .map(|s| s.to_string())
    }

    fn parse_tag_filters(parsed_args: &ParsedArgs) -> Vec<Regex> {
        parsed_args
            .get(ChangenogOptions::TAG_FILTER_REGEX.name)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|r| Regex::new(r).expect(&format!("invalid tag-filter-regex: {}", r)))
            .collect::<Vec<Regex>>()
    }

    fn parse_commit_filters(parsed_args: &ParsedArgs) -> Vec<Regex> {
        let default_val = HashSet::new();

        let preset_names = parsed_args
            .get(ChangenogOptions::COMMIT_FILTER_PRESET.name)
            .unwrap_or(&default_val);

        let commit_filter_presets = Self::get_commit_filter_presets(preset_names);

        parsed_args
            .get(ChangenogOptions::COMMIT_FILTER_REGEX.name)
            .unwrap_or(&default_val)
            .iter()
            .map(|r| Regex::new(r).expect(&format!("invalid commit-filter-regex: {}", r)))
            .chain(commit_filter_presets)
            .collect::<Vec<Regex>>()
    }

    /// Returns presets matching those passed
    fn get_commit_filter_presets(preset_names: &HashSet<String>) -> Vec<Regex> {
        let mut presets_map: HashMap<&str, Regex> = HashMap::new();

        presets_map.insert("angular", Regex::new(ANGULAR_REGEX).unwrap());
        presets_map.insert(
            "angular-readme-only-docs",
            Regex::new(ANGULAR_README_ONLY_DOCS_REGEX).unwrap(),
        );
        presets_map.insert("no-changelog", Regex::new(NO_CHANGELOG_REGEX).unwrap());
        presets_map.insert("no-semver", Regex::new(NO_SEMVER_REGEX).unwrap());

        preset_names
            .iter()
            .filter_map(|p| presets_map.get(p.as_str()).cloned())
            .collect::<Vec<Regex>>()
    }
}
