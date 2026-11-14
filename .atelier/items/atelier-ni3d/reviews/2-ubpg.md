---
schema: atelier.review
schema_version: 3
round: 2
commit: a0c236dc4ce32c99b489d5ca71b217be91525579
reviewers:
- thread:delegated-task:command%3Amcp%3A4743e866-2fda-43e5-bbfe-d74ef890eb59%3Adelegate-task%3A90984982-9de5-41b8-8074-a8f940f2def8
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T06:32:23.466363713Z
---

## Summary

Complexity: medium. The three libraries, their tests and both callers move
together, but the change adds no runtime component.

Value: high. The mission's later items need the final crate and module
names, and both programs now call the crate the design calls the core.

Risk of merging: low. Executable logic, record formats and test assertions
are unchanged, and the existing integration tests cover the callers.

The value warrants the scope. A smaller rename would leave the crate
boundaries this item is intended to remove.

Reviewed a0c236dc4ce32c99b489d5ca71b217be91525579 against the current base,
d9dfe16c9123a6f21d61d15f36a3b50d3a54fbd5, and checked every Done when
line and the Report. Compared with the first reviewed commit, b42ea92f7,
round 2 brings in the 0.6.0 release from master and its records; no Rust
source, test or architecture-document change followed the first review.

I read the rename-aware source and test diffs with `git diff -M -B`.
A scripted comparison accounted for all 78 Rust files on both sides,
mapping crate and module paths and comparing imports independently of
their order. I inspected the remaining doc comments, module declarations,
trailing commas and rustfmt changes. All 17 non-Rust test fixtures are
byte-for-byte identical. The tests change only crate/module paths, the
moved fixture directory and the explicitly allowed tracker-crate list;
their inputs and assertions remain the same.

Locked Cargo metadata lists exactly atelier-core, atelier-cli and
atelier-desk. Both programs depend only on atelier-core among the tracker
libraries, and the core depends on neither program. Comparing manifests
confirms the core's dependencies are exactly the union of the old three;
every external lockfile package is unchanged. The old libraries' files
are accounted for in model/, records/ and the crate root. Model files name
no filesystem or process APIs or sibling module. The source moves and
their inline-path exceptions match the Report.

I checked the three-row design table, the two mission rules in Decisions,
the Desk architecture paths and the remaining old-crate matches from
`git grep` outside .atelier/. Those matches describe the old tool or the
explicitly historical pre-merge layout, as the Report lists.

Validation on the reviewed branch passed: `cargo fmt -- --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo nextest run -j 4`,
locked Cargo metadata and `git diff --check`. Nextest ran 362 tests across
28 binaries: 362 passed and one additional test was skipped. I also ran
the exact reported base, b9f10590f550155c650e3d9a42f26cfc4f2d0077, in a
separate snapshot with its Git metadata: 362 passed and the same one
skipped across 30 binaries. The initial source archive lacked Git metadata
and failed the Desk build-commit test; adding the base's metadata resolved
that setup failure without changing source. Both runs used this checkout's
target/ and TMPDIR=$HOME/.cache/atelier-tmp/ni3d-review2, left in place.

The forge's CI changes, test and desk jobs all passed on the reviewed head.
I did not run pnpm e2e locally. Its only changed input is a made-up item's
title; I checked its browser-test references and found no assertion tied
to the old title. No live merge or deployment was performed.

No findings.
