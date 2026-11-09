---
schema: atelier.review
schema_version: 2
id: atelier-ppqm
title: review on atelier-7tul @ba6f39c6
target: atelier-7tul
identity: codex-reviewer (gpt-6.1-sol)
commit: ba6f39c6d5624b7ba0f503e58de4dbcb454c7b5c
created_at: 2026-11-09T03:36:50.121648716Z
event:
  event: review
---

Verdict: merge after R1 and R2 are fixed and independently re-reviewed; the typed CI gate removes redundant catch-up runs while keeping forge use optional. Approval withheld.
Sizes: complexity medium (provider, policy parsing, merge and land integration); value high (required forge delivery increment); risk medium (shared CLI and misleading land proof, with the real merge gate still checking the correct branch).
Contract: fail. A supported completed work branch with a sync merge can pass ci.passed against the wrong head, and a valid slash-containing Git remote name is rejected.
Fitness: fail pending the two targeted fixes; the base-policy authority, shell checks on the merged tree and no-forge compatibility otherwise look sound.
Findings:
- R1 atelier-oqrm, medium, FIX NOW, crates/atelier-app/src/ci_passed.rs:66. An ordinary branch HEAD merge is mistaken for the synthetic integration merge. Fresh scratch reproduction: legacy done work with src.rs and no CI runs changes from land: work in progress (ci.passed open) to land: passed after a clean sync of a record-only main commit; HEAD^2 has no src.rs. Pass the actual context/pins and test this land path.
- R2 atelier-kldl, low, FIX NOW, crates/atelier-app/src/forge.rs:92. upstream/github resolves through git remote get-url but the provider classifies it as a URL, and merge --check refuses before gh. Resolve remote names before falling back to URL parsing and cover slash-containing names.
Proof: independently read the brief, whole code/doc diff, ADR 0031, strategy, decision atelier-vmq2, item requirements, previous script and merge/land callers. Fresh cargo nextest run -j 4 -p atelier-cli --test forge_ci: 8/8 passed; -p atelier-cli --test merge_gates --features test-hooks: 98/98 passed; -p atelier-app -p atelier-workflow: 150/150 passed. All builds used CARGO_TARGET_DIR=/home/supadava/.cache/atelier-target/7tul-review and TMPDIR=/home/supadava/.cache/atelier-tmp/7tul-review. cargo fmt -- --check, git diff --check, cargo metadata --locked --format-version 1 and the branch binary atelier check passed (check emitted existing Done when warnings). Real branch binary merge atelier-7tul --check refused on stale planning approval and scripts/merge-ci-passed.sh finding no successful CI run. Read PR 62 checks: CI run 42556773837 was in progress on both Linux and macOS. Two independent scratch CLI reproductions confirmed R1 and R2; no source was edited.
Open questions: none blocking beyond the findings.
Residual risk and tests not run: full CLI suites and cross-repository smoke were not rerun; the targeted suites cover this review. CI and the retained base-policy script must pass separately before merge. As ADR 0031 accepts, a base gaining code can leave the combined tree untested until base CI runs. The changed docs restricting ci_workflow to a file name, using manual Next steps during this increment, and describing the latest push run for the advisory base note are consistent with this increment's scope.
Changes: only review records, pinned to ba6f39c6d5624b7ba0f503e58de4dbcb454c7b5c; no approval.
