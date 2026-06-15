---
created_at: "2026-06-15T04:27:04.652422045+00:00"
id: "atelier-znan"
evidence_type: "test"
captured_at: "2026-06-15T04:27:00.673396022+00:00"
command: "bash -lc '\nset -euo pipefail\nBIN=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d)\ntrap \"rm -rf \\\"$TMP\\\"\" EXIT\nset_body() {\n  id=$1; outcome=$2; evidence=$3\n  python3 - \"$id\" \"$outcome\" \"$evidence\" <<PY\nfrom pathlib import Path\nimport sys\nid,outcome,evidence=sys.argv[1:]\np=Path(\".atelier/issues\")/f\"{id}.md\"\ns=p.read_text()\ns=s.split(\"## Outcome\",1)[0]+f\"## Outcome\\n\\n- {outcome}\\n\\n## Evidence\\n\\n- {evidence}\\n\"\np.write_text(s)\nPY\n}\ncd \"$TMP\"\ngit init -q\ngit config user.email test@example.com\ngit config user.name Test\n$BIN init -q >/dev/null\ngit add . && git commit -qm init\nEPIC=$($BIN issue create -q --issue-type epic \"Review gate epic\")\nTASK=$($BIN issue create -q --issue-type task --parent \"$EPIC\" \"Local proof task\")\nBUG=$($BIN issue create -q --issue-type bug --parent \"$EPIC\" \"Local proof bug\")\nFEATURE=$($BIN issue create -q --issue-type feature --parent \"$EPIC\" \"Local proof feature\")\nset_body \"$EPIC\" \"Epic closes after review, validation, and child proof.\" \"Command transcript proves epic close rejection before review and success after validation plus child proof.\"\nset_body \"$TASK\" \"Task closes with local proof and no review transition.\" \"Evidence record proves task local proof before close.\"\nset_body \"$BUG\" \"Bug closes with local proof and no review transition.\" \"Evidence record proves bug local proof before close.\"\nset_body \"$FEATURE\" \"Feature closes with local proof and no review transition.\" \"Evidence record proves feature local proof before close.\"\n$BIN start \"$TASK\" >/dev/null\n$BIN evidence record -q --target issue/$TASK --kind test --result pass \"task local proof\" >/dev/null\n$BIN issue close -q \"$TASK\" --reason \"task closes with local proof\"\necho \"task local-proof close passed\"\n$BIN start \"$BUG\" >/dev/null\n$BIN evidence record -q --target issue/$BUG --kind test --result pass \"bug local proof\" >/dev/null\n$BIN issue close -q \"$BUG\" --reason \"bug closes with local proof\"\necho \"bug local-proof close passed\"\n$BIN start \"$FEATURE\" >/dev/null\n$BIN evidence record -q --target issue/$FEATURE --kind test --result pass \"feature local proof\" >/dev/null\n$BIN issue close -q \"$FEATURE\" --reason \"feature closes with local proof\"\necho \"feature local-proof close passed\"\nBAD_EPIC=$($BIN issue create -q --issue-type epic \"Blocked child-proof epic\")\nBAD_CHILD=$($BIN issue create -q --issue-type task --parent \"$BAD_EPIC\" \"Open child\")\nset_body \"$BAD_EPIC\" \"Epic close is blocked by missing child proof.\" \"Command transcript proves epic child-proof close rejection.\"\nset_body \"$BAD_CHILD\" \"Child remains open.\" \"Manual check confirms child is intentionally open.\"\n$BIN start \"$BAD_EPIC\" >/dev/null\n$BIN evidence record -q --target issue/$BAD_EPIC --kind review --result pass \"bad epic review evidence\" >/dev/null\n$BIN issue transition \"$BAD_EPIC\" request_review >/dev/null\n$BIN issue transition \"$BAD_EPIC\" request_validation >/dev/null\ngit add . && git commit -qm before-bad-close\nif $BIN issue close -q \"$BAD_EPIC\" --reason \"should fail\"; then\n  echo \"ERROR: bad epic close unexpectedly succeeded\" >&2\n  exit 1\nelse\n  echo \"bad epic close rejected because child proof is incomplete\"\nfi\n$BIN abandon \"$BAD_EPIC\" --reason \"continue workflow proof\" >/dev/null\n$BIN start \"$EPIC\" >/dev/null\n$BIN evidence record -q --target issue/$EPIC --kind review --result pass \"epic review evidence\" >/dev/null\nif $BIN issue close -q \"$EPIC\" --reason \"should fail before review\"; then\n  echo \"ERROR: epic close before review unexpectedly succeeded\" >&2\n  exit 1\nelse\n  echo \"epic close rejected before review/validation\"\nfi\n$BIN issue transition \"$EPIC\" request_review >/dev/null\n$BIN issue transition \"$EPIC\" request_validation >/dev/null\ngit add . && git commit -qm before-good-close\n$BIN issue close \"$EPIC\" --reason \"epic closes after review validation and child proof\" | rg \"Applied transition|To:\"\n$BIN issue show \"$EPIC\" | rg \"Status:|epic closes after review\"\n'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-11gp"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 1788
    summary: "Lint passed.\nApplied transition close to atelier-kbhx\nFrom:     in_progress\nTo:       done\nNext Commands\n-------------\n  atelier issue show atelier-kbhx\n  atelier issue transition atelier-kbhx --options\ntask local-proof close passed\nLint passed.\nApplied transition close to atelier-imvf\nFrom:     in_progress\nTo:       done\nNext Commands\n-------------\n  atelier issue show atelier-imvf\n  atelier issue transition atelier-imvf --options\nbug local-proof close passed\nLint passed.\nApplied transition close to atelier-ij5f\nFrom:     in_progress\nTo:       done\nNext Commands\n-------------\n  atelier issue show atelier-ij5f\n  atelier issue transition atelier-ij5f --options\nfeature local-proof close passed\nLint passed.\nIssue Transition atelier-cg7n - Blocked child-proof epic\n========================================================\nTransition: close\nFrom:       validation\nTo:         done\nCommand:    atelier issue transition atelier-cg7n close --reason \"...\"\nValidators\n----------\n  pass  proof_attached\n      passing validating evidence is linked\n  fail  epic_child_proof\n      epic child proof incomplete: atelier-hnec open\n  pass  blockers_clear\n      no open blockers\n  pass  lint_clear\n      lint passed\n  pass  durable_current\n      canonical export is current\n  pass  closeout_clean\n      git worktree is clean\nBlockers\n--------\n  validator epic_child_proof failed: epic child proof incomplete: atelier-hnec open\nGuidance\n--------\n  Closing atelier-cg7n requires attached evidence and no open blockers.\nbad epic close rejected because child proof is incomplete\nepic close rejected before review/validation\nApplied transition close to atelier-5nrp\nTo:       done\nStatus:   done\nepic closes after review validation and child proof\n  epic closes after review validation and child proof\n"
    truncated: false
  stderr:
    bytes: 3676
    summary: "Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRebuilt /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nProjection index was stale; rebuilt local SQLite projection from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRebuilt /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nProjection index was stale; rebuilt local SQLite projection from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nError: Transition 'close' is blocked for issue atelier-cg7n: validator epic_child_proof failed: epic child proof incomplete: atelier-hnec open\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nError: Issue atelier-5nrp has no terminal done-category transitions from status 'in_progress'; inspect `atelier issue transition atelier-5nrp --options`\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\nRefreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier\n"
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-11gp"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "bash -lc '\nset -euo pipefail\nBIN=/root/atelier/target/debug/atelier\nTMP=$(mktemp -d)\ntrap \"rm -rf \\\"$TMP\\\"\" EXIT\nset_body() {\n  id=$1; outcome=$2; evidence=$3\n  python3 - \"$id\" \"$outcome\" \"$evidence\" <<PY\nfrom pathlib import Path\nimport sys\nid,outcome,evidence=sys.argv[1:]\np=Path(\".atelier/issues\")/f\"{id}.md\"\ns=p.read_text()\ns=s.split(\"## Outcome\",1)[0]+f\"## Outcome\\n\\n- {outcome}\\n\\n## Evidence\\n\\n- {evidence}\\n\"\np.write_text(s)\nPY\n}\ncd \"$TMP\"\ngit init -q\ngit config user.email test@example.com\ngit config user.name Test\n$BIN init -q >/dev/null\ngit add . && git commit -qm init\nEPIC=$($BIN issue create -q --issue-type epic \"Review gate epic\")\nTASK=$($BIN issue create -q --issue-type task --parent \"$EPIC\" \"Local proof task\")\nBUG=$($BIN issue create -q --issue-type bug --parent \"$EPIC\" \"Local proof bug\")\nFEATURE=$($BIN issue create -q --issue-type feature --parent \"$EPIC\" \"Local proof feature\")\nset_body \"$EPIC\" \"Epic closes after review, validation, and child proof.\" \"Command transcript proves epic close rejection before review and success after validation plus child proof.\"\nset_body \"$TASK\" \"Task closes with local proof and no review transition.\" \"Evidence record proves task local proof before close.\"\nset_body \"$BUG\" \"Bug closes with local proof and no review transition.\" \"Evidence record proves bug local proof before close.\"\nset_body \"$FEATURE\" \"Feature closes with local proof and no review transition.\" \"Evidence record proves feature local proof before close.\"\n$BIN start \"$TASK\" >/dev/null\n$BIN evidence record -q --target issue/$TASK --kind test --result pass \"task local proof\" >/dev/null\n$BIN issue close -q \"$TASK\" --reason \"task closes with local proof\"\necho \"task local-proof close passed\"\n$BIN start \"$BUG\" >/dev/null\n$BIN evidence record -q --target issue/$BUG --kind test --result pass \"bug local proof\" >/dev/null\n$BIN issue close -q \"$BUG\" --reason \"bug closes with local proof\"\necho \"bug local-proof close passed\"\n$BIN start \"$FEATURE\" >/dev/null\n$BIN evidence record -q --target issue/$FEATURE --kind test --result pass \"feature local proof\" >/dev/null\n$BIN issue close -q \"$FEATURE\" --reason \"feature closes with local proof\"\necho \"feature local-proof close passed\"\nBAD_EPIC=$($BIN issue create -q --issue-type epic \"Blocked child-proof epic\")\nBAD_CHILD=$($BIN issue create -q --issue-type task --parent \"$BAD_EPIC\" \"Open child\")\nset_body \"$BAD_EPIC\" \"Epic close is blocked by missing child proof.\" \"Command transcript proves epic child-proof close rejection.\"\nset_body \"$BAD_CHILD\" \"Child remains open.\" \"Manual check confirms child is intentionally open.\"\n$BIN start \"$BAD_EPIC\" >/dev/null\n$BIN evidence record -q --target issue/$BAD_EPIC --kind review --result pass \"bad epic review evidence\" >/dev/null\n$BIN issue transition \"$BAD_EPIC\" request_review >/dev/null\n$BIN issue transition \"$BAD_EPIC\" request_validation >/dev/null\ngit add . && git commit -qm before-bad-close\nif $BIN issue close -q \"$BAD_EPIC\" --reason \"should fail\"; then\n  echo \"ERROR: bad epic close unexpectedly succeeded\" >&2\n  exit 1\nelse\n  echo \"bad epic close rejected because child proof is incomplete\"\nfi\n$BIN abandon \"$BAD_EPIC\" --reason \"continue workflow proof\" >/dev/null\n$BIN start \"$EPIC\" >/dev/null\n$BIN evidence record -q --target issue/$EPIC --kind review --result pass \"epic review evidence\" >/dev/null\nif $BIN issue close -q \"$EPIC\" --reason \"should fail before review\"; then\n  echo \"ERROR: epic close before review unexpectedly succeeded\" >&2\n  exit 1\nelse\n  echo \"epic close rejected before review/validation\"\nfi\n$BIN issue transition \"$EPIC\" request_review >/dev/null\n$BIN issue transition \"$EPIC\" request_validation >/dev/null\ngit add . && git commit -qm before-good-close\n$BIN issue close \"$EPIC\" --reason \"epic closes after review validation and child proof\" | rg \"Applied transition|To:\"\n$BIN issue show \"$EPIC\" | rg \"Status:|epic closes after review\"\n'"
updated_at: "2026-06-15T04:27:06.404229121+00:00"
---

