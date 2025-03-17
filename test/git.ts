import { run } from "./utils.js";

/**
 * Git operations manager with synchronization and verification
 */
export class GitManager {
  /**
   * Initialize a new Git repository
   */
  static init() {
    this.#executeGitCommand(["init"]);
  }

  /**
   * Add files to git staging
   */
  static add() {
    this.#executeGitCommand(["add", "-A"]);
    this.#sync();
  }

  /**
   * Create a commit with synchronization and verification
   * @param message - Commit message
   */
  static commit(message: string) {
    this.#executeGitCommand(["commit", "-m", message]);
    this.#sync();
  }

  /**
   * Create a tag with synchronization and verification
   * @param tagName - Name of the tag
   */
  static tag(tagName: string) {
    this.#executeGitCommand(["tag", tagName]);
    this.#sync();
  }

  /**
   * Set the remote URL for the repository
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
   * Ensure synchronization between operations
   */
  static #sync() {
    this.#executeGitCommand(["fsck", "--full"]);
  }
}
