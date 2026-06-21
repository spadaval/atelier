---
created_at: "2026-06-21T20:12:47.216896516+00:00"
id: "atelier-vxxj"
evidence_type: "validation"
captured_at: "2026-06-21T20:12:46.475790498+00:00"
command: "sh -c 'set -eu\nATELIER=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d)\nprintf \"workspace=%s\\n\" \"$TMP\"\ncd \"$TMP\"\n\"$ATELIER\" init\n\"$ATELIER\" issue create \"Scenario mission\" --issue-type mission\nMID=$(ls .atelier/issues/*.md | xargs -n1 basename | sed \"s/\\.md$//\" | head -1)\nprintf \"mission_id=%s\\n\" \"$MID\"\n\"$ATELIER\" issue create \"Scenario work\"\nWID=$(grep -l \"title: \\\"Scenario work\\\"\" .atelier/issues/*.md | xargs -n1 basename | sed \"s/\\.md$//\")\nprintf \"work_id=%s\\n\" \"$WID\"\n\"$ATELIER\" issue link \"$MID\" \"$WID\" --role advances\n\"$ATELIER\" issue table --kind mission\n\"$ATELIER\" issue status \"$MID\"\nif \"$ATELIER\" mission status \"$MID\" >/tmp/old-mission.out 2>/tmp/old-mission.err; then\n  cat /tmp/old-mission.out\n  echo \"old mission command unexpectedly succeeded\" >&2\n  exit 1\nelse\n  printf \"old mission command rejected:\\n\"\n  cat /tmp/old-mission.err\nfi\n\"$ATELIER\" evidence record --target \"issue/$WID\" --kind validation \"scenario work evidence\"\n\"$ATELIER\" evidence list --target \"issue/$WID\"'"
exit_status: "2"
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
title: "Mission rework scenario validation transcript"
updated_at: "2026-06-21T20:12:52.173763566+00:00"
---

## Summary

Mission rework scenario validation transcript

## Command

```console
sh -c 'set -eu
ATELIER=/root/atelier/target/debug/atelier
TMP=$(mktemp -d)
printf "workspace=%s\n" "$TMP"
cd "$TMP"
"$ATELIER" init
"$ATELIER" issue create "Scenario mission" --issue-type mission
MID=$(ls .atelier/issues/*.md | xargs -n1 basename | sed "s/\.md$//" | head -1)
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
"$ATELIER" evidence list --target "issue/$WID"'
```

Exit status: 2

## Stdout

Bytes: 4090
Truncated: no

```text
workspace=/tmp/tmp.s5eZO6wTwC
Created /tmp/tmp.s5eZO6wTwC/.atelier
Created /tmp/tmp.s5eZO6wTwC/.atelier/config.toml
Created /tmp/tmp.s5eZO6wTwC/.atelier/runtime/state.db
Created /tmp/tmp.s5eZO6wTwC/.atelier/workflow.yaml
Atelier initialized successfully!

Next steps:
  atelier lint                     # Verify tracker records and workflow setup
  atelier issue create "Task"     # Create the first tracked issue
  atelier man admin                # Review setup and repair guidance
Created mission objective atelier-jx2e - Scenario mission
Type:     mission
Priority: medium
File:     /tmp/tmp.s5eZO6wTwC/.atelier/issues/atelier-jx2e.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.s5eZO6wTwC/.atelier/issues/atelier-jx2e.md
  Validate this issue: atelier lint atelier-jx2e
  Inspect this issue: atelier issue show atelier-jx2e
  Inspect tracked work transitions: atelier issue transition atelier-jx2e --options
mission_id=atelier-jx2e
Created issue atelier-ftfj - Scenario work
Type:     task
Priority: medium
File:     /tmp/tmp.s5eZO6wTwC/.atelier/issues/atelier-ftfj.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.s5eZO6wTwC/.atelier/issues/atelier-ftfj.md
  Validate this issue: atelier lint atelier-ftfj
  Inspect this issue: atelier issue show atelier-ftfj
  Inspect tracked work transitions: atelier issue transition atelier-ftfj --options
work_id=atelier-ftfj
Linked atelier-jx2e -> atelier-ftfj (advances)
Next Commands
-------------
  atelier issue show atelier-jx2e
  atelier issue status atelier-jx2e
  atelier issue show atelier-ftfj
Issue Table: mission
====================
ID           Status       Health     Ready  Blocked  Done  Backlog  Title
atelier-jx2e ready        ready          1        0     0       0  Scenario mission

Next Commands
-------------
  Inspect one objective: atelier issue status <id>
  Open one objective record: atelier issue show <id>
  Browse grouped work: atelier issue list
Mission Status atelier-jx2e [ready] - Scenario mission
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
  ready atelier-ftfj - Scenario work | no open blockers; mission-linked root; proof checked by workflow validators

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
  atelier issue status atelier-jx2e --verbose
  atelier lint

Terminal Checks
---------------
Work: open - atelier-ftfj
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
  Inspect mission record (durable intent and linked work): atelier issue show atelier-jx2e
  Refresh mission status (current blockers and terminal checks): atelier issue status atelier-jx2e
  Inspect terminal check detail: atelier issue status atelier-jx2e --verbose
  Inspect selectable mission work transitions (1 selectable issue(s)): atelier issue transition atelier-ftfj --options
old mission command rejected:
error: unrecognized subcommand 'mission'

Usage: atelier [OPTIONS] <COMMAND>

For more information, try '--help'.
Attached evidence atelier-2add to issue atelier-ftfj (validates)
atelier-2add [evidence] recorded - scenario work evidence
=========================================================
Status:      recorded
Kind:        validation
Captured:    2026-06-21T20:12:47.043127945+00:00
Target:      issue/atelier-ftfj (validates)
Producer:    (none)
Path:        (none)
URI:         (none)
Created:     2026-06-21T20:12:47.043139930+00:00
Updated:     2026-06-21T20:12:47.131804148+00:00
Summary
-------
scenario work evidence
```

## Stderr

Bytes: 121
Truncated: no

```text
error: unexpected argument '--target' found

Usage: atelier evidence list [OPTIONS]

For more information, try '--help'.
```

