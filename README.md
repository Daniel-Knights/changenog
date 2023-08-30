# changenog

Zero-config changelog generator with monorepo support.

Parses Git commits since last entry, restricts them by current working directory, and adds a new entry to the changelog.

Running `changenog` from inside a subdirectory will find the nearest parent that's a git repository, and filter commits to only include those that have changes within that subdirectory.

Requirements:

- `package.json` with name and version
- At least one Git tag
- Monorepo package tags must be in the format `<package-name>/<version>`

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
