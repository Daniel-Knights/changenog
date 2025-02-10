import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const archMap = {
  x64: "x86_64",
  arm64: "aarch64",
};

const platformMap = {
  win32: "pc-windows-msvc",
  darwin: "apple-darwin",
  linux: "unknown-linux-gnu",
};

export function getTargetTriple() {
  const archName = archMap[process.arch];
  const platformName = platformMap[process.platform];

  if (!archName) {
    throw new Error(`[changenog]: unsupported architecture: ${process.arch}`);
  }

  if (!platformName) {
    throw new Error(`[changenog]: unsupported platform: ${process.platform}`);
  }

  return `${archName}-${platformName}`;
}

export function getAbsoluteTargetExePath(targetTriple) {
  return path.resolve(__dirname, "node_modules", ".bin", targetTriple);
}
