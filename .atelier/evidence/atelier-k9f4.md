---
created_at: "2026-07-09T16:44:53.674645672+00:00"
id: "atelier-k9f4"
evidence_type: "test"
captured_at: "2026-07-09T16:44:50.683823393+00:00"
command: "bash -lc 'set -euo pipefail; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; set +e; cargo nextest run --profile extended --run-ignored=only >\"$D/out\" 2>&1; STATUS=$?; set -e; test \"$STATUS\" -eq 4; grep -q \"Starting 0 tests across 9 binaries (684 tests skipped)\" \"$D/out\"; grep -q \"0 tests run: 0 passed, 684 skipped\" \"$D/out\"; grep -q \"error: no tests to run\" \"$D/out\"; echo \"PASS: ignored-only inventory is empty. The command returned expected nextest no-tests status 4 and reported 0 tests run, 684 skipped; no ignored validation scenario exists.\"; grep -E \"Starting 0 tests|0 tests run|error: no tests to run\" \"$D/out\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-g5fl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g5fl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail; D=$(mktemp -d); trap \"rm -rf $D\" EXIT; set +e; cargo nextest run --profile extended --run-ignored=only >\"$D/out\" 2>&1; STATUS=$?; set -e; test \"$STATUS\" -eq 4; grep -q \"Starting 0 tests across 9 binaries (684 tests skipped)\" \"$D/out\"; grep -q \"0 tests run: 0 passed, 684 skipped\" \"$D/out\"; grep -q \"error: no tests to run\" \"$D/out\"; echo \"PASS: ignored-only inventory is empty. The command returned expected nextest no-tests status 4 and reported 0 tests run, 684 skipped; no ignored validation scenario exists.\"; grep -E \"Starting 0 tests|0 tests run|error: no tests to run\" \"$D/out\"'"
updated_at: "2026-07-09T16:44:53.677808785+00:00"
---

## Summary

bash -lc 'set -euo pipefail; D=$(mktemp -d); trap "rm -rf $D" EXIT; set +e; cargo nextest run --profile extended --run-ignored=only >"$D/out" 2>&1; STATUS=$?; set -e; test "$STATUS" -eq 4; grep -q "Starting 0 tests across 9 binaries (684 tests skipped)" "$D/out"; grep -q "0 tests run: 0 passed, 684 skipped" "$D/out"; grep -q "error: no tests to run" "$D/out"; echo "PASS: ignored-only inventory is empty. The command returned expected nextest no-tests status 4 and reported 0 tests run, 684 skipped; no ignored validation scenario exists."; grep -E "Starting 0 tests|0 tests run|error: no tests to run" "$D/out"'

## Command

```console
bash -lc 'set -euo pipefail; D=$(mktemp -d); trap "rm -rf $D" EXIT; set +e; cargo nextest run --profile extended --run-ignored=only >"$D/out" 2>&1; STATUS=$?; set -e; test "$STATUS" -eq 4; grep -q "Starting 0 tests across 9 binaries (684 tests skipped)" "$D/out"; grep -q "0 tests run: 0 passed, 684 skipped" "$D/out"; grep -q "error: no tests to run" "$D/out"; echo "PASS: ignored-only inventory is empty. The command returned expected nextest no-tests status 4 and reported 0 tests run, 684 skipped; no ignored validation scenario exists."; grep -E "Starting 0 tests|0 tests run|error: no tests to run" "$D/out"'
```
Exit status: 0

## Stdout

Bytes: 315
Truncated: no

```text
PASS: ignored-only inventory is empty. The command returned expected nextest no-tests status 4 and reported 0 tests run, 684 skipped; no ignored validation scenario exists.
    Starting 0 tests across 9 binaries (684 tests skipped)
     Summary [   0.002s] 0 tests run: 0 passed, 684 skipped
error: no tests to run
```

## Stderr

Bytes: 0
Truncated: no

```text
```
