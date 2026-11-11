#!/usr/bin/env bash
# Checks that make-clones.sh refuses, before writing anything, a clones path
# that resolves inside the projects directory or a source repository:
# directly, through a symbolic link, through a symlinked source, or through
# a symlinked work directory. It uses a scratch projects directory, never
# ~/projects.
#
#   desk/service/make-clones.test.sh
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "${TMPDIR:-/tmp}"
g=$(mktemp -d)
trap 'rm -rf "$g"' EXIT
mkdir -p "$g/projects" "$g/elsewhere"
for r in astromech demiurge life-on-rails cosplay-studio; do mkdir -p "$g/projects/$r/inner"; done
ln -s "$g/elsewhere" "$g/projects/atelier"
ln -s "$g/projects/astromech/inner" "$g/alias"
ln -s "$g/projects/demiurge" "$g/aside.work"
tree() { find -L "$g/projects" "$g/elsewhere" | sort; }
failed=0
for out in projects projects/astromech/new alias alias/deeper elsewhere/out projects/atelier/out aside; do
  before=$(tree)
  if DESK_PROJECTS=$g/projects "$here/make-clones.sh" "$g/$out" > "$g/said" 2>&1; then
    echo "FAIL  $out: accepted"; failed=1
  elif [ "$(tree)" != "$before" ]; then
    echo "FAIL  $out: refused, but wrote: $(diff <(echo "$before") <(tree) | grep '^>' | head -3)"; failed=1
  else
    echo "ok    $out: $(head -1 "$g/said")"
  fi
done
exit $failed
