import { execFileSync } from "node:child_process";

import pkg from "../package.json";

const args = process.argv.slice(2);

execFileSync("pnpm", ["run", "checks"]);
execFileSync("pnpm", ["build"]);
execFileSync("pnpm", ["version", args[0]]);
execFileSync("tsx", ["./index.ts"]);
execFileSync("git", ["add", "."]);
execFileSync("git", ["commit", "-m", `docs(changelog): v${pkg.version}`]);
execFileSync("git", ["push"]);
execFileSync("git", ["push", "--tags"]);
execFileSync("pnpm", ["publish"]);
