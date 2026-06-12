---
created_at: "2026-06-12T22:26:53.407067489+00:00"
id: "atelier-191m"
data: "{\"captured_at\":\"2026-06-12T22:26:52.929448232+00:00\",\"command\":\"bash -lc '\\nset -euo pipefail\\nA=/root/atelier/target/debug/atelier\\nTMP=$(mktemp -d /tmp/atelier-6aor-health.XXXXXX)\\ncleanup() { rm -rf \\\"$TMP\\\"; }\\ntrap cleanup EXIT\\ncd \\\"$TMP\\\"\\ngit init -q\\n\\\"$A\\\" init >/dev/null\\n\\\"$A\\\" mission create \\\"Readable mission validation\\\" --body \\\"Readable mission intent\\\" --constraint \\\"Keep mission records reviewable\\\" --risk \\\"Projection drift\\\" --validation \\\"Run rebuild export lint doctor\\\" >/dev/null\\nMID=$(basename .atelier/missions/*.md .md)\\nprintf \\\"\\\\n$ %s mission show %s\\\\n\\\" \\\"$A\\\" \\\"$MID\\\"\\n\\\"$A\\\" mission show \\\"$MID\\\" | sed -n \\\"1,42p\\\"\\nprintf \\\"\\\\n$ %s mission status %s\\\\n\\\" \\\"$A\\\" \\\"$MID\\\"\\n\\\"$A\\\" mission status \\\"$MID\\\" | sed -n \\\"1,32p\\\"\\nprintf \\\"\\\\n$ %s rebuild\\\\n\\\" \\\"$A\\\"\\n\\\"$A\\\" rebuild\\nprintf \\\"\\\\n$ %s export --check\\\\n\\\" \\\"$A\\\"\\n\\\"$A\\\" export --check\\nprintf \\\"\\\\n$ %s lint\\\\n\\\" \\\"$A\\\"\\n\\\"$A\\\" lint\\nprintf \\\"\\\\n$ %s doctor\\\\n\\\" \\\"$A\\\"\\n\\\"$A\\\" doctor | sed -n \\\"1,28p\\\"\\n'\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":224,\"summary\":\"Refreshed projection in /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db from /tmp/atelier-6aor-health.ERQZEZ/.atelier\\nRebuilt /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db from /tmp/atelier-6aor-health.ERQZEZ/.atelier\\n\",\"truncated\":false},\"stdout\":{\"bytes\":2661,\"summary\":\"\\n$ /root/atelier/target/debug/atelier mission show atelier-z0ic\\nMission atelier-z0ic [ready] - Readable mission validation\\n==========================================================\\nStatus:   ready\\nCreated:  2026-06-12 18:26 -04:00\\nUpdated:  2026-06-12 18:26 -04:00\\n\\nIntent\\n------\\nReadable mission intent\\n\\nConstraints\\n-----------\\n- Keep mission records reviewable\\n\\nRisks\\n-----\\n- Projection drift\\n\\nValidation\\n----------\\n- Run rebuild export lint doctor\\n\\nProgress\\n--------\\nRecords: plans=0 milestones=0 evidence=0\\nWork: ready=0 blocked=0 done=0 backlog=0\\nMission Blockers: 0\\n\\nPlans\\n-----\\n(none)\\n\\nMilestones\\n----------\\n(none)\\n\\nEvidence\\n--------\\n(none)\\n\\nMission Blockers\\n----------------\\n\\n$ /root/atelier/target/debug/atelier mission status atelier-z0ic\\nLint passed.\\nMission Status atelier-z0ic [ready] - Readable mission validation\\n=================================================================\\nHealth:   needs-evidence\\nTracker:  ok\\nCloseout: blocked\\n\\nWork\\n----\\nTotal: none\\nEpics: none\\n\\nBlockers\\n--------\\n(none)\\n\\nEvidence\\n--------\\nGap: no evidence records are linked to this mission.\\n\\nCloseout Gates\\n--------------\\nWork: closed\\nBlockers: clear\\nEvidence: missing\\n  Next: atelier evidence add --kind validation --result pass \\\"...\\\"\\n  Next: atelier evidence attach <evidence-id> mission <mission-id>\\nValidator durable_state_current: pass\\nValidator issue_sections_parseable: pass\\nValidator evidence_attached: fail - no validating evidence link found\\nValidator no_open_blockers: pass\\nValidator no_blocking_lints: pass\\n\\n$ /root/atelier/target/debug/atelier rebuild\\nRuntime state rebuilt\\nState:    /tmp/atelier-6aor-health.ERQZEZ/.atelier\\nDatabase: /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db\\n\\nNext Commands\\n-------------\\n  atelier doctor\\n  atelier export --check\\n\\n$ /root/atelier/target/debug/atelier export --check\\nCanonical export is current\\nState: /tmp/atelier-6aor-health.ERQZEZ/.atelier\\n\\n$ /root/atelier/target/debug/atelier lint\\nLint passed.\\n\\n$ /root/atelier/target/debug/atelier doctor\\nDatabase: /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db\\nState: /tmp/atelier-6aor-health.ERQZEZ/.atelier\\nInstall health:\\n  config: ok\\n  ignored_runtime_paths: ok\\nProjection rebuild:\\n  state_dir: ok\\n  rebuild_ready: ok\\n  projection_fresh: ok\\n  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources\\nCache health:\\n  cache_dir: missing (optional)\\n  projection_metadata: ok\\nRuntime state:\\n  directory: ok\\n  database: ok\\n  local_tables: ok\\n  diagnostics: enabled\\nCompatibility:\\n  tables: comments\\nLegacy health:\\nconfig: ok\\ndatabase: ok\\nignore_rules: ok\\nprojection_fresh: ok\\nrebuild_ready: ok\\nruntime_state: ok\\nruntime_tables: ok\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-6aor\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6aor"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Positive mission show status rebuild export lint doctor transcript"
updated_at: "2026-06-12T22:26:54.664417116+00:00"
---

