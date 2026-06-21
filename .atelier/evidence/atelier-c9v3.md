---
created_at: "2026-06-21T20:13:17.086264362+00:00"
id: "atelier-c9v3"
evidence_type: "validation"
captured_at: "2026-06-21T20:13:16.288569186+00:00"
command: "sh -c 'set -eu\nATELIER=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d)\nprintf \"workspace=%s\\n\" \"$TMP\"\ncd \"$TMP\"\n\"$ATELIER\" init\n\"$ATELIER\" issue create \"Scenario mission\" --issue-type mission\nMID=$(grep -l \"issue_type: \\\"mission\\\"\" .atelier/issues/*.md | xargs -n1 basename | sed \"s/\\.md$//\")\nprintf \"mission_id=%s\\n\" \"$MID\"\n\"$ATELIER\" issue create \"Scenario work\"\nWID=$(grep -l \"title: \\\"Scenario work\\\"\" .atelier/issues/*.md | xargs -n1 basename | sed \"s/\\.md$//\")\nprintf \"work_id=%s\\n\" \"$WID\"\n\"$ATELIER\" issue link \"$MID\" \"$WID\" --role advances\n\"$ATELIER\" issue table --kind mission\n\"$ATELIER\" issue status \"$MID\"\nif \"$ATELIER\" mission status \"$MID\" >/tmp/old-mission.out 2>/tmp/old-mission.err; then\n  cat /tmp/old-mission.out\n  echo \"old mission command unexpectedly succeeded\" >&2\n  exit 1\nelse\n  printf \"old mission command rejected:\\n\"\n  cat /tmp/old-mission.err\nfi\n\"$ATELIER\" evidence record --target \"issue/$WID\" --kind validation \"scenario work evidence\"\nEID=$(grep -l \"scenario work evidence\" .atelier/evidence/*.md | xargs -n1 basename | sed \"s/\\.md$//\")\n\"$ATELIER\" evidence show \"$EID\"\n\"$ATELIER\" lint'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-76j0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-76j0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Passing mission rework scenario validation transcript"
updated_at: "2026-06-21T20:13:21.952931010+00:00"
---

## Summary

Passing mission rework scenario validation transcript

## Command

```console
sh -c 'set -eu
ATELIER=/root/atelier/target/debug/atelier
TMP=$(mktemp -d)
printf "workspace=%s\n" "$TMP"
cd "$TMP"
"$ATELIER" init
"$ATELIER" issue create "Scenario mission" --issue-type mission
MID=$(grep -l "issue_type: \"mission\"" .atelier/issues/*.md | xargs -n1 basename | sed "s/\.md$//")
printf "mission_id=%s\n" "$MID"
"$ATELIER" issue create "Scenario work"
WID=$(grep -l "title: \"Scenario work\"" .atelier/issues/*.md | xargs -n1 basename | sed "s/\.md$//")
printf "work_id=%s\n" "$WID"
"$ATELIER" issue link "$MID" "$WID" --role advances
"$ATELIER" issue table --kind mission
"$ATELIER" issue status "$MID"
if "$ATELIER" mission status "$MID" >/tmp/old-mission.out 2>/tmp/old-mission.err; then
  cat /tmp/old-mission.out
  echo "old mission command unexpectedly succeeded" >&2
  exit 1
else
  printf "old mission command rejected:\n"
  cat /tmp/old-mission.err
fi
"$ATELIER" evidence record --target "issue/$WID" --kind validation "scenario work evidence"
EID=$(grep -l "scenario work evidence" .atelier/evidence/*.md | xargs -n1 basename | sed "s/\.md$//")
"$ATELIER" evidence show "$EID"
"$ATELIER" lint'
```

Exit status: 0

## Stdout

Bytes: 4555
Truncated: yes

