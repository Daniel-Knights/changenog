build:
    cargo build --release
changenog:
    just build && ./target/release/changenog --overwrite --commit-filter-preset=angular --commit-filter-preset=angular-readme-only-docs
release-patch:
    pnpm tsx ./scripts/release patch
release-minor:
    pnpm tsx ./scripts/release minor
release-major:
    pnpm tsx ./scripts/release major