---
schema: atelier.review
schema_version: 3
round: 1
commit: b42ea92f730d64feea17666d748f2e7d9691bbf4
reviewers:
- thread:delegated-task:command%3Amcp%3A4743e866-2fda-43e5-bbfe-d74ef890eb59%3Adelegate-task%3Abc35fefe-339e-4c5c-bacf-12154f72a0bc
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T06:27:31.016327268Z
---

## Summary

Complexity: medium. Three library crates and their CLI and Desk callers
move together, although the edits are mechanical and introduce no new
runtime component.

Value: high. The mission's later items need the final crate and module
names, and this makes the documented core the library both programs call.

Risk of merging: low. No executable logic, record format or test assertion
changes; compilation and the existing integration tests cover the affected
callers, and the move is reversible.

The value warrants the change. A smaller rename would leave the crate
boundaries and duplicate names that this item is meant to remove.

I reviewed b42ea92f730d64feea17666d748f2e7d9691bbf4 against base
b9f10590f550155c650e3d9a42f26cfc4f2d0077 and checked every Done when
line and the Report. I read the rename-aware diff with `git diff -M -B`,
which pairs the overlapping `lib.rs` and `review.rs` moves correctly. A
script compared all 78 Rust source and test files after mapping their
crate and module paths and import order; I inspected the remaining
comment and rustfmt differences. All 17 test fixtures are byte-for-byte
identical. Test inputs and assertions are unchanged apart from the
explicitly allowed fixture path and tracker-crate list.

The workspace has exactly `atelier-core`, `atelier-cli` and
`atelier-desk`. The core's dependency union and the lockfile add no
external dependency. Its `model` files contain no filesystem or process
calls or references to sibling modules. The old libraries' source and
tests are accounted for in the new layout. The CLI and Desk use only the
merged tracker crate. I checked the three-row crate table, both mission
rules in Decisions, the updated Desk architecture paths, and every
remaining old-crate match outside `.atelier/`; those matches describe the
old tool or the documented pre-merge layout.

Validation on the reviewed branch passed: `cargo fmt -- --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo nextest run -j 4`,
and `git diff --check`. Nextest ran 362 tests: 362 passed, with one
additional test skipped. I also built and tested a separate snapshot of
the exact base: 362 passed and the same one skipped. The base has 30 test
binaries and the merged branch 28, as expected when the library crates
combine. Builds used this checkout's own `target/`; both test runs used
`TMPDIR=$HOME/.cache/atelier-tmp/ni3d-review`, which is left in place.

I did not run `pnpm e2e`. Its only changed file replaces a crate name in
one fixture title; I checked the browser tests' references to that fixture
and found no assertion tied to the old title. The forge reported no CI
check results when inspected, so these results are local validation.

No findings.
