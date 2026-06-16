---
created_at: "2026-06-12T22:26:53.407067489+00:00"
id: "atelier-191m"
evidence_type: "validation"
captured_at: "2026-06-12T22:26:52.929448232+00:00"
command: "bash -lc '\nset -euo pipefail\nA=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d /tmp/atelier-6aor-health.XXXXXX)\ncleanup() { rm -rf \"$TMP\"; }\ntrap cleanup EXIT\ncd \"$TMP\"\ngit init -q\n\"$A\" init >/dev/null\n\"$A\" mission create \"Readable mission validation\" --body \"Readable mission intent\" --constraint \"Keep mission records reviewable\" --risk \"Projection drift\" --validation \"Run rebuild export lint doctor\" >/dev/null\nMID=$(basename .atelier/missions/*.md .md)\nprintf \"\\n$ %s mission show %s\\n\" \"$A\" \"$MID\"\n\"$A\" mission show \"$MID\" | sed -n \"1,42p\"\nprintf \"\\n$ %s mission status %s\\n\" \"$A\" \"$MID\"\n\"$A\" mission status \"$MID\" | sed -n \"1,32p\"\nprintf \"\\n$ %s rebuild\\n\" \"$A\"\n\"$A\" rebuild\nprintf \"\\n$ %s export --check\\n\" \"$A\"\n\"$A\" export --check\nprintf \"\\n$ %s lint\\n\" \"$A\"\n\"$A\" lint\nprintf \"\\n$ %s doctor\\n\" \"$A\"\n\"$A\" doctor | sed -n \"1,28p\"\n'"
exit_status: "0"
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
status: "recorded"
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
