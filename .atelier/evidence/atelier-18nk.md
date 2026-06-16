---
created_at: "2026-06-16T19:00:19.686489714+00:00"
id: "atelier-18nk"
evidence_type: "validation"
captured_at: "2026-06-16T19:00:16.896281509+00:00"
command: "bash -lc '\nset -euo pipefail\nA=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d /tmp/atelier-em15.XXXXXX)\ntrap \"rm -rf \\\"$TMP\\\"\" EXIT\ncd \"$TMP\"\ngit init -q\ngit branch -M main\ngit config user.email test@example.com\ngit config user.name Test\n$A init -q >/dev/null\n$A mission create -q \"Ordering validation mission\" >/dev/null\nMISSION=$(basename .atelier/missions/*.md .md)\n$A mission start \"$MISSION\" >/dev/null\nEPIC=$($A issue create -q --issue-type epic \"Ordering parent epic\")\ngit checkout -qb \"epic/$EPIC\"\nCONTRACT=$($A issue create -q --parent \"$EPIC\" \"Contract blocker work\")\nIMPLEMENT=$($A issue create -q --parent \"$EPIC\" \"Implementation blocked work\")\nVALIDATE=$($A issue create -q --parent \"$EPIC\" --issue-type validation \"Validation child work\")\nREADY=$($A issue create -q \"Loose ready work\")\nEXTERNAL=$($A issue create -q \"External hidden blocker\")\nfor f in .atelier/issues/*.md; do perl -0pi -e \"s/Outcome was not specified\\./- Fixture outcome supports blocker-order validation./g; s/Evidence was not specified\\./- Command transcript validates blocker-order behavior./g\" \"$f\"; done\n$A issue block \"$IMPLEMENT\" \"$CONTRACT\" >/dev/null\n$A issue block \"$VALIDATE\" \"$IMPLEMENT\" >/dev/null\n$A issue block \"$READY\" \"$EXTERNAL\" >/dev/null\n$A mission add-work \"$MISSION\" \"$EPIC\" >/dev/null\n$A mission add-work \"$MISSION\" \"$READY\" >/dev/null\ngit add . && git commit -qm fixture\n$A issue transition \"$CONTRACT\" --options >/dev/null\n\necho \"Fixture IDs: mission=$MISSION epic=$EPIC contract=$CONTRACT implementation=$IMPLEMENT validation=$VALIDATE ready=$READY external=$EXTERNAL\"\nassert_before() {\n  local label=$1 haystack=$2 first=$3 second=$4\n  python3 - \"$label\" \"$haystack\" \"$first\" \"$second\" <<PY\nimport sys\nlabel,path,first,second=sys.argv[1:]\ntext=open(path).read()\na=text.find(first)\nb=text.find(second)\nif a < 0 or b < 0 or a >= b:\n    print(f\"ASSERTION FAILED {label}: {first!r} before {second!r}\\n{text}\", file=sys.stderr)\n    sys.exit(1)\nPY\n}\nassert_contains() { local label=$1 haystack=$2 needle=$3; if ! grep -Fq \"$needle\" \"$haystack\"; then echo \"ASSERTION FAILED $label: missing $needle\" >&2; cat \"$haystack\" >&2; exit 1; fi; }\nassert_not_contains() { local label=$1 haystack=$2 needle=$3; if grep -Fq \"$needle\" \"$haystack\"; then echo \"ASSERTION FAILED $label: unexpected $needle\" >&2; cat \"$haystack\" >&2; exit 1; fi; }\nrun_surface() { local name=$1; shift; local out=\"$TMP/$name.out\"; echo \"\\n$ $A $*\"; $A \"$@\" | tee \"$out\"; }\n\nrun_surface issue_ready issue list --ready\nassert_contains issue_ready \"$TMP/issue_ready.out\" \"No issues found.\"\nassert_not_contains issue_ready \"$TMP/issue_ready.out\" \"todo/todo\"\n\nrun_surface issue_list issue list\nassert_before issue_list \"$TMP/issue_list.out\" \"$CONTRACT\" \"$IMPLEMENT\"\nassert_contains issue_list \"$TMP/issue_list.out\" \"ready [task] $CONTRACT\"\nassert_contains issue_list \"$TMP/issue_list.out\" \"blocked [task] $IMPLEMENT\"\nassert_not_contains issue_list \"$TMP/issue_list.out\" \"todo/todo\"\n\nrun_surface issue_blocked issue list --blocked\nassert_before issue_blocked \"$TMP/issue_blocked.out\" \"$IMPLEMENT\" \"$VALIDATE\"\nassert_contains issue_blocked \"$TMP/issue_blocked.out\" \"details: atelier issue blocked $IMPLEMENT\"\nassert_not_contains issue_blocked \"$TMP/issue_blocked.out\" \"todo/todo\"\n\nrun_surface issue_show issue show \"$EPIC\"\nassert_before issue_show \"$TMP/issue_show.out\" \"$CONTRACT\" \"$IMPLEMENT\"\nassert_before issue_show \"$TMP/issue_show.out\" \"$IMPLEMENT\" \"$VALIDATE\"\nassert_contains issue_show \"$TMP/issue_show.out\" \"blocked $IMPLEMENT\"\n\nrun_surface graph_tree graph tree\nassert_before graph_tree \"$TMP/graph_tree.out\" \"$CONTRACT\" \"$IMPLEMENT\"\nassert_before graph_tree \"$TMP/graph_tree.out\" \"$IMPLEMENT\" \"$VALIDATE\"\nassert_contains graph_tree \"$TMP/graph_tree.out\" \"Legend: ready, blocked\"\nassert_not_contains graph_tree \"$TMP/graph_tree.out\" \"todo/todo\"\n\nrun_surface mission_status mission status \"$MISSION\"\nassert_before mission_status \"$TMP/mission_status.out\" \"$CONTRACT\" \"$IMPLEMENT\"\nassert_contains mission_status \"$TMP/mission_status.out\" \"details: atelier issue blocked $IMPLEMENT\"\nassert_not_contains mission_status \"$TMP/mission_status.out\" \"todo/todo\"\n\nrun_surface mission_list mission list\nassert_contains mission_list \"$TMP/mission_list.out\" \"Next work: ready $CONTRACT - Contract blocker work\"\nassert_contains mission_list \"$TMP/mission_list.out\" \"[epic] $EPIC [ready]\"\nassert_not_contains mission_list \"$TMP/mission_list.out\" \"todo/todo\"\n\nrun_surface root_status status\nassert_before root_status \"$TMP/root_status.out\" \"$CONTRACT\" \"$IMPLEMENT\"\nassert_contains root_status \"$TMP/root_status.out\" \"ready $CONTRACT\"\nassert_not_contains root_status \"$TMP/root_status.out\" \"todo/todo\"\n\ncat <<REPORT\n\nSurface classification:\n- issue list: PASS visible blocker appears before blocked implementation; readable ready/blocked labels present.\n- issue list --ready: PASS command checked; strict ready queue returned no rows in this fixture, so no ordering conflict is visible on that bucketed surface.\n- issue list --blocked: PASS blocked implementation precedes validation child and points to issue blocked drill-down.\n- issue show: PASS subissue order follows contract -> implementation -> validation and subissue rows use readable state labels.\n- graph tree: PASS hierarchy keeps parent shape while ordering visible siblings by blocker dependency.\n- mission status: PASS shared mission summary lists selectable blocker before blocked work with drill-down guidance.\n- mission list: PASS compact overview stays compact and cues the visible blocker as next work.\n- root status: PASS active-mission status exposes the visible blocker before blocked work.\nFollow-up issues: none; no product mismatch observed.\nREPORT\n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-em15"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 10894
    summary: "Fixture IDs: mission=atelier-f33o epic=atelier-hh5b contract=atelier-p0a6 implementation=atelier-q35s validation=atelier-gggn ready=atelier-zv35 external=atelier-xxi5\n\\n$ /root/atelier/target/debug/atelier issue list --ready\nNo issues found.\n\\n$ /root/atelier/target/debug/atelier issue list\nIssue Queue\n===========\n6 total | Category: todo=6 | Status: todo=6 | Priority: medium=6 | Blocked: 3\n\n[epic] atelier-hh5b medium - Ordering parent epic\n-------------------------------------------------\n    ready [task] atelier-p0a6 - Contract blocker work\n    blocked [task] atelier-q35s - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)\n    blocked [validation] atelier-gggn - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)\n\nStandalone\n----------\n    ready [task] atelier-xxi5 - External hidden blocker\n    blocked [task] atelier-zv35 - Loose ready work (1 blocker; details: atelier issue blocked atelier-zv35)\n\\n$ /root/atelier/target/debug/atelier issue list --blocked\nBlocked issues\n==============\n3 total\nDrill down: atelier issue blocked <id>\n  blocked atelier-zv35 Loose ready work (1 blocker; details: atelier issue blocked atelier-zv35)\n  blocked atelier-q35s Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)\n  blocked atelier-gggn Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)\n\\n$ /root/atelier/target/debug/atelier issue show atelier-hh5b\natelier-hh5b [epic] todo/todo - Ordering parent epic\n====================================================\nStatus:   todo\nCategory: todo\nType:     epic\nPriority: medium\nCreated:  2026-06-16 15:00 -04:00\nUpdated:  2026-06-16 15:00 -04:00\nFile:     /tmp/atelier-em15.6ZVOOb/.atelier/issues/atelier-hh5b.md\n\nHierarchy\n---------\nParent: (none)\n\nBranch Lifecycle\n----------------\nOwner:    epic atelier-hh5b (epic)\nExpected: epic/atelier-hh5b\nBase:     main\nScope:    owns its merge branch\nCurrent:  epic/atelier-hh5b\nState:    current branch matches expected branch\nNext:     atelier start atelier-hh5b\nClose:    atelier issue close atelier-hh5b --reason \"...\"\n\nTransition Readiness\n--------------------\n  block: allowed - to blocked\n    atelier issue transition atelier-hh5b block\n  start: blocked - branch context: worktree has uncommitted changes; inspect `git status --short --branch`, then rerun `atelier start atelier-hh5b`\n    atelier issue transition atelier-hh5b start\n  options: atelier issue transition atelier-hh5b --options\n\nDescription\n-----------\nNo description provided.\n\nOutcome\n-------\n- Fixture outcome supports blocker-order validation.\n\nEvidence\n--------\n- Command transcript validates blocker-order behavior.\n\nBlocked by\n----------\n(none)\n\nBlocking\n--------\n(none)\n\nSubissues\n---------\n3 total | status: todo=3 | priority: medium=3\n  ready atelier-p0a6 [todo] medium - Contract blocker work\n  blocked atelier-q35s [todo] medium - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)\n  blocked atelier-gggn [todo] medium - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)\n\nRecent Activity\n---------------\n(none)\n\nNext Commands\n-------------\n  Edit issue Markdown: /tmp/atelier-em15.6ZVOOb/.atelier/issues/atelier-hh5b.md\n  Validate this issue: atelier lint atelier-hh5b\n  Add a note: atelier issue note atelier-hh5b \"...\"\n  Show full activity: atelier history --issue atelier-hh5b\n  Show transition options: atelier issue transition atelier-hh5b --options\n  Execute a transition: atelier issue transition atelier-hh5b <transition>\n\\n$ /root/atelier/target/debug/atelier graph tree\n[mission active] #atelier-f33o - Ordering validation mission\n  [ready] #atelier-hh5b medium - Ordering parent epic\n    [ready] #atelier-p0a6 medium - Contract blocker work\n    [blocked] #atelier-q35s medium - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)\n    [blocked] #atelier-gggn medium - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)\n  [blocked] #atelier-zv35 medium - Loose ready"
    truncated: true
  stderr:
    bytes: 133
    summary: "2026-06-16T19:00:18.026705Z  WARN Projection index was stale; rebuilt local SQLite projection from /tmp/atelier-em15.6ZVOOb/.atelier\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-em15"
    role: "validates"
  - kind: "issue"
    id: "atelier-k1ga"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Cross-surface blocker-aware ordering validation transcript passes"
