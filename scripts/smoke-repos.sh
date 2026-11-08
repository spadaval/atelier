#!/usr/bin/env bash
# Smoke-test a new atelier binary against every local Atelier repository.
#
# It builds this tree's binary (target/debug/atelier), clones each repository
# into a scratch directory, and runs the same read-only commands there with the
# new binary and the installed one. It fails when the new binary errors or
# refuses where the installed one succeeds, which is how a change that breaks
# another repository's records shows up before review and before install.
#
# Commands: check, doctor, status, work missions, and review request on every
# local branch that an unfinished item owns (checked out in the clone).
#
# The repositories themselves are only read: each binary gets its own clone,
# made with `git clone --mirror` so every local branch comes along, and the
# clone's push URL is disabled. Uncommitted changes in a repository are not
# cloned.
#
# Usage: scripts/smoke-repos.sh [options] [REPO...]
#   REPO               repository to test; default: every directory under the
#                      scan root with a .atelier folder, one per git repository
#                      (worktrees of the same repository are skipped)
#   --root DIR         scan root [default: the directory holding this
#                      repository's main checkout]
#   --new PATH         new binary to test instead of building this tree
#   --installed PATH   baseline binary [default: atelier on PATH]
#   --keep             keep the scratch clones and command output
#
# Exit status: 0 when no regression, 1 when the new binary fails where the
# installed one passes, 2 on a usage or setup error.
set -euo pipefail

usage() { sed -n '/^# Usage:/,/^# Exit status/p' "$0" | sed 's/^# \{0,1\}//'; }

repo_root=$(cd "$(dirname "$0")/.." && pwd)
main_checkout=$(dirname "$(git -C "$repo_root" rev-parse --path-format=absolute --git-common-dir)")
root=$(dirname "$main_checkout")
new_bin=""
installed_bin=""
keep=0
repos=()

while [ $# -gt 0 ]; do
  case "$1" in
    --root) root=$2; shift 2 ;;
    --new) new_bin=$2; shift 2 ;;
    --installed) installed_bin=$2; shift 2 ;;
    --keep) keep=1; shift ;;
    -h|--help) usage; exit 0 ;;
    -*) echo "smoke-repos: unknown option $1" >&2; usage >&2; exit 2 ;;
    *) repos+=("$1"); shift ;;
  esac
done

if [ -z "$installed_bin" ]; then
  installed_bin=$(command -v atelier) || { echo "smoke-repos: no installed atelier on PATH; pass --installed" >&2; exit 2; }
fi
if [ -z "$new_bin" ]; then
  echo "Building the new binary: cargo build --locked -p atelier-cli"
  (cd "$repo_root" && cargo build --locked -q -p atelier-cli) || exit 2
  new_bin=$repo_root/target/debug/atelier
fi
new_bin=$(realpath "$new_bin")
installed_bin=$(realpath "$installed_bin")
echo "New:       $new_bin ($("$new_bin" --version))"
echo "Installed: $installed_bin ($("$installed_bin" --version))"

