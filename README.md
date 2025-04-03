# changenog

[![GitHub Tag](https://img.shields.io/github/v/tag/Daniel-Knights/changenog)](https://github.com/Daniel-Knights/changenog/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/changenog?color=%23E5B14D)](https://crates.io/crates/changenog)
[![NPM Version](https://img.shields.io/npm/v/changenog?color=%23CB0000)](https://www.npmjs.com/package/changenog)
[![GitHub Actions Test Workflow Status](https://img.shields.io/github/actions/workflow/status/Daniel-Knights/changenog/test.yml?label=tests)](https://github.com/Daniel-Knights/changenog/actions/workflows/test.yml)

Zero-config changelog generator with monorepo support.

Parses Git tags and commits since last entry, restricts them by current working directory, and adds any missing entries to the changelog.

See this repo's changelog for example output.

- [Usage](#usage)
  - [Rust](#rust)
  - [JS](#js)
  - [Manual](#manual)
- [Supported Platforms](#supported-platforms)
- [Options](#options)
  - [Filters](#filters)
    - [Presets](#presets)
- [Entry Strategy](#entry-strategy)
- [Monorepo Support](#monorepo-support)

## Usage

### Rust

https://crates.io/crates/changenog

```bash
cargo install changenog
```

```bash
changenog
```

### JS

https://www.npmjs.com/package/changenog

```bash
pnpm i -D changenog
```

```json
{
  "scripts": {
    "changenog": "changenog"
  }
}
```

### Manual

Binaries are available on the [releases page](https://github.com/Daniel-Knights/changenog/releases).

## Supported Platforms

| Platform | Architecture | Notes                   |
| -------- | ------------ | ----------------------- |
| Linux    | x86_64       | Tested on Ubuntu latest |
| Linux    | aarch64      | Tested on Ubuntu 22.04  |
| MacOS    | x86_64       | Tested on MacOS 13.0    |
| MacOS    | aarch64      | Tested on MacOS latest  |
| Windows  | x86_64       | Tested on Windows 11    |
| Windows  | aarch64      | Untested ⚠️             |

## Options

| Option                 | Type    | Description                                                                                          |
| ---------------------- | ------- | ---------------------------------------------------------------------------------------------------- |
| --overwrite            | boolean | overwrite existing changelog                                                                         |
| --root                 | string  | root dir relative to the current working directory. default: current working directory               |
| --output               | string  | output of the generated changelog. one of ['file', 'stdout']. default: 'file'                        |
| --no-links             | boolean | disable links                                                                                        |
| --remote-url           | string  | remote URL to use for links. default: origin                                                         |
| --max-entries          | number  | maximum number of entries to process. default: '100'                                                 |
| --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included                                             |
| --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included                                          |
| --commit-filter-preset | string  | filter preset(s) to use. one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver'] |

### Filters

Multiple `--tag-filter-regex`, `--commit-filter-preset` and `--commit-filter-regex` options can be passed. A tag/commit must match _all_ filters to be included.

For example:

```sh
changenog --commit-filter-preset=angular --commit-filter-regex='^(?!.*changelog).*$'
```

Would filter:

```
✅
docs(readme): update usage
feat: add new feature
fix: fix bug
perf: improve performance

❌
chore: update dependencies
ci(release): release version
docs(changelog): v0.1.0
my commit message
```

#### Presets

- `angular` - include only `feat`, `fix`, `perf`, and `docs` commits that match the [Angular commit message convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)
- `angular-readme-only-docs` - exclude Angular `docs` commits unless they have a scope of `readme`
- `no-changelog` - exclude all commits with `changelog` in the subject
- `no-semver` - exclude all commits that match the semver format, specifically, [this regex](https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string)

## Entry Strategy

This is the general strategy for building and filtering entries:

1. All entries are processed, restricted by `--max-entries`
2. Commits are filtered to only include those with changes in `--root`
3. If `<tag a>..<tag b>` have no commits, the tags are made part of the same entry
4. Commit filters are applied
5. If an entry has no commits, the entry is excluded
6. Tag filters are applied
7. If an entry has no tags, all commits are merged into the next entry

## Monorepo Support

If the git root is in a parent directory, `changenog` will filter commits to only include those that have changes within the subdirectory.

To include only the _tags_ that apply to that subdirectory, you can provide a `--tag-filter-regex` option, e.g. `--tag-filter-regex='my-package/.*'`. This is assuming you have a convention for tagging that scopes to that subdirectory.
