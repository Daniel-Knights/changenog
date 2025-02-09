import fs from "node:fs";

/*
  Using the targets described in rust-toolchain.yml, this script updates:
    - The targets in the release github workflow and the core
    - package.json optional dependencies
    - packages/js/targets
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
      os = "windows";
    } else if (target?.includes("apple")) {
      os = "macos";
    } else {
      os = "ubuntu";
    }

    return `          - target: ${target}
            os: ${os}-latest
`;
  })
  .join("");

const workflow = fs.readFileSync(".github/workflows/release.yml", "utf-8");

const newWorkflow = workflow
  .replaceAll(/ {10}- target: .*\n/g, "")
  .replaceAll(/ {12}os: .*\n/g, "")
  .replace(/include:\n/, `include:\n${newTargets}`);

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

delete corePkg.name;
delete corePkg.description;
delete corePkg.optionalDependencies;

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
    description: `${target} binary for Changenog, the changelog generator.`,
    bin: {
      changenog: `./changenog${os === "win32" ? ".exe" : ""}`,
    },
    ...corePkg,
    os: [os],
    cpu: [target?.includes("x86_64") ? "x64" : "arm64"],
  };

  fs.writeFileSync(
    `./packages/js/targets/${target}/package.json`,
    JSON.stringify(pkg, null, 2),
  );
});
