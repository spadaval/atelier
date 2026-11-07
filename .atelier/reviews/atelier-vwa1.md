---
schema: atelier.review
schema_version: 2
id: atelier-vwa1
title: 'crates/atelier-app/src/work_classification.rs:95-101: with several open needs-human children the note lists all of them but gives the commands only for children[0] and says ''make it a standalone validation''. Scenario: owner with H1 and H2 prints ''atelier-cn2x, atelier-pdr5 ... are each a child'' then ''atelier issue update atelier-cn2x --no-parent ...''; an agent pasting the commands detaches one, and the branch still cannot merge (start/prime then warn about the other, so it is recoverable). Suggest printing the command pair per child or saying ''for each of them''.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 863a7d908942ea4589f3fd104f98ab9f0eabca9c
created_at: 2026-10-16T19:45:03.072335732Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 5250d3697e223ace8b7fea7ee14e96c693f94bf2
    recorded_at: 2026-10-16T19:48:04.542992970Z
    resolution_commit: 45ccfad54324646a1525297033d0a55745c87734
    reason: 'human_children_note (work_classification.rs:94-111) now maps every child to its own ''atelier issue update <child> --no-parent, then atelier issue link <child> <owner> --role blocked_by'' pair, joined with ''; ''. Wording: one child gives ''X labelled needs-human is a child of P ... finishes it ... make it a standalone validation instead: <pair>.''; several give ''X, Y ... are each a child of P ... finishes them ... make them standalone validations instead: <pair X>; <pair Y>.'' Both read correctly. The single-child output still contains the strings the start and prime tests assert.'
    migrated_from: atelier-dujp
---

crates/atelier-app/src/work_classification.rs:95-101: with several open needs-human children the note lists all of them but gives the commands only for children[0] and says 'make it a standalone validation'. Scenario: owner with H1 and H2 prints 'atelier-cn2x, atelier-pdr5 ... are each a child' then 'atelier issue update atelier-cn2x --no-parent ...'; an agent pasting the commands detaches one, and the branch still cannot merge (start/prime then warn about the other, so it is recoverable). Suggest printing the command pair per child or saying 'for each of them'.
