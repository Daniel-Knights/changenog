# changenog

Zero-config changelog generator with monorepo support.

Parses Git commits since last entry, restricts them by current working directory, and adds a new entry to the changelog.

Running `changenog` from inside a subdirectory will find the nearest parent that's a git repository, and filter commits to only include those that have changes within that subdirectory.

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

- `--no-links` - Disable links

## Supported remote links

- GitHub
- GitLab
