import fs from "node:fs";

/*
  Using the targets defined in rust-toolchain.yml, this script updates:
    - The targets in the release github workflow
    - packages/js/core/package.json optional dependencies
    - packages/js/targets

  Other locations this script doesn't handle:
    - packages/js/core/loader.js target maps
*/

const targets = fs
  .readFileSync("rust-toolchain.toml", "utf-8")
  .match(/targets = \[([^\]]+)\]/g)![0]
  .split("\n")
  .map((s) => s.match(/"([^"]+)"/)?.[1])
  .filter(Boolean);

// Update the release workflow
const newTargets = targets
  .map((target) => {
    let os;

    if (target?.includes("windows")) {
      os = "windows-latest";
    } else if (target?.includes("apple")) {
      os = "macos-latest";
    } else if (target?.includes("linux")) {
      os = target.includes("x86_64") ? "ubuntu-latest" : "ubuntu-24.04-arm";
    }

    return `          - target: ${target}
            os: ${os}
`;
  })
  .join("");

const workflow = fs.readFileSync(".github/workflows/release.yml", "utf-8");

const newWorkflow = workflow
  .replaceAll(/ {10}- target: .*(\n|\r\n)/g, "")
  .replaceAll(/ {12}os: .*(\n|\r\n)/g, "")
  .replace(/include:(\n|\r\n)/, `include:\n${newTargets}`);

fs.writeFileSync(".github/workflows/release.yml", newWorkflow);

// Update the core package.json
const corePkg = JSON.parse(fs.readFileSync("./packages/js/core/package.json", "utf-8"));

const optionalDependencies = targets.reduce(
  (acc, target) => {
    acc[`@changenog/${target}`] = corePkg.version;

    return acc;
  },
  {} as Record<string, string>,
);

corePkg.optionalDependencies = optionalDependencies;

fs.writeFileSync(
  "./packages/js/core/package.json",
  `${JSON.stringify(corePkg, null, 2)}\n`,
);

// Update packages/js/targets
fs.rmSync("./packages/js/targets", { recursive: true, force: true });
fs.mkdirSync("./packages/js/targets");

targets.forEach((target) => {
  if (!fs.existsSync(`./packages/js/targets/${target}`)) {
    fs.mkdirSync(`./packages/js/targets/${target}`);
  }

  let os;

  if (target?.includes("windows")) {
    os = "win32";
  } else if (target?.includes("apple")) {
    os = "darwin";
  } else if (target?.includes("linux")) {
    os = "linux";
  }

  const pkg = {
    name: `@changenog/${target}`,
    version: corePkg.version,
    description: `${target} binary for Changenog, the changelog generator.`,
    author: corePkg.author,
    license: corePkg.license,
    // Can't be `changenog` as NPM doesn't link the binary correctly, PNPM handles it fine, though
    // https://github.com/npm/cli/issues/3446
    bin: {
      [`changenog-${target}`]: `./changenog${os === "win32" ? ".exe" : ""}`,
    },
    repository: corePkg.repository,
    bugs: corePkg.bugs,
    keywords: corePkg.keywords,
    os: [os],
    cpu: [target?.includes("x86_64") ? "x64" : "arm64"],
  };

  fs.writeFileSync(
    `./packages/js/targets/${target}/package.json`,
    `${JSON.stringify(pkg, null, 2)}\n`,
  );
});

console.warn(
  "WARNING: This script doesn't update `packages/js/core/loader.js`. Please check it to ensure it's correct.",
);
