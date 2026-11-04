#!/usr/bin/env bash
# Merge check: the work branch's code passed the CI workflow on its pull
# request. `atelier merge` runs this in a detached checkout of the merge
# commit, whose first parent is the base head and second the branch head.
#
# "Code" here is everything outside the tracker records (.atelier/issues,
# reviews, evidence); the workflow policy and config count as code.
#
# It passes when the branch changes no code, or when a branch commit whose
# code matches the branch head has a successful CI run and the base gained no
# code the branch does not contain. Then CI ran the code that merges; commits
# after that run that touch only records, such as review approvals, need no
# new run.
#
# It only works inside that merge checkout; elsewhere HEAD^2 does not exist.
set -euo pipefail

if ! git rev-parse -q --verify HEAD^2 >/dev/null; then
  echo "merge-ci-passed: HEAD is not a merge commit; this check runs only inside \`atelier merge\`'s merge checkout." >&2
  exit 2
fi

code=(. ':!.atelier/issues' ':!.atelier/reviews' ':!.atelier/evidence' ':!.atelier/runtime')
base=$(git rev-parse HEAD^1)
head=$(git rev-parse HEAD^2)
fork=$(git merge-base "$base" "$head")

if git diff --quiet "$fork" "$head" -- "${code[@]}"; then
  echo "merge-ci-passed: the branch changes only records, so it needs no CI run."
  exit 0
fi

if [ -n "$(git diff --name-only "$fork" "$base" -- "${code[@]}")" ]; then
  echo "merge-ci-passed: the base changed code since the branch left it, so CI did not run the code that would merge." >&2
  echo "Run \`atelier merge --sync\`, push the branch, and wait for CI to pass on the pull request." >&2
  exit 1
fi

for commit in $(git rev-list "$fork..$head"); do
  if [ -n "$(git diff --name-only "$commit" "$head" -- "${code[@]}")" ]; then
    break
  fi
  passed=$(gh run list --commit "$commit" --workflow ci.yml --limit 20 \
    --json status,conclusion \
    -q '[.[] | select(.status == "completed" and .conclusion == "success")] | length')
  if [ "$passed" -gt 0 ]; then
    echo "merge-ci-passed: CI passed on $commit, which has the branch head's code."
    exit 0
  fi
done

echo "merge-ci-passed: no successful CI run for the branch head's code (${head:0:8})." >&2
echo "Push the branch, open a pull request, and wait for its CI run to pass." >&2
exit 1
