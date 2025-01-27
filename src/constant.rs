pub const LOG_PREFIX: &str = "\x1b[33m[changenog]\x1b[0m";

//// Filter presets

pub const ANGULAR_REGEX: &str = r"(?x)^
    (?:feat|fix|perf|docs) # Type
    (?:\(.+?\))?!?: # Scope
    .* # Message
$";

pub const ANGULAR_README_ONLY_DOCS_REGEX: &str = r"(?x)^
    (?!docs(?!\()|docs\((?!readme\)))
    .*
$";

pub const NO_CHANGELOG_REGEX: &str = r"(?x)^
    (?!.*[^a-zA-Z]changelog[^a-zA-Z])
    .*
$";

// https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
pub const SEMVER_REGEX: &str = r"(?x)^
    (?!
        (0|[1-9]\d*)\.
        (0|[1-9]\d*)\.
        (0|[1-9]\d*)
        (?:-
            (
                (?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)
                (?:\.
                    (?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)
                )*
            )
        )?
        (?:\+
            ([0-9a-zA-Z-]+
                (?:\.[0-9a-zA-Z-]+)
            *)
        )?
    )
    .*
$";
