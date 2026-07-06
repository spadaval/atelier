---
created_at: "2026-07-06T18:52:12.738384507+00:00"
id: "atelier-vu1i"
evidence_type: "validation"
captured_at: "2026-07-06T18:52:12.521156054+00:00"
command: "bash -lc 'set -euo pipefail; doc=docs/spec/agent-factory/tracker-replacement-mvp.md; current=$(sed -n \"/## MVP Matrix/,/## Cutover Status/p\" \"$doc\"); if printf \"%s\\n\" \"$current\" | rg -i \"atelier (search|issue close|dep (add|remove)|lint|doctor|export|rebuild|import-beads|issue (claim|update [^|]*--claim)|session)\"; then echo \"retired Atelier command found in current MVP guidance\" >&2; exit 1; fi; for required in \"atelier issue list\" \"atelier issue show <id>\" \"atelier issue transition <id>\" \"atelier issue link <blocked> <blocker> --role blocked_by\" \"atelier issue unlink <blocked> <blocker> --role blocked_by\" \"atelier work ready\" \"atelier work blocked\" \"atelier check\" \"atelier check --fix\" \"atelier init --import-beads\" \"atelier evidence record\" \"atelier work missions\" \"atelier work mission <mission-id>\"; do rg -Fq \"$required\" \"$doc\" || { echo \"missing current path: $required\" >&2; exit 1; }; done; ! rg -Fq \"Atelier supports the following agent-facing command surface\" \"$doc\"; echo \"active MVP command guidance passed retired-path and required-current-path checks\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p0am"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "atelier-vqhi remediation: active MVP matrix and current crosswalk use supported command paths; retired forms remain only historical/non-normative"
updated_at: "2026-07-06T18:52:16.725721283+00:00"
---

## Summary

atelier-vqhi remediation: active MVP matrix and current crosswalk use supported command paths; retired forms remain only historical/non-normative

## Command

```console
bash -lc 'set -euo pipefail; doc=docs/spec/agent-factory/tracker-replacement-mvp.md; current=$(sed -n "/## MVP Matrix/,/## Cutover Status/p" "$doc"); if printf "%s\n" "$current" | rg -i "atelier (search|issue close|dep (add|remove)|lint|doctor|export|rebuild|import-beads|issue (claim|update [^|]*--claim)|session)"; then echo "retired Atelier command found in current MVP guidance" >&2; exit 1; fi; for required in "atelier issue list" "atelier issue show <id>" "atelier issue transition <id>" "atelier issue link <blocked> <blocker> --role blocked_by" "atelier issue unlink <blocked> <blocker> --role blocked_by" "atelier work ready" "atelier work blocked" "atelier check" "atelier check --fix" "atelier init --import-beads" "atelier evidence record" "atelier work missions" "atelier work mission <mission-id>"; do rg -Fq "$required" "$doc" || { echo "missing current path: $required" >&2; exit 1; }; done; ! rg -Fq "Atelier supports the following agent-facing command surface" "$doc"; echo "active MVP command guidance passed retired-path and required-current-path checks"'
```

Exit status: 0

## Stdout

Bytes: 81
Truncated: no

```text
active MVP command guidance passed retired-path and required-current-path checks
```

## Stderr

Bytes: 0
Truncated: no

```text
```

