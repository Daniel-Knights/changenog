import fs from "node:fs";

await fs.promises.copyFile("./README.md", "./packages/js/core/README.md");
await fs.promises.copyFile("./README.md", "./packages/python/core/README.md");
