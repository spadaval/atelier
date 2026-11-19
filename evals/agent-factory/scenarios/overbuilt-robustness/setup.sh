#!/usr/bin/env bash
# A sized case's fixture, built by harness/sized.sh from fixture/: a small
# tool tracked by Atelier, and an item's correct change in review. The
# agent is its reviewer. See rubric.md for what the change holds.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
exec bash "$here/../../harness/sized.sh" "$here" "$1"
