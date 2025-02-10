import fs from "node:fs";

import { getAbsoluteTargetExePath, getTargetTriple } from "./target.js";

// chmod +x the target binary
if (process.platform !== "win32") {
  const targetTriple = getTargetTriple();
  const targetExePath = getAbsoluteTargetExePath(targetTriple);

  await fs.promises.chmod(targetExePath, 0o755);
}
