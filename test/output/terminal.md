## test no_repo: ["--version"]

### stdout

```
1.2.0
```

## test no_repo: ["-v"]

### stdout

```
1.2.0
```

## test no_repo: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_repo: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_repo: ["--overwrite"]

### stdout

```
[33m[changenog][0m no entries generated.  exiting...
```

## test no_repo: ["--root=./bar/baz"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--output stdout"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--no-links"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--remote-url=https://www.my-repo.com"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--max-entries 2"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--tag-filter-regex=my-package/.*"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--commit-filter-regex ^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--commit-filter-preset=angular"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--commit-filter-preset=no-changelog"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_repo: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--version"]

### stdout

```
1.2.0
```

## test no_remote_or_tags: ["-v"]

### stdout

```
1.2.0
```

## test no_remote_or_tags: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_remote_or_tags: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_remote_or_tags: ["--overwrite"]

### stdout

```
[33m[changenog][0m no entries generated.  exiting...
```

## test no_remote_or_tags: ["--root=./bar/baz"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--output stdout"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--no-links"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--remote-url=https://www.my-repo.com"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--max-entries 2"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--tag-filter-regex=my-package/.*"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--commit-filter-regex ^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--commit-filter-preset=angular"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--commit-filter-preset=no-changelog"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote_or_tags: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test no_remote: ["--version"]

### stdout

```
1.2.0
```

## test no_remote: ["-v"]

### stdout

```
1.2.0
```

## test no_remote: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_remote: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test no_remote: ["--overwrite"]

## test no_remote: ["--root=./bar/baz"]

## test no_remote: ["--output stdout"]

### stdout

```
## my-package/v1.0.0, v1.0.0 (REPLACED)

- 1.0.0 (REPLACED)
- non-angular commit (REPLACED)
- refactor: refactor code (REPLACED)
- docs(changelog): 1.0.0 (REPLACED)
- chore(scripts): update scripts (REPLACED)
- feat(scope): add another feature (REPLACED)
- fix: fix bug (REPLACED)
- feat: add feature (REPLACED)
- docs(readme): update readme (REPLACED)

## v0.1.0 (REPLACED)

- 1.0.0 (REPLACED)
- non-angular commit (REPLACED)
- refactor: refactor code (REPLACED)
- docs(changelog): 1.0.0 (REPLACED)
- chore(scripts): update scripts (REPLACED)
- feat(scope): add another feature (REPLACED)
- fix: fix bug (REPLACED)
- feat: add feature (REPLACED)
- docs(readme): update readme (REPLACED)

## v0.0.1 (REPLACED)

- 1.0.0 (REPLACED)
- non-angular commit (REPLACED)
- refactor: refactor code (REPLACED)
- docs(changelog): 1.0.0 (REPLACED)
- chore(scripts): update scripts (REPLACED)
- feat(scope): add another feature (REPLACED)
- fix: fix bug (REPLACED)
- feat: add feature (REPLACED)
- docs(readme): update readme (REPLACED)
- feat: add baz (REPLACED)
- feat: add bar (REPLACED)
- feat: add foo (REPLACED)
```

## test no_remote: ["--no-links"]

## test no_remote: ["--remote-url=https://www.my-repo.com"]

## test no_remote: ["--max-entries 2"]

## test no_remote: ["--tag-filter-regex=my-package/.*"]

## test no_remote: ["--commit-filter-regex ^(?!.*changelog).*$"]

## test no_remote: ["--commit-filter-preset=angular"]

## test no_remote: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

## test no_remote: ["--commit-filter-preset=no-changelog"]

## test no_remote: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

## test with_remote: ["--version"]

### stdout

```
1.2.0
```

## test with_remote: ["-v"]

### stdout

```
1.2.0
```

## test with_remote: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test with_remote: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test with_remote: ["--overwrite"]

## test with_remote: ["--root=./bar/baz"]

## test with_remote: ["--output stdout"]

### stdout

```
## [my-package/v1.0.0, v1.0.0](https://www.my-remote.com/compare/v0.1.0...my-package/v1.0.0) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.1.0](https://www.my-remote.com/compare/v0.0.1...v0.1.0) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.0.1](https://www.my-remote.com/tags) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add baz ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add bar ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add foo ([REPLACED](https://www.my-remote.com/commit/REPLACED))
```

## test with_remote: ["--no-links"]

## test with_remote: ["--remote-url=https://www.my-repo.com"]

## test with_remote: ["--max-entries 2"]

## test with_remote: ["--tag-filter-regex=my-package/.*"]

## test with_remote: ["--commit-filter-regex ^(?!.*changelog).*$"]

## test with_remote: ["--commit-filter-preset=angular"]

## test with_remote: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

## test with_remote: ["--commit-filter-preset=no-changelog"]

## test with_remote: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

## test empty_changelog: ["--version"]

### stdout

```
1.2.0
```

## test empty_changelog: ["-v"]

### stdout

```
1.2.0
```

## test empty_changelog: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test empty_changelog: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test empty_changelog: ["--overwrite"]

## test empty_changelog: ["--root=./bar/baz"]

## test empty_changelog: ["--output stdout"]

### stdout

```
## [my-package/v1.0.0, v1.0.0](https://www.my-remote.com/compare/v0.1.0...my-package/v1.0.0) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.1.0](https://www.my-remote.com/compare/v0.0.1...v0.1.0) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.0.1](https://www.my-remote.com/tags) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add baz ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add bar ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add foo ([REPLACED](https://www.my-remote.com/commit/REPLACED))
```

