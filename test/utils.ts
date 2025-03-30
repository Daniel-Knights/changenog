import { spawnSync, SpawnSyncOptions } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

import { GitManager } from "./git.js";

/**
 * Runs a command in the test repo directory.
 *
 * Wrapper for `spawnSync`.
 */
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

/**
 * Makes a series of commits in `test/repo/${dir}`
 */
export async function commitAll(dir: "foo" | "bar" | "bar/baz", commits: string[]) {
  for (const msg of commits) {
    // Make change to `main.txt` in the passed directory
    await fs.promises.appendFile(`test/repo/${dir}/main.txt`, msg);

    GitManager.add();
    GitManager.commit(msg);
  }
}

/**
 * Runs a series of tests in the test repo directory.
 *
 * Optionally writes one of the existing changelogs to the test repo before each test.
 */
export async function suite(
  id: string,
  tests: string[][],
  existingChangelog?: "FULL" | "PARTIAL",
) {
  for (const testArgs of tests) {
    // Reset existing changelog
    if (existingChangelog) {
      await fs.promises.copyFile(
        `test/changelogs/${existingChangelog}.md`,
        "test/repo/CHANGELOG.md",
      );
    }

    await output(id, testArgs);
  }
}

/**
 * Runs `changenog` with passed arguments and outputs both stdout and changelog files.
 */
export async function output(id: string, args: string[]) {
  // Run `changenog`
  const result = run("../../target/release/changenog", args, {
    stdio: "pipe",
  });

  // Append output to `terminal.md`
  const formattedTerminalOutput = formatTerminalOutput(id, args, result);

  await fs.promises.appendFile("test/output/terminal.md", formattedTerminalOutput);

  // Normalise output filename
  const filename = args.join("_").replace(/[^a-zA-Z0-9_]+/g, "_");

  // Check each directory for a `CHANGELOG.md` file (really, it's only necessary
  // when the `--root` arg is used)
  const outputPromises = ["", "foo", "bar", "bar/baz"].map(async (dir) => {
    const sourcePath = path.normalize(`test/repo/${dir}/CHANGELOG.md`);

    if (!fs.existsSync(sourcePath)) return;

    // Match test repo directory structure for outputted changelogs
    const destPath = path.normalize(
      `test/output/changelogs/${dir || "root"}/${id}/${filename}.md`,
    );

    // Create directory if it doesn't exist
    if (!fs.existsSync(path.dirname(destPath))) {
      await fs.promises.mkdir(path.dirname(destPath), { recursive: true });
    }

    const content = await fs.promises.readFile(sourcePath, "utf-8");
    const replacedContent = replaceDynamicText(content);

    // Normalise file content and move to output directory
    await fs.promises.writeFile(sourcePath, replacedContent);
    await fs.promises.rename(sourcePath, destPath);
  });

  await Promise.all(outputPromises);
}

/**
 * Formats the result of running `changenog`, for appending to `terminal.md`.
 */
function formatTerminalOutput(
  testId: string,
  args: string[],
  result: ReturnType<typeof spawnSync>,
) {
  const normalisedStdout = replaceDynamicText(result.stdout.toString());
  const stderrString = result.stderr.toString();

  let formattedStdout = `## test ${testId}: ["${args.join(" ")}"]`;

  if (normalisedStdout) {
    formattedStdout += `\n\n### stdout\n\n\`\`\`\n${normalisedStdout}\`\`\``;
  }

  if (stderrString) {
    formattedStdout += `\n\n### stderr\n\n\`\`\`\n${stderrString}\`\`\``;
  }

  return `${formattedStdout}\n\n`;
}

/**
 * Replaces commit SHAs and dates with static values.
 */
function replaceDynamicText(content: string) {
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
      .replace(/\([^\)]+\)(?=\n|\r\n)/g, "(REPLACED)")
  );
}
