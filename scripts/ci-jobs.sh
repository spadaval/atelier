#!/usr/bin/env bash
# Prints which CI jobs a change needs, one `<job>=true|false` line per job,
# for the CI workflow's `changes` job to append to $GITHUB_OUTPUT.
#
# Usage: scripts/ci-jobs.sh <base> [<head>]
#
# A job is needed when the change touches a path that job can test. When the
# base is missing, all zeros (a new branch) or not in the clone, every job
# runs. Every change still gets a pull request, review and a CI run; a run
# whose jobs are skipped concludes success.
#
# rust (build and tests) reads everything except tracker records and Markdown
# outside crates/ and docs/strategies/: crates/ holds test fixtures, and a test
# parses every strategy.
set -euo pipefail

base=${1:-}
head=${2:-HEAD}

needs_rust() {
  case $1 in
    .atelier/issues/* | .atelier/reviews/* | .atelier/evidence/* | .atelier/runtime/*) return 1 ;;
    crates/* | docs/strategies/*) return 0 ;;
    *.md) return 1 ;;
    *) return 0 ;;
  esac
}

if [[ -z $base || $base =~ ^0+$ ]] || ! git rev-parse -q --verify "$base^{commit}" >/dev/null; then
  echo "ci-jobs: no base commit to compare against, so every job runs." >&2
  echo "rust=true"
  exit 0
fi

# --no-renames lists a renamed file under both names, so moving a fixture
# out of crates/ still counts as a change to crates/.
rust=false
while IFS= read -r path; do
  if needs_rust "$path"; then
    echo "ci-jobs: rust runs for $path." >&2
    rust=true
    break
  fi
done < <(git diff --no-renames --name-only "$base" "$head")
[ "$rust" = true ] || echo "ci-jobs: rust skipped; the change touches nothing it tests." >&2
echo "rust=$rust"