## test empty_changelog: ["--no-links"]

## test empty_changelog: ["--remote-url=https://www.my-repo.com"]

## test empty_changelog: ["--max-entries 2"]

## test empty_changelog: ["--tag-filter-regex=my-package/.*"]

## test empty_changelog: ["--commit-filter-regex ^(?!.*changelog).*$"]

## test empty_changelog: ["--commit-filter-preset=angular"]

## test empty_changelog: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

## test empty_changelog: ["--commit-filter-preset=no-changelog"]

## test empty_changelog: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

## test partial_changelog: ["--version"]

### stdout

```
1.2.0
```

## test partial_changelog: ["-v"]

### stdout

```
1.2.0
```

## test partial_changelog: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test partial_changelog: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test partial_changelog: ["--overwrite"]

## test partial_changelog: ["--root=./bar/baz"]

## test partial_changelog: ["--output stdout"]

### stdout

```
## [my-package/v1.0.0, v1.0.0](https://www.my-remote.com/compare/v0.1.0...my-package/v1.0.0) (REPLACED)

- 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- non-angular commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- refactor: refactor code ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(changelog): 1.0.0 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- chore(scripts): update scripts ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat(scope): add another feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- fix: fix bug ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- feat: add feature ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- docs(readme): update readme ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.1.0](https://www.my-remote.com/tags) (REPLACED)

- my-commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- my-commit2 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- my-commit3 ([REPLACED](https://www.my-remote.com/commit/REPLACED))

## [v0.0.1](https://www.my-remote.com/tags) (REPLACED)

- my-commit ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- my-commit2 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
- my-commit3 ([REPLACED](https://www.my-remote.com/commit/REPLACED))
```

## test partial_changelog: ["--no-links"]

## test partial_changelog: ["--remote-url=https://www.my-repo.com"]

## test partial_changelog: ["--max-entries 2"]

## test partial_changelog: ["--tag-filter-regex=my-package/.*"]

## test partial_changelog: ["--commit-filter-regex ^(?!.*changelog).*$"]

## test partial_changelog: ["--commit-filter-preset=angular"]

## test partial_changelog: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

## test partial_changelog: ["--commit-filter-preset=no-changelog"]

## test partial_changelog: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

## test full_changelog: ["--version"]

### stdout

```
1.2.0
```

## test full_changelog: ["-v"]

### stdout

```
1.2.0
```

## test full_changelog: ["--help"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test full_changelog: ["-h"]

### stdout

```
Changenog options:
  --overwrite            | boolean | overwrite existing changelog
  --root                 | string  | root dir relative to the current working directory.  default: current working directory
  --output               | string  | output of the generated changelog.  one of ['file', 'stdout'].  default: 'file'
  --no-links             | boolean | disable links
  --remote-url           | string  | remote URL to use for links.  default: origin
  --max-entries          | number  | maximum number of entries to process.  default: '100'
  --tag-filter-regex     | regex   | regex pattern(s) that each tag must match to be included
  --commit-filter-regex  | regex   | regex pattern(s) that each commit must match to be included
  --commit-filter-preset | string  | filter preset(s) to use.  one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver']
```

## test full_changelog: ["--overwrite"]

## test full_changelog: ["--root=./bar/baz"]

## test full_changelog: ["--output stdout"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--no-links"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--remote-url=https://www.my-repo.com"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--max-entries 2"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--tag-filter-regex=my-package/.*"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--commit-filter-regex ^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--commit-filter-preset=angular"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--commit-filter-preset angular --commit-filter-preset=angular-readme-only-docs"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--commit-filter-preset=no-changelog"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test full_changelog: ["--commit-filter-preset no-semver --commit-filter-regex=^(?!.*changelog).*$"]

### stdout

```
[33m[changenog][0m no new version(s).  exiting...
```

## test args: ["--overwrite val"]

### stderr

```
[33m[changenog][0m unexpected value for boolean option: '--overwrite=val'.  exiting...
```

## test args: ["--root"]

### stderr

```
[33m[changenog][0m expected value for arg: '--root'.  exiting...
```

## test args: ["--root=./quux"]

### stderr

```
[33m[changenog][0m invalid root path: '--root=./quux'.  err: 'No such file or directory (os error 2)'.  exiting...
```

## test args: ["--root=../../"]

### stderr

```
[33m[changenog][0m root must be within the current working directory: '--root=../../'.  exiting...
```

## test args: ["--max-entries foo"]

### stderr

```
[33m[changenog][0m unable to parse max-entries: '--max-entries=foo'.  err: 'invalid digit found in string'.  exiting...
```

## test args: ["--tag-filter-regex"]

### stderr

```
[33m[changenog][0m expected value for arg: '--tag-filter-regex'.  exiting...
```

## test args: ["--tag-filter-regex (?!)*"]

### stderr

```
[33m[changenog][0m invalid regex: '--tag-filter-regex=(?!)*'.  err: 'Parsing error at position 4: Target of repeat operator is invalid'.  exiting...
```

## test args: ["--commit-filter-preset=foo"]

### stderr

```
[33m[changenog][0m invalid value for option: '--commit-filter-preset=foo'.  must be one of ['angular', 'angular-readme-only-docs', 'no-changelog', 'no-semver'].  exiting...
```

