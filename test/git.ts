import { run } from "./utils.js";

/**
 * Git operations manager
 */
export class GitManager {
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
    this.#executeGitCommand(["commit", "-m", message]);
    this.#sync();
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
  static #executeGitCommand(args: string[]) {
    try {
      const output = run("git", args, { stdio: "pipe" });

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
