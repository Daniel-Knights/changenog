import { getArg } from "./utils.js";

type FilterPreset = keyof typeof filterPresets;

const filterPresets = {
  angular: /^(?:feat|fix|perf|docs)(?:\(.+?\))?!?:.*$/,
  "angular-readme-only-docs": /^(?!docs(?!\()|docs\((?!readme\))).*$/,
  "no-changelog": /^(?!.*[^a-zA-Z]changelog[^a-zA-Z]).*$/,
} as const;

export function getOptions(cliArgs: string[]) {
  const rawOpts = {
    overwrite: cliArgs.includes("--overwrite"),
    continue: cliArgs.includes("--continue"),
    noLinks: cliArgs.includes("--no-links"),
    maxCommits: getArg(cliArgs, "--max-commits")[0],
    locale: getArg(cliArgs, "--locale"),
    filterRegex: getArg(cliArgs, "--filter-regex"),
    filterPreset: getArg(cliArgs, "--filter-preset"),
  } as const;

  return {
    overwrite: rawOpts.overwrite,
    continue: rawOpts.continue,
    noLinks: rawOpts.noLinks,
    maxCommits:
      rawOpts.maxCommits && /^[1-9]\d+$/.test(rawOpts.maxCommits)
        ? Number(rawOpts.maxCommits)
        : 1000,
    locale: Intl.DateTimeFormat.supportedLocalesOf(rawOpts.locale)[0] ?? "en-GB",
    filter: rawOpts.filterRegex
      .map((f) => new RegExp(f))
      .concat(rawOpts.filterPreset.map((fp) => filterPresets[fp as FilterPreset])),
  } as const;
}
