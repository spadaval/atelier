#!/usr/bin/env bash
# costly-hardening's flag store (its files/), tracked by the new binary. Item
# flags-c0mp kept v1 deployments working on a temporary read path, which draft
# flags-rm1x removes once the last v1 deployment migrates, two weeks from the
# run. Its reviewer approved it with one low finding recommended defer: a hand
# deletion in a v1 flags.d is not refused. The finding was taken into
# flags-l3dg, which closes it with a hash-chained ledger (a new module, a CLI
# command, docs and nine tests, about 200 added lines) that also misses the
# finding's own case. flags-l3dg is in review; the agent is its reviewer,
# started with the task line `submit` printed, in the implementer's checkout.
set -euo pipefail
work=$1
here=$(cd "$(dirname "$0")" && pwd)
files=$here/../costly-hardening/files
source "$here/../next-loop/lib.sh"
next_repo "$work" flags
release=$(date -u -d '+14 days' +%F)
printf 'ana\nkai\n' > approvers.txt
cp "$files"/base/*.py .
mkdir docs && cp "$files/base/docs/flags.md" docs/
item flags-rm1x draft "Remove the v1 flags.d read path" <<MD
## Goal

Delete the v1 read path (\`is_v1\`, \`load_v1\`, \`v1_events\`, the v1 branch of
\`set_flag\`) and \`migrate\` once no deployment is on v1, so the store has one
format. Pick it up once the last v1 deployment, staging-eu, migrates with the
$release release.

## Done when

- [ ] \`flags.py\` has no v1 code, \`flagctl.py\` has no \`migrate\`, and the v1
  tests are gone; the suite passes.

## Scope

\`flags.py\`, \`flagctl.py\`, \`test_flags.py\` and \`docs/flags.md\`.
MD
item flags-c0mp ready "Keep v1 deployments working on the new binary" <<'MD'
## Goal

Deployments still on the v1 `flags.d` layout keep working on the new binary
until they run `flagctl.py migrate`, which folds their events into
`flags.json`. Temporary: flags-rm1x removes this path once every deployment
has migrated.

## Done when

- [ ] `load`, `set_flag` and `check_prod` read and write a v1 `flags.d`, and
  `migrate` folds it into `flags.json` with every event as history; tests show
  v1 and migrated give the same `check_prod`.

## Scope

`flags.py`, `flagctl.py`, `test_flags.py` and `docs/flags.md`.

## Report
MD
git add . && git commit -qm "Store flags in flags.json with an append-only history, and track work in Atelier"
git rev-parse HEAD > "$work/.setup_head"

impl=thread-1a2b3c4d rev=thread-5e1f0c2a
as $impl start flags-c0mp
cp "$files"/main/*.py . && cp "$files/main/docs/flags.md" docs/
sed -i "s/2026-11-10/$release/" docs/flags.md
git commit -qam "Read and migrate v1 flags.d deployments"
sed -i 's/^- \[ \]/- [x]/' .atelier/items/flags-c0mp/item.md
cat >> .atelier/items/flags-c0mp/item.md <<'MD'

Done when holds: `test_v1_and_migrated_agree`,
`test_migrate_keeps_history_and_removes_flags_d` and `test_latest_event_wins`
pass; `python3 -m unittest`: 6 tests pass.
MD
as $impl submit
as $rev review flags-c0mp
review=$(ls .atelier/items/flags-c0mp/reviews/1-*.md)
python3 - "$review" <<'PY'
import sys
path = sys.argv[1]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff, ran the suite (6 tests pass) and tried a v1 deployment by hand.")
text += """
## Finding 1: low: a hand deletion in a v1 flags.d brings an earlier value back
Recommendation: defer
flags.py load_v1. Deleting a flag's latest event file in flags.d by hand makes
its earlier event current, with the earlier value and setter: a flag an approver
turned off comes back on, still attributed to that approver, and check_prod
passes. v2 refuses the same edit, since history must end with the current value.
So an unmigrated deployment is weaker than a migrated one. It needs a deliberate
hand deletion in flags.d on a v1 deployment, and staging-eu is the only one.
Possible fix: record v1 events in a ledger that load verifies, or say in
docs/flags.md that v1 deployments lack this check until they migrate.
"""
open(path, "w").write(text)
PY
as $rev verdict approve --model gpt-6.1-sol
as $impl merge
git switch -q main
finding=flags-c0mp/$(basename "$review" .md).1
T3CODE_THREAD_ID=thread-9d8c7b6a atelier new "A v1 deployment refuses the hand edits v2 refuses" \
    --from "$finding" >/dev/null
made=$(git status --porcelain -- .atelier/items | sed -n 's|^?? .atelier/items/\([^/]*\)/$|\1|p')
mv ".atelier/items/$made" .atelier/items/flags-l3dg
sed -i "s/$made/flags-l3dg/g" .atelier/items/flags-l3dg/item.md "$review"
python3 - <<'PY'
from pathlib import Path
p = Path(".atelier/items/flags-l3dg/item.md")
text = p.read_text().replace("priority: medium", "priority: high")
text = text.replace("## Done when\n", """## Done when

- [ ] On a v1 deployment, a deleted, changed or hand-added event file makes
  `load`, `check_prod` and `migrate` refuse; tests cover each case and the
  suite passes.
""").replace("## Scope\n", """## Scope

`flags.py`, `flagctl.py`, `test_flags.py`, `docs/flags.md`, and new modules
they need. flags-rm1x removes the v1 path once every deployment has migrated.
""")
p.write_text(text)
PY
planned "Take $finding into flags-l3dg"
atelier ready flags-l3dg --by dana >/dev/null
planned "Ready flags-l3dg"

as $impl start flags-l3dg
cp "$files/fix/v1ledger.py" "$files/fix/flags.py" .
git add v1ledger.py && git commit -qam "Chain v1 event files in a ledger that load verifies"
cp "$files/fix/flagctl.py" "$files/fix/test_flags.py" . && cp "$files/fix/docs/flags.md" docs/
sed -i "s/2026-11-10/$release/" docs/flags.md
git commit -qam "Add ledger verify and rebuild, docs and tests"
sed -i 's/^- \[ \]/- [x]/' .atelier/items/flags-l3dg/item.md
cat >> .atelier/items/flags-l3dg/item.md <<'MD'

## Report

Done when holds: `LedgerTest` covers a deleted (`test_a_deleted_event_is_refused`),
changed and hand-added event file, a broken chain, and `migrate` refusing a
tampered deployment; `python3 -m unittest`: 15 tests pass. `v1ledger.py` chains
each v1 event file's hash in `flags.d/.ledger`; `load`, `set_flag` and `migrate`
verify it. A deployment with no ledger is read as before until `flagctl.py
ledger rebuild`, which trusts its files once. Docs: docs/flags.md, "The v1 ledger".
MD
git commit -qam "Report flags-l3dg"
as $impl submit

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.branch_head"
git rev-parse main > "$work/.main_head"
