build-cmd := "cargo build --release -Z unstable-options --artifact-dir"

build:
    cargo build --release
build-js:
    {{ build-cmd }} ./packages/js
pack-js:
    just build-js && pnpm pack --dir ./packages/js --pack-destination ../../
changenog:
    ./target/release/changenog --overwrite --commit-filter-preset=angular --commit-filter-preset=angular-readme-only-docs
release-patch:
    pnpm tsx ./scripts/release patch
release-minor:
    pnpm tsx ./scripts/release minor
release-major:
    pnpm tsx ./scripts/release major