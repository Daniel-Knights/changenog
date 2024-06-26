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

function getArg(arg: string): string | undefined {
  return cliArgs.find((a) => a.startsWith(`${arg}=`))?.split("=")[1];
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

function getGitRoot(dir = process.cwd(), callCount = 0): string {
  if (callCount > 20) {
    exit("unable to find git root", true);
  }

  if (callCount > 0 && pkg.name && pkg.version) {
    isMonorepoPackage = true;
  }

  if (fs.existsSync(path.join(dir, ".git"))) {
    return dir;
  }

  return getGitRoot(path.resolve(dir, ".."), callCount + 1);
}

const gitRoot = getGitRoot();

const packageTags = execSync(
  'git tag -l --sort=-creatordate --format="%(creatordate:iso-strict)//%(refname:short)"',
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

const tagsSincePrevEntry = packageTags
  .filter((t) => {
    return !prevEntryDate || new Date(t.date) > prevEntryDate;
  })
  .reverse(); // Oldest to newest

if (tagsSincePrevEntry.length === 0) {
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
const relativePackagePath = path.relative(gitRoot, process.cwd()).replace(/\\/g, "/");
const maxCommitsArg = getArg("--max-commits");

const commitsSincePrevEntry = gitlog
  .default({
    repo: process.cwd(),
    number: maxCommitsArg ? Number(maxCommitsArg) : 1000,
    // `after` means `>=`, so we have to add 1s to prevent commits made at the same
    // time as previous entry being returned
    after: prevEntryDate && new Date(prevEntryDate.getTime() + 1000).toString(),
  })
  // Filter out NPM version commits and merge commits
  .filter((commit) => {
    // Restrict to current package
    if (!commit.files.some((f) => f.startsWith(relativePackagePath))) {
      return false;
    }

    const isNpmVersionCommit = new RegExp(`^${SEMVER_REGEX.source}$`).test(
      commit.subject,
    );

    return !isNpmVersionCommit && commit.files.length > 0;
  })
  .reverse(); // Oldest to newest

const dateFormatter = new Intl.DateTimeFormat(undefined, {
  dateStyle: "short",
});

let newChangelog = existingChangelog;

tagsSincePrevEntry.forEach((tag) => {
  const tagDate = new Date(tag.date);

  // Splice commits since prev tag
  const spliceIndex = commitsSincePrevEntry.findIndex((commit) => {
    return new Date(commit.authorDate) > tagDate;
  });
  const entryCommits = commitsSincePrevEntry
    .splice(0, spliceIndex === -1 ? commitsSincePrevEntry.length : spliceIndex)
    .reverse();

  if (entryCommits.length === 0) return;

  // Format compare URL
  const prevTagIndex = packageTags.findIndex((t) => t.name === tag.name) + 1;
  const prevTag = packageTags[prevTagIndex];
  const compareUrl = prevTag
    ? `${remoteUrl}/compare/${prevTag?.name}...${tag.name}`
    : `${remoteUrl}/tags`;

  // Format version heading
  const tagVersion = parseVersion(tag.name);
  const formattedDate = dateFormatter.format(tagDate);
  const versionHeading = remoteUrl
    ? `## [${tagVersion}](${compareUrl}) (${formattedDate})`
    : `## ${tagVersion} (${formattedDate})`;

  // Format commits
  const formattedCommits = entryCommits
    .map((c) => {
      return remoteUrl
        ? `- ${c.subject} ([${c.abbrevHash}](${remoteUrl}/commit/${c.hash}))`
        : `- ${c.subject} (${c.abbrevHash})`;
    })
    .join("\n");

  newChangelog = `${versionHeading}\n\n${formattedCommits}\n\n${newChangelog}`;
});

fs.writeFileSync(dest, newChangelog.trim());
