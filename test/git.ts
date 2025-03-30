import { SpawnSyncOptions } from "node:child_process";

import { run } from "./utils.js";

/**
 * Git operations manager
 */
export class GitManager {
  // The docs say Unix time time is supported, but instead I get 'fatal: invalid date format: 0 +0100'
  // (on Linux), so we have to use a Date object instead
  static #committerDate = new Date(0);

  /**
   * Initializes a new Git repository
   */
  static init() {
    this.#executeGitCommand(["init"]);
  }

  /**
   * Adds files to git staging
   */
  static add() {
    this.#executeGitCommand(["add", "-A"]);
    this.#sync();
  }

  /**
   * Commits staged files
   * @param message - Commit message
   */
  static commit(message: string) {
    this.#executeGitCommand(["commit", "-m", message], {
      env: {
        // Ensure distinct commit dates
        GIT_COMMITTER_DATE: this.#committerDate.toISOString(),
        // These are needed when setting committer date, no idea why
        GIT_AUTHOR_NAME: ".",
        GIT_COMMITTER_NAME: ".",
      },
    });

    this.#sync();
    this.#committerDate.setTime(this.#committerDate.getTime() + 1000);
  }

  /**
   * Creates a new tag
   * @param tagName - Name of the tag
   */
  static tag(tagName: string) {
    this.#executeGitCommand(["tag", tagName]);
    this.#sync();
  }

  /**
   * Sets the remote URL for the repo
   * @param url - Remote URL
   */
  static setRemote(url: string) {
    this.#executeGitCommand(["remote", "add", "origin", url]);
  }

  //// Private

  /**
   * Executes a git command and returns the result
   * @param args - Args passed to Git
   * @returns Command output
   */
  static #executeGitCommand(args: string[], options?: SpawnSyncOptions) {
    try {
      const output = run("git", args, { stdio: "pipe", ...options });

      if (output.status !== 0) {
        throw output.stderr.toString();
      }

      return output.stdout?.toString().trim();
    } catch (error) {
      throw new Error(`Git command failed: git ${args.join(" ")}\n${error}`);
    }
  }

  /**
   * Ensures Git has completed any filesystem operations before continuing
   */
  static #sync() {
    this.#executeGitCommand(["fsck"]);
  }
}
