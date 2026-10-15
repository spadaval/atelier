---
created_at: "2026-10-15T19:29:46.782461179+00:00"
id: "atelier-ofmq"
evidence_type: "validation"
commit: "8f13d015eda0d0d702e77bf318fb686565e62955"
result: "pass"
captured_at: "2026-10-15T19:29:46.781990170+00:00"
agent_identity: "independent-validator-2026-10-15"
target:
  kind: "issue"
  id: "atelier-xz11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-cmi9"
    role: "validates"
  - kind: "issue"
    id: "atelier-xz11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S5 PASS (independent-validator-2026-10-15, candidate 8f13d015). Non-test Rust = 19,433 lines (<20,000): scripts/count_non_test_rust.py 8f13d015 (physical lines incl. blanks/comments from git objects; excludes 16 files under tests/ and the cfg(test) module src/test_records.rs (124 lines); masks comments/literals and removes 42 complete #[cfg(test)] item spans = 6,810 lines; self-check 9/9). Per crate: app 5163, cli 9263, core 634, records 2644, workflow 1729. Independent cross-check /tmp/xz11/indent_count.py (rustfmt-indentation item ends, no lexing): 19,438; per-file diff only human_output.rs (+5, my method ends a multi-line '#[cfg(test)] fn ... (\\n ...\\n) -> Self {' early). Raw: all .rs 43,558; outside tests/ 26,324 (the external ~26k figure includes cfg(test) modules). At 76cde2a9 the same script gave 19,390. Commands: atelier --help lists init, setup, prime, land, status, work, issue, start, bundle, evidence, review, merge, check, prune; no clap 'hide' attributes in source; branch, doctor, man, export, import, history, telemetry, activity, forgejo, room, cache, sync all rejected as unknown. Source/Cargo.lock grep: no forgejo, sqlite/rusqlite, review room, transition action, activity, beads, telemetry, doctor, man-page code (only stale test doc comments mention export/import)."
updated_at: "2026-10-15T19:30:08.486343859+00:00"
---

S5 PASS (independent-validator-2026-10-15, candidate 8f13d015). Non-test Rust = 19,433 lines (<20,000): scripts/count_non_test_rust.py 8f13d015 (physical lines incl. blanks/comments from git objects; excludes 16 files under tests/ and the cfg(test) module src/test_records.rs (124 lines); masks comments/literals and removes 42 complete #[cfg(test)] item spans = 6,810 lines; self-check 9/9). Per crate: app 5163, cli 9263, core 634, records 2644, workflow 1729. Independent cross-check /tmp/xz11/indent_count.py (rustfmt-indentation item ends, no lexing): 19,438; per-file diff only human_output.rs (+5, my method ends a multi-line '#[cfg(test)] fn ... (\n ...\n) -> Self {' early). Raw: all .rs 43,558; outside tests/ 26,324 (the external ~26k figure includes cfg(test) modules). At 76cde2a9 the same script gave 19,390. Commands: atelier --help lists init, setup, prime, land, status, work, issue, start, bundle, evidence, review, merge, check, prune; no clap 'hide' attributes in source; branch, doctor, man, export, import, history, telemetry, activity, forgejo, room, cache, sync all rejected as unknown. Source/Cargo.lock grep: no forgejo, sqlite/rusqlite, review room, transition action, activity, beads, telemetry, doctor, man-page code (only stale test doc comments mention export/import).
