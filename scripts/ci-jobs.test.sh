#!/usr/bin/env bash
# Tests scripts/ci-jobs.sh against scratch repositories. CI's `changes` job
# runs it before trusting the script's answer.
set -euo pipefail

script=$(cd "$(dirname "$0")" && pwd)/ci-jobs.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failed=0

g() { git -c user.name=ci -c user.email=ci@ci.invalid "$@"; }

# expect <rust> <desk> <extension> <evals> <case> <path>...: whether a
# commit touching the paths needs the rust, desk, extension and evals jobs.
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

# check "<rust> <desk> <extension> <evals>" <case> <repo> <base>: the
# script answers rust=<rust>, desk=<desk>, extension=<extension> and
# evals=<evals>.
check() {
  local got want rust desk extension evals
  read -r rust desk extension evals <<< "$1"
  got=$(cd "$3" && "$script" "$4" HEAD 2>/dev/null | paste -sd' ')
  want="rust=$rust desk=$desk extension=$extension evals=$evals"
  if [ "$got" = "$want" ]; then
    echo "ok   $2"
  else
    echo "FAIL $2: want $want, got ${got:-nothing}"
    failed=1
  fi
}

expect false false false false markdown-at-root README.md
expect false false false false markdown-in-docs docs/adr/0001-x.md docs/strategies/y.md
expect false false false false records-only .atelier/issues/a.md .atelier/reviews/a.toml .atelier/evidence/a.md
expect false false false false markdown-and-records CONTEXT.md .atelier/issues/a.md
expect false false false false new-layout-records .atelier/items/a/item.md .atelier/missions/m/mission.md
expect true false false false new-layout-config .atelier/config.toml
expect true false false false markdown-under-crates crates/a/tests/fixtures/new.md
expect true false false false rust-source crates/a/src/lib.rs
expect true false true true lockfile Cargo.lock
expect true false false false workflow-policy .atelier/workflow.yaml
expect true false true true github-workflow .github/workflows/ci.yml
expect true false false false script scripts/x.sh
expect true false false true skill skills/agent-factory/SKILL.md skills/agent-factory/evals.json
expect true false true true markdown-and-code README.md Cargo.toml
expect false true true false desk-app desk/src/pages/Mission.tsx desk/package.json
expect true true true false desk-api-types desk/src/api/MissionPage.ts
expect false true true false desk-readme desk/README.md
expect false true true false desk-fixtures desk/e2e/fixtures.ts
expect true true true false rust-and-desk crates/a/src/lib.rs desk/src/App.tsx
expect true false true true toolchain rust-toolchain.toml
expect false false true false t3-extension t3-extension/custom.js t3-extension/install.py t3-extension/README.md
expect true true true false t3-extension-and-desk-api t3-extension/custom.js desk/src/api/MissionPage.ts
expect true false true false desk-server crates/atelier-desk/src/api.rs
expect true false false true core-alone crates/atelier-core/src/repo.rs
expect true false false false nested-cargo-toml crates/a/Cargo.toml
expect true false true true workspace-cargo-toml Cargo.toml
expect true false true true ci-jobs-script scripts/ci-jobs.sh
expect true false false false ci-jobs-test scripts/ci-jobs.test.sh
expect true false false false other-workflow .github/workflows/v-release.yml
expect false false true false t3-extension-test t3-extension/test-extension.mjs
expect true false false true cli crates/atelier-cli/src/main.rs
expect false false false true eval-scenario evals/agent-factory/scenarios/x/setup.sh evals/agent-factory/README.md
expect false false false true skill-reference skills/agent-factory/roles/implementer.md

# Moving a fixture out of crates/ changes crates/.
repo="$scratch/rename"
g init -q "$repo"
mkdir -p "$repo/crates/a" "$repo/docs"
echo fixture > "$repo/crates/a/f.md"
g -C "$repo" add -A
g -C "$repo" commit -qm base
g -C "$repo" mv crates/a/f.md docs/f.md
g -C "$repo" commit -qm move
check "true false false false" rename-out-of-crates "$repo" HEAD^

check "true true true true" empty-base "$repo" ""
check "true true true true" zero-base "$repo" 0000000000000000000000000000000000000000
check "true true true true" missing-base "$repo" 1234567890abcdef1234567890abcdef12345678

exit "$failed"
