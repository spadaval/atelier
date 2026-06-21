---
created_at: "2026-06-21T18:48:17.374007090+00:00"
id: "atelier-ex4h"
evidence_type: "test"
captured_at: "2026-06-21T18:48:08.046362095+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier status\ntarget/debug/atelier issue table --kind mission\ntarget/debug/atelier issue list --ready\ntarget/debug/atelier issue status atelier-53bu\ntarget/debug/atelier help | rg \"atelier issue table --kind mission\"\ntarget/debug/atelier issue status --help | rg \"Usage: atelier issue status \\[OPTIONS\\] <ID>\"\ntarget/debug/atelier man manager | rg \"atelier issue table --kind mission\"\ntarget/debug/atelier man manager | rg \"atelier issue status <id>\"\nif target/debug/atelier issue status >/tmp/e7t1-no-id-status.out 2>&1; then\n  echo \"no-ID issue status unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg \"Usage: atelier issue status <ID>\" /tmp/e7t1-no-id-status.out\n! rg \"Default first command: `atelier issue status`\" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-e7t1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e7t1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ntarget/debug/atelier status\ntarget/debug/atelier issue table --kind mission\ntarget/debug/atelier issue list --ready\ntarget/debug/atelier issue status atelier-53bu\ntarget/debug/atelier help | rg \"atelier issue table --kind mission\"\ntarget/debug/atelier issue status --help | rg \"Usage: atelier issue status \\[OPTIONS\\] <ID>\"\ntarget/debug/atelier man manager | rg \"atelier issue table --kind mission\"\ntarget/debug/atelier man manager | rg \"atelier issue status <id>\"\nif target/debug/atelier issue status >/tmp/e7t1-no-id-status.out 2>&1; then\n  echo \"no-ID issue status unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg \"Usage: atelier issue status <ID>\" /tmp/e7t1-no-id-status.out\n! rg \"Default first command: `atelier issue status`\" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests\n'"
updated_at: "2026-06-21T18:49:42.110639223+00:00"
---

## Summary

bash -lc 'set -euo pipefail
target/debug/atelier status
target/debug/atelier issue table --kind mission
target/debug/atelier issue list --ready
target/debug/atelier issue status atelier-53bu
target/debug/atelier help | rg "atelier issue table --kind mission"
target/debug/atelier issue status --help | rg "Usage: atelier issue status \[OPTIONS\] <ID>"
target/debug/atelier man manager | rg "atelier issue table --kind mission"
target/debug/atelier man manager | rg "atelier issue status <id>"
if target/debug/atelier issue status >/tmp/e7t1-no-id-status.out 2>&1; then
  echo "no-ID issue status unexpectedly succeeded" >&2
  exit 1
fi
rg "Usage: atelier issue status <ID>" /tmp/e7t1-no-id-status.out
! rg "Default first command: `atelier issue status`" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests
'

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier status
target/debug/atelier issue table --kind mission
target/debug/atelier issue list --ready
target/debug/atelier issue status atelier-53bu
target/debug/atelier help | rg "atelier issue table --kind mission"
target/debug/atelier issue status --help | rg "Usage: atelier issue status \[OPTIONS\] <ID>"
target/debug/atelier man manager | rg "atelier issue table --kind mission"
target/debug/atelier man manager | rg "atelier issue status <id>"
if target/debug/atelier issue status >/tmp/e7t1-no-id-status.out 2>&1; then
  echo "no-ID issue status unexpectedly succeeded" >&2
  exit 1
fi
rg "Usage: atelier issue status <ID>" /tmp/e7t1-no-id-status.out
! rg "Default first command: `atelier issue status`" docs .agents/skills/agent-factory crates/atelier-cli/src crates/atelier-cli/tests
'
```

Exit status: 0

## Stdout

Bytes: 9832
Truncated: yes

```text
Atelier Status
==============
Tracker:       current
Ready work:    12
Current work:  1 issue(s)
  active atelier-e7t1 - Split mission discovery out of issue status [worker]
