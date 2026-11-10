# Scorecard baseline: counted measures

Written by `scripts/scorecard/counted.py` from the snapshot described by `manifest.json`. Window: 2026-10-14T00:00:00Z to the cutoff, 2026-11-10T05:14:58Z. T3 schema: T3 statev2 with orchestration_v2 projections, as copied on 2026-11-10 (manifest schema_sha256 ebd75cd5...). How each number is taken is in `scripts/scorecard/README.md`.

| Measure | atelier | astromech | demiurge | life-on-rails | cosplay-studio |
| --- | --- | --- | --- | --- | --- |
| Merged items (done in window) | 243 (305) | 339 (383) | 83 (107) | 48 (51) | 8 (23) |
| Merged items with a tied thread | 147 | 196 | 27 | 34 | 3 |
| Threads tied to a merged item | 346 of 721 | 476 of 602 | 64 of 105 | 89 of 296 | 6 of 49 |
| Process share of shell commands | 28.1% of 14295 | 17.9% of 17485 | 7.6% of 3021 | 20.4% of 5314 | 12.2% of 408 |
| Process share of token cost | 21.6% (342 of 346 threads) | 13.2% (476 of 476 threads) | 12.5% (64 of 64 threads) | 16.2% (89 of 89 threads) | 7.0% (6 of 6 threads) |
| Confusion per 100 atelier calls | 25.13 of 4640 | 23.57 of 4140 | 16.39 of 671 | 22.52 of 1239 | 24.27 of 342 |
| Refused or errored atelier calls | 567 | 410 | 43 | 79 | 37 |
| Help lookups | 632 | 589 | 72 | 213 | 50 |
| Tokens per merged item, median USD | 3.06 (142 items; 342 of 346 threads) | 3.22 (191 items; 476 of 476 threads) | 7.9 (27 items; 64 of 64 threads) | 11.96 (34 items; 89 of 89 threads) | 13.81 (3 items; 6 of 6 threads) |
| Tokens per merged item, mean USD | 7.61 | 9.11 | 11.79 | 13.4 | 18.2 |
| Calls before first edit, median tool calls | 22.0 (80 threads) | 18.0 (132 threads) | 10.0 (49 threads) | 17.0 (31 threads) | 21.0 (4 threads) |
| Calls before first edit, median shell commands | 20.0 | 16.5 | 10.0 | 16.0 | 17.5 |
| Rework | 10.8% of 57671 lines | 7.2% of 209453 lines | 3.4% of 38189 lines | 5.5% of 74203 lines | n/a of 0 lines |
| Rework: commits old enough to count | 169 (e2c03600..5abb3651) | 116 (d242981a..320366a0) | 34 (8f8a86fe..a658fe5d) | 33 (82cfadf8..b1aa4880) | 0 (-..-) |
| Record-only share of commits | 76.0% of 2545 | 73.6% of 2492 | 69.3% of 737 | 52.1% of 539 | 69.2% of 156 |
| Non-test lines | 48989 Rust | 78713 Rust | 73155 TypeScript | 29285 TypeScript | 8380 TypeScript |

Each token number has its coverage beside it: threads tied to a merged item that have a surviving transcript, out of those threads. Tokens are not estimated for threads without one. An item's tokens count only when every thread tied to it has a transcript and every request in them has a list price.

Rework counts lines that reached the base from 2026-10-14T00:00:00Z until 2026-10-27T05:14:58Z, so each was watched for 14 days before the cutoff; the row above names the first and last first-parent commit old enough to count.

## What counted as a test

- atelier: Rust (.rs); tests are as `scripts/count_non_test_rust.py` counts them: files whose path contains `/tests/`, `src/test_records.rs`, and `#[cfg(test)]` items.
- astromech: Rust (.rs); tests are as `scripts/count_non_test_rust.py` counts them: files whose path contains `/tests/`, `src/test_records.rs`, and `#[cfg(test)]` items.
- demiurge: TypeScript (.ts, .tsx); tests are a path with a `test`, `tests`, `__tests__`, `androidTest`, `testFixtures`, `e2e`, `fixture` or `fixtures` directory, a file named `*.test.*`, `*.spec.*`, `*_test.*` or `*-test.*`, or a Kotlin file ending `Test.kt` or `Tests.kt`.
- life-on-rails: TypeScript (.ts, .tsx); tests are a path with a `test`, `tests`, `__tests__`, `androidTest`, `testFixtures`, `e2e`, `fixture` or `fixtures` directory, a file named `*.test.*`, `*.spec.*`, `*_test.*` or `*-test.*`, or a Kotlin file ending `Test.kt` or `Tests.kt`.
- cosplay-studio: TypeScript (.ts, .tsx); tests are a path with a `test`, `tests`, `__tests__`, `androidTest`, `testFixtures`, `e2e`, `fixture` or `fixtures` directory, a file named `*.test.*`, `*.spec.*`, `*_test.*` or `*-test.*`, or a Kotlin file ending `Test.kt` or `Tests.kt`.

## List prices

USD per million tokens, from the 2026-11 assessment's table. A model not listed is counted but not priced.

| Model | Input | Cache read | Cache write | Output |
| --- | --- | --- | --- | --- |
| claude-haiku-5-5 | 1.0 | 0.1 | 2.0 | 5.0 |
| claude-opus-5-5 | 4.0 | 0.2 | 8.0 | 20.0 |
| claude-sonnet-5-5 | 3.0 | 0.3 | 6.0 | 15.0 |
| gpt-6-luna | 0.1 | 0.01 | 0.1 | 0.5 |
| gpt-6.1-sol | 2.0 | 0.1 | 2.0 | 10.0 |
| grok-4.6-build | 2.0 | 0.5 | 2.0 | 6.0 |
