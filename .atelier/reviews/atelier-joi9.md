---
schema: atelier.review
schema_version: 1
id: atelier-joi9
title: 'touched_claims touches only the direct neighbours of a changed issue: the issue itself, its parent, its blocks and its children. But blockers.transitive_none_open follows dependency_closure, which goes on through terminal blockers and also takes blockers inherited from the parent. A done claim whose closure gains an open issue more than one link away is skipped whenever no issue in between is checked and fails. Two cases, both reproduced with only the CLI on the base with an upstream (land passes, doctor reports ''blockers.transitive_none_open: completion blockers: <new>''): (1) inherited: done child X of open parent P; atelier issue link P W --role blocked_by with W open. Only W''s record changes, which touches W and P; P is not a claim, and X is skipped. (2) transitive through superseded: done X blocked_by superseded Y; atelier issue link Y W --role blocked_by. Y is touched, but superseded claims skip the blocker check, and X is skipped. Fix: run the blocker and work.terminal checks for every retained claim, since they read only the loaded records; or close touched over reverse dependency edges (blocked-by transitively, plus children of a touched parent) before skipping. Add both cases to the land test.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6115f1e9021c5c9d59228e425f3a5b1c5f767091
created_at: 2026-10-16T23:40:59.805685003Z
event:
  event: finding
  recommendation: fix_now
---

touched_claims touches only the direct neighbours of a changed issue: the issue itself, its parent, its blocks and its children. But blockers.transitive_none_open follows dependency_closure, which goes on through terminal blockers and also takes blockers inherited from the parent. A done claim whose closure gains an open issue more than one link away is skipped whenever no issue in between is checked and fails. Two cases, both reproduced with only the CLI on the base with an upstream (land passes, doctor reports 'blockers.transitive_none_open: completion blockers: <new>'): (1) inherited: done child X of open parent P; atelier issue link P W --role blocked_by with W open. Only W's record changes, which touches W and P; P is not a claim, and X is skipped. (2) transitive through superseded: done X blocked_by superseded Y; atelier issue link Y W --role blocked_by. Y is touched, but superseded claims skip the blocker check, and X is skipped. Fix: run the blocker and work.terminal checks for every retained claim, since they read only the loaded records; or close touched over reverse dependency edges (blocked-by transitively, plus children of a touched parent) before skipping. Add both cases to the land test.
