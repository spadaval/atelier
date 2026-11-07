---
schema: atelier.review
schema_version: 1
id: atelier-rjmn
title: finding on atelier-s66u @9603ad9e
target: atelier-s66u
identity: lighter-workflow reviewer for atelier-s66u (claude-opus-5-5)
commit: 9603ad9e630db3d0da6f7018d9abe5d8b3576b71
created_at: 2026-11-07T00:56:43.572976172Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/adr/0026-evidence-records-only-what-nothing-else-records.md:64-67, docs/product/validation.md:112-115, crates/atelier-app/src/evidence_freshness.rs:455-458 justify 'a later pass clears a failure for good' by 'a receipt always describes committed state: command capture refuses uncommitted code'. That is false for manual receipts: add_returning_id (commands/evidence.rs:91) pins HEAD and never checks the checkout. Reproduced: fail captured, uncommitted fix, 'evidence record --kind validation --result pass ...' (output: 'stale (uncommitted code changes)'), fix discarded with git stash, pass record committed; the fail reads superseded and nothing blocks. Before this branch a dirty manual pass went stale as soon as the fix was committed (or differed), so the fail came back; now it clears for good. The author noted this under Surprises, but the governing ADR and docs state the opposite, and a reader deciding whether the shortcut is acceptable is misled. Fix: say plainly that command capture refuses uncommitted code, that a manual pass is the recorder's judgment, records whatever the checkout holds, and clears a fail the same way; and that any later pass clears it whatever its command or pin (the 'simplicity wins' decision). Refusing dirty manual records would be a new refusal and goes to Sumeet, so keep it to wording unless he chooses otherwise.
