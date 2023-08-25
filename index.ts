#!/usr/bin/env node
import gitlog from "gitlog";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

type JSONValue =
  | string
  | number
  | boolean
  | null
  | { [x: string]: JSONValue }
  | Array<JSONValue>;

function isJsonObj(val: unknown): val is Record<string, JSONValue> {
  return !!val && typeof val === "object" && !Array.isArray(val);
}

const cliArgs = process.argv.slice(2);

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
  throw new Error("Could not find git root");
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
  throw new Error("No new commits");
}

const pkgBuffer = fs.readFileSync(path.join(process.cwd(), "package.json"));
const pkg: JSONValue = JSON.parse(pkgBuffer.toString());

if (!isJsonObj(pkg)) {
  throw new Error("Unable to parse package.json");
}

if (prevVersion === pkg.version) {
  throw new Error("No new version");
}

function getRemoteUrl(): string {
  if (cliArgs.includes("--no-links")) {
    return "";
  }

  let remoteUrl = "";

  try {
    remoteUrl = execFileSync("git", ["config", "--get", "remote.origin.url"])
      .toString()
      .trim();
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
