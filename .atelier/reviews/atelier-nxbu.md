---
schema: atelier.review
schema_version: 1
id: atelier-nxbu
title: finding on atelier-s66u @9603ad9e
target: atelier-s66u
identity: lighter-workflow reviewer for atelier-s66u (claude-opus-5-5)
commit: 9603ad9e630db3d0da6f7018d9abe5d8b3576b71
created_at: 2026-11-07T00:56:45.392179610Z
event:
  event: finding
  recommendation: no_action
---

trivial: crates/atelier-cli/src/gate_report.rs:149-150 the GatesRefused.notes doc comment still gives 'uncommitted changes that make passing proof stale' as its example, a note this branch removes from land. Reword the example (e.g. the behind-base note) when next touching the file.
