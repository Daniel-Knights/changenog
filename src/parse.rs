use fancy_regex::Regex;

/// https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
pub const SEMVER_REGEX: &str = r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?";

/// Returns matched semver version
pub fn match_version(match_str: &str) -> Option<String> {
    Regex::new(SEMVER_REGEX)
        .unwrap()
        .find(match_str)
        .unwrap()
        .map(|m| m.as_str().to_string())
}
