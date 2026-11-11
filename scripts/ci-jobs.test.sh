#!/usr/bin/env bash
# Tests scripts/ci-jobs.sh against scratch repositories. CI's `changes` job
# runs it before trusting the script's answer.
set -euo pipefail

script=$(cd "$(dirname "$0")" && pwd)/ci-jobs.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failed=0

g() { git -c user.name=ci -c user.email=ci@ci.invalid "$@"; }

# expect <rust> <web> <next> <desk> <case> <path>...: whether a commit
# touching the paths needs the rust, web, next and desk jobs.
expect() {
  local want="$1 $2 $3 $4" name=$5
  shift 5
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

# check "<rust> <web> <next> <desk>" <case> <repo> <base>: the script
# answers rust=<rust>, web=<web>, next=<next> and desk=<desk>.
check() {
  local got want rust web next desk
  read -r rust web next desk <<< "$1"
  got=$(cd "$3" && "$script" "$4" HEAD 2>/dev/null | paste -sd' ')
  want="rust=$rust web=$web next=$next desk=$desk"
  if [ "$got" = "$want" ]; then
    echo "ok   $2"
  else
    echo "FAIL $2: want $want, got ${got:-nothing}"
    failed=1
  fi
}

expect false false false false markdown-at-root README.md
expect false false false false markdown-in-docs docs/adr/0001-x.md docs/product/y.md
expect false false false false records-only .atelier/issues/a.md .atelier/reviews/a.toml .atelier/evidence/a.md
expect false false false false markdown-and-records CONTEXT.md .atelier/issues/a.md
expect true true false false strategy docs/strategies/s.md
expect true true false false strategy-and-records docs/strategies/s.md .atelier/issues/a.md
expect true true false false markdown-under-crates crates/a/tests/fixtures/new.md
expect true true false false rust-source crates/a/src/lib.rs
expect true true false false workflow-policy .atelier/workflow.yaml
expect true true true false github-workflow .github/workflows/ci.yml
expect true true false false script scripts/x.sh
expect true true false false markdown-and-code README.md Cargo.toml
expect true true false false web-app crates/atelier-site/web/src/App.tsx
expect true true false false web-lockfile crates/atelier-site/web/pnpm-lock.yaml
expect false false true false next-source next/crates/atelier-core/src/lib.rs
expect false false true false next-lockfile next/Cargo.lock
expect false false true false next-markdown next/crates/atelier-core/README.md
expect true true true false old-and-next crates/a/src/lib.rs next/Cargo.toml
expect false false false true desk-app desk/src/pages/Mission.tsx desk/package.json
expect false false true true desk-api-types desk/src/api/MissionPage.ts
expect false false false true desk-readme desk/README.md
expect false false false true desk-fixtures desk/e2e/fixtures.ts
expect true true false true rust-and-desk crates/a/src/lib.rs desk/src/App.tsx
expect true true true false toolchain rust-toolchain.toml
expect true true true false cargo-config .cargo/config.toml

# Moving a fixture out of crates/ changes crates/.
repo="$scratch/rename"
g init -q "$repo"
mkdir -p "$repo/crates/a" "$repo/docs"
echo fixture > "$repo/crates/a/f.md"
g -C "$repo" add -A
g -C "$repo" commit -qm base
g -C "$repo" mv crates/a/f.md docs/f.md
g -C "$repo" commit -qm move
check "true true false false" rename-out-of-crates "$repo" HEAD^

check "true true true true" empty-base "$repo" ""
check "true true true true" zero-base "$repo" 0000000000000000000000000000000000000000
check "true true true true" missing-base "$repo" 1234567890abcdef1234567890abcdef12345678

exit "$failed"
