---
created_at: "2026-06-12T21:59:28.308443735+00:00"
id: "atelier-fg6q"
evidence_type: "validation"
captured_at: "2026-06-12T21:59:26.735691738+00:00"
command: "bash -lc 'set -euo pipefail\nbin=/root/atelier/target/debug/atelier\ntmp=$(mktemp -d)\ncd \"$tmp\"\ngit init -q\n\"$bin\" init\n\"$bin\" mission create \"Relationship proof\" --body \"Typed relationship intent\" --constraint \"Use explicit relationship buckets\" --validation \"Show filtered mission output\"\nmission=$(basename .atelier/missions/*.md .md)\n\"$bin\" issue create \"Counted work\" --issue-type task\nwork=$(basename .atelier/issues/*.md .md)\n\"$bin\" issue create \"Validation work\" --issue-type validation\nvalidation=$(ls .atelier/issues/*.md | xargs -n1 basename | sed \"s/.md$//\" | grep -v \"^$work$\" | head -1)\n\"$bin\" issue create \"Supporting reference\" --issue-type task\nsupport=$(ls .atelier/issues/*.md | xargs -n1 basename | sed \"s/.md$//\" | grep -v \"^$work$\" | grep -v \"^$validation$\" | head -1)\n\"$bin\" issue create \"Direct blocker\" --issue-type task\nblocker=$(ls .atelier/issues/*.md | xargs -n1 basename | sed \"s/.md$//\" | grep -v \"^$work$\" | grep -v \"^$validation$\" | grep -v \"^$support$\" | head -1)\n\"$bin\" mission add-work \"$mission\" \"$work\"\n\"$bin\" mission add-work \"$mission\" \"$validation\"\n\"$bin\" mission add-blocker \"$mission\" \"$blocker\"\n\"$bin\" evidence add --kind validation --result pass \"relationship evidence proof\"\nevidence=$(basename .atelier/evidence/*.md .md)\n\"$bin\" evidence attach \"$evidence\" mission \"$mission\"\nmission_file=\".atelier/missions/$mission.md\"\nawk -v id=\"$support\" '\"'\"'{ if ($0 == \"schema: \\\"atelier.mission\\\"\") { print \"  - kind: \\\"issue\\\"\"; print \"    id: \\\"\" id \"\\\"\"; print \"    type: \\\"related\\\"\" } print }'\"'\"' \"$mission_file\" > \"$mission_file.tmp\"\nmv \"$mission_file.tmp\" \"$mission_file\"\n\"$bin\" rebuild\n\"$bin\" export --check\n\"$bin\" lint\ngrep -q \"type: \\\"advances\\\"\" \"$mission_file\"\ngrep -q \"type: \\\"blocked_by\\\"\" \"$mission_file\"\ngrep -q \"type: \\\"related\\\"\" \"$mission_file\"\ngrep -q \"role: \\\"validates\\\"\" \".atelier/evidence/$evidence.md\"\n\"$bin\" mission show \"$mission\" | tee show.out\ngrep -q \"Records: plans=0 milestones=0 evidence=1\" show.out\ngrep -q \"Work: ready=2 blocked=0 done=0 backlog=0\" show.out\ngrep -q \"Mission Blockers: 1\" show.out\ngrep -q \"Supporting Records\" show.out\ngrep -q \"Supporting reference (related)\" show.out\nlinked_work=$(sed -n \"/Linked Work/,/Supporting Records/p\" show.out)\nif printf \"%s\" \"$linked_work\" | grep -q \"Supporting reference\"; then\n  echo \"supporting reference leaked into linked work\"\n  exit 1\nfi\n\"$bin\" mission status \"$mission\" | tee status.out\ngrep -q \"Total: 2 ready\" status.out\ngrep -q \"Mission blockers: 1 open\" status.out\n'"
exit_status: "0"
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7r55"
    role: "validates"
  - kind: "issue"
    id: "atelier-8ec6"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Mission relationship semantics CLI transcript: advances blocked_by supporting records and evidence validates"
updated_at: "2026-06-12T21:59:58.823475335+00:00"
---

Mission relationship semantics CLI transcript: advances blocked_by supporting records and evidence validates

