---
created_at: "2026-10-23T00:03:14.296460401+00:00"
id: "atelier-9hp8"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:13.527549490+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s4.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s4.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S4"
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
title: "S4 claim: linking new work to a shared mission or blocker leaves that record unchanged. Method: s4.sh, scratch repo; on two branches from the same base, create an issue and link it 'advances' to a shared mission and 'blocked_by' a shared blocker. Observed: git status shows only the new issue file changed; mission and blocker files are byte-identical (git hash-object); the two branches merge with no conflict; atelier check passes; issue show on the mission lists both advancing items and on the blocker lists both under Blocking (read from the one-sided links). Limits: none beyond scratch scale."
updated_at: "2026-10-23T00:03:14.297445847+00:00"
---

## Summary

S4 claim: linking new work to a shared mission or blocker leaves that record unchanged. Method: s4.sh, scratch repo; on two branches from the same base, create an issue and link it 'advances' to a shared mission and 'blocked_by' a shared blocker. Observed: git status shows only the new issue file changed; mission and blocker files are byte-identical (git hash-object); the two branches merge with no conflict; atelier check passes; issue show on the mission lists both advancing items and on the blocker lists both under Blocking (read from the one-sided links). Limits: none beyond scratch scale.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s4.sh
```

Exit status: 0

## Stdout

Bytes: 759
Truncated: no

```text
On branch main
Your branch is ahead of 'origin/main' by 1 commit.
  (use "git push" to publish your local commits)

nothing to commit, working tree clean
s4-i2je
s4-i2je
one changed:  M .atelier/issues/s4-i2je.md 
ok: mission and blocker records byte-identical on one
s4-j839
s4-j839
two changed:  M .atelier/issues/s4-j839.md 
ok: mission and blocker records byte-identical on two
ok: branches one and two merge without conflict
ok: atelier check passes after merge
Blocking
--------
(none)

Linked Issues
-------------
  advances s4-i2je todo medium work one
  advances s4-j839 todo medium work two

Blocking
--------
  s4-i2je todo medium work one
  s4-j839 todo medium work two

ok: mission shows both advancing items
ok: blocker shows both waiting items
```

## Stderr

Bytes: 0
Truncated: no

```text
```

