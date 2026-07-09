---
created_at: "2026-07-09T16:29:50.107971960+00:00"
id: "atelier-8k4e"
evidence_type: "validation"
captured_at: "2026-07-09T16:29:50.107959915+00:00"
target:
  kind: "issue"
  id: "atelier-nzu9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-nzu9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent epic validation at exact candidate 177ffc70: PASS outcome 1—issue list rendered a flat Issue Inventory over 50 records with representative mission/type, done-category, validation-status, label and priority filters; quiet output was IDs only, byte-identical across repeated runs, sorted deterministically, and --limit 2 returned exactly two IDs; unmatched label produced the explicit empty state; human output contained no Subissues, Hierarchy, Dashboard, Blocked by, or Ready Work formatting. PASS rejection/help contract—invalid category and invalid status failed with specific errors; removed --ready and --blocked failed as unexpected arguments; help exposed status/category/issue-type/label/priority/limit and omitted removed flags. PASS outcome 2—operational selection routes are present as atelier work ready and atelier work blocked. PASS focused integration regression evidence atelier-mqtk. PASS cargo fmt -- --check and git diff --check. Ignored-test review: the focused test is active (1 run, 472 filtered/skipped by name selection), not ignored; no ignored test is relied upon. Docs/help consistency: candidate help matches the inventory contract. Residual risk: full workspace suite deferred to final mission validation; global installed CLI uses newer cache schema than this exact historical candidate, so all behavioral proof used ./target/debug/atelier built from 177ffc70."
updated_at: "2026-07-09T16:29:50.110041067+00:00"
---

Independent epic validation at exact candidate 177ffc70: PASS outcome 1—issue list rendered a flat Issue Inventory over 50 records with representative mission/type, done-category, validation-status, label and priority filters; quiet output was IDs only, byte-identical across repeated runs, sorted deterministically, and --limit 2 returned exactly two IDs; unmatched label produced the explicit empty state; human output contained no Subissues, Hierarchy, Dashboard, Blocked by, or Ready Work formatting. PASS rejection/help contract—invalid category and invalid status failed with specific errors; removed --ready and --blocked failed as unexpected arguments; help exposed status/category/issue-type/label/priority/limit and omitted removed flags. PASS outcome 2—operational selection routes are present as atelier work ready and atelier work blocked. PASS focused integration regression evidence atelier-mqtk. PASS cargo fmt -- --check and git diff --check. Ignored-test review: the focused test is active (1 run, 472 filtered/skipped by name selection), not ignored; no ignored test is relied upon. Docs/help consistency: candidate help matches the inventory contract. Residual risk: full workspace suite deferred to final mission validation; global installed CLI uses newer cache schema than this exact historical candidate, so all behavioral proof used ./target/debug/atelier built from 177ffc70.