declare -A seen=() aliases=()
# One source per git repository: worktrees and the main checkout share a
# common dir, and the main checkout is what gets cloned.
if [ ${#repos[@]} -eq 0 ]; then
  for dir in "$root"/*/; do
    [ -d "$dir.atelier" ] || continue
    common=$(git -C "$dir" rev-parse --path-format=absolute --git-common-dir 2>/dev/null) || continue
    src=$(dirname "$common")
    [ -d "$src/.atelier" ] || continue
    if [ -z "${seen[$src]:-}" ]; then
      seen[$src]=1
      repos+=("$src")
    fi
    # A symlink to a repository (worldcrafter -> demiurge) is that repository.
    dir=${dir%/}
    if [ -L "$dir" ] && [ "$(realpath "$dir")" = "$src" ]; then
      aliases[$src]+=" $(basename "$dir")"
    fi
  done
fi
if [ ${#repos[@]} -eq 0 ]; then
  echo "smoke-repos: no Atelier repositories found under $root" >&2
  exit 2
fi

scratch=$(mktemp -d "${TMPDIR:-/tmp}/atelier-smoke.XXXXXX")
if [ "$keep" -eq 1 ]; then
  echo "Scratch:   $scratch (kept)"
else
  trap 'rm -rf "$scratch"' EXIT
fi
echo

# clone SRC DEST: a working copy of every ref in SRC that cannot push back.
clone() {
  git clone -q --mirror "$1" "$2/.git"
  git -C "$2" config core.bare false
  git -C "$2" config remote.origin.pushurl /dev/null/atelier-smoke-no-push
  git -C "$2" checkout -q -f
}

# snapshot SRC: its refs, HEAD and working tree state, to show the run left it
# alone.
snapshot() {
  {
    git -C "$1" for-each-ref --format='%(refname) %(objectname)'
    git -C "$1" rev-parse HEAD
    git -C "$1" status --porcelain=v1 --untracked-files=all
  } | sha256sum
}

# Branches an unfinished item owns, as "branch item" lines, read from each
# branch's own records.
owned_branches() {
  local clone=$1 branch file id
  git -C "$clone" for-each-ref --format='%(refname:short)' refs/heads | while read -r branch; do
    file=$(git -C "$clone" grep -l -E "^  work_branch: \"?${branch}\"?$" "refs/heads/$branch" -- .atelier/issues 2>/dev/null | head -n1) || true
    [ -n "$file" ] || continue
    file=${file#refs/heads/"$branch":}
    git -C "$clone" show "refs/heads/$branch:$file" | grep -q -E '^status: "?(done|superseded)"?$' && continue
    id=$(basename "$file" .md)
    echo "$branch $id"
  done
}

failures=0
for src in "${repos[@]}"; do
  name=$(basename "$src")
  work=$scratch/$name
  mkdir -p "$work"
  clone "$src" "$work/installed"
  clone "$src" "$work/new"
  head=$(git -C "$src" rev-parse --short HEAD)
  dirty=$(git -C "$src" status --porcelain | wc -l)
  note=""
  [ "$dirty" -gt 0 ] && note=" ($dirty uncommitted paths not cloned)"
  [ -n "${aliases[$src]:-}" ] && note="$note (also linked as${aliases[$src]})"
  echo "== $name @ $head$note"
  before=$(snapshot "$src")

  # Each step is "label|branch to check out or -|atelier args".
  steps=("check|-|check" "doctor|-|doctor" "status|-|status" "work missions|-|work missions")
  while read -r branch id; do
    [ -n "$branch" ] || continue
    steps+=("review request $id|$branch|review request $id")
  done < <(owned_branches "$work/installed")

  repo_failures=0
  n=0
  for step in "${steps[@]}"; do
    IFS='|' read -r label branch args <<<"$step"
    n=$((n + 1))
    codes=()
    for side in installed new; do
      bin=$installed_bin
      [ "$side" = new ] && bin=$new_bin
      dir=$work/$side
      [ "$branch" = - ] || git -C "$dir" checkout -q -f "$branch"
      read -r -a argv <<<"$args"
      set +e
      (cd "$dir" && NO_COLOR=1 timeout 300 "$bin" "${argv[@]}") >"$work/$n.$side.out" 2>&1
      codes+=($?)
      set -e
    done
    ci=${codes[0]} cn=${codes[1]}
    if [ "$ci" -eq 0 ] && [ "$cn" -ne 0 ]; then
      verdict="REGRESSION"
      repo_failures=$((repo_failures + 1))
    elif [ "$ci" -ne 0 ] && [ "$cn" -eq 0 ]; then
      verdict="new passes"
    else
      verdict="same"
    fi
    printf '  %-40s installed %3s  new %3s  %s\n' "$label" "$ci" "$cn" "$verdict"
    if [ "$verdict" = REGRESSION ]; then
      sed 's/^/      new: /' "$work/$n.new.out" | tail -n 15
    fi
  done
  if [ "$repo_failures" -eq 0 ]; then
    echo "  ok: ${#steps[@]} commands, no regression"
  else
    echo "  FAIL: $repo_failures of ${#steps[@]} commands regressed"
  fi
  failures=$((failures + repo_failures))
  if [ "$(snapshot "$src")" != "$before" ]; then
    echo "  warning: $src changed during the run (refs, HEAD or working tree); was someone working there?"
  fi
  echo
done

if [ "$failures" -gt 0 ]; then
  echo "smoke-repos: $failures regression(s): the new binary fails where the installed one passes."
  exit 1
fi
echo "smoke-repos: no regression across ${#repos[@]} repositories."
