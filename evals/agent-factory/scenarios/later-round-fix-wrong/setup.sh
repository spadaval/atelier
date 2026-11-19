#!/usr/bin/env bash
# A later-round case's fixture, built by harness/later_round.sh: a small
# tool tracked by Atelier, and an item in round 2 of review, whose round-1
# findings are fixed. The agent is round 2's reviewer. See rubric.md for
# the flaw in the text round 1 reviewed and the fix left alone.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
exec bash "$here/../../harness/later_round.sh" "$here" "$1"
