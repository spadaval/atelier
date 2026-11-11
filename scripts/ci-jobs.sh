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
# rust (the workspace's build and tests, and `atelier check` on the records)
# reads everything except tracker records under .atelier/issues, reviews,
# evidence and runtime, Markdown outside crates/, and the Desk's app desk/.
# It does read desk/src/api/, the app's types generated from atelier-desk's,
# which a test there compares.
#
# desk (the Desk's app: typecheck, Vitest, build and Playwright on fixture
# responses) runs when a change touches desk/, and only then: it builds no
# Rust, and a change to atelier-desk's types reaches it as a change to
# desk/src/api/.
set -euo pipefail

base=${1:-}
head=${2:-HEAD}

needs_rust() {
  case $1 in
    desk/src/api/*) return 0 ;;
    .atelier/issues/* | .atelier/reviews/* | .atelier/evidence/* | .atelier/runtime/* | desk/*) return 1 ;;
    crates/*) return 0 ;;
    *.md) return 1 ;;
    *) return 0 ;;
  esac
}

if [[ -z $base || $base =~ ^0+$ ]] || ! git rev-parse -q --verify "$base^{commit}" >/dev/null; then
  echo "ci-jobs: no base commit to compare against, so every job runs." >&2
  echo "rust=true"
  echo "desk=true"
  exit 0
fi

# --no-renames lists a renamed file under both names, so moving a fixture
# out of crates/ still counts as a change to crates/.
rust=false
desk=false
while IFS= read -r path; do
  if [ "$rust" = false ] && needs_rust "$path"; then
    echo "ci-jobs: rust runs for $path." >&2
    rust=true
  fi
  if [ "$desk" = false ] && [[ $path == desk/* ]]; then
    echo "ci-jobs: desk runs for $path." >&2
    desk=true
  fi
done < <(git diff --no-renames --name-only "$base" "$head")
[ "$rust" = true ] || echo "ci-jobs: rust skipped; the change touches nothing it tests." >&2
[ "$desk" = true ] || echo "ci-jobs: desk skipped; the change touches nothing in desk/." >&2
echo "rust=$rust"
echo "desk=$desk"
