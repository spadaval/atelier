#!/usr/bin/env bash
# Makes or refreshes the converted scratch clones the Desk serves:
#
#   desk/service/make-clones.sh <clones directory> [<new atelier binary>]
#
# For each of atelier, astromech, demiurge, life-on-rails and cosplay-studio
# under ~/projects (or $DESK_PROJECTS), it clones the local repository's base
# branch into <clones directory>/<name>, points the clone's push URL at a path
# that does not exist so nothing can be pushed from it, converts its records
# with scripts/cutover/convert_records.py, commits the conversion, verifies it
# against a fresh conversion of the base, and runs the new `atelier check`.
#
# It only reads the repositories under ~/projects: `git clone` copies them
# (with --no-hardlinks, so no file is shared), and every later command runs
# in the clone. A clone is built in <clones directory>.work, outside the
# directory the Desk serves, and moved into place only when every step
# passed, so a failure leaves the clone the Desk serves as it was. Each
# repository's log is <clones directory>.work/<name>.log. It replaces only a directory that is already a scratch clone with
# pushes disabled. The new binary defaults to next/target/debug/atelier.
# It prints one line per step and exits non-zero when any repository failed.
set -euo pipefail
[ $# -ge 1 ] && [ $# -le 2 ] || { echo "usage: $0 <clones directory> [<new atelier binary>]" >&2; exit 2; }
here=$(cd "$(dirname "$0")" && pwd)
top=$(cd "$here/../.." && pwd)
convert=$top/scripts/cutover/convert_records.py
atelier=${2:-$top/next/target/debug/atelier}
projects=$(realpath -e -- "${DESK_PROJECTS:-$HOME/projects}")
NO_PUSH=/nonexistent/atelier-desk-scratch-clone
[ -x "$atelier" ] || { echo "No new atelier binary at $atelier: build it with cargo build -j 4 --manifest-path next/Cargo.toml" >&2; exit 2; }
[ -f "$convert" ] || { echo "No converter at $convert" >&2; exit 2; }
atelier=$(cd "$(dirname "$atelier")" && pwd)/$(basename "$atelier")
# Before anything is written: the clones and their work directory, with
# every symbolic link resolved, may not lie inside ~/projects or inside a
# repository the clones are made from.
out=$(realpath -m -- "$1")
work=$(realpath -m -- "$out.work")
inside() { case $1/ in "$2"/*) true ;; *) false ;; esac; }
sources=("$projects")
for r in atelier astromech demiurge life-on-rails cosplay-studio; do
  [ ! -e "$projects/$r" ] || sources+=("$(realpath -e -- "$projects/$r")")
done
for at in "${sources[@]}"; do
  for path in "$out" "$work"; do
    if inside "$path" "$at"; then
      echo "$1 needs $path, which is inside $at: give a scratch directory outside it" >&2
      exit 2
    fi
  done
done
mkdir -p "$out" "$work/tmp"
export TMPDIR=$work/tmp
# No harness's thread reaches the clones, and the conversion's commit names
# no person.
unset T3CODE_THREAD_ID CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID GROK_SESSION_ID
export GIT_AUTHOR_NAME="atelier-desk clones" GIT_AUTHOR_EMAIL=desk-clones@localhost.invalid
export GIT_COMMITTER_NAME=$GIT_AUTHOR_NAME GIT_COMMITTER_EMAIL=$GIT_AUTHOR_EMAIL
echo "Clones in $out, converted by $convert, checked by $atelier"

# A directory may be replaced only when it is a scratch clone that cannot push.
scratch_clone() {
  [ -d "$1/.git" ] && case $(git -C "$1" config --get remote.origin.pushurl || true) in /nonexistent/*) true ;; *) false ;; esac
}

note() { echo "$*" > "$work/note"; }

# One step, run in the new clone once it exists: its output to the
# repository's log, and one line.
step() {
  local name=$1
  shift
  if (cd "$work" && { [ ! -d "$new" ] || cd "$new"; } && "$@") >> "$log" 2>&1; then
    printf '%-15s %-8s ok  %s\n' "$repo" "$name" "$(cat "$work/note" 2>/dev/null)"
    rm -f "$work/note"
  else
    printf '%-15s %-8s FAILED; see %s\n' "$repo" "$name" "$log"
    return 1
  fi
}

# Each step chains its commands with &&: it runs as an `if` condition, where
# `set -e` does not apply.
clone() {
  git clone -q --no-hardlinks --single-branch -b "$base" "$src" "$new" &&
    git -C "$new" remote set-url --push origin "$NO_PUSH" &&
    [ "$(git -C "$new" remote get-url --push origin)" = "$NO_PUSH" ] &&
    note "$base at $(git -C "$new" rev-parse --short HEAD); pushes go to $NO_PUSH"
}
convert() {
  python3 "$convert" && git add -A && git commit -qm "Convert Atelier records to the new layout" &&
    note "$(ls .atelier/items | wc -l) items, $(ls .atelier/missions 2>/dev/null | wc -l) missions; committed $(git rev-parse --short HEAD)"
}
# The conversion is the clone's one new commit, so its parent is the base.
verify() { python3 "$convert" --verify HEAD^ > "$work/verify" && note "$(tail -1 "$work/verify")"; }
check() { "$atelier" check > "$work/check" && note "$(head -1 "$work/check")"; }

failed=0
for spec in atelier:master astromech:main demiurge:master life-on-rails:main cosplay-studio:main; do
  repo=${spec%%:*} base=${spec#*:} src=$projects/$repo
  dest=$out/$repo new=$work/$repo.new log=$work/$repo.log
  : > "$log"
  rm -rf "$new"
  if [ -e "$dest" ] && ! scratch_clone "$dest"; then
    printf '%-15s refused: %s is not a scratch clone with pushes disabled, so it is left alone\n' "$repo" "$dest"
    failed=1
    continue
  fi
  if step clone clone && step convert convert && step verify verify && step check check; then
    rm -rf "$work/$repo.old"
    [ ! -e "$dest" ] || mv "$dest" "$work/$repo.old"
    mv "$new" "$dest"
    rm -rf "$work/$repo.old"
  else
    rm -rf "$new"
    failed=1
  fi
done
if [ $failed -eq 0 ]; then
  echo "All five clones are converted and pass check."
else
  echo "Some clones failed; the Desk still serves the ones it had."
fi
exit $failed
