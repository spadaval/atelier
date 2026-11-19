#!/usr/bin/env bash
# The setup the later-round cases share (later-round-cosmetic,
# later-round-wrong and later-round-fix-wrong): a small Python tool tracked
# by Atelier, with one item one round of review on. Round 1's review asked,
# in two fix-now findings, for two real defects; the implementer fixed both
# and submitted again. The agent is round 2's reviewer, started with the
# task line `submit` prints, in the implementer's checkout on the item's
# branch.
#
# Usage: later_round.sh <case dir> <work dir>. The item, its trees and
# round 1's review are in harness/later-round/:
#   case.sh     ID, SLUG, TITLE, COMMIT (round 1's change) and FIX (the fix)
#   item.md     the item's body: Goal, Done when and Scope
#   report.md   the Report as submitted for round 1
#   fixed.md    what the implementer adds to it with the fix
#   round-1.md  round 1's summary and findings
#   base/       the tree on main before the item
#   change/     the files round 1's change writes, over base/
#   fix/        the files the fix writes, over change/
# The case's fixture/week_lines.py goes where change/ and fix/ hold the
# line `# week_lines: ...`, so it is the same in both commits: text round 1
# reviewed and the fix left alone. later-round-cosmetic and
# later-round-wrong differ in it alone. A case's fixture/fix.patch, if it
# has one, is applied over fix/, so the fix commit holds the case's flaw:
# later-round-fix-wrong's.
set -euo pipefail
case_dir=$(cd "$1" && pwd)
work=$2
shared=$(cd "$(dirname "$0")" && pwd)/later-round
source "$shared/case.sh"
source "$case_dir/../loop/lib.sh"

# place <dir>: copy <dir>'s files over the checkout, with the case's
# week_lines in hours.py.
place() {
    cp -r "$1/." .
    python3 - "$case_dir/fixture/week_lines.py" hours.py <<'PY'
import sys
function, path = open(sys.argv[1]).read(), sys.argv[2]
code = open(path).read()
marker = "# week_lines: each case's own, from its fixture/week_lines.py\n"
assert code.count(marker) == 1, f"{path} holds no week_lines line"
open(path, "w").write(code.replace(marker, function))
PY
}

next_repo "$work" "$SLUG"
cp -r "$shared/base/." .
item "$ID" ready "$TITLE" < "$shared/item.md"
git add . && git commit -qm "Start the timesheet tool, and track work in Atelier"
git rev-parse HEAD > "$work/.main_head"
cp "$work/.main_head" "$work/.setup_head"

impl=thread-1a2b3c4d
begin $impl "$ID"
place "$shared/change"
git add -A . && git commit -qm "$COMMIT"
record=.atelier/items/$ID/item.md
sed -i 's/^- \[ \]/- [x]/' "$record"
{ printf '\n## Report\n\n'; cat "$shared/report.md"; } >> "$record"
as $impl submit
git rev-parse HEAD > "$work/.round_1_head"

# Round 1, by a thread that is neither the agent nor the implementer.
as thread-5e1f0c2a review "$ID"
review=$(ls .atelier/items/"$ID"/reviews/1-*.md)
python3 - "$review" "$shared/round-1.md" <<'PY'
import sys
path, round_1 = sys.argv[1:]
summary, findings = open(round_1).read().split("\n## ", 1)
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->", summary.strip())
open(path, "w").write(text.rstrip("\n") + "\n\n## " + findings)
PY
judge thread-5e1f0c2a changes --model gpt-6.1-sol

# The fix, answered under Responses, and submitted for round 2.
place "$shared/fix"
if [ -f "$case_dir/fixture/fix.patch" ]; then
    git apply "$case_dir/fixture/fix.patch"
fi
git add -A . && git commit -qm "$FIX"
fix=$(git rev-parse --short HEAD)
stem=$(basename "$review" .md)
{ cat "$shared/fixed.md"
  printf '\n### Responses\n\n- %s.1: fixed in %s\n- %s.2: fixed in %s\n' "$stem" "$fix" "$stem" "$fix"
} >> "$record"
as $impl submit

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.branch_head"
