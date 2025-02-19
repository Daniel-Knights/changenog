import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

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

export function commit(msg: string, dir: "foo" | "bar" | "bar/baz") {
  fs.appendFileSync(`test/repo/${dir}/main.txt`, msg);

  run("git", ["add", "."]);
  run("git", ["commit", "-m", msg]);
}

export function output(id: string, args: string[]) {
  const result = run("../../target/release/changenog", args, {
    stdio: "pipe",
  });

  fs.appendFileSync("test/output/stdout.txt", `test ${id}: ["${args.join(", ")}"]\n`);
  fs.appendFileSync(
    "test/output/stdout.txt",
    `${replaceDynamicValues(result.stdout.toString())}\n\n`,
  );

  const filename = args.join("_").replace(/[^a-zA-Z0-9_]+/g, "_");

  ["", "foo", "bar", "bar/baz"].forEach((dir) => {
    const sourcePath = path.normalize(`test/repo/${dir}/CHANGELOG.md`);

    if (!fs.existsSync(sourcePath)) return;

    const replacedContent = replaceDynamicValues(fs.readFileSync(sourcePath, "utf-8"));
    const destPath = path.normalize(
      `test/output/changelogs/${dir || "root"}/${id}/${filename}.md`,
    );

    if (!fs.existsSync(path.dirname(destPath))) {
      fs.mkdirSync(path.dirname(destPath), { recursive: true });
    }

    fs.writeFileSync(sourcePath, replacedContent);
    fs.renameSync(sourcePath, destPath);
  });
}

function replaceDynamicValues(content: string) {
  const shaMatches = content.match(/(?<=\/commit\/)[^\)]+/g) ?? [];

  return (
    shaMatches
      .reduce((acc, sha) => {
        const shaHead = sha.substring(0, 7);
        const shaTail = sha.substring(7);
        const shaRegex = new RegExp(`${shaHead}(${shaTail})?`, "g");

        return acc.replace(shaRegex, "REPLACED");
      }, content)
      // Replace no-link shas and dates
      .replace(/\([^\)]+\)(?=\n)/g, "(REPLACED)")
  );
}
