---
schema: atelier.review
schema_version: 1
id: atelier-eoxo
title: 'crates/atelier-cli/src/commands/bundle.rs:125-127 — an update entry''s id still uses its own wording: unknown ''zzzz'' gives ''Update u names missing issue zzzz'' (no near matches or Next step), and an evidence ID gives ''names missing issue p3by'' rather than the wrong-kind message. Pre-existing and outside this item''s evidence-reference scope, but it is a bundle ID that does not fail like a command argument, which the new schema.md sentence could be read to cover. Suggested: resolve it with index.resolve(RecordKind::Issue, id) in a follow-up.'
target: atelier-lr8o
identity: Independent reviewer (Claude subagent)
commit: 647d6b38852f63a1397ce2446734ab241bc1ef76
created_at: 2026-10-17T09:05:51.780892386Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/bundle.rs:125-127 — an update entry's id still uses its own wording: unknown 'zzzz' gives 'Update u names missing issue zzzz' (no near matches or Next step), and an evidence ID gives 'names missing issue p3by' rather than the wrong-kind message. Pre-existing and outside this item's evidence-reference scope, but it is a bundle ID that does not fail like a command argument, which the new schema.md sentence could be read to cover. Suggested: resolve it with index.resolve(RecordKind::Issue, id) in a follow-up.
