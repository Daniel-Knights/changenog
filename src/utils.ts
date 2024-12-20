export type JSONValue =
  | string
  | number
  | boolean
  | null
  | { [x: string]: JSONValue }
  | Array<JSONValue>;

const LOG_PREFIX = "\x1b[33m[changenog]\x1b[0m";

// https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
export const SEMVER_REGEX =
  /(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?/;

export function isJsonObj(val: unknown): val is Record<string, JSONValue> {
  return !!val && typeof val === "object" && !Array.isArray(val);
}

export function getArg(cliArgs: string[], arg: string): string[] {
  return cliArgs.filter((a) => a.startsWith(`${arg}=`))?.map((a) => a.split("=")[1]!);
}

export function parseVersion(version: string | undefined): string | undefined {
  return version?.match(SEMVER_REGEX)?.[0];
}

export function exit(message: string, shouldContinue: boolean, error?: boolean) {
  const formattedMessage = `${LOG_PREFIX} ${message}`;

  if (shouldContinue) {
    if (error) {
      console.error(`Error: ${formattedMessage}`);
    } else {
      console.log(`${formattedMessage}, exiting...`);
    }

    process.exitCode = 0;
  } else {
    if (error) {
      throw new Error(formattedMessage);
    }

    console.log(`${formattedMessage}, exiting...`);

    process.exitCode = 1;
  }
}
