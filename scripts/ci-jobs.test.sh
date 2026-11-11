#!/usr/bin/env bash
# Tests scripts/ci-jobs.sh against scratch repositories. CI's `changes` job
# runs it before trusting the script's answer.
set -euo pipefail

script=$(cd "$(dirname "$0")" && pwd)/ci-jobs.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failed=0

g() { git -c user.name=ci -c user.email=ci@ci.invalid "$@"; }

# expect <rust> <desk> <case> <path>...: whether a commit touching the
# paths needs the rust and desk jobs.
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

# check "<rust> <desk>" <case> <repo> <base>: the script answers
# rust=<rust> and desk=<desk>.
check() {
  local got want rust desk
  read -r rust desk <<< "$1"
  got=$(cd "$3" && "$script" "$4" HEAD 2>/dev/null | paste -sd' ')
  want="rust=$rust desk=$desk"
  if [ "$got" = "$want" ]; then
    echo "ok   $2"
  else
    echo "FAIL $2: want $want, got ${got:-nothing}"
    failed=1
  fi
}

expect false false markdown-at-root README.md
expect false false markdown-in-docs docs/adr/0001-x.md docs/strategies/y.md
expect false false records-only .atelier/issues/a.md .atelier/reviews/a.toml .atelier/evidence/a.md
expect false false markdown-and-records CONTEXT.md .atelier/issues/a.md
expect false false new-layout-records .atelier/items/a/item.md .atelier/missions/m/mission.md
expect true false new-layout-config .atelier/config.toml
expect true false markdown-under-crates crates/a/tests/fixtures/new.md
expect true false rust-source crates/a/src/lib.rs
expect true false lockfile Cargo.lock
expect true false workflow-policy .atelier/workflow.yaml
expect true false github-workflow .github/workflows/ci.yml
expect true false script scripts/x.sh
expect true false skill skills/agent-factory/SKILL.md skills/agent-factory/evals.json
expect true false markdown-and-code README.md Cargo.toml
expect false true desk-app desk/src/pages/Mission.tsx desk/package.json
expect true true desk-api-types desk/src/api/MissionPage.ts
expect false true desk-readme desk/README.md
expect false true desk-fixtures desk/e2e/fixtures.ts
expect true true rust-and-desk crates/a/src/lib.rs desk/src/App.tsx
expect true false toolchain rust-toolchain.toml

# Moving a fixture out of crates/ changes crates/.
repo="$scratch/rename"
g init -q "$repo"
mkdir -p "$repo/crates/a" "$repo/docs"
echo fixture > "$repo/crates/a/f.md"
g -C "$repo" add -A
g -C "$repo" commit -qm base
g -C "$repo" mv crates/a/f.md docs/f.md
g -C "$repo" commit -qm move
check "true false" rename-out-of-crates "$repo" HEAD^

check "true true" empty-base "$repo" ""
check "true true" zero-base "$repo" 0000000000000000000000000000000000000000
check "true true" missing-base "$repo" 1234567890abcdef1234567890abcdef12345678

exit "$failed"
