---
created_at: "2026-06-20T21:53:14.706615977+00:00"
id: "atelier-iay2"
evidence_type: "validation"
captured_at: "2026-06-20T21:53:11.419842660+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ngit diff --check\nif rg -n \"commands::plan|pub mod tested|pub mod label|pub mod plan|commands::tested|commands::label|super::plan|Commands::(Plan|Tested|Label)\" crates/atelier-cli/src crates/atelier-cli/tests docs/product/command-audit; then\n  exit 1\nfi\ncargo build -p atelier-cli\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-6hcl"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6hcl"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Retired command module cleanup validation"
updated_at: "2026-06-20T21:53:19.328255178+00:00"
---

## Summary

Retired command module cleanup validation

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
git diff --check
if rg -n "commands::plan|pub mod tested|pub mod label|pub mod plan|commands::tested|commands::label|super::plan|Commands::(Plan|Tested|Label)" crates/atelier-cli/src crates/atelier-cli/tests docs/product/command-audit; then
  exit 1
fi
cargo build -p atelier-cli
'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 139
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.02s
```

