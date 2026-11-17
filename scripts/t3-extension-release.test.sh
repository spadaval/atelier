#!/usr/bin/env bash
# Tests scripts/t3-extension-release.sh against a scratch repository whose
# master is a line of merges, with GitHub's answers about CI runs served by a
# stand-in `gh` from fixtures. CI's `changes` job runs it. Its bundle and
# verify cases build this checkout's t3-extension/, so they need Node and pnpm.
set -euo pipefail

script=$(cd "$(dirname "$0")" && pwd)/t3-extension-release.sh
checkout=$(cd "$(dirname "$0")/.." && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
failed=0
repo=$scratch/repo
fixtures=$scratch/fixtures
mkdir -p "$fixtures" "$scratch/bin"

# The stand-in answers `gh api <url> --jq <expr>` from fixtures/<url>.json,
# with / ? & = in the URL replaced by _, through jq as gh does.
cat > "$scratch/bin/gh" <<'EOF'
#!/usr/bin/env bash
[ "$1" = api ] && [ "$3" = --jq ] || { echo "stand-in gh: unexpected $*" >&2; exit 2; }
file=$FIXTURES/$(printf '%s' "$2" | tr '/?&=' '____').json
[ -f "$file" ] || { echo '{"workflow_runs":[],"jobs":[]}' | jq -r "$4"; exit 0; }
jq -r "$4" "$file"
EOF
chmod +x "$scratch/bin/gh"
export PATH="$scratch/bin:$PATH" FIXTURES=$fixtures GITHUB_REPOSITORY=o/r

g() { git -C "$repo" -c user.name=ci -c user.email=ci@ci.invalid "$@"; }

# merge <id> <path>...: a branch changing the paths, merged into master with
# a merge commit named as `atelier merge` names it.
merge() {
  local id=$1
  shift
  g switch -q -c "$id" master
  for path in "$@"; do
    mkdir -p "$repo/$(dirname "$path")"
    echo "$id" >> "$repo/$path"
  done
  g add -A
  g commit -qm "Change for $id"
  g switch -q master
  g merge -q --no-ff -m "Merge $id: Change $*" "$id"
  g rev-parse HEAD
}

# ci <commit> <run status> <run conclusion> [<extension job conclusion>]:
# GitHub's CI run on master for the commit. Run ids count up.
run_id=100
ci() {
  run_id=$((run_id + 1))
  printf '{"workflow_runs":[{"id":%s,"path":".github/workflows/ci.yml","status":"%s","conclusion":"%s"},{"id":9,"path":".github/workflows/v-release.yml","status":"completed","conclusion":"success"}]}\n' \
    "$run_id" "$2" "$3" > "$fixtures/repos_o_r_actions_runs_head_sha_$1_event_push_per_page_100.json"
  if [ -n "${4:-}" ]; then
    printf '{"jobs":[{"name":"changes","conclusion":"success"},{"name":"extension","conclusion":"%s"}]}\n' \
      "$4" > "$fixtures/repos_o_r_actions_runs_${run_id}_jobs.json"
  fi
}

# expect <accepted|refused> <case> <commit> <tested|reason>: check accepts
# the commit and prints <tested>, or refuses it with <reason> in its message.
expect() {
  local got
  if got=$(cd "$repo" && "$script" check "$3" master 2>"$scratch/err"); then
    if [ "$1" = accepted ] && [ "$got" = "$4" ]; then
      echo "ok   $2"
      return
    fi
    echo "FAIL $2: accepted with tested commit $got"
  elif [ "$1" = refused ] && grep -q -- "$4" "$scratch/err"; then
    echo "ok   $2"
    return
  else
    echo "FAIL $2: refused: $(cat "$scratch/err")"
  fi
  failed=1
}

git init -q -b master "$repo"
mkdir -p "$repo/t3-extension" "$repo/crates/atelier-core"
echo one > "$repo/t3-extension/custom.js"
echo one > "$repo/README.md"
g add -A
g commit -qm "Start"

extension=$(merge atelier-aaaa t3-extension/custom.js)
ci "$extension" completed success success
records1=$(merge atelier-bbbb .atelier/items/atelier-bbbb/item.md)
ci "$records1" completed success skipped
records2=$(merge backlog/x .atelier/items/atelier-cccc/item.md)
ci "$records2" completed success skipped
expect accepted extension-merge "$extension" "$extension"
expect accepted two-records-merges-later "$records2" "$extension"

core=$(merge atelier-dddd crates/atelier-core/src/lib.rs)
ci "$core" completed failure skipped
expect accepted core-change-does-not-test-extension "$core" "$extension"

desk=$(merge atelier-eeee crates/atelier-desk/src/api.rs)
ci "$desk" completed failure failure
records3=$(merge atelier-ffff .atelier/items/atelier-ffff/item.md)
ci "$records3" completed success skipped
expect refused desk-change-failed "$records3" "CI run [0-9]* for $desk concluded failure"

skipped=$(merge atelier-gggg t3-extension/custom.js)
ci "$skipped" completed success skipped
expect refused tested-commit-skipped-job "$skipped" "its extension job was skipped"

running=$(merge atelier-hhhh t3-extension/install.py)
ci "$running" in_progress ""
expect refused ci-not-finished "$running" "has not finished (in_progress)"

norun=$(merge atelier-iiii t3-extension/install.py)
expect refused no-ci-run "$norun" "no CI run on master"

g switch -q -c side master
echo side >> "$repo/t3-extension/custom.js"
g commit -qam "Side change"
expect refused not-on-master "$(g rev-parse HEAD)" "not a merge on master"
expect refused branch-commit-merged-to-master "$(g rev-parse atelier-aaaa)" "not a merge on master"
g switch -q master

# Release notes: the merges that changed t3-extension/ since the previous tag.
g tag t3-extension-v0.1.0 "$records2"
g tag t3-extension-v0.2.0 "$norun"
notes() {
  local got
  got=$(cd "$repo" && "$script" notes "$2" | sed -n 's/^- \([^:]*\):.*/\1/p' | paste -sd' ')
  if [ "$got" = "$3" ]; then echo "ok   $1"; else echo "FAIL $1: want $3, got $got"; failed=1; fi
}
# "Start" added t3-extension/, so it stands for the import, which is left out.
notes notes-first-release t3-extension-v0.1.0 "atelier-aaaa"
notes notes-since-previous-tag t3-extension-v0.2.0 "atelier-iiii atelier-hhhh atelier-gggg"

# The bundle builds the extension: this checkout's t3-extension/, as its
# files stand (tracked and new, not ignored), committed to the scratch master.
rm -rf "$repo/t3-extension"
git -C "$checkout" ls-files -z --cached --others --exclude-standard -- t3-extension |
  while IFS= read -r -d '' path; do
    [ -f "$checkout/$path" ] || continue
    mkdir -p "$repo/$(dirname "$path")"
    cp "$checkout/$path" "$repo/$path"
  done
g add -A
g commit -qm "The extension as this checkout has it"

# The bundle holds t3-extension/ at HEAD but the build's inputs, the built
# dist/custom.js and VERSION, under one directory.
out=$scratch/out
(cd "$repo" && "$script" bundle 0.2.0 "$out" >/dev/null 2>"$scratch/err") ||
  { echo "FAIL bundle: $(cat "$scratch/err")"; failed=1; }
mkdir "$scratch/unpacked" "$scratch/tree"
python3 -m zipfile -e "$out/t3-extension-0.2.0.zip" "$scratch/unpacked"
bundled=$scratch/unpacked/t3-extension-0.2.0
git -C "$repo" archive HEAD:t3-extension | tar -x -C "$scratch/tree"
(cd "$scratch/tree" && rm -rf src package.json pnpm-lock.yaml tsconfig.json vite.config.ts)
mkdir "$scratch/tree/dist"
cp "$bundled/dist/custom.js" "$scratch/tree/dist/custom.js"
echo 0.2.0 > "$scratch/tree/VERSION"
if diff -r "$scratch/tree" "$bundled" >/dev/null &&
  [ ! -e "$bundled/src" ] && [ ! -e "$bundled/node_modules" ] &&
  grep -q 'var T3Custom = ' "$bundled/dist/custom.js" &&
  (cd "$out" && sha256sum -c --quiet t3-extension-0.2.0.zip.sha256); then
  echo "ok   bundle"
else
  echo "FAIL bundle"
  failed=1
fi

# verify accepts it, installing it with no Node on PATH.
if (cd "$repo" && "$script" verify "$out/t3-extension-0.2.0.zip" 0.2.0 2>"$scratch/err"); then
  echo "ok   verify"
else
  echo "FAIL verify: $(cat "$scratch/err")"
  failed=1
fi

# verify refuses a bundle that carries src/, the build's input.
mkdir "$scratch/extra"
cp -r "$bundled" "$scratch/extra/"
mkdir "$scratch/extra/t3-extension-0.2.0/src"
cp "$repo/t3-extension/src/main.ts" "$scratch/extra/t3-extension-0.2.0/src/"
rm "$out/t3-extension-0.2.0.zip"
(cd "$scratch/extra" && python3 -m zipfile -c "$out/t3-extension-0.2.0.zip" t3-extension-0.2.0)
(cd "$out" && sha256sum t3-extension-0.2.0.zip > t3-extension-0.2.0.zip.sha256)
if (cd "$repo" && "$script" verify "$out/t3-extension-0.2.0.zip" 0.2.0 >/dev/null 2>"$scratch/err"); then
  echo "FAIL verify-refuses-src: accepted"
  failed=1
elif grep -q "differ from t3-extension/ at HEAD" "$scratch/err"; then
  echo "ok   verify-refuses-src"
else
  echo "FAIL verify-refuses-src: $(cat "$scratch/err")"
  failed=1
fi

exit "$failed"
