#!/usr/bin/env bash
# The setup the five sized cases share (overbuilt-generality, overbuilt-layer,
# overbuilt-robustness, control-large and control-lean): a small Python tool
# tracked by Atelier, with one item whose correct change is submitted for
# review. The agent is its reviewer, started with the task line `submit`
# prints, in the implementer's checkout on the item's branch.
#
# Usage: sized.sh <case dir> <work dir>. The case's fixture/ holds:
#   case.sh    ID, SLUG, TITLE and COMMIT, the change's commit message
#   item.md    the item's body: Goal, Done when and Scope
#   report.md  the Report the implementer writes
#   base/      the tree on main before the item
#   change/    the files the change writes, over base/
# A case's lean/, its lean reference, stays outside the fixture: this script
# never reads it, and the reviewer never sees it.
set -euo pipefail
case_dir=$(cd "$1" && pwd)
work=$2
fixture=$case_dir/fixture
source "$fixture/case.sh"
source "$case_dir/../loop/lib.sh"
next_repo "$work" "$SLUG"
cp -r "$fixture/base/." .
item "$ID" ready "$TITLE" < "$fixture/item.md"
git add . && git commit -qm "Start the tool, and track work in Atelier"
git rev-parse HEAD > "$work/.main_head"
cp "$work/.main_head" "$work/.setup_head"

impl=thread-1a2b3c4d
begin $impl "$ID"
cp -r "$fixture/change/." .
git add -A . && git commit -qm "$COMMIT"
sed -i 's/^- \[ \]/- [x]/' ".atelier/items/$ID/item.md"
{ printf '\n## Report\n\n'; cat "$fixture/report.md"; } >> ".atelier/items/$ID/item.md"
as $impl submit

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.branch_head"
