---
schema: atelier.review
schema_version: 1
id: atelier-g0oy
title: 'crates/atelier-cli/src/commands/mission_review.rs:646-656: the record label is bounded by words, not characters, while issue labels are capped at 60 characters (line 637). Finding summaries usually open with paths, so the atelier-r6bt page shows labels such as ''“crates/atelier-app/src/record_history.rs:172 with crates/atelier-app/src/mission_review.rs:116: scope added after…”'' (about 110 characters) inline in a Work sentence, and a URL or long path could run longer. Fix: also cap the label at 60 characters, as issue_ref labels are.'
target: atelier-nexd
identity: Independent reviewer 3 (Claude subagent)
commit: 1f8465d99fbeaa96241d5b0f1031bf6259a43aa9
created_at: 2026-10-17T22:12:50.963112049Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/mission_review.rs:646-656: the record label is bounded by words, not characters, while issue labels are capped at 60 characters (line 637). Finding summaries usually open with paths, so the atelier-r6bt page shows labels such as '“crates/atelier-app/src/record_history.rs:172 with crates/atelier-app/src/mission_review.rs:116: scope added after…”' (about 110 characters) inline in a Work sentence, and a URL or long path could run longer. Fix: also cap the label at 60 characters, as issue_ref labels are.