Active mission: none (4 current)
Active roles:   worker=1

Local State
-----------
Branch:   epic/atelier-vays
Checkout: dirty (18 entries)
   M .atelier/issues/atelier-e7t1.md
   M crates/atelier-cli/src/commands/issue.rs
   M crates/atelier-cli/src/commands/man.rs
Tracker:  current

Branch Policy
----------------
Current branch: epic/atelier-vays
Base branch:    master
Branch owner:   epic atelier-vays (epic)
Active work:
  atelier-e7t1 - owner epic atelier-vays (epic) | expected epic/atelier-vays | ok

Evidence Status
---------------
Attached Proof: missing - 13 issue(s) without validating evidence; 0 attached
  Missing: atelier-62po
  Missing: atelier-9n3r
  Missing: atelier-a0h0
  Missing: 10 more issue(s)
  Next: atelier evidence record --target issue/<id> --kind validation "..."
  Next: atelier evidence attach <evidence-id> issue <issue-id>

Recent Activity
---------------
(no active mission)

Next Actions
------------
  Inspect mission choices (4 current mission(s), none active): atelier issue table --kind mission
  Choose ready work (12 ready issue(s) available): atelier issue list --ready
  Inspect selected work transitions (ready work exists): atelier issue transition <issue-id> --options
Issue Table: mission
====================
ID           Status       Health     Ready  Blocked  Done  Backlog  Title
atelier-24xn ready        blocked        3        6     0       0  Prune stale Atelier artifacts and branches
atelier-53bu ready        blocked        7       12     3       0  Make workflow obligations explicit and minimal
atelier-e3pk ready        steady         0        0     0       0  Superseded by 0v3f: Add session-aware Forgejo PR coordination

Next Commands
-------------
  Inspect one objective: atelier issue status <id>
  Open one objective record: atelier issue show <id>
  Browse grouped work: atelier issue list
Issue Queue
===========
11 total | Category: todo=11 | Status: todo=11 | Priority: high=8, medium=3 | Blocked: 0

[epic] atelier-ncq9 high - Epic: Move mission lifecycle into workflow policy
----------------------------------------------------------------------------
    ready [feature] atelier-s43l - Require mission type declaration and workflow coverage

[epic] atelier-txf6 high - Epic: Retention and prune contract
-------------------------------------------------------------
    ready [task] atelier-kpa1 - Define retention classes and prune safety contract
    ready [spike] atelier-bd8j - Inventory current artifact and branch growth

[epic] atelier-vays high - Epic: Collapse mission command surface into issue commands (context; parent blocked)
---------------------------------------------------------------------------------------------------------------
  blocked by 1 external blocker; details: atelier issue blocked atelier-vays
    ready [feature] atelier-62po - Make issue show/status own mission objective views

[epic] atelier-9n3r medium - Epic: Make evidence requirements workflow-driven
-----------------------------------------------------------------------------
    ready [feature] atelier-e5ye - Stop parser and lint from requiring evidence prose by default
    ready [feature] atelier-od9a - Make evidence validator failures provide simple help hints

Standalone
----------
    ready [epic] atelier-ikuv - Superseded umbrella: Command surface consolidation and removal
    ready [bug] atelier-a0h0 - Clarify or remove generic issue fields storage
Lint passed.
Mission Status atelier-53bu [ready] - Make workflow obligations explicit and minimal
====================================================================================
Health:   blocked
Tracker:  ok
Terminal: blocked

Work
----
Total: 7 ready, 12 blocked, 3 done, 1 backlog
  [epic] atelier-vays [todo] high - Epic: Collapse mission command surface into issue commands | 1 ready, 2 blocked, 1 backlog
  [epic] atelier-ncq9 [todo] high - Epic: Move mission lifecycle in
```

## Stderr

Bytes: 194
Truncated: no

```text
rg: the literal "\n" is not allowed in a regex

Consider enabling multiline mode with the --multiline flag (or -U for short).
When multiline mode is enabled, new line characters can be matched.
```

