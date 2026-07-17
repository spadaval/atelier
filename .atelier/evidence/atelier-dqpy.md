---
created_at: "2026-07-17T00:06:07.210799047+00:00"
id: "atelier-dqpy"
evidence_type: "validation"
captured_at: "2026-07-17T00:06:00.594621855+00:00"
command: "bash -lc '\nset -euo pipefail\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\n$bin check atelier-h3wg\n$bin check atelier-qi40\ngit diff --check 3f82f97c447a5f59fc186f5d3a8d513fbf9e1300..5e2feebcbe4812b0c2e529a180e8e0be11151b7a\nif rg -n \"#\\[ignore\" crates/atelier-cli/tests/cli_integration/setup_guidance.rs crates/atelier-cli/tests/cli_integration/mission_plan_review_lifecycle.rs; then exit 1; fi\nprintf \"%s\\n\" \"PASS tracker checks h3wg+qi40; reviewed implementation diff whitespace-clean; no ignored markers in focused test modules\"\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-h3wg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h3wg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc '\nset -euo pipefail\nbin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier\n$bin check atelier-h3wg\n$bin check atelier-qi40\ngit diff --check 3f82f97c447a5f59fc186f5d3a8d513fbf9e1300..5e2feebcbe4812b0c2e529a180e8e0be11151b7a\nif rg -n \"#\\[ignore\" crates/atelier-cli/tests/cli_integration/setup_guidance.rs crates/atelier-cli/tests/cli_integration/mission_plan_review_lifecycle.rs; then exit 1; fi\nprintf \"%s\\n\" \"PASS tracker checks h3wg+qi40; reviewed implementation diff whitespace-clean; no ignored markers in focused test modules\"\n'"
updated_at: "2026-07-17T00:06:07.212931603+00:00"
---

## Summary

bash -lc '
set -euo pipefail
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
$bin check atelier-h3wg
$bin check atelier-qi40
git diff --check 3f82f97c447a5f59fc186f5d3a8d513fbf9e1300..5e2feebcbe4812b0c2e529a180e8e0be11151b7a
if rg -n "#\[ignore" crates/atelier-cli/tests/cli_integration/setup_guidance.rs crates/atelier-cli/tests/cli_integration/mission_plan_review_lifecycle.rs; then exit 1; fi
printf "%s\n" "PASS tracker checks h3wg+qi40; reviewed implementation diff whitespace-clean; no ignored markers in focused test modules"
'

## Command

```console
bash -lc '
set -euo pipefail
bin=/root/atelier-worktrees/atelier-p2wk/target/debug/atelier
$bin check atelier-h3wg
$bin check atelier-qi40
git diff --check 3f82f97c447a5f59fc186f5d3a8d513fbf9e1300..5e2feebcbe4812b0c2e529a180e8e0be11151b7a
if rg -n "#\[ignore" crates/atelier-cli/tests/cli_integration/setup_guidance.rs crates/atelier-cli/tests/cli_integration/mission_plan_review_lifecycle.rs; then exit 1; fi
printf "%s\n" "PASS tracker checks h3wg+qi40; reviewed implementation diff whitespace-clean; no ignored markers in focused test modules"
'
```

Exit status: 0

## Stdout

Bytes: 147
Truncated: no

```text
Lint passed.
Lint passed.
PASS tracker checks h3wg+qi40; reviewed implementation diff whitespace-clean; no ignored markers in focused test modules
```

## Stderr

Bytes: 0
Truncated: no

```text
```
