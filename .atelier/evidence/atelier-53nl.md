---
created_at: "2026-06-12T21:58:55.522451132+00:00"
id: "atelier-53nl"
data: "{\"captured_at\":\"2026-06-12T21:58:54.781615132+00:00\",\"command\":\"bash -lc 'set -euo pipefail\\nbin=/root/atelier/target/debug/atelier\\ntmp=$(mktemp -d)\\ncd \\\"$tmp\\\"\\n\\\"$bin\\\" init\\n\\\"$bin\\\" mission create \\\"Readable mission proof\\\" --body \\\"Readable intent\\\" --constraint \\\"Use typed sections\\\" --risk \\\"Relationship drift\\\" --validation \\\"Run focused checks\\\"\\nmission=$(basename .atelier/missions/*.md .md)\\n\\\"$bin\\\" mission update \\\"$mission\\\" --body \\\"Updated readable intent\\\" --constraint \\\"Updated typed constraint\\\" --risk \\\"Updated relationship drift\\\" --validation \\\"Run mission status\\\"\\n\\\"$bin\\\" mission show \\\"$mission\\\"\\n\\\"$bin\\\" mission status \\\"$mission\\\"\\n\\\"$bin\\\" export --check\\n\\\"$bin\\\" lint\\nif grep -q \\\"^data:\\\" \\\".atelier/missions/$mission.md\\\"; then\\n  echo \\\"unexpected mission data front matter\\\"\\n  sed -n \\\"1,80p\\\" \\\".atelier/missions/$mission.md\\\"\\n  exit 1\\nfi\\necho \\\"no escaped mission data front matter\\\"\\nsed -n \\\"1,90p\\\" \\\".atelier/missions/$mission.md\\\"\\n'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":192,\"summary\":\"Refreshed projection in /tmp/tmp.7PohXwF0HP/.atelier/state.db from /tmp/tmp.7PohXwF0HP/.atelier\\nRefreshed projection in /tmp/tmp.7PohXwF0HP/.atelier/state.db from /tmp/tmp.7PohXwF0HP/.atelier\\n\",\"truncated\":false},\"stdout\":{\"bytes\":3606,\"summary\":\"Created /tmp/tmp.7PohXwF0HP/.atelier\\nCreated /tmp/tmp.7PohXwF0HP/.atelier/config.toml\\nCreated /tmp/tmp.7PohXwF0HP/.atelier/state.db\\nAtelier initialized successfully!\\n\\nNext steps:\\n  atelier issue create \\\"Task\\\"     # Create an issue\\n  atelier start <issue-id>         # Start tracked work\\nMission atelier-qar8: Readable mission proof\\nStatus: ready\\n\\n## Intent\\n\\nReadable intent\\n\\n## Constraints\\n\\n- Use typed sections\\n\\n## Risks\\n\\n- Relationship drift\\n\\n## Validation\\n\\n- Run focused checks\\nMission atelier-qar8: Readable mission proof\\nStatus: ready\\n\\n## Intent\\n\\nUpdated readable intent\\n\\n## Constraints\\n\\n- Updated typed constraint\\n\\n## Risks\\n\\n- Updated relationship drift\\n\\n## Validation\\n\\n- Run mission status\\nMission atelier-qar8 [ready] - Readable mission proof\\n=====================================================\\nStatus:   ready\\nCreated:  2026-06-12 17:58 -04:00\\nUpdated:  2026-06-12 17:58 -04:00\\n\\nIntent\\n------\\nUpdated readable intent\\n\\nConstraints\\n-----------\\n- Updated typed constraint\\n\\nRisks\\n-----\\n- Updated relationship drift\\n\\nValidation\\n----------\\n- Run mission status\\n\\nProgress\\n--------\\nRecords: plans=0 milestones=0 evidence=0\\nWork: ready=0 blocked=0 done=0 backlog=0\\nMission Blockers: 0\\n\\nPlans\\n-----\\n(none)\\n\\nMilestones\\n----------\\n(none)\\n\\nEvidence\\n--------\\n(none)\\n\\nMission Blockers\\n----------------\\n(none)\\n\\nLinked Work\\n-----------\\n(none)\\n\\nSupporting Records\\n------------------\\n(none)\\n\\nEvidence Gaps\\n-------------\\n  No evidence records are linked to this mission.\\n\\nNext Commands\\n-------------\\n  atelier mission status atelier-qar8\\n  atelier mission show atelier-qar8\\n  atelier mission add-work atelier-qar8 <issue-id>\\n  atelier mission status atelier-qar8\\nLint passed.\\nMission Status atelier-qar8 [ready] - Readable mission proof\\n============================================================\\nHealth:   needs-evidence\\nTracker:  ok\\nCloseout: blocked\\n\\nWork\\n----\\nTotal: none\\nEpics: none\\n\\nBlockers\\n--------\\n(none)\\n\\nEvidence\\n--------\\nGap: no evidence records are linked to this mission.\\n\\nCloseout Gates\\n--------------\\nWork: closed\\nBlockers: clear\\nEvidence: missing\\n  Next: atelier evidence add --kind validation --result pass \\\"...\\\"\\n  Next: atelier evidence attach <evidence-id> mission <mission-id>\\nValidator durable_state_current: pass\\nValidator issue_sections_parseable: pass\\nValidator evidence_attached: fail - no validating evidence link found\\nValidator no_open_blockers: pass\\nValidator no_blocking_lints: pass\\nValidator ignored_tests_reviewed: pass\\nValidator git_worktree_clean: fail - git status failed: fatal: not a git repository (or any of the parent directories): .git\\n\\nValidators\\n----------\\n2 closeout validator failure detected.\\n  fail  evidence_attached - no validating evidence link found\\n  fail  git_worktree_clean - git status failed: fatal: not a git repository (or any of the parent directories): .git\\n\\nActive Work\\n-----------\\n(none)\\n\\nNext Commands\\n-------------\\n  atelier mission show atelier-qar8\\n  atelier mission status atelier-qar8\\n  atelier evidence add --kind validation --result pass \\\"...\\\"\\n  atelier doctor\\nCanonical export is current\\nState: /tmp/tmp.7PohXwF0HP/.atelier\\nLint passed.\\nno escaped mission data front matter\\n---\\ncreated_at: \\\"2026-06-12T21:58:55.264955+00:00\\\"\\nid: \\\"atelier-qar8\\\"\\nlabels:\\n- \\\"mission\\\"\\nrelationships:\\n  blocks: []\\n  children: []\\n  attachments: []\\n  relates: []\\nschema: \\\"atelier.mission\\\"\\nschema_version: 1\\nstatus: \\\"ready\\\"\\ntitle: \\\"Readable mission proof\\\"\\nupdated_at: \\\"2026-06-12T21:58:55.349234449+00:00\\\"\\n---\\n\\n## Intent\\n\\nUpdated readable intent\\n\\n## Constraints\\n\\n- Updated typed constraint\\n\\n## Risks\\n\\n- Updated relationship drift\\n\\n## Validation\\n\\n- Run mission status\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-8ec6\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7r55"
    type: "validates"
  - kind: "issue"
    id: "atelier-8ec6"
    type: "validates"
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Mission readable record CLI transcript: create update show status export lint and no data writer"
updated_at: "2026-06-12T21:59:40.415348216+00:00"
---

