---
schema: atelier.review
schema_version: 1
id: atelier-cl47
title: approval on atelier-2bjz @3783831b
target: atelier-2bjz
identity: claude-review-2bjz
commit: 3783831beff2ea98f04fe81f62b7664d3f53a7b6
created_at: 2026-11-05T00:32:08.884450815Z
event:
  event: approval
---

Re-review of 6ea3eb5f..3783831b after the earlier approval atelier-7xtc. Fix 081b9acc (atelier-nu4k): misspelled_label now requires the label's first letter and one insertion, deletion, substitution or adjacent transposition, ignoring case; checked one_edit_apart's prefix-strip cases by hand (Reasons, Raeson, Rejectd, reason, REJECTED refused; Person, Region, Lesson, Season, Treason, Selected, Detected, Rejects pass, each in the unit test). The atelier-juyb record's new decision note and Close Reason match the code. Sync merge 4708c70d: its tree (5e5be685) is identical to git merge-tree of its parents 081b9acc and 9bc7576c, so it is a clean Git merge bringing in atelier-cej5's path_log test split; dispositions l6x3 (kt8y no_action) and y397/jsg3 (nu4k fix_now, resolved) are sound. Ran at head: git diff --check, cargo fmt --check, cargo metadata --locked, atelier check (pass), nextest atelier-core 12/12, atelier-app 107/107, atelier-cli decision|debrief|brief|status|note 99/99. CI run 41939393696 at 3783831b: ubuntu-latest and macos-latest both passed.
