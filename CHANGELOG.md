## [0.6.0](https://github.com/Daniel-Knights/changenog/compare/v0.5.2...v0.6.0) (3/27/24)

- fix: wrap git tag flag in quotes ([990c61d](https://github.com/Daniel-Knights/changenog/commit/990c61dd1ad4d1b950e859766f79bfcdbbe19f0a))
- chore(deps): bump ([418fed0](https://github.com/Daniel-Knights/changenog/commit/418fed0bf0d20b1a95f62cc5239653ac065c2a53))
- refactor(text formatting): var ordering and comments ([afe81dd](https://github.com/Daniel-Knights/changenog/commit/afe81dd8c91d40ea18b446e46995ae960acee71a))
- docs(changelog): v0.5.2 ([03afabd](https://github.com/Daniel-Knights/changenog/commit/03afabd614c632c01dd4ca7416d14a0d6418a2cb))

## [0.5.2](https://github.com/Daniel-Knights/changenog/compare/v0.5.1...v0.5.2) (31/08/2023)

- refactor: simplify get git root ([a107024](https://github.com/Daniel-Knights/changenog/commit/a10702480ca86117503e4cd7b06b13f087e8f2ac))
- fix: handle if no commits later than tag ([dcccc40](https://github.com/Daniel-Knights/changenog/commit/dcccc406bf3f85e91449b901b5bcb6d48a54cf6d))

## [0.5.1](https://github.com/Daniel-Knights/changenog/compare/v0.5.0...v0.5.1) (30/08/2023)

- perf: prevent unnecessary iterations ([4f148e1](https://github.com/Daniel-Knights/changenog/commit/4f148e147a414493a224a222333d83bc5ed8d721))
- docs(changelog): regenerate from scratch ([69cb7d7](https://github.com/Daniel-Knights/changenog/commit/69cb7d75e49f030dd7e5cf2bcfc1ff162b422834))
- docs(changelog): v0.5.0 ([c9b7811](https://github.com/Daniel-Knights/changenog/commit/c9b7811a84e293c06eaf2843370c509698f8db47))

## [0.5.0](https://github.com/Daniel-Knights/changenog/compare/v0.4.1...v0.5.0) (30/08/2023)

- feat(links): link to all tags if no previous version ([33e7b88](https://github.com/Daniel-Knights/changenog/commit/33e7b8890799fb76f619faafdf710809a4cf8923))
- docs(readme): tag parsing, remove requirements ([075da27](https://github.com/Daniel-Knights/changenog/commit/075da27effd1745c631a3de51528b97341254022))
- feat(option): max-commits, get arg fn ([20748cd](https://github.com/Daniel-Knights/changenog/commit/20748cd725f6b252c200cc975b1d506e345d5f90))
- feat: shorter date formats ([e69f7d1](https://github.com/Daniel-Knights/changenog/commit/e69f7d175ba850cdd9b80a9ec32c10ed27ed8ed6))
- feat: parse git tags, add all entries since last ([b1d9b0c](https://github.com/Daniel-Knights/changenog/commit/b1d9b0ce6840daa65597fe1e03e54a830aa95c09))
- chore: vscode launch config ([62bfd7d](https://github.com/Daniel-Knights/changenog/commit/62bfd7dccba522e985b4fb51dd55408c6b5e6a5c))
- fix: prev version and date regex ([0d1bd12](https://github.com/Daniel-Knights/changenog/commit/0d1bd12284d598522d90afefefb9a399398c6b70))
- docs(changelog): v0.4.1 ([de9f58f](https://github.com/Daniel-Knights/changenog/commit/de9f58f35acf64bf428963081700e1194e8e7aaa))

## [0.4.1](https://github.com/Daniel-Knights/changenog/compare/v0.4.0...v0.4.1) (29/08/2023)

- fix(links): only add compare link if prev tag exists ([8615182](https://github.com/Daniel-Knights/changenog/commit/86151826594b0644ba6f34ac4f07d1786ee91bb3))

## [0.4.0](https://github.com/Daniel-Knights/changenog/compare/v0.3.0...v0.4.0) (29/08/2023)

- docs(changelog): v0.4.0 ([26682e9](https://github.com/Daniel-Knights/changenog/commit/26682e9a57ad279e31e9582b2e9aee6bf38b7942))
- feat: parse git tags ([d37ec62](https://github.com/Daniel-Knights/changenog/commit/d37ec62d46144a20811ada3aa1e62973c21baa95))
- fix(links): remove git url suffix ([f73758d](https://github.com/Daniel-Knights/changenog/commit/f73758d6d87d8c9ffdb3994b94e0b4f0df0d1689))
- refactor(release): rename exec to run ([72c877e](https://github.com/Daniel-Knights/changenog/commit/72c877e8196ae68e1bc6545be1cde5d5c3388103))
- docs(changelog): v0.3.0 ([c0ac752](https://github.com/Daniel-Knights/changenog/commit/c0ac752c41eeb10fecfb097bb2af19423141180a))

## [0.3.0](https://github.com/Daniel-Knights/changenog/compare/v0.2.0...v0.3.0) (27/08/2023)

- feat: use exec over exec file ([33600df](https://github.com/Daniel-Knights/changenog/commit/33600df8a0ab55289ddce4baac3c038e0c99dfd3))
- ci(release): use spawn ([ea6d618](https://github.com/Daniel-Knights/changenog/commit/ea6d61880376d976c319c489bfaee34762055e97))

## [0.2.0](https://github.com/Daniel-Knights/changenog/compare/v0.1.1...v0.2.0) (25/08/2023)

- docs(changelog): v0.2.0 ([87103c0](https://github.com/Daniel-Knights/changenog/commit/87103c000fdcf7683c19f013e6092eddcf9e11f9))
- feat(option): continue ([3c7b6aa](https://github.com/Daniel-Knights/changenog/commit/3c7b6aa3d480fc1e545f6726fc39908e29651780))
- ci(release): clean dist ([25b9df3](https://github.com/Daniel-Knights/changenog/commit/25b9df37fa12268323eafa6202478370877381e0))
- build(ts): only include index js ([ab675de](https://github.com/Daniel-Knights/changenog/commit/ab675deec0615e3382df411b04ae2c6d4081b6f5))
- chore(deps): remove fs-extra types ([18120da](https://github.com/Daniel-Knights/changenog/commit/18120da98a9c7296ff63467e77b95d7d3397c0ac))
- feat: correct pkg type checks ([a720362](https://github.com/Daniel-Knights/changenog/commit/a7203627758093ae1de414f045c8f8768dda8cb0))

## [0.1.1](https://github.com/Daniel-Knights/changenog/compare/v0.1.0...v0.1.1) (25/08/2023)

- docs(changelog): v0.1.1 ([26ee1fd](https://github.com/Daniel-Knights/changenog/commit/26ee1fd8305c5480a7402a5614652a2e2f2e002e))
- ci(release): read package version at correct point ([bbec888](https://github.com/Daniel-Knights/changenog/commit/bbec8888f1184d2d5030a7e10c92d3a20a08ef5b))
- fix(links): compare link format ([f8e4a54](https://github.com/Daniel-Knights/changenog/commit/f8e4a54fb3dcc03e79f6d46931c84ff038d24ea6))

## [0.1.0](https://github.com/Daniel-Knights/changenog/compare/v0.0.1...v0.1.0) (25/08/2023)

- docs(changelog): v0.0.1 ([5b31fd6](https://github.com/Daniel-Knights/changenog/commit/5b31fd6059a84f0751b846d01f328ea85b6a9e03))
- ci(release): add release script ([aa28f1e](https://github.com/Daniel-Knights/changenog/commit/aa28f1ee3eb686d4dfd5fe23a7658d64543a0078))
- docs: flesh out readme, add description, links and keywords ([96c0212](https://github.com/Daniel-Knights/changenog/commit/96c0212b4f0d9bf0eaeaccc3c0b34efd4a84fb6d))
- chore(scripts):fix release command order ([73e0527](https://github.com/Daniel-Knights/changenog/commit/73e052797679e4f83cf031928c4f142743cb6b26))
- feat(deps): drop fs-extra, sync ([7dd00eb](https://github.com/Daniel-Knights/changenog/commit/7dd00eb8b19d7001bef786d280c83511c0ae8724))
- docs(changelog): 0.0.1 ([06dc838](https://github.com/Daniel-Knights/changenog/commit/06dc83839ae80164dd489ce70f5b2993147a60ef))
- feat(links): get remote url from git log, github vs gitlab checks ([ddbcec4](https://github.com/Daniel-Knights/changenog/commit/ddbcec44e41495d826abcf7207d45635dc5a1c21))

## [0.0.1](https://github.com/Daniel-Knights/changenog/tags) (25/08/2023)

- init ([53debdb](https://github.com/Daniel-Knights/changenog/commit/53debdbb7b10d3f0150e653d30456d6f796d8e5e))