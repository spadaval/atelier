---
created_at: "2026-06-15T18:11:27.435152568+00:00"
id: "atelier-zzt8"
evidence_type: "validation"
captured_at: "2026-06-15T18:11:27.435036751+00:00"
target:
  kind: "issue"
  id: "atelier-rxgn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rxgn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Markdown round-trip validation after records extraction: cargo test -p atelier-records -- --nocapture passed 34 tests covering issues, missions, milestones, plans, evidence, relationships, and record-store mutation paths; target/debug/atelier export --check reported canonical export current; direct Markdown edit scenario in a temp initialized repo changed .atelier/issues/atelier-evbv.md title/body, then target/debug/atelier doctor --fix repaired projection and issue show, issue list --ready, and search all reflected 'Round trip edited title' and 'After direct Markdown edit'."
updated_at: "2026-06-15T18:11:33.468536939+00:00"
---

Markdown round-trip validation after records extraction: cargo test -p atelier-records -- --nocapture passed 34 tests covering issues, missions, milestones, plans, evidence, relationships, and record-store mutation paths; target/debug/atelier export --check reported canonical export current; direct Markdown edit scenario in a temp initialized repo changed .atelier/issues/atelier-evbv.md title/body, then target/debug/atelier doctor --fix repaired projection and issue show, issue list --ready, and search all reflected 'Round trip edited title' and 'After direct Markdown edit'.
