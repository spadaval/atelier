---
schema: atelier.review
schema_version: 1
id: atelier-dujp
title: 'human_children_note (work_classification.rs:94-111) now maps every child to its own ''atelier issue update <child> --no-parent, then atelier issue link <child> <owner> --role blocked_by'' pair, joined with ''; ''. Wording: one child gives ''X labelled needs-human is a child of P ... finishes it ... make it a standalone validation instead: <pair>.''; several give ''X, Y ... are each a child of P ... finishes them ... make them standalone validations instead: <pair X>; <pair Y>.'' Both read correctly. The single-child output still contains the strings the start and prime tests assert.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 5250d3697e223ace8b7fea7ee14e96c693f94bf2
created_at: 2026-10-16T19:48:04.542992970Z
event:
  event: disposition
  finding: atelier-vwa1
  decision: fix_now
  supersedes: null
  resolution_commit: 45ccfad54324646a1525297033d0a55745c87734
  follow_up: null
---

human_children_note (work_classification.rs:94-111) now maps every child to its own 'atelier issue update <child> --no-parent, then atelier issue link <child> <owner> --role blocked_by' pair, joined with '; '. Wording: one child gives 'X labelled needs-human is a child of P ... finishes it ... make it a standalone validation instead: <pair>.'; several give 'X, Y ... are each a child of P ... finishes them ... make them standalone validations instead: <pair X>; <pair Y>.' Both read correctly. The single-child output still contains the strings the start and prime tests assert.
