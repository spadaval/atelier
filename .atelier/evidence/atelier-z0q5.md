---
created_at: "2026-10-28T01:19:37.683903197+00:00"
id: "atelier-z0q5"
evidence_type: "ci"
commit: "35aab9d13627f1c7d86bf0a94199c010de586ddf"
result: "pass"
captured_at: "2026-10-28T01:19:37.683794494+00:00"
uri: "https://github.com/spadaval/atelier/actions/runs/40614244272"
agent_identity: "claude-opus-5-5 (orchestrator)"
target:
  kind: "issue"
  id: "atelier-lnwf"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-lnwf"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Negative check: probe commit 9714dde5 (a misformatted fn appended to atelier-core/src/lib.rs) failed CI run 40614244272 at the 'cargo fmt -- --check' step on both ubuntu-latest and macos-latest. The probe was then dropped from the branch (force-pushed back to 35aab9d1)."
updated_at: "2026-10-28T01:19:37.684689502+00:00"
---

Negative check: probe commit 9714dde5 (a misformatted fn appended to atelier-core/src/lib.rs) failed CI run 40614244272 at the 'cargo fmt -- --check' step on both ubuntu-latest and macos-latest. The probe was then dropped from the branch (force-pushed back to 35aab9d1).
