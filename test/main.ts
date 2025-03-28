import fs from "node:fs";

import { GitManager } from "./git.js";
import { commitAll, suite } from "./utils.js";

const startTimestamp = Date.now();

const tests = [
  ["--version"],
  ["-v"],
  ["--help"],
  ["-h"],
  ["--overwrite"],
  ["--root=./bar/baz"],
  ["--output=stdout"],
  ["--no-links"],
  ["--remote-url=https://www.my-repo.com"],
  ["--max-entries=2"],
  ["--tag-filter-regex=my-package/.*"],
  ["--commit-filter-regex=^(?!.*changelog).*$"],
  ["--commit-filter-preset=angular"],
  ["--commit-filter-preset=angular", "--commit-filter-preset=angular-readme-only-docs"],
  ["--commit-filter-preset=no-changelog"],
  ["--commit-filter-preset=no-semver", "--commit-filter-regex=^(?!.*changelog).*$"],
];

if (fs.existsSync("test/output/stdout.txt")) {
  await fs.promises.rm("test/output/stdout.txt");
}

if (fs.existsSync("test/output/changelogs")) {
  await fs.promises.rm("test/output/changelogs", { recursive: true });
}

if (fs.existsSync("test/repo")) {
  await fs.promises.rm("test/repo", { recursive: true });
}

await fs.promises.mkdir("test/repo");
await fs.promises.mkdir("test/repo/foo");
await fs.promises.mkdir("test/repo/bar");
await fs.promises.mkdir("test/repo/bar/baz");

GitManager.init();

// Run without remote, tags, or commits
await suite("no_repo", tests);

// Run without remote or tags
await commitAll("foo", ["feat: add foo"]);
await commitAll("bar", ["feat: add bar"]);
await commitAll("bar/baz", ["feat: add baz"]);

await suite("no_remote_or_tags", tests);

// Add mock commits and tags
const mockCommits = [
  "docs(readme): update readme",
  "feat: add feature",
  "fix: fix bug",
  "feat(scope): add another feature",
  "chore(scripts): update scripts",
  "docs(changelog): 1.0.0",
  "refactor: refactor code",
  "non-angular commit",
  "1.0.0",
];

await commitAll("foo", mockCommits);

GitManager.tag("v0.0.1");

await commitAll("bar", mockCommits);

GitManager.tag("v0.1.0");

await commitAll("bar/baz", mockCommits);

GitManager.tag("my-package/v1.0.0");
GitManager.tag("v1.0.0");

// Run without remote
await suite("no_remote", tests);

// Run with remote
GitManager.setRemote("https://www.my-remote.com");

await suite("with_remote", tests);

// Run with empty changelog
await fs.promises.writeFile("test/repo/CHANGELOG.md", "");
await suite("empty_changelog", tests);

// Run with partial changelog
await suite("partial_changelog", tests, "PARTIAL");

// Run with full changelog
await suite("full_changelog", tests, "FULL");

// Cleanup
await fs.promises.rm("test/repo", { recursive: true });

console.log(`Tests completed in ${Date.now() - startTimestamp}ms`);
