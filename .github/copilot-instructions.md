# Copilot Cloud Agent Instructions

## Project Overview

**changenog** is a zero-config changelog generator with monorepo support, written in Rust. It parses Git tags and commits since the last changelog entry, filters them by the current working directory, and writes missing entries to a `CHANGELOG.md` file. It is distributed as a CLI binary via Cargo (crates.io), npm, and PyPI.

## Repository Structure

```
src/              # Rust source code (core logic)
├── main.rs       # Entry point
├── changelog.rs  # Changelog generation
├── release.rs    # Release/entry building
├── cli/          # CLI argument parsing and validation
├── git/          # Git operations (tags, commits, root)
├── constant.rs   # Constants
├── log.rs        # Logging utilities
└── utils.rs      # Shared utilities

packages/
├── js/           # npm package wrapper (binary distribution)
└── python/       # PyPI package wrapper (binary distribution)

test/
├── main.ts       # Test runner (TypeScript, executed via tsx)
├── git.ts        # Git helper for tests
├── utils.ts      # Test utilities
├── changelogs/   # Fixture changelogs for test scenarios
└── output/       # Snapshot output (committed; changes indicate test failure)

scripts/          # Dev scripts (TypeScript): readme, release, version, toolchain
```

## Build & Development Commands

This project uses [`just`](https://github.com/casey/just) as a task runner.

| Command | Description |
|---------|-------------|
| `just build` | Build the Rust binary (`cargo build --release`) |
| `just test` | Build and run the full test suite |
| `just changenog` | Run changenog on this repo |
| `just readme` | Regenerate the README |
| `just version <kind>` | Bump version (major/minor/patch) |
| `just release <kind>` | Full release workflow |
| `just toolchain` | Run toolchain script |

### Prerequisites

- **Rust** (edition 2021; see `rust-toolchain.toml` for target list)
- **Node.js** >= 22
- **pnpm** 11.4.0 (set via `packageManager` field in `package.json`)
- **just** (command runner)

### Installing Dependencies

```bash
pnpm install
```

There is no separate Rust dependency install step; `cargo build` handles it.

### Running Tests

```bash
just test
```

This builds the release binary and runs the TypeScript test harness (`pnpm tsx ./test/main.ts`). Tests work by:

1. Creating a temporary git repo in `test/repo/`
2. Running the built binary with various argument combinations
3. Comparing output against committed snapshots in `test/output/`

**If test output changes**, the test runner prints a diff and fails. If the changes are expected, commit the updated `test/output/` files.

### Linting

```bash
pnpm eslint .
```

ESLint is configured via `eslint.config.mjs` with Prettier integration. Prettier config is in `.prettierrc`.

For Rust:

```bash
cargo clippy
cargo fmt --check
```

## Code Conventions

### Rust

- Edition 2021, stable toolchain
- Dependencies: `fancy-regex` (regex), `chrono` (dates)
- Release profile optimizes for binary size (LTO, strip, `opt-level = "s"`)
- Module structure uses `mod.rs` files for sub-modules

### TypeScript (scripts & tests)

- ESM modules (`"type": "module"` in `package.json`)
- Strict TypeScript (`tsconfig.json` with all strict flags enabled)
- `tsx` is used to run `.ts` files directly (no compile step)
- Prettier: 2 spaces, no tabs, double quotes, trailing commas, 90 char print width
- Import order enforced by `@trivago/prettier-plugin-sort-imports`

### Commits

Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification:

- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation changes
- `perf:` — performance improvement
- `refactor:` — code refactoring
- `chore:` — maintenance tasks
- `ci:` — CI/CD changes

### Git & Branching

- Tags follow `v<major>.<minor>.<patch>` format (e.g., `v2.0.7`)
- The CI test workflow triggers on version tags (`v*.*.*`)

## Key Architectural Notes

- The Rust binary is the single source of truth for changelog generation logic
- The `packages/js` and `packages/python` directories are thin wrappers that download and invoke the pre-built platform binary
- The test harness is a snapshot-based system: `test/output/` contains expected outputs that are committed to the repo
- There is no separate unit test framework for Rust (`cargo test` is not used); all testing goes through the integration test harness via `just test`

## Common Pitfalls

- **Test output changes**: When modifying changelog generation logic, the snapshot files in `test/output/` will likely need updating. Run `just test`, review the diff, and commit the new snapshots if correct.
- **No `cargo test`**: Do not add or rely on `cargo test`; the project uses a custom integration test approach.
- **pnpm workspace**: The root `pnpm-workspace.yaml` exists but only configures allowed builds for native dependencies. There is no workspace linking between packages.
- **Cross-platform line endings**: The test setup forces LF line endings via `.gitattributes` in the test repo to ensure consistent snapshots across platforms.
