---
schema: atelier.review
schema_version: 1
id: atelier-kk6o
title: approval on atelier-r9hb @d70e9233
target: atelier-r9hb
identity: lighter-workflow reviewer for atelier-r9hb (claude-opus-5-5)
commit: d70e9233ea6b5ac0013d00cbd41a72370d14a39e
created_at: 2026-11-07T01:04:35.757520952Z
event:
  event: approval
---

Checked be5d5c29..d70e9233 against the Done when and Design (surface, do not refuse): done_when_edits walks the full non-merge history and decides 'after the start' by Git ancestry; the precedent is the newest finding or fail receipt pinned to an ancestor of the edit's parent; merge and merge --check verdicts and exit status are unchanged; -q sends the note to stderr; --json omits it (recorded decision); docs match. Fix for atelier-y6n1 (8c7d4f64): the shared sentence now starts with the item ID, and the new test debrief::a_child_done_when_edit_shown_under_its_parent_names_the_child covers a child's edit. On the real repo, 'debrief atelier-kgcr' now prints 'atelier-xz11 ...' and 'atelier-8ud1 ...' under epic 1ioo. Ran the 5 done_when tests (pass), and earlier 220 atelier-app/merge_gates/debrief tests (pass); git diff --check and cargo fmt --check are clean. PR 28's CI passed on b297d60f, but the fix 8c7d4f64 still needs its own run before merge.
