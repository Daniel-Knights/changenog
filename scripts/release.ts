import { execFileSync } from "node:child_process";

const args = process.argv.slice(2);

execFileSync("pnpm", ["run", "checks"]);
execFileSync("pnpm", ["build"]);
execFileSync("pnpm", ["version", args[0]]);
execFileSync("tsx", ["./index.ts"]);
execFileSync("git", ["add", "."]);

const pkg = await import("../package.json");

execFileSync("git", ["commit", "-m", `docs(changelog): v${pkg.default.version}`]);
execFileSync("git", ["push"]);
execFileSync("git", ["push", "--tags"]);
execFileSync("pnpm", ["publish"]);