Command: bash -lc 'set -euo pipefail
bin=/root/atelier/target/debug/atelier
tmp=$(mktemp -d)
cd "$tmp"
git init -q
"$bin" init
"$bin" mission create "Relationship proof" --body "Typed relationship intent" --constraint "Use explicit relationship buckets" --validation "Show filtered mission output"
mission=$(basename .atelier/missions/*.md .md)
"$bin" issue create "Counted work" --issue-type task
work=$(basename .atelier/issues/*.md .md)
"$bin" issue create "Validation work" --issue-type validation
validation=$(ls .atelier/issues/*.md | xargs -n1 basename | sed "s/.md$//" | grep -v "^$work$" | head -1)
"$bin" issue create "Supporting reference" --issue-type task
support=$(ls .atelier/issues/*.md | xargs -n1 basename | sed "s/.md$//" | grep -v "^$work$" | grep -v "^$validation$" | head -1)
"$bin" issue create "Direct blocker" --issue-type task
blocker=$(ls .atelier/issues/*.md | xargs -n1 basename | sed "s/.md$//" | grep -v "^$work$" | grep -v "^$validation$" | grep -v "^$support$" | head -1)
"$bin" mission add-work "$mission" "$work"
"$bin" mission add-work "$mission" "$validation"
"$bin" mission add-blocker "$mission" "$blocker"
"$bin" evidence add --kind validation --result pass "relationship evidence proof"
evidence=$(basename .atelier/evidence/*.md .md)
"$bin" evidence attach "$evidence" mission "$mission"
mission_file=".atelier/missions/$mission.md"
awk -v id="$support" '"'"'{ if ($0 == "schema: \"atelier.mission\"") { print "  - kind: \"issue\""; print "    id: \"" id "\""; print "    type: \"related\"" } print }'"'"' "$mission_file" > "$mission_file.tmp"
mv "$mission_file.tmp" "$mission_file"
"$bin" rebuild
"$bin" export --check
"$bin" lint
grep -q "type: \"advances\"" "$mission_file"
grep -q "type: \"blocked_by\"" "$mission_file"
grep -q "type: \"related\"" "$mission_file"
grep -q "role: \"validates\"" ".atelier/evidence/$evidence.md"
"$bin" mission show "$mission" | tee show.out
grep -q "Records: plans=0 milestones=0 evidence=1" show.out
grep -q "Work: ready=2 blocked=0 done=0 backlog=0" show.out
grep -q "Mission Blockers: 1" show.out
grep -q "Supporting Records" show.out
grep -q "Supporting reference (related)" show.out
linked_work=$(sed -n "/Linked Work/,/Supporting Records/p" show.out)
if printf "%s" "$linked_work" | grep -q "Supporting reference"; then
  echo "supporting reference leaked into linked work"
  exit 1
fi
"$bin" mission status "$mission" | tee status.out
grep -q "Total: 2 ready" status.out
grep -q "Mission blockers: 1 open" status.out
'
Exit status: 0

Stdout summary (truncated):
Created /tmp/tmp.nTkH966FF9/.atelier
Created /tmp/tmp.nTkH966FF9/.atelier/config.toml
Created /tmp/tmp.nTkH966FF9/.atelier/state.db
Atelier initialized successfully!

Next steps:
  atelier issue create "Task"     # Create an issue
  atelier start <issue-id>         # Start tracked work
Mission atelier-1ayl: Relationship proof
Status: ready

## Intent

Typed relationship intent

## Constraints

- Use explicit relationship buckets

## Risks

- None.

## Validation

- Show filtered mission output
Created issue atelier-8fjj - Counted work
Type:     task
Priority: medium
File:     /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-8fjj.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-8fjj.md
  Validate this issue: atelier lint atelier-8fjj
  Inspect this issue: atelier issue show atelier-8fjj
  Start tracked work: atelier start atelier-8fjj
Created issue atelier-5ubw - Validation work
Type:     validation
Priority: medium
File:     /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-5ubw.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-5ubw.md
  Validate this issue: atelier lint atelier-5ubw
  Inspect this issue: atelier issue show atelier-5ubw
  Start tracked work: atelier start atelier-5ubw
Created issue atelier-8c6q - Supporting reference
Type:     task
Priority: medium
File:     /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-8c6q.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-8c6q.md
  Validate this issue: atelier lint atelier-8c6q
  Inspect this issue: atelier issue show atelier-8c6q
  Start tracked work: atelier start atelier-8c6q
Created issue atelier-gr75 - Direct blocker
Type:     task
Priority: medium
File:     /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-gr75.md

Next Commands
-------------
  Edit issue Markdown: /tmp/tmp.nTkH966FF9/.atelier/issues/atelier-gr75.md
  Validate this issue: atelier lint atelier-gr75
  Inspect this issue: atelier issue show atelier-gr75
  Start tracked work: atelier start atelier-gr75
Added work atelier-8fjj to mission atelier-1ayl
Added work atelier-5ubw to mission atelier-1ayl
Added blocker atelier-gr75 to mission atelier-1ayl
atelier-3fg3 [evidence] pass - relationship evidence proof
==========================================================
Result:      pass
Kind:        validation
Captured:    2026-06-12T21:59:27.937424834+00:00
Producer:    (none)
Path:        (none)
URI:         (none)
Created:     2026-06-12T21:59:27.937467020+00:00
Updated:     2026-06-12T21:59:27.937467020+00:00
Summary
-------
relationship evidence proof
Attached evidence atelier-3fg3 to mission atelier-1ayl (validates)
Runtime state rebuilt
State:    /tmp/tmp.nTkH966FF9/.atelier
Database: /tmp/tmp.nTkH966FF9/.atelier/state.db

Next Commands
-------------
  atelier doctor
  atelier export --check
Canonical export is current
State: /tmp/tmp.nTkH966FF9/.atelier
Lint passed.
Mission atelier-1ayl [ready] - Relationship proof
=================================================
Status:   ready
Created:  2026-06-12 17:59 -04:00
Updated:  2026-06-12 17:59 -04:00

Intent
------
Typed relationship intent

Constraints
-----------
- Use explicit relationship buckets

Risks
-----
- None.

Validation
----------
- Show filtered mission output

Progress
--------
Records: plans=0 milestones=0 evidence=1
Work: ready=2 blocked=0 done=0 backlog=0
Mission Blockers: 1

Plans
-----
(none)

Milestones
----------
(none)

Evidence
--------
  atelier-3fg3 [pass] - relationship evidence proof

Mission Blockers
----------------
  atelier-gr75 [open] medium task - Direct blocker (blocked by) (open blocker)

Linked Work
-----------
Ready (2)
  atelier-5ubw [open] medium validation - Validation work (advances)
  atelier-8fjj [open] medium task - Counted work (advances)

Supporting Records
------------------
  atelier-8c6q [open] - Supporting reference (related)

Evidence Gaps
-------------
(none)

Next Commands
-------------
  atelier mission status atelier-1ayl
  atelier mission show atelier-1ayl
  atelier miss

Stderr summary:
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Refreshed projection in /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier
Rebuilt /tmp/tmp.nTkH966FF9/.atelier/state.db from /tmp/tmp.nTkH966FF9/.atelier