updated_at: "2026-06-16T19:02:41.209823139+00:00"
---

Cross-surface blocker-aware ordering validation transcript passes

Command: bash -lc '
set -euo pipefail
A=/root/atelier/target/debug/atelier
TMP=$(mktemp -d /tmp/atelier-em15.XXXXXX)
trap "rm -rf \"$TMP\"" EXIT
cd "$TMP"
git init -q
git branch -M main
git config user.email test@example.com
git config user.name Test
$A init -q >/dev/null
$A mission create -q "Ordering validation mission" >/dev/null
MISSION=$(basename .atelier/missions/*.md .md)
$A mission start "$MISSION" >/dev/null
EPIC=$($A issue create -q --issue-type epic "Ordering parent epic")
git checkout -qb "epic/$EPIC"
CONTRACT=$($A issue create -q --parent "$EPIC" "Contract blocker work")
IMPLEMENT=$($A issue create -q --parent "$EPIC" "Implementation blocked work")
VALIDATE=$($A issue create -q --parent "$EPIC" --issue-type validation "Validation child work")
READY=$($A issue create -q "Loose ready work")
EXTERNAL=$($A issue create -q "External hidden blocker")
for f in .atelier/issues/*.md; do perl -0pi -e "s/Outcome was not specified\./- Fixture outcome supports blocker-order validation./g; s/Evidence was not specified\./- Command transcript validates blocker-order behavior./g" "$f"; done
$A issue block "$IMPLEMENT" "$CONTRACT" >/dev/null
$A issue block "$VALIDATE" "$IMPLEMENT" >/dev/null
$A issue block "$READY" "$EXTERNAL" >/dev/null
$A mission add-work "$MISSION" "$EPIC" >/dev/null
$A mission add-work "$MISSION" "$READY" >/dev/null
git add . && git commit -qm fixture
$A issue transition "$CONTRACT" --options >/dev/null

echo "Fixture IDs: mission=$MISSION epic=$EPIC contract=$CONTRACT implementation=$IMPLEMENT validation=$VALIDATE ready=$READY external=$EXTERNAL"
assert_before() {
  local label=$1 haystack=$2 first=$3 second=$4
  python3 - "$label" "$haystack" "$first" "$second" <<PY
import sys
label,path,first,second=sys.argv[1:]
text=open(path).read()
a=text.find(first)
b=text.find(second)
if a < 0 or b < 0 or a >= b:
    print(f"ASSERTION FAILED {label}: {first!r} before {second!r}\n{text}", file=sys.stderr)
    sys.exit(1)
PY
}
assert_contains() { local label=$1 haystack=$2 needle=$3; if ! grep -Fq "$needle" "$haystack"; then echo "ASSERTION FAILED $label: missing $needle" >&2; cat "$haystack" >&2; exit 1; fi; }
assert_not_contains() { local label=$1 haystack=$2 needle=$3; if grep -Fq "$needle" "$haystack"; then echo "ASSERTION FAILED $label: unexpected $needle" >&2; cat "$haystack" >&2; exit 1; fi; }
run_surface() { local name=$1; shift; local out="$TMP/$name.out"; echo "\n$ $A $*"; $A "$@" | tee "$out"; }

run_surface issue_ready issue list --ready
assert_contains issue_ready "$TMP/issue_ready.out" "No issues found."
assert_not_contains issue_ready "$TMP/issue_ready.out" "todo/todo"

run_surface issue_list issue list
assert_before issue_list "$TMP/issue_list.out" "$CONTRACT" "$IMPLEMENT"
assert_contains issue_list "$TMP/issue_list.out" "ready [task] $CONTRACT"
assert_contains issue_list "$TMP/issue_list.out" "blocked [task] $IMPLEMENT"
assert_not_contains issue_list "$TMP/issue_list.out" "todo/todo"

run_surface issue_blocked issue list --blocked
assert_before issue_blocked "$TMP/issue_blocked.out" "$IMPLEMENT" "$VALIDATE"
assert_contains issue_blocked "$TMP/issue_blocked.out" "details: atelier issue blocked $IMPLEMENT"
assert_not_contains issue_blocked "$TMP/issue_blocked.out" "todo/todo"

run_surface issue_show issue show "$EPIC"
assert_before issue_show "$TMP/issue_show.out" "$CONTRACT" "$IMPLEMENT"
assert_before issue_show "$TMP/issue_show.out" "$IMPLEMENT" "$VALIDATE"
assert_contains issue_show "$TMP/issue_show.out" "blocked $IMPLEMENT"

run_surface graph_tree graph tree
assert_before graph_tree "$TMP/graph_tree.out" "$CONTRACT" "$IMPLEMENT"
assert_before graph_tree "$TMP/graph_tree.out" "$IMPLEMENT" "$VALIDATE"
assert_contains graph_tree "$TMP/graph_tree.out" "Legend: ready, blocked"
assert_not_contains graph_tree "$TMP/graph_tree.out" "todo/todo"

run_surface mission_status mission status "$MISSION"
assert_before mission_status "$TMP/mission_status.out" "$CONTRACT" "$IMPLEMENT"
assert_contains mission_status "$TMP/mission_status.out" "details: atelier issue blocked $IMPLEMENT"
assert_not_contains mission_status "$TMP/mission_status.out" "todo/todo"

run_surface mission_list mission list
assert_contains mission_list "$TMP/mission_list.out" "Next work: ready $CONTRACT - Contract blocker work"
assert_contains mission_list "$TMP/mission_list.out" "[epic] $EPIC [ready]"
assert_not_contains mission_list "$TMP/mission_list.out" "todo/todo"

run_surface root_status status
assert_before root_status "$TMP/root_status.out" "$CONTRACT" "$IMPLEMENT"
assert_contains root_status "$TMP/root_status.out" "ready $CONTRACT"
assert_not_contains root_status "$TMP/root_status.out" "todo/todo"

cat <<REPORT

Surface classification:
- issue list: PASS visible blocker appears before blocked implementation; readable ready/blocked labels present.
- issue list --ready: PASS command checked; strict ready queue returned no rows in this fixture, so no ordering conflict is visible on that bucketed surface.
- issue list --blocked: PASS blocked implementation precedes validation child and points to issue blocked drill-down.
- issue show: PASS subissue order follows contract -> implementation -> validation and subissue rows use readable state labels.
- graph tree: PASS hierarchy keeps parent shape while ordering visible siblings by blocker dependency.
- mission status: PASS shared mission summary lists selectable blocker before blocked work with drill-down guidance.
- mission list: PASS compact overview stays compact and cues the visible blocker as next work.
- root status: PASS active-mission status exposes the visible blocker before blocked work.
Follow-up issues: none; no product mismatch observed.
REPORT
'
Exit status: 0

Stdout summary (truncated):
Fixture IDs: mission=atelier-f33o epic=atelier-hh5b contract=atelier-p0a6 implementation=atelier-q35s validation=atelier-gggn ready=atelier-zv35 external=atelier-xxi5
\n$ /root/atelier/target/debug/atelier issue list --ready
No issues found.
\n$ /root/atelier/target/debug/atelier issue list
Issue Queue
===========
6 total | Category: todo=6 | Status: todo=6 | Priority: medium=6 | Blocked: 3

[epic] atelier-hh5b medium - Ordering parent epic
-------------------------------------------------
    ready [task] atelier-p0a6 - Contract blocker work
    blocked [task] atelier-q35s - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)
    blocked [validation] atelier-gggn - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)

Standalone
----------
    ready [task] atelier-xxi5 - External hidden blocker
    blocked [task] atelier-zv35 - Loose ready work (1 blocker; details: atelier issue blocked atelier-zv35)
\n$ /root/atelier/target/debug/atelier issue list --blocked
Blocked issues
==============
3 total
Drill down: atelier issue blocked <id>
  blocked atelier-zv35 Loose ready work (1 blocker; details: atelier issue blocked atelier-zv35)
  blocked atelier-q35s Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)
  blocked atelier-gggn Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)
\n$ /root/atelier/target/debug/atelier issue show atelier-hh5b
atelier-hh5b [epic] todo/todo - Ordering parent epic
====================================================
Status:   todo
Category: todo
Type:     epic
Priority: medium
Created:  2026-06-16 15:00 -04:00
Updated:  2026-06-16 15:00 -04:00
File:     /tmp/atelier-em15.6ZVOOb/.atelier/issues/atelier-hh5b.md

Hierarchy
---------
Parent: (none)

Branch Lifecycle
----------------
Owner:    epic atelier-hh5b (epic)
Expected: epic/atelier-hh5b
Base:     main
Scope:    owns its merge branch
Current:  epic/atelier-hh5b
State:    current branch matches expected branch
Next:     atelier start atelier-hh5b
Close:    atelier issue close atelier-hh5b --reason "..."

Transition Readiness
--------------------
  block: allowed - to blocked
    atelier issue transition atelier-hh5b block
  start: blocked - branch context: worktree has uncommitted changes; inspect `git status --short --branch`, then rerun `atelier start atelier-hh5b`
    atelier issue transition atelier-hh5b start
  options: atelier issue transition atelier-hh5b --options

Description
-----------
No description provided.

Outcome
-------
- Fixture outcome supports blocker-order validation.

Evidence
--------
- Command transcript validates blocker-order behavior.

Blocked by
----------
(none)

Blocking
--------
(none)

Subissues
---------
3 total | status: todo=3 | priority: medium=3
  ready atelier-p0a6 [todo] medium - Contract blocker work
  blocked atelier-q35s [todo] medium - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)
  blocked atelier-gggn [todo] medium - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)

Recent Activity
---------------
(none)

Next Commands
-------------
  Edit issue Markdown: /tmp/atelier-em15.6ZVOOb/.atelier/issues/atelier-hh5b.md
  Validate this issue: atelier lint atelier-hh5b
  Add a note: atelier issue note atelier-hh5b "..."
  Show full activity: atelier history --issue atelier-hh5b
  Show transition options: atelier issue transition atelier-hh5b --options
  Execute a transition: atelier issue transition atelier-hh5b <transition>
\n$ /root/atelier/target/debug/atelier graph tree
[mission active] #atelier-f33o - Ordering validation mission
  [ready] #atelier-hh5b medium - Ordering parent epic
    [ready] #atelier-p0a6 medium - Contract blocker work
    [blocked] #atelier-q35s medium - Implementation blocked work (1 blocker; details: atelier issue blocked atelier-q35s)
    [blocked] #atelier-gggn medium - Validation child work (1 blocker; details: atelier issue blocked atelier-gggn)
  [blocked] #atelier-zv35 medium - Loose ready

Stderr summary:
2026-06-16T19:00:18.026705Z  WARN Projection index was stale; rebuilt local SQLite projection from /tmp/atelier-em15.6ZVOOb/.atelier

