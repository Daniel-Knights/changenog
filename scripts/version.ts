import fs from "node:fs";
import { fileURLToPath } from "node:url";

/*
  This script bumps the version of all core packages.
  It can be run directly or imported by other scripts.
  It bumps versions independently, so core package versions can fall out of sync without issue.
  To bump sub-packages, run the `toolchain` script.
*/

const TOML_VERSION_REGEX = /version = "([^"]+)"/;

export async function bumpCoreVersions(kind: "major" | "minor" | "patch") {
  const newCargoVersion = await bumpCargoVersion(kind);
  const newPyprojectVersion = await bumpPyprojectVersion(kind);
  const newPackageJsonVersion = await bumpPackageJsonVersion(kind);

  return {
    cargoToml: newCargoVersion,
    pyprojectToml: newPyprojectVersion,
    packageJson: newPackageJsonVersion,
  };
}

async function bumpCargoVersion(kind: "major" | "minor" | "patch") {
  const cargoToml = fs.readFileSync("Cargo.toml", "utf-8");
  const [cargoHead, cargoTail] = cargoToml.split("[package]", 2);
  const [, cargoVersion] = cargoTail!.match(TOML_VERSION_REGEX)!;
  const newCargoVersion = bumpVersion(cargoVersion!, kind);

  const newCargoTail = cargoTail!.replace(
    TOML_VERSION_REGEX,
    `version = "${newCargoVersion}"`,
  );

  await fs.promises.writeFile("Cargo.toml", `${cargoHead}[package]${newCargoTail}`);

  return newCargoVersion;
}

async function bumpPyprojectVersion(kind: "major" | "minor" | "patch") {
  const pyprojectToml = fs.readFileSync("./packages/python/core/pyproject.toml", "utf-8");
  const [pyprojectHead, pyprojectTail] = pyprojectToml.split("[project]", 2);
  const [, pyprojectVersion] = pyprojectTail!.match(TOML_VERSION_REGEX)!;
  const newPyprojectVersion = bumpVersion(pyprojectVersion!, kind);

  const newPyprojectTail = pyprojectTail!.replace(
    TOML_VERSION_REGEX,
    `version = "${newPyprojectVersion}"`,
  );

  const newPyprojectToml = `${pyprojectHead}[project]${newPyprojectTail}`;

  await fs.promises.writeFile("./packages/python/core/pyproject.toml", newPyprojectToml);

  return newPyprojectVersion;
}

async function bumpPackageJsonVersion(kind: "major" | "minor" | "patch") {
  const packageJson = fs.readFileSync("./packages/js/core/package.json", "utf-8");
  const parsedPackageJson = JSON.parse(packageJson);
  const newPackageJsonVersion = bumpVersion(parsedPackageJson.version, kind);

  parsedPackageJson.version = newPackageJsonVersion;

  const newPackageJson = `${JSON.stringify(parsedPackageJson, null, 2)}\n`;

  await fs.promises.writeFile("./packages/js/core/package.json", newPackageJson);

  return newPackageJsonVersion;
}

//// Main

if (isMain()) {
  const args = process.argv.slice(2);
  const kind = args[0] as "major" | "minor" | "patch";

  bumpCoreVersions(kind);
}

//// Helpers

function bumpVersion(versionStr: string, kind: "major" | "minor" | "patch") {
  const versionFields = versionStr.split(".").map((n) => Number(n));

  if (!/major|minor|patch/.test(kind)) {
    throw new Error("Invalid version kind.");
  }

  if (versionFields.length !== 3) {
    throw new Error("Invalid version string.");
  }

  switch (kind) {
    case "major": {
      versionFields[0]! += 1;
      versionFields[1] = 0;
      versionFields[2] = 0;

      break;
    }
    case "minor": {
      versionFields[1]! += 1;
      versionFields[2] = 0;

      break;
    }
    case "patch": {
      versionFields[2]! += 1;
    }
  }

  return versionFields.join(".");
}

function isMain() {
  return removeExt(process.argv[1]) === removeExt(fileURLToPath(import.meta.url));
}

function removeExt(filename?: string) {
  return filename?.replace(/\.[^/.]+$/, "");
}