bash -lc '
set -euo pipefail
BIN=/root/atelier/target/debug/atelier
TMP=$(mktemp -d)
trap "rm -rf \"$TMP\"" EXIT
set_body() {
  id=$1; outcome=$2; evidence=$3
  python3 - "$id" "$outcome" "$evidence" <<PY
from pathlib import Path
import sys
id,outcome,evidence=sys.argv[1:]
p=Path(".atelier/issues")/f"{id}.md"
s=p.read_text()
s=s.split("## Outcome",1)[0]+f"## Outcome\n\n- {outcome}\n\n## Evidence\n\n- {evidence}\n"
p.write_text(s)
PY
}
cd "$TMP"
git init -q
git config user.email test@example.com
git config user.name Test
$BIN init -q >/dev/null
git add . && git commit -qm init
EPIC=$($BIN issue create -q --issue-type epic "Review gate epic")
TASK=$($BIN issue create -q --issue-type task --parent "$EPIC" "Local proof task")
BUG=$($BIN issue create -q --issue-type bug --parent "$EPIC" "Local proof bug")
FEATURE=$($BIN issue create -q --issue-type feature --parent "$EPIC" "Local proof feature")
set_body "$EPIC" "Epic closes after review, validation, and child proof." "Command transcript proves epic close rejection before review and success after validation plus child proof."
set_body "$TASK" "Task closes with local proof and no review transition." "Evidence record proves task local proof before close."
set_body "$BUG" "Bug closes with local proof and no review transition." "Evidence record proves bug local proof before close."
set_body "$FEATURE" "Feature closes with local proof and no review transition." "Evidence record proves feature local proof before close."
$BIN start "$TASK" >/dev/null
$BIN evidence record -q --target issue/$TASK --kind test --result pass "task local proof" >/dev/null
$BIN issue close -q "$TASK" --reason "task closes with local proof"
echo "task local-proof close passed"
$BIN start "$BUG" >/dev/null
$BIN evidence record -q --target issue/$BUG --kind test --result pass "bug local proof" >/dev/null
$BIN issue close -q "$BUG" --reason "bug closes with local proof"
echo "bug local-proof close passed"
$BIN start "$FEATURE" >/dev/null
$BIN evidence record -q --target issue/$FEATURE --kind test --result pass "feature local proof" >/dev/null
$BIN issue close -q "$FEATURE" --reason "feature closes with local proof"
echo "feature local-proof close passed"
BAD_EPIC=$($BIN issue create -q --issue-type epic "Blocked child-proof epic")
BAD_CHILD=$($BIN issue create -q --issue-type task --parent "$BAD_EPIC" "Open child")
set_body "$BAD_EPIC" "Epic close is blocked by missing child proof." "Command transcript proves epic child-proof close rejection."
set_body "$BAD_CHILD" "Child remains open." "Manual check confirms child is intentionally open."
$BIN start "$BAD_EPIC" >/dev/null
$BIN evidence record -q --target issue/$BAD_EPIC --kind review --result pass "bad epic review evidence" >/dev/null
$BIN issue transition "$BAD_EPIC" request_review >/dev/null
$BIN issue transition "$BAD_EPIC" request_validation >/dev/null
git add . && git commit -qm before-bad-close
if $BIN issue close -q "$BAD_EPIC" --reason "should fail"; then
  echo "ERROR: bad epic close unexpectedly succeeded" >&2
  exit 1
