---
schema: atelier.review
schema_version: 1
id: atelier-asxi
title: 'crates/atelier-app/src/record_index.rs:993-997 — resolve_any returns the first kind that matches, so a short ID that names both an issue and an evidence record (possible only across project slugs; same full ID across kinds is refused at load as a duplicate) silently resolves to the issue, while the same collision within one kind is an ambiguity error. Reproduced: issue lr8o-try-w0e8 plus evidence other-w0e8, a bundle evidence entry with validates [{id: w0e8}] previews valid against the issue. Because validates accepts issues and evidence, the receipt could link the wrong record. Not a regression: before this change short refs resolved only issues. Suggested: report a cross-kind short match as ambiguous, or document the issue-first order.'
target: atelier-lr8o
identity: Independent reviewer (Claude subagent)
commit: 647d6b38852f63a1397ce2446734ab241bc1ef76
created_at: 2026-10-17T09:05:51.247098228Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_index.rs:993-997 — resolve_any returns the first kind that matches, so a short ID that names both an issue and an evidence record (possible only across project slugs; same full ID across kinds is refused at load as a duplicate) silently resolves to the issue, while the same collision within one kind is an ambiguity error. Reproduced: issue lr8o-try-w0e8 plus evidence other-w0e8, a bundle evidence entry with validates [{id: w0e8}] previews valid against the issue. Because validates accepts issues and evidence, the receipt could link the wrong record. Not a regression: before this change short refs resolved only issues. Suggested: report a cross-kind short match as ambiguous, or document the issue-first order.
