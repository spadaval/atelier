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
# rust (the old workspace's build and tests) reads everything except tracker
# records, Markdown outside crates/ and docs/strategies/, the new workspace
# next/ and the Desk's app desk/: crates/ holds test fixtures, and a test
# parses every strategy.
#
# next (the new workspace's build and tests) reads next/ and what configures
# its build from outside it: the toolchain, .cargo/ and the CI workflow. It
# also reads desk/src/api/, the app's types generated from atelier-desk's,
# which a test there compares.
#
# web (the site's app: typecheck, Vitest, build and Playwright) runs whenever
# rust does, since the API it shows is the shared view models any Rust change
# can change, and whenever crates/atelier-site/web/ changes.
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
    .atelier/issues/* | .atelier/reviews/* | .atelier/evidence/* | .atelier/runtime/* | next/* | desk/*) return 1 ;;
    crates/* | docs/strategies/*) return 0 ;;
    *.md) return 1 ;;
    *) return 0 ;;
  esac
}

needs_next() {
  case $1 in
    next/* | desk/src/api/* | rust-toolchain.toml | .cargo/* | .github/workflows/*) return 0 ;;
    *) return 1 ;;
  esac
}

if [[ -z $base || $base =~ ^0+$ ]] || ! git rev-parse -q --verify "$base^{commit}" >/dev/null; then
  echo "ci-jobs: no base commit to compare against, so every job runs." >&2
  echo "rust=true"
  echo "web=true"
  echo "next=true"
  echo "desk=true"
  exit 0
fi

# --no-renames lists a renamed file under both names, so moving a fixture
# out of crates/ still counts as a change to crates/.
rust=false
web=false
next=false
desk=false
while IFS= read -r path; do
  if [ "$rust" = false ] && needs_rust "$path"; then
    echo "ci-jobs: rust runs for $path." >&2
    rust=true
  fi
  if [ "$web" = false ] && [[ $path == crates/atelier-site/web/* ]]; then
    echo "ci-jobs: web runs for $path." >&2
    web=true
  fi
  if [ "$next" = false ] && needs_next "$path"; then
    echo "ci-jobs: next runs for $path." >&2
    next=true
  fi
  if [ "$desk" = false ] && [[ $path == desk/* ]]; then
    echo "ci-jobs: desk runs for $path." >&2
    desk=true
  fi
done < <(git diff --no-renames --name-only "$base" "$head")
[ "$rust" = true ] || echo "ci-jobs: rust skipped; the change touches nothing it tests." >&2
if [ "$rust" = true ]; then
  web=true
fi
[ "$web" = true ] || echo "ci-jobs: web skipped; the change touches neither Rust nor the app." >&2
[ "$next" = true ] || echo "ci-jobs: next skipped; the change touches nothing it builds." >&2
[ "$desk" = true ] || echo "ci-jobs: desk skipped; the change touches nothing in desk/." >&2
echo "rust=$rust"
echo "web=$web"
echo "next=$next"
echo "desk=$desk"
