---
schema: atelier.review
schema_version: 1
id: atelier-vwa1
title: 'crates/atelier-app/src/work_classification.rs:95-101: with several open needs-human children the note lists all of them but gives the commands only for children[0] and says ''make it a standalone validation''. Scenario: owner with H1 and H2 prints ''atelier-cn2x, atelier-pdr5 ... are each a child'' then ''atelier issue update atelier-cn2x --no-parent ...''; an agent pasting the commands detaches one, and the branch still cannot merge (start/prime then warn about the other, so it is recoverable). Suggest printing the command pair per child or saying ''for each of them''.'
target: atelier-1jll
identity: Independent reviewer (Claude subagent)
commit: 863a7d908942ea4589f3fd104f98ab9f0eabca9c
created_at: 2026-10-16T19:45:03.072335732Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/work_classification.rs:95-101: with several open needs-human children the note lists all of them but gives the commands only for children[0] and says 'make it a standalone validation'. Scenario: owner with H1 and H2 prints 'atelier-cn2x, atelier-pdr5 ... are each a child' then 'atelier issue update atelier-cn2x --no-parent ...'; an agent pasting the commands detaches one, and the branch still cannot merge (start/prime then warn about the other, so it is recoverable). Suggest printing the command pair per child or saying 'for each of them'.
