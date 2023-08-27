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

const cliArgs = process.argv.slice(2);

function isJsonObj(val: unknown): val is Record<string, JSONValue> {
  return !!val && typeof val === "object" && !Array.isArray(val);
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

function getGitRoot(dir: string, callCount = 0): string | undefined {
  if (callCount > 20) {
    return;
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

const relativePackagePath = path.relative(gitRoot, process.cwd()).replace(/\\/g, "/");
const dest = path.join(process.cwd(), "CHANGELOG.md");
const hasExisting = fs.existsSync(dest);
const existingChangelog = hasExisting ? `\n\n${fs.readFileSync(dest)}` : "";
const prevVersion = existingChangelog.match(/(?<=## \[?)\d+\.\d+\.\d+/)?.[0];
const prevDate = existingChangelog.match(/(?<=\().+(?=\)$)/m)?.[0];

const allCommits = gitlog.default({
  repo: process.cwd(),
  number: 1000,
  since: prevDate,
});
// Filter out version commits and merge commits
const filteredCommits = allCommits.filter((commit) => {
  if (!commit.files.some((f) => f.startsWith(relativePackagePath))) {
    return false;
  }

  return !/\d+\.\d+\.\d+/.test(commit.subject) && commit.files.length > 0;
});

if (filteredCommits.length === 0) {
  exit("no new commits");
}

const pkgBuffer = fs.readFileSync(path.join(process.cwd(), "package.json"));
const pkg: JSONValue = JSON.parse(pkgBuffer.toString());

if (!isJsonObj(pkg)) {
  exit("unable to parse package.json", true);
}

if (prevVersion === pkg.version) {
  exit("no new version");
}

function getRemoteUrl(): string {
  if (cliArgs.includes("--no-links")) {
    return "";
  }

  let remoteUrl = "";

  try {
    remoteUrl = execSync("git config --get remote.origin.url").toString().trim();
  } catch {
    if (
      isJsonObj(pkg) &&
      isJsonObj(pkg.repository) &&
      typeof pkg.repository.url === "string"
    ) {
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
const compareUrl = `${remoteUrl}/compare/v${prevVersion}...v${pkg.version}`;

const currentDate = new Intl.DateTimeFormat(undefined, {
  dateStyle: "short",
  timeStyle: "medium",
}).format(new Date());

const versionHeading =
  remoteUrl && prevVersion
    ? `## [${pkg.version}](${compareUrl}) (${currentDate})\n\n`
    : `## ${pkg.version} (${currentDate})\n\n`;

const formattedCommits = filteredCommits
  .map((c) => {
    return remoteUrl
      ? `- ${c.subject} ([${c.abbrevHash}](${remoteUrl}/commit/${c.hash}))`
      : `- ${c.subject} (${c.abbrevHash})`;
  })
  .join("\n");

fs.writeFileSync(dest, `${versionHeading}${formattedCommits}${existingChangelog}`);
