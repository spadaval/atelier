---
created_at: "2026-11-11T09:19:08.313474499+00:00"
id: "atelier-2meo"
evidence_type: "validation"
commit: "d03c1d6d40c329d9a64ec0d5b6e855fe6dac9294"
result: "pass"
captured_at: "2026-11-11T09:19:08.313428813+00:00"
target:
  kind: "issue"
  id: "atelier-u1ko"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-u1ko"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 0669218b (after the fixes for atelier-jkys and atelier-y64p and the sync with master d86ab548): cargo build, fmt, clippy -D warnings and nextest (285, the Desk's dependencies and stale-types tests among them) pass; cargo install --root installs only atelier (55 crates, none of the Desk's); dist plan ships one app; rehearse.sh passes on all five in 37 s; trials.sh passes all 26 trials in 505 s with the real host's fingerprint, the Desk's service and clones included, unchanged; the five new trials failed against 5b066ecc's scripts before the fix. CI run 42993900303 (changes, test, desk) and Release run 42993900141 passed on 0669218b after merge --check showed review.current refusing; master and origin/master d86ab548 before and after."
updated_at: "2026-11-11T09:19:08.496414537+00:00"
---

At 0669218b (after the fixes for atelier-jkys and atelier-y64p and the sync with master d86ab548): cargo build, fmt, clippy -D warnings and nextest (285, the Desk's dependencies and stale-types tests among them) pass; cargo install --root installs only atelier (55 crates, none of the Desk's); dist plan ships one app; rehearse.sh passes on all five in 37 s; trials.sh passes all 26 trials in 505 s with the real host's fingerprint, the Desk's service and clones included, unchanged; the five new trials failed against 5b066ecc's scripts before the fix. CI run 42993900303 (changes, test, desk) and Release run 42993900141 passed on 0669218b after merge --check showed review.current refusing; master and origin/master d86ab548 before and after.