Positive mission show status rebuild export lint doctor transcript

Command: bash -lc '
set -euo pipefail
A=/root/atelier/target/debug/atelier
TMP=$(mktemp -d /tmp/atelier-6aor-health.XXXXXX)
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT
cd "$TMP"
git init -q
"$A" init >/dev/null
"$A" mission create "Readable mission validation" --body "Readable mission intent" --constraint "Keep mission records reviewable" --risk "Projection drift" --validation "Run rebuild export lint doctor" >/dev/null
MID=$(basename .atelier/missions/*.md .md)
printf "\n$ %s mission show %s\n" "$A" "$MID"
"$A" mission show "$MID" | sed -n "1,42p"
printf "\n$ %s mission status %s\n" "$A" "$MID"
"$A" mission status "$MID" | sed -n "1,32p"
printf "\n$ %s rebuild\n" "$A"
"$A" rebuild
printf "\n$ %s export --check\n" "$A"
"$A" export --check
printf "\n$ %s lint\n" "$A"
"$A" lint
printf "\n$ %s doctor\n" "$A"
"$A" doctor | sed -n "1,28p"
'
Exit status: 0

Stdout summary:

$ /root/atelier/target/debug/atelier mission show atelier-z0ic
Mission atelier-z0ic [ready] - Readable mission validation
==========================================================
Status:   ready
Created:  2026-06-12 18:26 -04:00
Updated:  2026-06-12 18:26 -04:00

Intent
------
Readable mission intent

Constraints
-----------
- Keep mission records reviewable

Risks
-----
- Projection drift

Validation
----------
- Run rebuild export lint doctor

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

$ /root/atelier/target/debug/atelier mission status atelier-z0ic
Lint passed.
Mission Status atelier-z0ic [ready] - Readable mission validation
=================================================================
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

$ /root/atelier/target/debug/atelier rebuild
Runtime state rebuilt
State:    /tmp/atelier-6aor-health.ERQZEZ/.atelier
Database: /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db

Next Commands
-------------
  atelier doctor
  atelier export --check

$ /root/atelier/target/debug/atelier export --check
Canonical export is current
State: /tmp/atelier-6aor-health.ERQZEZ/.atelier

$ /root/atelier/target/debug/atelier lint
Lint passed.

$ /root/atelier/target/debug/atelier doctor
Database: /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db
State: /tmp/atelier-6aor-health.ERQZEZ/.atelier
Install health:
  config: ok
  ignored_runtime_paths: ok
Projection rebuild:
  state_dir: ok
  rebuild_ready: ok
  projection_fresh: ok
  tables: issues, labels, dependencies, relations, records, record_links, projection_index_sources
Cache health:
  cache_dir: missing (optional)
  projection_metadata: ok
Runtime state:
  directory: ok
  database: ok
  local_tables: ok
  diagnostics: enabled
Compatibility:
  tables: comments
Legacy health:
config: ok
database: ok
ignore_rules: ok
projection_fresh: ok
rebuild_ready: ok
runtime_state: ok
runtime_tables: ok

Stderr summary:
Refreshed projection in /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db from /tmp/atelier-6aor-health.ERQZEZ/.atelier
Rebuilt /tmp/atelier-6aor-health.ERQZEZ/.atelier/state.db from /tmp/atelier-6aor-health.ERQZEZ/.atelier
