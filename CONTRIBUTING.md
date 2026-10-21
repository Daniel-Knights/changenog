## Release Checklist

1. Add new API tokens for NPM and Crates.io
2. Bump Cargo deps and third-party GitHub actions
3. Run `just release <kind>`. This:
   1. Checks for uncommitted changes and runs the tests
   2. Bumps core package versions and syncs toolchain targets
   3. Pauses for review, then commits and tags the release
   4. Generates the changelog, pauses for review, then commits
   5. Pushes the commits and tag, triggering the CI workflow
