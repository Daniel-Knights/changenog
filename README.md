# changenog

<!-- TODO: mention semver requirement -->

Zero-config changelog generator with monorepo support.

Parses Git tags and commits since last entry, restricts them by current working directory, and adds any missing entries to the changelog.

Running `changenog` from inside a subdirectory will find the nearest parent that's a git repository, and filter commits to only include those that have changes within that subdirectory.

## Usage

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

See this repo's changelog for example output.

## Options

- `--overwrite` - overwrite existing changelog
- `--no-links` - disable links
- `--max-commits=<number>` - maximum number of commits to process (default: 1000)
- `--remote-url=<string>` - remote URL to use for links (default: origin)
- `--tag-prefix=<string>` - prefix for git tags, required for monorepo packages
- `--filter-preset=<string>` - filter preset(s) to use
- `--filter-regex=<string>` - regex pattern(s) that each commit must match to be included

### Filters

Multiple `--filter-preset` and `--filter-regex` options can be used. A commit must match _all_ filters to be included.

For example:

```sh
changenog --filter-preset=angular --filter-regex='^(?!.*changelog).*$'
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