else
  echo "bad epic close rejected because child proof is incomplete"
fi
$BIN abandon "$BAD_EPIC" --reason "continue workflow proof" >/dev/null
$BIN start "$EPIC" >/dev/null
$BIN evidence record -q --target issue/$EPIC --kind review --result pass "epic review evidence" >/dev/null
if $BIN issue close -q "$EPIC" --reason "should fail before review"; then
  echo "ERROR: epic close before review unexpectedly succeeded" >&2
  exit 1
else
  echo "epic close rejected before review/validation"
fi
$BIN issue transition "$EPIC" request_review >/dev/null
$BIN issue transition "$EPIC" request_validation >/dev/null
git add . && git commit -qm before-good-close
$BIN issue close "$EPIC" --reason "epic closes after review validation and child proof" | rg "Applied transition|To:"
$BIN issue show "$EPIC" | rg "Status:|epic closes after review"
'

Command: bash -lc '
set -euo pipefail
BIN=/root/atelier/target/debug/atelier
TMP=$(mktemp -d)
trap "rm -rf \"$TMP\"" EXIT
set_body() {
  id=$1; outcome=$2; evidence=$3
  python3 - "$id" "$outcome" "$evidence" <<PY
from pathlib import Path
import sys
id,outcome,evidence=sys.argv[1:]
p=Path(".atelier/issues")/f"{id}.md"
s=p.read_text()
s=s.split("## Outcome",1)[0]+f"## Outcome\n\n- {outcome}\n\n## Evidence\n\n- {evidence}\n"
p.write_text(s)
PY
}
cd "$TMP"
git init -q
git config user.email test@example.com
git config user.name Test
$BIN init -q >/dev/null
git add . && git commit -qm init
EPIC=$($BIN issue create -q --issue-type epic "Review gate epic")
TASK=$($BIN issue create -q --issue-type task --parent "$EPIC" "Local proof task")
BUG=$($BIN issue create -q --issue-type bug --parent "$EPIC" "Local proof bug")
FEATURE=$($BIN issue create -q --issue-type feature --parent "$EPIC" "Local proof feature")
set_body "$EPIC" "Epic closes after review, validation, and child proof." "Command transcript proves epic close rejection before review and success after validation plus child proof."
set_body "$TASK" "Task closes with local proof and no review transition." "Evidence record proves task local proof before close."
set_body "$BUG" "Bug closes with local proof and no review transition." "Evidence record proves bug local proof before close."
set_body "$FEATURE" "Feature closes with local proof and no review transition." "Evidence record proves feature local proof before close."
$BIN start "$TASK" >/dev/null
$BIN evidence record -q --target issue/$TASK --kind test --result pass "task local proof" >/dev/null
$BIN issue close -q "$TASK" --reason "task closes with local proof"
echo "task local-proof close passed"
$BIN start "$BUG" >/dev/null
$BIN evidence record -q --target issue/$BUG --kind test --result pass "bug local proof" >/dev/null
$BIN issue close -q "$BUG" --reason "bug closes with local proof"
echo "bug local-proof close passed"
$BIN start "$FEATURE" >/dev/null
$BIN evidence record -q --target issue/$FEATURE --kind test --result pass "feature local proof" >/dev/null
$BIN issue close -q "$FEATURE" --reason "feature closes with local proof"
echo "feature local-proof close passed"
BAD_EPIC=$($BIN issue create -q --issue-type epic "Blocked child-proof epic")
BAD_CHILD=$($BIN issue create -q --issue-type task --parent "$BAD_EPIC" "Open child")
set_body "$BAD_EPIC" "Epic close is blocked by missing child proof." "Command transcript proves epic child-proof close rejection."
set_body "$BAD_CHILD" "Child remains open." "Manual check confirms child is intentionally open."
$BIN start "$BAD_EPIC" >/dev/null
$BIN evidence record -q --target issue/$BAD_EPIC --kind review --result pass "bad epic review evidence" >/dev/null
$BIN issue transition "$BAD_EPIC" request_review >/dev/null
$BIN issue transition "$BAD_EPIC" request_validation >/dev/null
git add . && git commit -qm before-bad-close
if $BIN issue close -q "$BAD_EPIC" --reason "should fail"; then
  echo "ERROR: bad epic close unexpectedly succeeded" >&2
  exit 1
