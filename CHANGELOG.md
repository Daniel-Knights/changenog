## [v2.0.0](https://github.com/Daniel-Knights/changenog/compare/v1.2.0...v2.0.0) (04/02/2025)

- feat(options): input-path and output options, uniform handling of expected values and defaults ([c1f60cc](https://github.com/Daniel-Knights/changenog/commit/c1f60cc06d49915ba0bb8f9cde058908ed84c8af))
- docs(readme): update usage guide ([6caa0a5](https://github.com/Daniel-Knights/changenog/commit/6caa0a56728da9fc006fe13581b7e9a96e93dc27))
- fix(options): correct arg descriptions and log formatting ([7326b92](https://github.com/Daniel-Knights/changenog/commit/7326b928ead7e3f82e3e90f6a04908a35de48287))
- feat(options): print version and help ([728ef88](https://github.com/Daniel-Knights/changenog/commit/728ef8830ea9057af6b1b77d4f7bbadd3a99e2e0))
- feat(options): structured option definitions, exit on boolean flags with values ([08c70fc](https://github.com/Daniel-Knights/changenog/commit/08c70fc6bb812bb2f8af5520bcfa9d86894d17c3))
- feat(options): improve args processing, support space separated args, exit on invalid args ([5f1cadf](https://github.com/Daniel-Knights/changenog/commit/5f1cadf291e22878ccd582741376381f2140deaf))
- perf: pass slice refs as args instead of vectors ([bbfbd13](https://github.com/Daniel-Knights/changenog/commit/bbfbd13ad986a518cb6f17c0e4626da95317db72))
- fix(parse): prev entry tag regex ([de24610](https://github.com/Daniel-Knights/changenog/commit/de24610f068cf519e58934c741627726476b28cc))
- fix(git): return none on empty remote url ([a192cdf](https://github.com/Daniel-Knights/changenog/commit/a192cdfdf2c3701630b282105942422942778fb0))
- docs(readme): add description for no-semver filter preset ([b605d29](https://github.com/Daniel-Knights/changenog/commit/b605d297a14551edacbe135e8d278db80bfbaa9c))
- feat(filters): no-semver preset ([cb4b5af](https://github.com/Daniel-Knights/changenog/commit/cb4b5af4678750a3f11a0a5845cb1ef3a58f8c77))
- feat!: remove semver requirement, replace tag-prefix flag with tag-filter-regex, rename commit filter flags ([29dfbca](https://github.com/Daniel-Knights/changenog/commit/29dfbca6dd49a74c5ffc1883a85959038b323c67))
- feat!: rust rewrite first draft, remove continue flag, remove locale flag and force ddmmyyyy, add remote-url flag, add tag-prefix flag and remove package name heuristics ([5a1bc24](https://github.com/Daniel-Knights/changenog/commit/5a1bc24647aa04ac13f265bc42527d662eb2b7f3))

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

## [v0.1.0](https://github.com/Daniel-Knights/changenog/compare/v0.0.1...v0.1.0) (25/08/2023)

- feat(deps): drop fs-extra, sync ([7dd00eb](https://github.com/Daniel-Knights/changenog/commit/7dd00eb8b19d7001bef786d280c83511c0ae8724))
- feat(links): get remote url from git log, github vs gitlab checks ([ddbcec4](https://github.com/Daniel-Knights/changenog/commit/ddbcec44e41495d826abcf7207d45635dc5a1c21))