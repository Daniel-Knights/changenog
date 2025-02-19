import fs from "node:fs";

import { commit, output, run } from "./utils.js";

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
  ["--max-commits=3"],
  ["--tag-filter-regex='my-package/.*'"],
  ["--commit-filter-regex='^(?!.*changelog).*$'"],
  ["--commit-filter-preset=angular"],
  ["--commit-filter-preset=angular", "--commit-filter-preset=angular-readme-only-docs"],
  ["--commit-filter-preset=no-changelog"],
  ["--commit-filter-preset=no-semver", "--commit-filter-regex='^(?!.*changelog).*$'"],
];

if (fs.existsSync("test/output/stdout.txt")) {
  fs.rmSync("test/output/stdout.txt");
}

if (fs.existsSync("test/output/changelogs")) {
  fs.rmSync("test/output/changelogs", { recursive: true });
}

if (fs.existsSync("test/repo")) {
  fs.rmSync("test/repo", { recursive: true });
}

fs.mkdirSync("test/repo");
fs.mkdirSync("test/repo/foo");
fs.mkdirSync("test/repo/bar");
fs.mkdirSync("test/repo/bar/baz");

run("git", ["init"]);

// Run without remote, tags, or commits
tests.forEach((testArgs) => {
  output("no_repo", testArgs);
});

// Run without remote or tags
commit("feat: add foo", "foo");
commit("feat: add bar", "bar");
commit("feat: add baz", "bar/baz");

tests.forEach((testArgs) => {
  output("no_remote_or_tags", testArgs);
});

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

mockCommits.forEach((commitMessage) => {
  commit(commitMessage, "foo");
});

run("git", ["tag", "v0.0.1"]);

mockCommits.forEach((commitMessage) => {
  commit(commitMessage, "bar");
});

run("git", ["tag", "v0.1.0"]);

mockCommits.forEach((commitMessage) => {
  commit(commitMessage, "bar/baz");
});

run("git", ["tag", "my-package/v1.0.0"]);
run("git", ["tag", "v1.0.0"]);

// Run without remote
tests.forEach((testArgs) => {
  output("no_remote", testArgs);
});

// Run with remote
run("git", ["config", "remote.origin.url", "https://www.my-remote.com"]);

tests.forEach((testArgs) => {
  output("with_remote", testArgs);
});

// Run with empty changelog
fs.writeFileSync("test/repo/CHANGELOG.md", "");

tests.forEach((testArgs) => {
  output("empty_changelog", testArgs);
});

// Run with partial changelog
fs.copyFileSync("test/changelogs/PARTIAL.md", "test/repo/CHANGELOG.md");

tests.forEach((testArgs) => {
  output("partial_changelog", testArgs);
});

// Run with full changelog
fs.copyFileSync("test/changelogs/FULL.md", "test/repo/CHANGELOG.md");

tests.forEach((testArgs) => {
  output("full_changelog", testArgs);
});

// Cleanup
fs.rmSync("test/repo", { recursive: true });