else
  echo "bad epic close rejected because child proof is incomplete"
fi
$BIN abandon "$BAD_EPIC" --reason "continue workflow proof" >/dev/null
$BIN start "$EPIC" >/dev/null
$BIN evidence record -q --target issue/$EPIC --kind review --result pass "epic review evidence" >/dev/null
if $BIN issue close -q "$EPIC" --reason "should fail before review"; then
  echo "ERROR: epic close before review unexpectedly succeeded" >&2
  exit 1
else
  echo "epic close rejected before review/validation"
fi
$BIN issue transition "$EPIC" request_review >/dev/null
$BIN issue transition "$EPIC" request_validation >/dev/null
git add . && git commit -qm before-good-close
$BIN issue close "$EPIC" --reason "epic closes after review validation and child proof" | rg "Applied transition|To:"
$BIN issue show "$EPIC" | rg "Status:|epic closes after review"
'
Exit status: 0

Stdout summary:
Lint passed.
Applied transition close to atelier-kbhx
From:     in_progress
To:       done
Next Commands
-------------
  atelier issue show atelier-kbhx
  atelier issue transition atelier-kbhx --options
task local-proof close passed
Lint passed.
Applied transition close to atelier-imvf
From:     in_progress
To:       done
Next Commands
-------------
  atelier issue show atelier-imvf
  atelier issue transition atelier-imvf --options
