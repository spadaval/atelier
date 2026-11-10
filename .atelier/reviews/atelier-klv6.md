---
schema: atelier.review
schema_version: 2
id: atelier-klv6
title: finding on atelier-1sbw @f317ac78
target: atelier-1sbw
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:19.671154175Z
event:
  event: finding
  recommendation: fix_now
---

R2 .atelier/issues/atelier-1sbw.md:36: medium. Moving and splitting strategies without a link rule breaks the specifications that the new mission documents point to. In this repository redesign-2-core.md links to redesign-product.md and redesign-architecture.md relative to docs/strategies; copied to .atelier/missions/atelier-fcb7/design.md, those same links resolve to nonexistent files beside design.md. The open gardener and cutover strategies have the same problem with architecture anchors. References in generated item Context text to an old strategy path also become stale when that strategy is removed. Define rebasing for relative links in moved content and replacement of moved-strategy references within generated records, preserving anchors. Add a fixture where a moved strategy links to an unmoved document and another moved strategy, and verify that the generated links resolve. This needs no new command, record kind or compatibility layer.
