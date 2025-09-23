import { spawnSync, SpawnSyncOptions } from "node:child_process";
import readline from "node:readline";

import { bumpCoreVersions } from "./version.js";

const args = process.argv.slice(2);

if (run("git", ["status", "-s"], { stdio: "pipe" }).stdout.toString()) {
  throw new Error("Please commit all changes before running this script.");
}

if (run("just", ["test"]).status !== 0) {
  throw new Error("Tests failed. Please fix them before releasing.");
}

//// Main

const newVersions = await bumpCoreVersions(args[0] as "major" | "minor" | "patch");
const newTag = `v${newVersions.cargoToml}`;

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

run("just", ["toolchain"]);

await confirmChanges();

run("git", ["add", "."]);
run("git", ["commit", "-m", `chore(release): prepare ${newTag}`]);
run("git", ["tag", newTag]);
run("just", ["changenog"]);

await confirmChanges();

run("git", ["add", "."]);
run("git", ["commit", "-m", `chore(release): ${newTag}`]);
run("git", ["push"]);
run("git", ["push", "--tags"]);

rl.close();

//// Helper functions

function run(
  cmd: string,
  passedArgs: string[],
  options: SpawnSyncOptions = { stdio: "inherit" },
): ReturnType<typeof spawnSync> {
  return spawnSync(cmd, passedArgs, options);
}

function confirmChanges() {
  console.log("Review changes then press enter to continue...");

  return new Promise<void>((res) => {
    rl.on("line", () => {
      res();
    });
  });
}
