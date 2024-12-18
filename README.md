# changenog

Zero-config changelog generator with monorepo support.

Parses Git tags and commits since last entry, restricts them by current working directory, and adds any missing entries to the changelog.

Running `changenog` from inside a subdirectory will find the nearest parent that's a git repository, and filter commits to only include those that have changes within that subdirectory.

**NOTE:** monorepo packages must be tagged in `<package-name>/<version>` format.

## Usage

```bash
npm i -D changenog
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

- `--no-links` - disable links
- `--continue` - allow subsequent commands to run on incomplete exit
- `--max-commits=<number>` - maximum number of commits to process (default: 1000)
- `--locale=<string>` - valid locale string passed to `Intl.DateTimeFormat` (default: `en-GB`)
