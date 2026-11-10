---
created_at: "2026-11-10T20:14:24.931220589+00:00"
id: "atelier-xzvf"
evidence_type: "test"
commit: "0e001abe1d118027704408dea9f6c71b912db0c2"
result: "pass"
captured_at: "2026-11-10T20:14:24.931106938+00:00"
target:
  kind: "issue"
  id: "atelier-yptz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yptz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 0e001abe in next/: cargo fmt --check clean; cargo clippy --all-targets -- -D warnings clean; cargo nextest run 145 passed. The new test the_printed_commit_works_from_a_subdirectory_and_with_paths_a_shell_would_split was seen failing at its subdirectory step with the fix stashed, and at its quoting step with quoting disabled, before passing with both."
updated_at: "2026-11-10T20:14:25.149839117+00:00"
---

At 0e001abe in next/: cargo fmt --check clean; cargo clippy --all-targets -- -D warnings clean; cargo nextest run 145 passed. The new test the_printed_commit_works_from_a_subdirectory_and_with_paths_a_shell_would_split was seen failing at its subdirectory step with the fix stashed, and at its quoting step with quoting disabled, before passing with both.
