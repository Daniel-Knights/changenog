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

export async function commit(dir: "foo" | "bar" | "bar/baz", commits: string[]) {
  for (const msg of commits) {
    await fs.promises.appendFile(`test/repo/${dir}/main.txt`, msg);

    run("git", ["add", "."]);
    run("git", ["commit", "-m", msg]);
  }
}

export async function suite(id: string, tests: string[][]) {
  for (const testArgs of tests) {
    await output(id, testArgs);
  }
}

export async function output(id: string, args: string[]) {
  const result = run("../../target/release/changenog", args, {
    stdio: "pipe",
  });

  await fs.promises.appendFile(
    "test/output/stdout.txt",
    `test ${id}: ["${args.join(", ")}"]\n`,
  );

  await fs.promises.appendFile(
    "test/output/stdout.txt",
    `${replaceDynamicValues(result.stdout.toString())}\n\n`,
  );

  const filename = args.join("_").replace(/[^a-zA-Z0-9_]+/g, "_");

  const outputPromises = ["", "foo", "bar", "bar/baz"].map(async (dir) => {
    const sourcePath = path.normalize(`test/repo/${dir}/CHANGELOG.md`);

    if (!fs.existsSync(sourcePath)) return;

    const destPath = path.normalize(
      `test/output/changelogs/${dir || "root"}/${id}/${filename}.md`,
    );

    if (!fs.existsSync(path.dirname(destPath))) {
      await fs.promises.mkdir(path.dirname(destPath), { recursive: true });
    }

    const content = await fs.promises.readFile(sourcePath, "utf-8");
    const replacedContent = replaceDynamicValues(content);

    await fs.promises.writeFile(sourcePath, replacedContent);
    await fs.promises.rename(sourcePath, destPath);
  });

  await Promise.all(outputPromises);
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
