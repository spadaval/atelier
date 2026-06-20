---
created_at: "2026-06-20T21:24:43.992885548+00:00"
id: "atelier-fmyr"
evidence_type: "validation"
captured_at: "2026-06-20T21:24:43.672477130+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-oc4x > /tmp/atelier-oc4x-show.txt\nrg -q \"2 total \\| status: done=2\" /tmp/atelier-oc4x-show.txt\nrg -q \"linked_pull_request_merge_status_with_client\" crates/atelier-app/src/pr.rs\nrg -q \"app_pr::linked_pull_request_merge_status_with_client\" crates/atelier-cli/src/commands/workflow.rs\nhelp_out=$(target/debug/atelier forgejo roles provision --help)\ncase \"$help_out\" in *\"--write-config\"*) echo \"$help_out\"; exit 1 ;; esac\necho \"review/provider epic children done and retained surfaces validated\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-oc4x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-oc4x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ntarget/debug/atelier issue show atelier-oc4x > /tmp/atelier-oc4x-show.txt\nrg -q \"2 total \\| status: done=2\" /tmp/atelier-oc4x-show.txt\nrg -q \"linked_pull_request_merge_status_with_client\" crates/atelier-app/src/pr.rs\nrg -q \"app_pr::linked_pull_request_merge_status_with_client\" crates/atelier-cli/src/commands/workflow.rs\nhelp_out=$(target/debug/atelier forgejo roles provision --help)\ncase \"$help_out\" in *\"--write-config\"*) echo \"$help_out\"; exit 1 ;; esac\necho \"review/provider epic children done and retained surfaces validated\"'"
updated_at: "2026-06-20T21:24:48.898362285+00:00"
---

## Summary

bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-oc4x > /tmp/atelier-oc4x-show.txt
rg -q "2 total \| status: done=2" /tmp/atelier-oc4x-show.txt
rg -q "linked_pull_request_merge_status_with_client" crates/atelier-app/src/pr.rs
rg -q "app_pr::linked_pull_request_merge_status_with_client" crates/atelier-cli/src/commands/workflow.rs
help_out=$(target/debug/atelier forgejo roles provision --help)
case "$help_out" in *"--write-config"*) echo "$help_out"; exit 1 ;; esac
echo "review/provider epic children done and retained surfaces validated"'

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier issue show atelier-oc4x > /tmp/atelier-oc4x-show.txt
rg -q "2 total \| status: done=2" /tmp/atelier-oc4x-show.txt
rg -q "linked_pull_request_merge_status_with_client" crates/atelier-app/src/pr.rs
rg -q "app_pr::linked_pull_request_merge_status_with_client" crates/atelier-cli/src/commands/workflow.rs
help_out=$(target/debug/atelier forgejo roles provision --help)
case "$help_out" in *"--write-config"*) echo "$help_out"; exit 1 ;; esac
echo "review/provider epic children done and retained surfaces validated"'
```

Exit status: 0

## Stdout

Bytes: 67
Truncated: no

```text
review/provider epic children done and retained surfaces validated
```

## Stderr

Bytes: 0
Truncated: no

```text
```

