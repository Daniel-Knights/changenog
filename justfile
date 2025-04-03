build:
    cargo build --release
test:
    just build && pnpm tsx ./test/main
changenog:
    just build && ./target/release/changenog --overwrite --commit-filter-preset=angular --commit-filter-preset=angular-readme-only-docs
toolchain:
    pnpm tsx ./scripts/toolchain
release-patch:
    pnpm tsx ./scripts/release patch
release-minor:
    pnpm tsx ./scripts/release minor
release-major:
    pnpm tsx ./scripts/release major