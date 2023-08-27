import { spawnSync } from "node:child_process";
import fs from "node:fs";

const args = process.argv.slice(2);

fs.rmSync("dist", { recursive: true, force: true });

function exec(cmd: string, passedArgs: string[]): void {
  spawnSync(cmd, passedArgs, { stdio: "inherit" });
}

exec("pnpm", ["run", "checks"]);
exec("pnpm", ["build"]);
exec("pnpm", ["version", args[0]]);
exec("tsx", ["./index.ts"]);
exec("git", ["add", "."]);

const pkg = await import("../package.json");

exec("git", ["commit", "-m", `docs(changelog): v${pkg.default.version}`]);
exec("git", ["push"]);
exec("git", ["push", "--tags"]);
exec("pnpm", ["publish"]);
