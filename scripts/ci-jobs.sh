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
# evidence and runtime, Markdown outside crates/, the Desk's app desk/, the
# T3 Code extension t3-extension/, and the evals in evals/, which no Rust
# reads.
# It does read desk/src/api/, the app's types generated from atelier-desk's,
# which a test there compares.
#
# desk (the Desk's app: typecheck, Vitest, build and Playwright on fixture
# responses) runs when a change touches desk/, and only then: it builds no
# Rust, and a change to atelier-desk's types reaches it as a change to
# desk/src/api/.
#
# extension (the T3 Code extension's tests in t3-extension/, against an
# atelier-desk and its app built from the same commit) runs when a change
# touches t3-extension/, desk/, crates/atelier-desk/, the workspace's
# Cargo.toml, Cargo.lock or rust-toolchain.toml, the CI workflow or this
# script. Not for crates/atelier-core alone: the fields the extension reads
# are defined in atelier-desk, and the rust job tests the core.
#
# evals (Agent Factory's eval fixtures and harness tests,
# evals/agent-factory/test.sh, against an atelier built from the same
# commit; no model is called) runs when a change touches what they drive or
# stage: crates/atelier-cli/, crates/atelier-core/, the workspace's
# Cargo.toml, Cargo.lock or rust-toolchain.toml, skills/agent-factory/,
# evals/, the CI workflow or this script.
set -euo pipefail

base=${1:-}
head=${2:-HEAD}

needs_extension() {
  case $1 in
    t3-extension/* | desk/* | crates/atelier-desk/* | Cargo.toml | Cargo.lock | rust-toolchain.toml | .github/workflows/ci.yml | scripts/ci-jobs.sh) return 0 ;;
    *) return 1 ;;
  esac
}

needs_evals() {
  case $1 in
    crates/atelier-cli/* | crates/atelier-core/* | Cargo.toml | Cargo.lock | rust-toolchain.toml | skills/agent-factory/* | evals/* | .github/workflows/ci.yml | scripts/ci-jobs.sh) return 0 ;;
    *) return 1 ;;
  esac
}

needs_rust() {
  case $1 in
    desk/src/api/*) return 0 ;;
    .atelier/issues/* | .atelier/reviews/* | .atelier/evidence/* | .atelier/runtime/* | desk/* | t3-extension/* | evals/*) return 1 ;;
    crates/*) return 0 ;;
    *.md) return 1 ;;
    *) return 0 ;;
  esac
}

if [[ -z $base || $base =~ ^0+$ ]] || ! git rev-parse -q --verify "$base^{commit}" >/dev/null; then
  echo "ci-jobs: no base commit to compare against, so every job runs." >&2
  echo "rust=true"
  echo "desk=true"
  echo "extension=true"
  echo "evals=true"
  exit 0
fi

# --no-renames lists a renamed file under both names, so moving a fixture
# out of crates/ still counts as a change to crates/.
rust=false
desk=false
extension=false
evals=false
while IFS= read -r path; do
  if [ "$rust" = false ] && needs_rust "$path"; then
    echo "ci-jobs: rust runs for $path." >&2
    rust=true
  fi
  if [ "$desk" = false ] && [[ $path == desk/* ]]; then
    echo "ci-jobs: desk runs for $path." >&2
    desk=true
  fi
  if [ "$extension" = false ] && needs_extension "$path"; then
    echo "ci-jobs: extension runs for $path." >&2
    extension=true
  fi
  if [ "$evals" = false ] && needs_evals "$path"; then
    echo "ci-jobs: evals runs for $path." >&2
    evals=true
  fi
done < <(git diff --no-renames --name-only "$base" "$head")
[ "$rust" = true ] || echo "ci-jobs: rust skipped; the change touches nothing it tests." >&2
[ "$desk" = true ] || echo "ci-jobs: desk skipped; the change touches nothing in desk/." >&2
[ "$extension" = true ] || echo "ci-jobs: extension skipped; the change touches nothing it tests." >&2
[ "$evals" = true ] || echo "ci-jobs: evals skipped; the change touches nothing they test." >&2
echo "rust=$rust"
echo "desk=$desk"
echo "extension=$extension"
echo "evals=$evals"
