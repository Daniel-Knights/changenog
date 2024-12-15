import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";

const args = process.argv.slice(2);

function run(
  cmd: string,
  passedArgs: string[],
  options: SpawnSyncOptions = { stdio: "inherit" },
): ReturnType<typeof spawnSync> {
  return spawnSync(cmd, passedArgs, options);
}

if (run("git", ["status", "-s"], { stdio: "pipe" }).stdout.toString()) {
  throw new Error("Please commit all changes before running this script.");
}

fs.rmSync("dist", { recursive: true, force: true });

run("pnpm", ["run", "checks"]);
run("pnpm", ["build"]);
run("pnpm", ["version", args[0]]);
run("tsx", ["./index.ts"]);
run("git", ["add", "."]);

const pkg = await import("../package.json");

run("git", ["commit", "-m", `docs(changelog): v${pkg.default.version}`]);
run("git", ["push"]);
run("git", ["push", "--tags"]);
run("pnpm", ["publish"]);
