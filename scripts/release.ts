import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";

const args = process.argv.slice(2);

if (run("git", ["status", "-s"], { stdio: "pipe" }).stdout.toString()) {
  throw new Error("Please commit all changes before running this script.");
}

//// Main

run("just", ["build"]);

const cargoToml = fs.readFileSync("Cargo.toml", "utf-8");
const packageJson = JSON.parse(
  fs.readFileSync("./packages/js/core/package.json", "utf-8"),
);
const [, version] = cargoToml.match(/version = "([^"]+)"/)!;
const newVersion = bumpVersion(version!, args[0]!);
const newTag = `v${newVersion}`;

packageJson.version = newVersion;

fs.writeFileSync("Cargo.toml", cargoToml.replace(version!, newVersion));
fs.writeFileSync(
  "./packages/js/core/package.json",
  `${JSON.stringify(packageJson, null, 2)}\n`,
);

run("git", ["tag", newTag]);
run("just", ["changenog"]);
run("git", ["add", "."]);
run("git", ["commit", "-m", `chore(release): ${newTag}`]);
run("git", ["push"]);
run("git", ["push", "--tags"]);

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
