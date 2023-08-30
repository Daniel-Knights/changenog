#!/usr/bin/env node
import gitlog from "gitlog";
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

type JSONValue =
  | string
  | number
  | boolean
  | null
  | { [x: string]: JSONValue }
  | Array<JSONValue>;

const LOG_PREFIX = "\x1b[33m[changenog]\x1b[0m";
// https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
const SEMVER_REGEX =
  /(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/;

const cliArgs = process.argv.slice(2);

let isMonorepoPackage = false;

function isJsonObj(val: unknown): val is Record<string, JSONValue> {
  return !!val && typeof val === "object" && !Array.isArray(val);
}

function parseVersion(version: string | undefined): string | undefined {
  return version?.match(SEMVER_REGEX)?.[0];
}

function exit(message: string, error?: boolean): never {
  const formattedMessage = `${LOG_PREFIX} ${message}`;

  if (cliArgs.includes("--continue")) {
    if (error) {
      console.error(`Error: ${formattedMessage}`);
    } else {
      console.log(`${formattedMessage}, exiting...`);
    }

    process.exit(0);
  }

  if (error) {
    throw new Error(formattedMessage);
  }

  console.log(`${formattedMessage}, exiting...`);
  process.exit(1);
}

const pkgBuffer = fs.readFileSync(path.join(process.cwd(), "package.json"));
const pkg: Record<string, JSONValue> = JSON.parse(pkgBuffer.toString());

function getGitRoot(dir: string, callCount = 0): string | undefined {
  if (callCount > 20) {
    return;
  }

  if (callCount > 0 && pkg.name && pkg.version) {
    isMonorepoPackage = true;
  }

  if (fs.existsSync(path.join(dir, ".git"))) {
    return dir;
  }

  return getGitRoot(path.resolve(dir, ".."), callCount + 1);
}

const gitRoot = getGitRoot(process.cwd());

if (!gitRoot) {
  exit("unable to find git root", true);
}

const packageTags = execSync(
  "git tag -l --sort=-creatordate --format=%(creatordate:iso-strict)//%(refname:short)",
  { encoding: "utf-8" },
)
  .split("\n")
  .map((t) => {
    return t.match(/(?<date>.+?)\/\/(?<name>.+)/)?.groups as {
      date: string;
      name: string;
    };
  })
  .filter((t) => {
    if (!t) return false;

    return !isMonorepoPackage || t.name.startsWith(`${pkg.name}/`);
  });

const dest = path.join(process.cwd(), "CHANGELOG.md");
const existingChangelog = fs.existsSync(dest) ? fs.readFileSync(dest, "utf-8") : "";
const prevEntryVersion = parseVersion(existingChangelog);
const prevEntryTag = packageTags.find((t) => {
  return parseVersion(t.name) === prevEntryVersion;
});
const prevEntryDate = prevEntryTag ? new Date(prevEntryTag.date) : undefined;

const tagsSinceLastEntry = packageTags.filter((t) => {
  return !prevEntryDate || new Date(t.date) > prevEntryDate;
});

if (tagsSinceLastEntry.length === 0) {
  exit("no new version(s)");
}

function getRemoteUrl(): string {
  if (cliArgs.includes("--no-links")) {
    return "";
  }

  let remoteUrl = "";

  try {
    remoteUrl = execSync("git config --get remote.origin.url")
      .toString()
      .replace(".git", "")
      .trim();
  } catch {
    if (isJsonObj(pkg.repository) && typeof pkg.repository.url === "string") {
      remoteUrl = pkg.repository.url.trim();
    }
  }

  try {
    // `new URL` will throw an error if the URL is invalid
    return new URL(remoteUrl).toString();
  } catch {
    return "";
  }
}

const remoteUrl = getRemoteUrl();
const reversedTags = tagsSinceLastEntry.reverse();
const relativePackagePath = path.relative(gitRoot, process.cwd()).replace(/\\/g, "/");
const allCommits = gitlog.default({
  repo: process.cwd(),
  number: 1000,
});
// Filter out NPM version commits and merge commits
const filteredCommits = allCommits.filter((commit) => {
  // Restrict to current package
  if (!commit.files.some((f) => f.startsWith(relativePackagePath))) {
    return false;
  }

  const isNpmVersionCommit = new RegExp(`^${SEMVER_REGEX.source}$`).test(commit.subject);

  return !isNpmVersionCommit && commit.files.length > 0;
});

const dateFormatter = new Intl.DateTimeFormat(undefined, {
  dateStyle: "short",
});

let newChangelog = existingChangelog;

reversedTags.forEach((tag) => {
  const tagDate = new Date(tag.date);
  const prevTagIndex = packageTags.findIndex((t) => t.name === tag.name) + 1;
  const prevTag = packageTags[prevTagIndex];
  const prevTagDate = prevTag ? new Date(prevTag.date) : prevEntryDate;

  const entryCommits = filteredCommits.filter((commit) => {
    const commitDate = new Date(commit.authorDate);

    if (!prevTagDate) {
      return commitDate <= tagDate;
    }

    return commitDate <= tagDate && commitDate > prevTagDate;
  });

  if (entryCommits.length === 0) return;

  const tagVersion = parseVersion(tag.name);
  const compareUrl = `${remoteUrl}/compare/${prevTag?.name}...${tag.name}`;
  const formattedDate = dateFormatter.format(tagDate);

  const versionHeading =
    remoteUrl && prevTag
      ? `## [${tagVersion}](${compareUrl}) (${formattedDate})\n\n`
      : `## ${tagVersion} (${formattedDate})\n\n`;

  const formattedCommits = entryCommits
    .map((c) => {
      return remoteUrl
        ? `- ${c.subject} ([${c.abbrevHash}](${remoteUrl}/commit/${c.hash}))`
        : `- ${c.subject} (${c.abbrevHash})`;
    })
    .join("\n");

  newChangelog = `${versionHeading}${formattedCommits}\n\n${newChangelog}`;
});

fs.writeFileSync(dest, newChangelog.trim());
