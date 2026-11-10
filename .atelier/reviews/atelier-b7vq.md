---
schema: atelier.review
schema_version: 2
id: atelier-b7vq
title: approval on atelier-mal9 @e3a7e6ee
target: atelier-mal9
identity: sol-mal9-r4
commit: e3a7e6ee4530a0587dbbb02b9066c8c49acf8227
created_at: 2026-11-10T15:44:06.692648992Z
event:
  event: approval
---

Verdict: merge. This supplies the schema-version-3 record store required by the rewrite without a dependency on the installed workspace.
Sizes: complexity high (a new record format and its cross-review rules); value high (the prerequisite for the new commands); risk medium (durable state and finding closure rules, isolated in next/).
Contract: pass. Checked the whole branch against every current Done when line, the Records specification and the recorded decisions accepting the config and fence changes.
Fitness: pass. Domain rules, Markdown parsing and filesystem access have distinct owners; the added structures serve the record contract. No new findings and no fix_now recommendation. The four earlier findings, atelier-wvqx, atelier-6a6v, atelier-x7cu and atelier-86l6, are fixed at the pinned head.
Proof freshly run: git diff --check HEAD^ HEAD -- . ':!.atelier' and the whole-range whitespace check; next/ cargo fmt -- --check, cargo metadata --locked --format-version 1, cargo build --locked --all-targets -j 4, and cargo nextest run --locked --build-jobs 4 --test-threads 4 (38 passed), with the specified TMPDIR. Root formatting and locked metadata also passed, addressed by --manifest-path from next/ (formatting required --all there). scripts/ci-jobs.test.sh passed all 24 cases. The counter self-check passed all 9 cases; HEAD next reports 1,223 non-test lines. No file under root crates/ changed and next/target is ignored. Two extra read-only compiled probes passed: inline code spans of lengths 3 through 8 do not hide findings, and 40 top-level fence cases preserve body text without settling an example Taken line.
Isolation proof inspected: the workflow deletes ../crates before its first cargo step; the runner executes its run steps through bash -e in next/. Inspected evidence atelier-aoip and the existing scratch commit 5995d75f, including its old-crate dependency and use. Did not recreate or rerun the scratch injection experiment, which would require code writes outside this read-only review. The fresh build and test results above are from the review worktree, with root crates present.
Open questions: none blocking. Residual risk: legal unclosed fences remain accepted by the recorded decision; callers carrying a finding must name the latest carrier. Future command integration, mission records and commit-object loading are outside this item. The full old-workspace test suite, cross-repository smoke test and remote CI were not run in this constrained review.
