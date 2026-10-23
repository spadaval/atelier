---
created_at: "2026-10-23T00:03:45.750935476+00:00"
id: "atelier-n2ql"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:44.642695408+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s9.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s9.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S9"
  result: "pass"
criteria_mission: "atelier-k8h8"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k8h8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S9 claim: a step's records reach the base in one commit, and a trivial change lands on the base with a visible exemption, with no issue or work branch. Method: s9.sh, scratch repo: on a work branch, evidence record --stage-records, issue transition close --stage-records, review approve --commit-records; then on main a prose typo commit, land, review trivial --base <commit> --commit-records, land, review list, a later commit, land. Observed: the three records (evidence, issue, review) land in exactly one commit with a clean tree, and that branch merges; land refuses the unexempted base commit with land_base_unreviewed, passes after the exemption, review list shows 'TrivialExemption base main <commit> dev ...', no branch other than main exists, and a later commit is refused again. Limits: issue create, issue note, issue link and start have no --commit-records, so steps that include them still need a hand commit (outside atelier-4ixu's listed commands)."
updated_at: "2026-10-23T00:03:45.751841474+00:00"
---

## Summary

S9 claim: a step's records reach the base in one commit, and a trivial change lands on the base with a visible exemption, with no issue or work branch. Method: s9.sh, scratch repo: on a work branch, evidence record --stage-records, issue transition close --stage-records, review approve --commit-records; then on main a prose typo commit, land, review trivial --base <commit> --commit-records, land, review list, a later commit, land. Observed: the three records (evidence, issue, review) land in exactly one commit with a clean tree, and that branch merges; land refuses the unexempted base commit with land_base_unreviewed, passes after the exemption, review list shows 'TrivialExemption base main <commit> dev ...', no branch other than main exists, and a later commit is refused again. Limits: issue create, issue note, issue link and start have no --commit-records, so steps that include them still need a hand commit (outside atelier-4ixu's listed commands).

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s9.sh
```

Exit status: 0

## Stdout

Bytes: 739
Truncated: no

```text
step commits: 1; files: .atelier/evidence/s9-e1jf.md .atelier/issues/s9-hgo4.md .atelier/reviews/s9-j9ue.md 
ok: evidence, close and approval records in one commit, nothing left uncommitted
ok: merged (Merged task/s9-hgo4 into main at 7f3a03f0; deleted local wor)
land before exemption rc=1: land_base_unreviewed
It covers this commit alone. Commit this record on main.
Committed 1 record as 76865920.
land after exemption rc=0: land: passed
ok: exempted base commit lands
review list:
s9-xos5 TrivialExemption base main 7387457f dev heading typo; correctness evident from the diff
ok: exemption visible in review list
ok: no work branch was created
later commit land rc=1: land_base_unreviewed
ok: exemption does not cover a later commit
```

## Stderr

Bytes: 0
Truncated: no

```text
```

