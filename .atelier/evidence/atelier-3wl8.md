---
created_at: "2026-11-10T21:37:15.917073708+00:00"
id: "atelier-3wl8"
evidence_type: "test"
commit: "b0b73a1c39c3f92c8a344720e3b6c5156b6b7240"
result: "pass"
captured_at: "2026-11-10T21:37:15.916960026+00:00"
target:
  kind: "issue"
  id: "atelier-rsfm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rsfm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "CI job 'next' from .github/workflows/ci.yml run as written by run_next_job.py in a scratch clone of branch atelier-rsfm at b0b73a1c (rm -rf ../crates; cargo fmt -- --check; cargo build --locked --all-targets; cargo nextest run --locked): all steps passed, 154 of 154 tests. git diff --check master...HEAD clean; git diff --stat master...HEAD -- crates/ empty. Old code with the gh double slowed 0.1 s failed at_the_bound_merge_exits_not_yet_and_changes_nothing 4 of 6 runs with the CI text; each new test failed on the old code for its reason."
updated_at: "2026-11-10T21:37:16.137862461+00:00"
---

CI job 'next' from .github/workflows/ci.yml run as written by run_next_job.py in a scratch clone of branch atelier-rsfm at b0b73a1c (rm -rf ../crates; cargo fmt -- --check; cargo build --locked --all-targets; cargo nextest run --locked): all steps passed, 154 of 154 tests. git diff --check master...HEAD clean; git diff --stat master...HEAD -- crates/ empty. Old code with the gh double slowed 0.1 s failed at_the_bound_merge_exits_not_yet_and_changes_nothing 4 of 6 runs with the CI text; each new test failed on the old code for its reason.
