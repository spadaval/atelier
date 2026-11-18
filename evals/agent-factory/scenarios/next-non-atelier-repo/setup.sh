#!/usr/bin/env bash
# repo.sh's orders export: a small Python repository on main with no
# .atelier/ and no remote, and the binary under test first on PATH, where `atelier
# status` says this is not an Atelier repository.
set -euo pipefail
work=$1
here=$(cd "$(dirname "$0")" && pwd)
: "${EVAL_ATELIER:?set EVAL_ATELIER to the new atelier binary}"
bash "$here/repo.sh" "$work"
mkdir -p "$work/bin"
cp "$EVAL_ATELIER" "$work/bin/atelier"
git -C "$work/repo" config user.name "Eval Agent"
git -C "$work/repo" config user.email agent@example.com
