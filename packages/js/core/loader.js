#!/usr/bin/env node
import { spawnSync } from "node:child_process";

import { getAbsoluteTargetExePath, getTargetTriple } from "./target.js";

const targetTriple = getTargetTriple();
const targetExePath = getAbsoluteTargetExePath(targetTriple);

const result = spawnSync(targetExePath, process.argv.slice(2), {
  stdio: "inherit",
});

if (result.error) {
  console.error(
    `[changenog]: failed to execute binary for target: ${targetTriple}.  ensure the` +
      ` correct optional dependency is installed (@changenog/${targetTriple})\n`,
  );

  throw result.error;
}