Mission readable record CLI transcript: create update show status export lint and no data writer

Command: bash -lc 'set -euo pipefail
bin=/root/atelier/target/debug/atelier
tmp=$(mktemp -d)
cd "$tmp"
"$bin" init
"$bin" mission create "Readable mission proof" --body "Readable intent" --constraint "Use typed sections" --risk "Relationship drift" --validation "Run focused checks"
mission=$(basename .atelier/missions/*.md .md)
"$bin" mission update "$mission" --body "Updated readable intent" --constraint "Updated typed constraint" --risk "Updated relationship drift" --validation "Run mission status"
"$bin" mission show "$mission"
"$bin" mission status "$mission"
"$bin" export --check
"$bin" lint
if grep -q "^data:" ".atelier/missions/$mission.md"; then
  echo "unexpected mission data front matter"
  sed -n "1,80p" ".atelier/missions/$mission.md"
  exit 1
fi
echo "no escaped mission data front matter"
sed -n "1,90p" ".atelier/missions/$mission.md"
'
Exit status: 0

Stdout summary:
Created /tmp/tmp.7PohXwF0HP/.atelier
Created /tmp/tmp.7PohXwF0HP/.atelier/config.toml
Created /tmp/tmp.7PohXwF0HP/.atelier/state.db
Atelier initialized successfully!

Next steps:
  atelier issue create "Task"     # Create an issue
  atelier start <issue-id>         # Start tracked work
Mission atelier-qar8: Readable mission proof
Status: ready

## Intent

Readable intent

## Constraints

- Use typed sections

## Risks

- Relationship drift

## Validation

- Run focused checks
Mission atelier-qar8: Readable mission proof
Status: ready

## Intent

Updated readable intent

## Constraints

- Updated typed constraint

## Risks

- Updated relationship drift

## Validation

- Run mission status
Mission atelier-qar8 [ready] - Readable mission proof
=====================================================
Status:   ready
Created:  2026-06-12 17:58 -04:00
Updated:  2026-06-12 17:58 -04:00

Intent
------
Updated readable intent

Constraints
-----------
- Updated typed constraint

Risks
-----
- Updated relationship drift

Validation
----------
- Run mission status

Progress
--------
Records: plans=0 milestones=0 evidence=0
Work: ready=0 blocked=0 done=0 backlog=0
Mission Blockers: 0

Plans
-----
(none)

Milestones
----------
(none)

Evidence
--------
(none)

Mission Blockers
----------------
(none)

Linked Work
-----------
(none)

Supporting Records
------------------
(none)

Evidence Gaps
-------------
  No evidence records are linked to this mission.

Next Commands
-------------
  atelier mission status atelier-qar8
  atelier mission show atelier-qar8
  atelier mission add-work atelier-qar8 <issue-id>
  atelier mission status atelier-qar8
Lint passed.
Mission Status atelier-qar8 [ready] - Readable mission proof
============================================================
Health:   needs-evidence
Tracker:  ok
Closeout: blocked

Work
----
Total: none
Epics: none

Blockers
--------
(none)

Evidence
--------
Gap: no evidence records are linked to this mission.

Closeout Gates
--------------
Work: closed
Blockers: clear
Evidence: missing
  Next: atelier evidence add --kind validation --result pass "..."
  Next: atelier evidence attach <evidence-id> mission <mission-id>
Validator durable_state_current: pass
Validator issue_sections_parseable: pass
Validator evidence_attached: fail - no validating evidence link found
Validator no_open_blockers: pass
Validator no_blocking_lints: pass
Validator ignored_tests_reviewed: pass
Validator git_worktree_clean: fail - git status failed: fatal: not a git repository (or any of the parent directories): .git

Validators
----------
2 closeout validator failure detected.
  fail  evidence_attached - no validating evidence link found
  fail  git_worktree_clean - git status failed: fatal: not a git repository (or any of the parent directories): .git

Active Work
-----------
(none)

Next Commands
-------------
  atelier mission show atelier-qar8
  atelier mission status atelier-qar8
  atelier evidence add --kind validation --result pass "..."
  atelier doctor
Canonical export is current
State: /tmp/tmp.7PohXwF0HP/.atelier
Lint passed.
no escaped mission data front matter
---
created_at: "2026-06-12T21:58:55.264955+00:00"
id: "atelier-qar8"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Readable mission proof"
updated_at: "2026-06-12T21:58:55.349234449+00:00"
---

## Intent

Updated readable intent

## Constraints

- Updated typed constraint

## Risks

- Updated relationship drift

## Validation

- Run mission status

Stderr summary:
Refreshed projection in /tmp/tmp.7PohXwF0HP/.atelier/state.db from /tmp/tmp.7PohXwF0HP/.atelier
Refreshed projection in /tmp/tmp.7PohXwF0HP/.atelier/state.db from /tmp/tmp.7PohXwF0HP/.atelier

