---
schema: atelier.review
schema_version: 2
id: atelier-86l6
title: finding on atelier-mal9 @5744c0ea
target: atelier-mal9
identity: sol-mal9-r3
commit: 5744c0ea90f3d38e7c0a514d8b347ba2893ac480
created_at: 2026-11-10T15:34:29.431977875Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-records/src/document.rs:37 An inline backtick code span hides every subsequent finding. Reproduced on the pinned head with a review whose Summary contains the standalone line ```inline code``` followed by ## Finding 1: Still broken and Recommendation: fix now, with verdict: approve. CommonMark parses the line as a paragraph containing inline code (a backtick fence info string cannot contain backticks), but the opening branch ignores rest and starts an unterminated fence: findings() and open_findings() return empty lists and validate() returns no problems. Removing that line correctly reports the open finding; removing Recommendation while keeping the line is also wrongly accepted. Reject an opening backtick fence whose info string contains a backtick, without weakening the closing-fence rules, and cover both approval and missing-Recommendation cases.
