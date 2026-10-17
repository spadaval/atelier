---
schema: atelier.review
schema_version: 1
id: atelier-yx83
title: 'crates/atelier-app/src/record_history.rs:62 and :198: versions are ordered by committer date across parallel branches, and merge commits list no files under --name-only (diff-merges off), so a merged result never becomes a version. Reproduced in a scratch repo: item started on main; closed on a feature branch; an unrelated edit to the item record (labels) committed later on main; feature merged with --no-ff. The item is done on HEAD, yet the debrief reports ''reopened <item>''. The same mechanism makes ''closed'' None for a mission closed on a branch while main touched its record, and then proof age runs to HEAD. This repository''s workflow (feature branches merged into master while master gets record commits, such as mission links) hits this. Fix: read the mainline history with --first-parent and --diff-merges=first-parent (or -m), so each merge adds the record as it reached the base, and judge reopened/closed along that line. Add fixtures for a branch close merged around a concurrent base edit.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:15.522908700Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/record_history.rs:62 and :198: versions are ordered by committer date across parallel branches, and merge commits list no files under --name-only (diff-merges off), so a merged result never becomes a version. Reproduced in a scratch repo: item started on main; closed on a feature branch; an unrelated edit to the item record (labels) committed later on main; feature merged with --no-ff. The item is done on HEAD, yet the debrief reports 'reopened <item>'. The same mechanism makes 'closed' None for a mission closed on a branch while main touched its record, and then proof age runs to HEAD. This repository's workflow (feature branches merged into master while master gets record commits, such as mission links) hits this. Fix: read the mainline history with --first-parent and --diff-merges=first-parent (or -m), so each merge adds the record as it reached the base, and judge reopened/closed along that line. Add fixtures for a branch close merged around a concurrent base edit.
