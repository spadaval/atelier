---
created_at: "2026-10-23T00:03:43.864522828+00:00"
id: "atelier-vn0d"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:42.260492367+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s7.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s7.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S7"
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
title: "S7 claim: an approval stays current when a follow-up is linked to its mission and across a close commit touching only the item's record; a planning commit adding an issue to a mission re-checks no done claim in it. Method: s7.sh, scratch repo: mission M with item A; on task/A, approve, then file a follow-up (discovered_from A) and link it advances M, commit; then close A in a commit touching only A's record. Observed: merge --check 'pass review.current' after the link and after the close, 'pass claims.reviewed'; land A passes; merge integrates. On main, with A done and a newer failing receipt for A already on the upstream, a commit planning a new issue into M lands ('land: passed'); control: a note on A's own record makes land refuse 'A evidence.no_newer_failure', showing the check would fire if A were re-checked. Limits: with one-sided links the planning commit no longer edits the mission record at all, so the scenario cannot separate 'scope change ignored' from 'mission record untouched'."
updated_at: "2026-10-23T00:03:43.865503315+00:00"
---

## Summary

S7 claim: an approval stays current when a follow-up is linked to its mission and across a close commit touching only the item's record; a planning commit adding an issue to a mission re-checks no done claim in it. Method: s7.sh, scratch repo: mission M with item A; on task/A, approve, then file a follow-up (discovered_from A) and link it advances M, commit; then close A in a commit touching only A's record. Observed: merge --check 'pass review.current' after the link and after the close, 'pass claims.reviewed'; land A passes; merge integrates. On main, with A done and a newer failing receipt for A already on the upstream, a commit planning a new issue into M lands ('land: passed'); control: a note on A's own record makes land refuse 'A evidence.no_newer_failure', showing the check would fire if A were re-checked. Limits: with one-sided links the planning commit no longer edits the mission record at all, so the scenario cannot separate 'scope change ignored' from 'mission record untouched'.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s7.sh
```

Exit status: 0

## Stdout

Bytes: 710
Truncated: no

```text
pass review.current: current independent approval recorded
ok: approval current after linking a follow-up to the mission
close commit touches:  M .atelier/issues/s7-uws0.md 
pass review.current: current independent approval recorded
pass claims.reviewed: every done claim merging adds is covered by a current accepted approval
ok: approval current across the close commit (merge)
land rc=0: land: passed
ok: land agrees
ok: merged
planning commits touch: .atelier/issues/s7-ou1y.md 
land on main rc=0: land: passed
ok: planning commit re-checks no done claim in the mission
control land rc=1: s7-uws0 evidence.no_newer_failure
ok: control shows land would catch s7-uws0's failing proof when its record changes
```

## Stderr

Bytes: 0
Truncated: no

```text
```

