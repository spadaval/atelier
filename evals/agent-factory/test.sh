#!/usr/bin/env bash
# Runs the agent-factory evals' own tests, which start no agent and call no
# model: harness/test_run.py, and each scenario's test_grade.py, which runs
# its setup.sh and walks the binary through passing and failing paths.
# CI's evals job runs this only when evals/ or CI config changes.
#
# Usage: evals/agent-factory/test.sh [<atelier binary>]
# The binary defaults to target/debug/atelier; build it first with
# `cargo build -p atelier-cli`.
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
bin=${1:-$here/../../target/debug/atelier}
[ -x "$bin" ] || { echo "No atelier binary at $bin: build it with cargo build -p atelier-cli." >&2; exit 2; }
export EVAL_ATELIER=$(realpath "$bin") ATELIER_NO_UPDATE_CHECK=1
unset ATELIER_SKIP T3CODE_THREAD_ID CLAUDE_CODE_SESSION_ID CODEX_SESSION_ID CODEX_THREAD_ID

failed=()
echo "== harness"
(cd "$here/harness" && python3 -m unittest test_run test_confine) || failed+=(harness)
for dir in "$here"/scenarios/*/; do
    name=$(basename "$dir")
    echo "== $name"
    if [ ! -f "$dir/test_grade.py" ]; then
        echo "$name has no test_grade.py; every scenario's grader needs fixtures." >&2
        failed+=("$name")
        continue
    fi
    (cd "$dir" && python3 -m unittest) || failed+=("$name")
done
if [ ${#failed[@]} -gt 0 ]; then
    echo "Failed: ${failed[*]}" >&2
    exit 1
fi
echo "All eval fixtures and harness tests pass."
