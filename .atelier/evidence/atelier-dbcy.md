---
created_at: "2026-06-20T21:19:17.306716350+00:00"
id: "atelier-dbcy"
evidence_type: "validation"
captured_at: "2026-06-20T21:19:17.143941311+00:00"
command: "bash -lc 'set -euo pipefail\nhelp_out=$(target/debug/atelier forgejo roles provision --help)\ncase \"$help_out\" in *\"--write-config\"*) echo \"$help_out\"; exit 1 ;; esac\nif target/debug/atelier forgejo roles provision --write-config >/tmp/forgejo-write-config.out 2>/tmp/forgejo-write-config.err; then\n  echo \"removed --write-config flag unexpectedly succeeded\"\n  exit 1\nfi\nrg -q \"unexpected argument .--write-config.\" /tmp/forgejo-write-config.err\nrg -q \"forgejo roles provision\" docs/product/command-audit/forgejo.md\necho \"forgejo roles provision help omits --write-config and rejects it as unexpected\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3d81"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3d81"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\nhelp_out=$(target/debug/atelier forgejo roles provision --help)\ncase \"$help_out\" in *\"--write-config\"*) echo \"$help_out\"; exit 1 ;; esac\nif target/debug/atelier forgejo roles provision --write-config >/tmp/forgejo-write-config.out 2>/tmp/forgejo-write-config.err; then\n  echo \"removed --write-config flag unexpectedly succeeded\"\n  exit 1\nfi\nrg -q \"unexpected argument .--write-config.\" /tmp/forgejo-write-config.err\nrg -q \"forgejo roles provision\" docs/product/command-audit/forgejo.md\necho \"forgejo roles provision help omits --write-config and rejects it as unexpected\"'"
updated_at: "2026-06-20T21:19:22.015895540+00:00"
---

## Summary

bash -lc 'set -euo pipefail
help_out=$(target/debug/atelier forgejo roles provision --help)
case "$help_out" in *"--write-config"*) echo "$help_out"; exit 1 ;; esac
if target/debug/atelier forgejo roles provision --write-config >/tmp/forgejo-write-config.out 2>/tmp/forgejo-write-config.err; then
  echo "removed --write-config flag unexpectedly succeeded"
  exit 1
fi
rg -q "unexpected argument .--write-config." /tmp/forgejo-write-config.err
rg -q "forgejo roles provision" docs/product/command-audit/forgejo.md
echo "forgejo roles provision help omits --write-config and rejects it as unexpected"'

## Command

```console
bash -lc 'set -euo pipefail
help_out=$(target/debug/atelier forgejo roles provision --help)
case "$help_out" in *"--write-config"*) echo "$help_out"; exit 1 ;; esac
if target/debug/atelier forgejo roles provision --write-config >/tmp/forgejo-write-config.out 2>/tmp/forgejo-write-config.err; then
  echo "removed --write-config flag unexpectedly succeeded"
  exit 1
fi
rg -q "unexpected argument .--write-config." /tmp/forgejo-write-config.err
rg -q "forgejo roles provision" docs/product/command-audit/forgejo.md
echo "forgejo roles provision help omits --write-config and rejects it as unexpected"'
```

Exit status: 0

## Stdout

Bytes: 79
Truncated: no

```text
forgejo roles provision help omits --write-config and rejects it as unexpected
```

## Stderr

Bytes: 0
Truncated: no

```text
```

