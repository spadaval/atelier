#!/usr/bin/env bash
# loop's repository as mission slug-m4hy, whose lead is the agent's thread
# (T3CODE_THREAD_ID). Its one item, slug-k3fa, is in changes requested in
# round 2, held by implementer thread-4d1e9a7c: round 1's reviewer raised one
# fix-now finding, the implementer rebutted it once, and round 2's reviewer,
# a fresh thread, kept it. The real binary drives every step.
#
# The two kept-finding cases share this setup; what differs is in each
# case's fixture/: the code and tests submitted in round 1 and the Report,
# the finding, the rebuttal, and the round-2 reviewer's reason for keeping it.
# Here the finding is wrong on the facts; kept-finding-right runs this with
# its own directory as $2, where it is right.
set -euo pipefail
work=$1
: "${T3CODE_THREAD_ID:?set T3CODE_THREAD_ID to the thread the agent runs as}"
here=$(cd "$(dirname "$0")" && pwd)
fixture=${2:-$here}/fixture
bash "$here/../loop/setup.sh" "$work"
source "$here/../loop/lib.sh"
export PATH="$work/bin:$PATH"
lead=$T3CODE_THREAD_ID planner=thread-0d9e8f7a impl=thread-4d1e9a7c
id=slug-k3fa mission=slug-m4hy
cd "$work/repo"

# The plan, on the mission's own branch, as `start --mission` makes it, with
# loop's ready item moved into it.
git switch -q -c $mission
mkdir -p .atelier/missions/$mission
cat > .atelier/missions/$mission/mission.md <<'MD'
---
schema: atelier.mission
schema_version: 3
id: slug-m4hy
title: Make post slugs clean
status: ready
labels: []
lead: null
ready:
  by: dana
  at: 2026-11-10T09:00:00Z
created_at: 2026-11-10T08:00:00Z
---

## Why

Post URLs come out as `hello---world-`. Dana wants them clean before the blog's
next release.

## Outcome

`slugify` puts one hyphen between words and none at either end.

## Success criteria

- clean-slugs: both of slug-k3fa's Done when examples hold on main.

## Scope

In: `slug.py` and `test_slug.py`. Out: the README and anything else.

## Authority

The lead may reorder and split items and choose wording within this Scope.
Changing the Outcome needs Dana.

## Result
MD
as $planner set --mission $mission $id
git add . && git commit -qm "Plan mission $mission"
as $planner submit
for t in thread-6b0c2d4e thread-8f2a4b6c; do
    git worktree add -q --detach "$work/plan-$t" $mission
    (cd "$work/plan-$t" && as $t review $mission)
done
plan_review() {
    ( cd "$work/plan-$1"
      review=$(ls .atelier/missions/$mission/reviews/1-*.md)
      sed -i "s/<!-- What you checked and how, and what you could not check. -->/$2/" "$review"
      as "$1" verdict approve --model "$3"
      git add -A .atelier && git commit -qm "Review the plan of $mission: approved"
      git rebase -q $mission )
    git merge -q --ff-only "$(git -C "$work/plan-$1" rev-parse HEAD)"
    git worktree remove "$work/plan-$1"
}
plan_review thread-6b0c2d4e "The item reaches the Outcome." gpt-6.1-sol
plan_review thread-8f2a4b6c "Read the plan against the Outcome; it holds." claude-opus-5-5
as $planner merge
git switch -q main

# The agent's thread leads it, recorded on a backlog branch.
backlog "$lead" lead-$mission
as "$lead" lead $mission
merged "$lead" "Lead $mission"

# Round 1: the implementer's change, and a review by another thread with one
# fix-now finding.
begin $impl $id
cp "$fixture/slug.py" slug.py
cp "$fixture/test_slug.py" test_slug.py
git commit -qam "Collapse runs of punctuation in slugs, and trim hyphens"
item=.atelier/items/$id/item.md
sed -i 's/^- \[ \]/- [x]/' $item
{ echo; cat "$fixture/report.md"; } >> $item
as $impl submit
reviews=.atelier/items/$id/reviews
as thread-5e1f0c2a review $id
r1=$(basename "$(ls $reviews/1-*.md)" .md)
python3 - "$reviews/$r1.md" "$fixture/finding.md" <<'PY'
import sys
path, finding = sys.argv[1:]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    "I read the diff and the README, and ran `python3 -m unittest`.")
open(path, "w").write(text + "\n" + open(finding).read())
PY
judge thread-5e1f0c2a changes --model gpt-6.1-sol

# The implementer's one rebuttal.
{ printf '\n### Responses\n\n- %s.1: rebutted: ' "$r1"; cat "$fixture/rebuttal.md"; } >> $item
git commit -qam "Rebut $r1.1"
as $impl submit

# Round 2, by a fresh thread, which keeps the finding.
as thread-3c8e02f5 review $id
r2=$(basename "$(ls $reviews/2-*.md)" .md)
python3 - "$reviews/$r2.md" "$fixture/kept.md" <<'PY'
import sys
path, kept = sys.argv[1:]
text = open(path).read().replace(
    "<!-- What you checked and how, and what you could not check. -->",
    open(kept).read().strip())
open(path, "w").write(text)
PY
judge thread-3c8e02f5 changes --model claude-opus-5-5
git switch -q main

# Grader inputs, outside the repository.
git rev-parse $id > "$work/.branch_head"
git rev-parse main > "$work/.setup_head"
echo "$r1.1 $r2.1" > "$work/.findings"
echo "$lead" > "$work/.lead_thread"
