/* eslint-disable n/no-unpublished-import */
import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";
import { Octokit } from "octokit";

const args = process.argv.slice(2);

if (run("git", ["status", "-s"], { stdio: "pipe" }).stdout.toString()) {
  throw new Error("Please commit all changes before running this script.");
}

//// Main

const envFile = fs.readFileSync(".env", "utf-8");
const [, githubToken] = envFile.match(/GITHUB_TOKEN=(.+)/)!;
const octokit = new Octokit({ auth: githubToken });

await octokit.rest.users.getAuthenticated(); // Will throw if token is invalid

run("just", ["build"]);

fs.copyFileSync("./target/release/changenog", "./packages/js/changenog");
fs.copyFileSync("README.md", "./packages/js/README.md");

const cargoToml = fs.readFileSync("Cargo.toml", "utf-8");
const packageJson = JSON.parse(fs.readFileSync("./packages/js/package.json", "utf-8"));
const [, version] = cargoToml.match(/version = "([^"]+)"/)!;
const newVersion = bumpVersion(version!, args[0]!);
const newTag = `v${newVersion}`;

packageJson.version = newVersion;

fs.writeFileSync("Cargo.toml", cargoToml.replace(version!, newVersion));
fs.writeFileSync(
  "./packages/js/package.json",
  `${JSON.stringify(packageJson, null, 2)}\n`,
);

run("git", ["tag", newTag]);
run("just", ["changenog"]);
run("git", ["add", "."]);
run("git", ["commit", "-m", `chore(release): ${newTag}`]);
run("git", ["push"]);
run("git", ["push", "--tags"]);
run("cargo", ["publish"]);
run("pnpm", ["publish", "./packages/js"]);

// Create release and upload binary asset
const { data: release } = await octokit.rest.repos.createRelease({
  owner: "Daniel-Knights",
  repo: "changenog",
  tag_name: newTag,
  body: "See [CHANGELOG.md](https://github.com/Daniel-Knights/changenog/blob/main/CHANGELOG.md) for details.",
});

await octokit.rest.repos.uploadReleaseAsset({
  owner: "Daniel-Knights",
  repo: "changenog",
  name: "changenog",
  release_id: release.id,
  data: fs.readFileSync("./target/release/changenog", "utf-8"),
});

//// Helper functions

function bumpVersion(versionStr: string, kind: string) {
  const versionFields = versionStr.split(".").map((n) => Number(n));

  if (!/major|minor|patch/.test(kind)) {
    throw new Error("Invalid version kind.");
  }

  if (versionFields.length !== 3) {
    throw new Error("Invalid version string.");
  }

  switch (kind) {
    case "major": {
      versionFields[0]! += 1;
      versionFields[1] = 0;
      versionFields[2] = 0;

      break;
    }
    case "minor": {
      versionFields[1]! += 1;
      versionFields[2] = 0;

      break;
    }
    case "patch": {
      versionFields[2]! += 1;
    }
  }

  return versionFields.join(".");
}

function run(
  cmd: string,
  passedArgs: string[],
  options: SpawnSyncOptions = { stdio: "inherit" },
): ReturnType<typeof spawnSync> {
  return spawnSync(cmd, passedArgs, options);
}
