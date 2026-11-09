#!/usr/bin/env bash
# Tests scripts/ci-jobs.sh against scratch repositories. CI's `changes` job
# runs it before trusting the script's answer.
set -euo pipefail

script=$(cd "$(dirname "$0")" && pwd)/ci-jobs.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failed=0

g() { git -c user.name=ci -c user.email=ci@ci.invalid "$@"; }

# expect <rust> <web> <case> <path>...: whether a commit touching the paths
# needs the rust job and the web job.
expect() {
  local want="$1 $2" name=$3
  shift 3
  local repo="$scratch/$name"
  g init -q "$repo"
  mkdir -p "$repo/crates/a/tests/fixtures" "$repo/docs/strategies"
  echo base > "$repo/crates/a/tests/fixtures/old.md"
  echo base > "$repo/README.md"
  g -C "$repo" add -A
  g -C "$repo" commit -qm base
  for path in "$@"; do
    mkdir -p "$repo/$(dirname "$path")"
    echo change >> "$repo/$path"
  done
  g -C "$repo" add -A
  g -C "$repo" commit -qm change
  check "$want" "$name" "$repo" HEAD^
}

# check "<rust> <web>" <case> <repo> <base>: the script answers rust=<rust>
# and web=<web>.
check() {
  local got want
  got=$(cd "$3" && "$script" "$4" HEAD 2>/dev/null | paste -sd' ')
  want="rust=${1% *} web=${1#* }"
  if [ "$got" = "$want" ]; then
    echo "ok   $2"
  else
    echo "FAIL $2: want $want, got ${got:-nothing}"
    failed=1
  fi
}

expect false false markdown-at-root README.md
expect false false markdown-in-docs docs/adr/0001-x.md docs/product/y.md
expect false false records-only .atelier/issues/a.md .atelier/reviews/a.toml .atelier/evidence/a.md
expect false false markdown-and-records CONTEXT.md .atelier/issues/a.md
expect true true strategy docs/strategies/s.md
expect true true strategy-and-records docs/strategies/s.md .atelier/issues/a.md
expect true true markdown-under-crates crates/a/tests/fixtures/new.md
expect true true rust-source crates/a/src/lib.rs
expect true true workflow-policy .atelier/workflow.yaml
expect true true github-workflow .github/workflows/ci.yml
expect true true script scripts/x.sh
expect true true markdown-and-code README.md Cargo.toml
expect true true web-app crates/atelier-site/web/src/App.tsx
expect true true web-lockfile crates/atelier-site/web/pnpm-lock.yaml

# Moving a fixture out of crates/ changes crates/.
repo="$scratch/rename"
g init -q "$repo"
mkdir -p "$repo/crates/a" "$repo/docs"
echo fixture > "$repo/crates/a/f.md"
g -C "$repo" add -A
g -C "$repo" commit -qm base
g -C "$repo" mv crates/a/f.md docs/f.md
g -C "$repo" commit -qm move
check "true true" rename-out-of-crates "$repo" HEAD^

check "true true" empty-base "$repo" ""
check "true true" zero-base "$repo" 0000000000000000000000000000000000000000
check "true true" missing-base "$repo" 1234567890abcdef1234567890abcdef12345678

exit "$failed"
