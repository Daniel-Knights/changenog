# changenog

Zero-config changelog generator with monorepo support.

Parses Git tags and commits since last entry, restricts them by current working directory, and adds any missing entries to the changelog.

See this repo's changelog for example output.

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

## Options

| Option                 | Type    | Description                                                                                          |
| ---------------------- | ------- | ---------------------------------------------------------------------------------------------------- |
| --overwrite            | boolean | overwrite existing changelog                                                                         |
| --input-path           | string  | path to the source changelog within the current working directory. default: 'CHANGELOG.md'           |
| --output               | string  | output of the generated changelog. one of ['file', 'stdout']. default: 'file'                        |
| --no-links             | boolean | disable links                                                                                        |
| --remote-url           | string  | remote URL to use for links. default: origin                                                         |
| --max-commits          | number  | maximum number of commits to process. default: '1000'                                                |
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

## Monorepo Support

When run inside a subdirectory, `changenog` will find the nearest parent that's a git repository, and filter commits to only include those that have changes within the subdirectory.

To include only the _tags_ that apply to that subdirectory, you can provide a `--tag-filter-regex` option, e.g. `--tag-filter-regex='my-package/.*'`.
