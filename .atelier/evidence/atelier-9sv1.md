---
created_at: "2026-10-23T00:03:15.615950332+00:00"
id: "atelier-9sv1"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:14.691825741+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s5.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s5.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S5"
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
title: "S5 claim: an owned branch does not trust local base commits no land has checked. Method: s5.sh, scratch repo with local bare origin: close item X with passing proof and push; commit a newer failing receipt for X on main without pushing; atelier start new item W from that main and run land W on task/W. Observed: land refuses with 'X evidence.no_newer_failure: newer failing evidence'; also refuses after --unset-upstream; control: after pushing main (the commit is upstream, therefore trusted), land W passes, so the refusal came from the unchecked local commit. Note: my first attempt (a done claim committed on main) was refused earlier by start itself with land_base_done_claim, which also blocks starting from such a base."
updated_at: "2026-10-23T00:03:15.617219877+00:00"
---

## Summary

S5 claim: an owned branch does not trust local base commits no land has checked. Method: s5.sh, scratch repo with local bare origin: close item X with passing proof and push; commit a newer failing receipt for X on main without pushing; atelier start new item W from that main and run land W on task/W. Observed: land refuses with 'X evidence.no_newer_failure: newer failing evidence'; also refuses after --unset-upstream; control: after pushing main (the commit is upstream, therefore trusted), land W passes, so the refusal came from the unchecked local commit. Note: my first attempt (a done claim committed on main) was refused earlier by start itself with land_base_done_claim, which also blocks starting from such a base.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s5.sh
```

Exit status: 0

## Stdout

Bytes: 506
Truncated: no

```text
on branch: task/s5-pyzw
--- land on owned branch: rc=1
s5-o1ir evidence.no_newer_failure: newer failing evidence s5-myb2
land: refused — 1 check (land_invariants_unmet)
ok: owned land refuses the unchecked base commit's failing proof
--- without an upstream: rc=1
ok: refuses without an upstream
--- control after main reaches the upstream: rc=0
Landing checks passed. No commits or records created.
land: passed
ok: upstream history is trusted, so the refusal above came from the unchecked local commit
```

## Stderr

Bytes: 0
Truncated: no

```text
```

