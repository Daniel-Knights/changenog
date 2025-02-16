import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";

export function run(
  cmd: string,
  args: string[],
  options: SpawnSyncOptions = { stdio: "inherit" },
): ReturnType<typeof spawnSync> {
  return spawnSync(cmd, args, {
    cwd: "test/repo",
    ...options,
  });
}

export function output(args: string[]) {
  const result = run("../../target/release/changenog", args, {
    stdio: "pipe",
  });

  fs.appendFileSync("test/output/stdout.txt", `test: ["${args.join(", ")}"]\n`);
  fs.appendFileSync("test/output/stdout.txt", `${result.stdout ?? result.stderr}\n\n`);

  const filename = args.join("_").replace(/[^a-zA-Z0-9_]+/g, "_");

  ["/", "foo/", "bar/", "bar/baz/"].forEach((dir) => {
    if (!fs.existsSync(`test/repo/${dir}CHANGELOG.md`)) return;

    fs.renameSync(
      `test/repo/${dir}CHANGELOG.md`,
      `test/output/changelogs/${dir}${filename}.md`,
    );
  });
}

export function commit(dir: string, message: string) {
  fs.appendFileSync(`test/repo/${dir}/main.txt`, message);

  run("git", ["add", "."]);
  run("git", ["commit", "-m", message]);
}
