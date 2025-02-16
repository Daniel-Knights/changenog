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

fs.mkdirSync("test/output/changelogs");
fs.mkdirSync("test/output/changelogs/foo");
fs.mkdirSync("test/output/changelogs/bar");
fs.mkdirSync("test/output/changelogs/bar/baz");

fs.mkdirSync("test/repo");
fs.mkdirSync("test/repo/foo");
fs.mkdirSync("test/repo/bar");
fs.mkdirSync("test/repo/bar/baz");

run("git", ["init"]);

// Run without remote, tags, or commits
tests.forEach((testArgs) => {
  output(testArgs);
});

// Run without remote or tags
commit("foo", "feat: add foo");
commit("bar", "feat: add bar");
commit("bar/baz", "feat: add baz");

tests.forEach((testArgs) => {
  output(testArgs);
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
  commit("foo", commitMessage);
});

run("git", ["tag", "v0.0.1"]);

mockCommits.forEach((commitMessage) => {
  commit("bar", commitMessage);
});

run("git", ["tag", "v0.1.0"]);

mockCommits.forEach((commitMessage) => {
  commit("bar/baz", commitMessage);
});

run("git", ["tag", "my-package/v1.0.0"]);
run("git", ["tag", "v1.0.0"]);

// Run without remote
tests.forEach((testArgs) => {
  output(testArgs);
});

// Run with remote
run("git", ["config", "remote.origin.url", "https://www.my-remote.com"]);

tests.forEach((testArgs) => {
  output(testArgs);
});

// Run with empty changelog
fs.writeFileSync("test/repo/CHANGELOG.md", "");

tests.forEach((testArgs) => {
  output(testArgs);
});

// Run with partial changelog
fs.writeFileSync("test/repo/CHANGELOG.md", fs.readFileSync("test/changelogs/PARTIAL.md"));

tests.forEach((testArgs) => {
  output(testArgs);
});

// Run with full changelog
fs.writeFileSync("test/repo/CHANGELOG.md", fs.readFileSync("test/changelogs/FULL.md"));

tests.forEach((testArgs) => {
  output(testArgs);
});

// Cleanup
run("git", ["log", "--oneline"]);
fs.rmSync("test/repo", { recursive: true });
