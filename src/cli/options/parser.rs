use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    path::PathBuf,
};

use fancy_regex::Regex;

use crate::{
    cli::validator::{ValidateArgError, Validator},
    constant::{
        ANGULAR_README_ONLY_DOCS_REGEX, ANGULAR_REGEX, NO_CHANGELOG_REGEX, NO_SEMVER_REGEX,
    },
};

use super::options::ChangenogOptions;

//// Structs

type ParsedArgs = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub enum ParseArgError {
    Unknown(String),
    Invalid(ValidateArgError),
}

impl Display for ParseArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseArgError::Unknown(v) => write!(f, "unknown arg: {v}"),
            ParseArgError::Invalid(e) => write!(f, "{e}"),
        }
    }
}

//// Implementations

impl ChangenogOptions {
    /// Gets parsed options from CLI args
    pub fn from_args(cli_args: &[String]) -> Result<ChangenogOptions, ParseArgError> {
        let parsed_args = ChangenogOptions::parse_args(cli_args)?;

        Ok(ChangenogOptions {
            overwrite: Self::parse_overwrite(&parsed_args),
            root: Self::parse_root(&parsed_args),
            output: Self::parse_output(&parsed_args),
            no_links: Self::parse_no_links(&parsed_args),
            max_entries: Self::parse_max_entries(&parsed_args),
            remote_url: Self::parse_remote_url(&parsed_args),
            tag_filters: Self::parse_tag_filters(&parsed_args),
            commit_filters: Self::parse_commit_filters(&parsed_args),
        })
    }

    //// Private

    fn parse_args(cli_args: &[String]) -> Result<ParsedArgs, ParseArgError> {
        let mut parsed_args: ParsedArgs = HashMap::new();

        // Next may be the value for current, so skip it
        let mut skip_next = false;

        for (i, arg) in cli_args.iter().enumerate() {
            if skip_next {
                skip_next = false;

                continue;
            }

            let (key, val) = arg.split_once('=').unwrap_or((arg, ""));
            let found_opt = ChangenogOptions::DEFINITIONS.iter().find(|f| f.name == key);

            // Exit on unknown arg
            if let None = found_opt {
                return Err(ParseArgError::Unknown(key.to_string()));
            }

            let opt = found_opt.unwrap();
            let next_arg = &cli_args.get(i + 1);

            let val = if !val.is_empty() {
                // `<key>=<val>` format
                val.to_string()
            } else if next_arg.is_none() || next_arg.unwrap().starts_with("--") {
                // If next arg starts with `--`, assume bool and insert empty string
                String::new()
            } else {
                // Next arg is value for current
                next_arg.unwrap().clone()
            };

            // Skip next arg if it's the value for current
            skip_next = next_arg.is_some_and(|a| a == &val);

            // Validate arg
            if let Err(err) = Validator::validate_arg(opt, &val) {
                return Err(ParseArgError::Invalid(err));
            }

            parsed_args
                .entry(opt.name.to_string())
                .or_insert(HashSet::new())
                .insert(val);
        }

        Ok(parsed_args)
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
            .map(|r| Regex::new(r).expect(&format!("invalid tag-filter-regex: '{}'", r)))
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
            .map(|r| Regex::new(r).expect(&format!("invalid commit-filter-regex: '{}'", r)))
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