```text
workspace=/tmp/tmp.z6TmGRD3cR
Created /tmp/tmp.z6TmGRD3cR/.atelier
Created /tmp/tmp.z6TmGRD3cR/.atelier/config.toml
Created /tmp/tmp.z6TmGRD3cR/.atelier/runtime/state.db
Created /tmp/tmp.z6TmGRD3cR/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created mission objective atelier-xn6e - Scenario mission
Type:     mission
Priority: medium
File:     /tmp/tmp.z6TmGRD3cR/.atelier/issues/atelier-xn6e.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.z6TmGRD3cR/.atelier/issues/atelier-xn6e.md
  Validate this issue: atelier lint atelier-xn6e
  Inspect this issue: atelier issue show atelier-xn6e
  Inspect tracked work transitions: atelier issue transition atelier-xn6e --options
mission_id=atelier-xn6e
Created issue atelier-0gyy - Scenario work
Type:     task
Priority: medium
File:     /tmp/tmp.z6TmGRD3cR/.atelier/issues/atelier-0gyy.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.z6TmGRD3cR/.atelier/issues/atelier-0gyy.md
  Validate this issue: atelier lint atelier-0gyy
  Inspect this issue: atelier issue show atelier-0gyy
  Inspect tracked work transitions: atelier issue transition atelier-0gyy --options
work_id=atelier-0gyy
Linked atelier-xn6e -> atelier-0gyy (advances)
Next Commands
-------------
  atelier issue show atelier-xn6e
  atelier issue status atelier-xn6e
  atelier issue show atelier-0gyy
Issue Table: mission
====================
ID           Status       Health     Ready  Blocked  Done  Backlog  Title
atelier-xn6e ready        ready          1        0     0       0  Scenario mission

Next Commands
-------------
  Inspect one objective: atelier issue status <id>
  Open one objective record: atelier issue show <id>
  Browse grouped work: atelier issue list
Mission Status atelier-xn6e [ready] - Scenario mission
======================================================
Health:   ready
Tracker:  ok
Terminal: blocked

Work
----
Total: 1 ready
Epics: none
Other: 1 ready

Selectable Work
---------------
  ready atelier-0gyy - Scenario work | no open blockers; mission-linked root; proof checked by workflow validators

Blocked Work
------------
(none)

Blockers
--------
(none)

Evidence
--------
Direct mission evidence: none

Reliability
-----------
Projection Freshness: current
Malformed Work: none
Missing Outcome Sections: none
Graph Hygiene: clear
Open Blockers: none
Drill-downs:
  atelier issue status atelier-xn6e --verbose
  atelier lint

Terminal Checks
---------------
Work: open - atelier-0gyy
  Next: atelier issue transition <issue-id> close --reason "..."
Blockers: clear
Tracker State: current
Linked Issue Records: parseable

Branch Policy
-------------
Current branch: (detached)
Base branch:    main
Owner branches: none
Dirty state: clean
Branch mismatches: none

Active Work
-----------
(none)

Next Commands
-------------
  Inspect mission record (durable intent and linked work): atelier issue show atelier-xn6e
  Refresh mission status (current blockers and terminal checks): atelier issue status atelier-xn6e
  Inspect terminal check detail: atelier issue status atelier-xn6e --verbose
  Inspect selectable mission work transitions (1 selectable issue(s)): atelier issue transition atelier-0gyy --options
old mission command rejected:
error: unrecognized subcommand 'mission'

Usage: atelier [OPTIONS] <COMMAND>

For more information, try '--help'.
Attached evidence atelier-61m0 to issue atelier-0gyy (validates)
atelier-61m0 [evidence] recorded - scenario work evidence
=========================================================
Status:      recorded
Kind:        validation
Captured:    2026-06-21T20:13:16.879258394+00:00
Target:      issue/atelier-0gyy (validates)
Producer:    (none)
Path:        (none)
URI:         (none)
Created:     2026-06-21T20:13:16.879265485+00:00
Updated:     2026-06-21T20:13:16.969592740+00:00
Summary
-------
scenario work evidence
atelie
```

## Stderr

Bytes: 0
Truncated: no

```text
```