bug local-proof close passed
Lint passed.
Applied transition close to atelier-ij5f
From:     in_progress
To:       done
Next Commands
-------------
  atelier issue show atelier-ij5f
  atelier issue transition atelier-ij5f --options
feature local-proof close passed
Lint passed.
Issue Transition atelier-cg7n - Blocked child-proof epic
========================================================
Transition: close
From:       validation
To:         done
Command:    atelier issue transition atelier-cg7n close --reason "..."
Validators
----------
  pass  proof_attached
      passing validating evidence is linked
  fail  epic_child_proof
      epic child proof incomplete: atelier-hnec open
  pass  blockers_clear
      no open blockers
  pass  lint_clear
      lint passed
  pass  durable_current
      canonical export is current
  pass  closeout_clean
      git worktree is clean
Blockers
--------
  validator epic_child_proof failed: epic child proof incomplete: atelier-hnec open
Guidance
--------
  Closing atelier-cg7n requires attached evidence and no open blockers.
bad epic close rejected because child proof is incomplete
epic close rejected before review/validation
Applied transition close to atelier-5nrp
To:       done
Status:   done
epic closes after review validation and child proof
  epic closes after review validation and child proof

Stderr summary:
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Rebuilt /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Projection index was stale; rebuilt local SQLite projection from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Rebuilt /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Projection index was stale; rebuilt local SQLite projection from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Error: Transition 'close' is blocked for issue atelier-cg7n: validator epic_child_proof failed: epic child proof incomplete: atelier-hnec open
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Error: Issue atelier-5nrp has no terminal done-category transitions from status 'in_progress'; inspect `atelier issue transition atelier-5nrp --options`
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier
Refreshed projection in /tmp/tmp.g4xBK4RhdM/.atelier/runtime/state.db from /tmp/tmp.g4xBK4RhdM/.atelier

