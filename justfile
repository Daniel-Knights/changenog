build-cmd := "cargo build --release -Z unstable-options --artifact-dir"

run-prebuilt:
		./target/debug/changenog --overwrite
build:
    cargo build --release
build-js:
    {{ build-cmd }} ./packages/js
pack-js:
		pnpm pack --dir ./packages/js --pack-destination ../../
release-patch:
    tsx ./scripts/release patch
release-minor:
    tsx ./scripts/release minor
release-major:
    tsx ./scripts/release major