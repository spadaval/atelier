---
created_at: "2026-11-09T15:34:19.472350415+00:00"
id: "atelier-92gg"
evidence_type: "validation"
commit: "e9b97752c7c263d9bcc2116a6d6947e4b412c89e"
result: "pass"
captured_at: "2026-11-09T15:34:19.472302986+00:00"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-t7pb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t7pb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Same output from a build of master f0f43a7a and the new build, compared with diff -r over saved stdout+stderr+exit status: atelier debrief <m> and debrief <m> -q on all 22 missions from atelier work missions --all -q (44 outputs, 13 Done when edit lines among them); atelier merge --check and atelier review request --identity reviewer on the 3 open branches (atelier-2js4-probe, atelier-7o4r, atelier-xc5y; 6 outputs, no edits on them) and on 3 merged branches recreated at their pre-merge tips in a scratch clone (feature/atelier-hwxm, bug/atelier-mgl0, task/atelier-vy5i; 6 outputs with 4 distinct edits, one after finding atelier-ilt4). 56 outputs, all identical. Before the Rust change, a Python check found one walk bucketed by path equal to each id's own git log --reverse --full-history --format='%H %P' for all 516 records over 517 merges. scripts/smoke-repos.sh: no regression across 5 repositories."
updated_at: "2026-11-09T15:34:19.601682645+00:00"
---

Same output from a build of master f0f43a7a and the new build, compared with diff -r over saved stdout+stderr+exit status: atelier debrief <m> and debrief <m> -q on all 22 missions from atelier work missions --all -q (44 outputs, 13 Done when edit lines among them); atelier merge --check and atelier review request --identity reviewer on the 3 open branches (atelier-2js4-probe, atelier-7o4r, atelier-xc5y; 6 outputs, no edits on them) and on 3 merged branches recreated at their pre-merge tips in a scratch clone (feature/atelier-hwxm, bug/atelier-mgl0, task/atelier-vy5i; 6 outputs with 4 distinct edits, one after finding atelier-ilt4). 56 outputs, all identical. Before the Rust change, a Python check found one walk bucketed by path equal to each id's own git log --reverse --full-history --format='%H %P' for all 516 records over 517 merges. scripts/smoke-repos.sh: no regression across 5 repositories.
