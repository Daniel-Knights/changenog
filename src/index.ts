#!/usr/bin/env node
import gitlog from "gitlog";
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

import {
  exit,
  getArg,
  isJsonObj,
  JSONValue,
  parseVersion,
  SEMVER_REGEX,
} from "./utils.js";

const cliArgs = process.argv.slice(2);
const pkgBuffer = fs.readFileSync(path.join(process.cwd(), "package.json"));
const pkg: Record<string, JSONValue> = JSON.parse(pkgBuffer.toString());

const opts = {
  continue: cliArgs.includes("--continue"),
  noLinks: cliArgs.includes("--no-links"),
  maxCommits: getArg(cliArgs, "--max-commits"),
} as const;

async function main() {
  const gitRoot = getGitRoot();
  if (!gitRoot) return;

  const dest = path.join(process.cwd(), "CHANGELOG.md");
  const existingChangelog = fs.existsSync(dest) ? fs.readFileSync(dest, "utf-8") : "";
  const prevEntryVersion = parseVersion(existingChangelog);
  const allTags = getTags(gitRoot.isMonorepo);

  const prevEntryTag = allTags.find((t) => {
    return parseVersion(t.name) === prevEntryVersion;
  });

  const prevEntryDate = prevEntryTag ? new Date(prevEntryTag.date) : undefined;
  const tagsSince = getTagsSince(allTags, prevEntryDate);

  if (tagsSince.length === 0) {
    exit("no new version(s)", opts.continue);

    return;
  }

  const commitsSince = await getCommitsSince(gitRoot.dir, prevEntryDate);

  const newChangelog = getNewChangelog(
    existingChangelog,
    allTags,
    tagsSince,
    commitsSince,
  );

  fs.writeFileSync(dest, newChangelog);
}

function getNewChangelog(
  existingChangelog: string,
  allTags: GitTag[],
  tagsSince: GitTag[],
  commitsSince: GitCommit[],
) {
  const dateFormatter = new Intl.DateTimeFormat(undefined, {
    dateStyle: "short",
  });

  const remoteUrl = getRemoteUrl();

  let newChangelog = existingChangelog;

  tagsSince.forEach((tag) => {
    const tagDate = new Date(tag.date);

    // Splice commits since prev tag
    const spliceIndex = commitsSince.findIndex((commit) => {
      return new Date(commit.authorDate) > tagDate;
    });
    const entryCommits = commitsSince
      .splice(0, spliceIndex === -1 ? commitsSince.length : spliceIndex)
      .reverse();

    if (entryCommits.length === 0) return;

    // Format compare URL
    const prevTagIndex = allTags.findIndex((t) => t.name === tag.name) + 1;
    const prevTag = allTags[prevTagIndex];
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

  return newChangelog.trim();
}

function getGitRoot(dir = process.cwd(), callCount = 0) {
  if (callCount > 20) {
    exit("unable to find git root", opts.continue, true);

    return;
  }

  if (fs.existsSync(path.join(dir, ".git"))) {
    return {
      dir,
      isMonorepo: callCount > 0 && !!pkg.name && !!pkg.version,
    };
  }

  return getGitRoot(path.resolve(dir, ".."), callCount + 1);
}

function getRemoteUrl(): string {
  if (opts.noLinks) {
    return "";
  }

  let url = "";

  try {
    url = execSync("git config --get remote.origin.url")
      .toString()
      .replace(".git", "")
      .trim();
  } catch {
    if (isJsonObj(pkg.repository) && typeof pkg.repository.url === "string") {
      url = pkg.repository.url.trim();
    }
  }

  try {
    // `new URL` will throw an error if the URL is invalid
    return new URL(url).toString();
  } catch {
    return "";
  }
}

function getTags(isMonorepo: boolean) {
  return execSync(
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

      return !isMonorepo || t.name.startsWith(`${pkg.name}/`);
    });
}

function getTagsSince(allTags: GitTag[], prevEntryDate?: Date) {
  return allTags
    .filter((t) => {
      return !prevEntryDate || new Date(t.date) > prevEntryDate;
    })
    .reverse(); // Oldest to newest
}

async function getCommitsSince(gitRootDir: string, prevEntryDate?: Date) {
  const relPackagePath = path.relative(gitRootDir, process.cwd()).replace(/\\/g, "/");

  const commits = await gitlog({
    repo: process.cwd(),
    number: opts.maxCommits ? Number(opts.maxCommits) : 1000,
    // `after` means `>=`, so we have to add 1s to prevent commits made at the same
    // time as previous entry being returned
    after: prevEntryDate && new Date(prevEntryDate.getTime() + 1000).toString(),
  });

  return (
    commits
      // Filter out NPM version commits and merge commits
      .filter((commit) => {
        // Restrict to current package
        if (!commit.files.some((f) => f.startsWith(relPackagePath))) {
          return false;
        }

        const isNpmVersionCommit = new RegExp(`^${SEMVER_REGEX.source}$`).test(
          commit.subject,
        );

        return !isNpmVersionCommit && commit.files.length > 0;
      })
      .reverse() // Oldest to newest
  );
}

main();

type GitTag = {
  date: string;
  name: string;
};

type GitCommit = {
  subject: string;
  authorDate: string;
  hash: string;
  abbrevHash: string;
};
