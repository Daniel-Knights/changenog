## [v2.0.0](https://github.com/Daniel-Knights/changenog/compare/v1.2.0...v2.0.0) (03/04/2025)

- feat(cli): validate regex values ([07a3187](https://github.com/Daniel-Knights/changenog/commit/07a3187dad8950dd545b4b87aef3bb0efbcae027))
- fix(entry): don't merge commits from latest entry if filtered out ([4271c87](https://github.com/Daniel-Knights/changenog/commit/4271c879f94207bf77cbc5b62045e08881f09665))
- fix(entry): sort entry tags by name ([30484e6](https://github.com/Daniel-Knights/changenog/commit/30484e6b6301cca9028397bab2de4fa0a482b08a))
- feat: improve error-handling, fix arg format parsing ([7db4761](https://github.com/Daniel-Knights/changenog/commit/7db47610f996d2f226e2a66d786be78da7d0a9df))
- feat!: replace max-commits arg with max-entries, cement entry building strategy, use tag range logs instead of target commits ([955863e](https://github.com/Daniel-Knights/changenog/commit/955863e11c451f658533f529e91a22be4072bd95))
- feat: exit on no entries generated ([c8f1e8f](https://github.com/Daniel-Knights/changenog/commit/c8f1e8fcdf3b7110777a3f90b2274345a9d9e334))
- feat: handle multiple tags pointing to the same commit ([1b4249d](https://github.com/Daniel-Knights/changenog/commit/1b4249d4fa5144e2c2bbc23febbfadebb7d83057))
- feat(git): remove 1s commit offset ([8ab0799](https://github.com/Daniel-Knights/changenog/commit/8ab0799723822019da342d3580f49f954f37ac4a))
- feat(options): add root arg ([220df2e](https://github.com/Daniel-Knights/changenog/commit/220df2edeb3e036c4f41edbc65fb266e88eda46b))
- feat(changelog): ensure ending newline ([b852a21](https://github.com/Daniel-Knights/changenog/commit/b852a211fd46fab2f3e442f3bc447e5ee07babeb))
- feat(npm): prepend targets to bin commands ([d942038](https://github.com/Daniel-Knights/changenog/commit/d942038ffb7ee06c7734596cc2b8b4d2f28fc71c))
- feat(core): dynamic target exe loading ([cdb093d](https://github.com/Daniel-Knights/changenog/commit/cdb093dc2101892f1092ef9370ea1b300efa8815))
- feat!: target binaries ([f57ec11](https://github.com/Daniel-Knights/changenog/commit/f57ec11fae4663e39bfe596e0b6544b08e1e0821))
- feat(options): output option ([ed216b8](https://github.com/Daniel-Knights/changenog/commit/ed216b89df4b20fecdb8a78d7db43d0de43e5fa7))
- feat(options): print version and help ([59b09a1](https://github.com/Daniel-Knights/changenog/commit/59b09a17b39dd7f9096cd72fc459a0e3dc57c1a7))
- feat(options): exit on boolean flags with values ([b5691e9](https://github.com/Daniel-Knights/changenog/commit/b5691e90b4c09ea2eb33df8fd383bce05a703097))
- feat(options): support space separated args, exit on invalid args ([be8cfbe](https://github.com/Daniel-Knights/changenog/commit/be8cfbef1e1e7aab9c9900599fdb7504eb10e1a8))
- feat(filters): no-semver preset ([1383957](https://github.com/Daniel-Knights/changenog/commit/13839578a032bca92caba88144cda64a9042fef4))
- feat!: remove semver requirement, replace tag-prefix flag with tag-filter-regex, rename commit filter flags ([6ee76a6](https://github.com/Daniel-Knights/changenog/commit/6ee76a65b99e49889c84e28c0262b8ddeabb5716))
- feat!: remove continue flag, remove locale flag and force ddmmyyyy, add remote-url flag, add tag-prefix flag and remove package name heuristics ([4c893f7](https://github.com/Daniel-Knights/changenog/commit/4c893f7238e8de252cb645bbdd0d8c94e9c3dbc7))

## [v1.2.0](https://github.com/Daniel-Knights/changenog/compare/v1.1.0...v1.2.0) (22/12/2024)

- feat(filters): angular-readme-only-docs ([42f42f8](https://github.com/Daniel-Knights/changenog/commit/42f42f80520bd50b9234f62d5bff72964d987acf))

## [v1.1.0](https://github.com/Daniel-Knights/changenog/compare/v1.0.0...v1.1.0) (20/12/2024)

- feat(options): overwrite ([ce34638](https://github.com/Daniel-Knights/changenog/commit/ce3463842e22893fb388685043b196e2e8fb013f))
- feat: filters ([04d5b33](https://github.com/Daniel-Knights/changenog/commit/04d5b33cb697bb33536aaeab511d1e3c0adae569))

## [v1.0.0](https://github.com/Daniel-Knights/changenog/compare/v0.6.0...v1.0.0) (18/12/2024)

- feat: consistent locale for date formatting, add locale cli option ([e7f7019](https://github.com/Daniel-Knights/changenog/commit/e7f701966b85a721d5fc169f1ac1bc9911e3cf13))

## [v0.6.0](https://github.com/Daniel-Knights/changenog/compare/v0.5.2...v0.6.0) (27/03/2024)

- fix: wrap git tag flag in quotes ([990c61d](https://github.com/Daniel-Knights/changenog/commit/990c61dd1ad4d1b950e859766f79bfcdbbe19f0a))

## [v0.5.2](https://github.com/Daniel-Knights/changenog/compare/v0.5.1...v0.5.2) (31/08/2023)

- fix: handle if no commits later than tag ([dcccc40](https://github.com/Daniel-Knights/changenog/commit/dcccc406bf3f85e91449b901b5bcb6d48a54cf6d))

## [v0.5.1](https://github.com/Daniel-Knights/changenog/compare/v0.5.0...v0.5.1) (30/08/2023)

- perf: prevent unnecessary iterations ([4f148e1](https://github.com/Daniel-Knights/changenog/commit/4f148e147a414493a224a222333d83bc5ed8d721))

## [v0.5.0](https://github.com/Daniel-Knights/changenog/compare/v0.4.1...v0.5.0) (30/08/2023)

- feat(links): link to all tags if no previous version ([33e7b88](https://github.com/Daniel-Knights/changenog/commit/33e7b8890799fb76f619faafdf710809a4cf8923))
- docs(readme): tag parsing, remove requirements ([075da27](https://github.com/Daniel-Knights/changenog/commit/075da27effd1745c631a3de51528b97341254022))
- feat(option): max-commits, get arg fn ([20748cd](https://github.com/Daniel-Knights/changenog/commit/20748cd725f6b252c200cc975b1d506e345d5f90))
- feat: shorter date formats ([e69f7d1](https://github.com/Daniel-Knights/changenog/commit/e69f7d175ba850cdd9b80a9ec32c10ed27ed8ed6))
- feat: parse git tags, add all entries since last ([b1d9b0c](https://github.com/Daniel-Knights/changenog/commit/b1d9b0ce6840daa65597fe1e03e54a830aa95c09))
- fix: prev version and date regex ([0d1bd12](https://github.com/Daniel-Knights/changenog/commit/0d1bd12284d598522d90afefefb9a399398c6b70))

## [v0.4.1](https://github.com/Daniel-Knights/changenog/compare/v0.4.0...v0.4.1) (29/08/2023)

- fix(links): only add compare link if prev tag exists ([8615182](https://github.com/Daniel-Knights/changenog/commit/86151826594b0644ba6f34ac4f07d1786ee91bb3))

## [v0.4.0](https://github.com/Daniel-Knights/changenog/compare/v0.3.0...v0.4.0) (29/08/2023)

- feat: parse git tags ([d37ec62](https://github.com/Daniel-Knights/changenog/commit/d37ec62d46144a20811ada3aa1e62973c21baa95))
- fix(links): remove git url suffix ([f73758d](https://github.com/Daniel-Knights/changenog/commit/f73758d6d87d8c9ffdb3994b94e0b4f0df0d1689))

## [v0.3.0](https://github.com/Daniel-Knights/changenog/compare/v0.2.0...v0.3.0) (27/08/2023)

- feat: use exec over exec file ([33600df](https://github.com/Daniel-Knights/changenog/commit/33600df8a0ab55289ddce4baac3c038e0c99dfd3))

## [v0.2.0](https://github.com/Daniel-Knights/changenog/compare/v0.1.1...v0.2.0) (25/08/2023)

- feat(option): continue ([3c7b6aa](https://github.com/Daniel-Knights/changenog/commit/3c7b6aa3d480fc1e545f6726fc39908e29651780))
- feat: correct pkg type checks ([a720362](https://github.com/Daniel-Knights/changenog/commit/a7203627758093ae1de414f045c8f8768dda8cb0))

## [v0.1.1](https://github.com/Daniel-Knights/changenog/compare/v0.1.0...v0.1.1) (25/08/2023)

- fix(links): compare link format ([f8e4a54](https://github.com/Daniel-Knights/changenog/commit/f8e4a54fb3dcc03e79f6d46931c84ff038d24ea6))

## [v0.1.0](https://github.com/Daniel-Knights/changenog/tags) (25/08/2023)

- feat(deps): drop fs-extra, sync ([7dd00eb](https://github.com/Daniel-Knights/changenog/commit/7dd00eb8b19d7001bef786d280c83511c0ae8724))
- feat(links): get remote url from git log, github vs gitlab checks ([ddbcec4](https://github.com/Daniel-Knights/changenog/commit/ddbcec44e41495d826abcf7207d45635dc5a1c21))
