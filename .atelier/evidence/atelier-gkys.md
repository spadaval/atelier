---
created_at: "2026-11-11T03:48:32.815211222+00:00"
id: "atelier-gkys"
evidence_type: "test"
commit: "b6edd8e8b6d5abdc77b1c237f93ec8db35bda921"
result: "pass"
captured_at: "2026-11-11T03:48:32.623014101+00:00"
command: "bash -c 'set -e; export PATH=$HOME/.cache/atelier-qr5s/orb2/bin:$PATH; for c in status check show list new set ready drop close submit escalate review decide lead; do atelier $c --help >/dev/null; echo \"$c 0\"; done'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-orb2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-orb2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Every atelier command the new skill names has a --help that exits 0 in the prebuilt new binary"
updated_at: "2026-11-11T03:48:32.973957849+00:00"
---

## Summary

Every atelier command the new skill names has a --help that exits 0 in the prebuilt new binary

## Command

```console
bash -c 'set -e; export PATH=$HOME/.cache/atelier-qr5s/orb2/bin:$PATH; for c in status check show list new set ready drop close submit escalate review decide lead; do atelier $c --help >/dev/null; echo "$c 0"; done'
```

Exit status: 0

## Stdout

Bytes: 111
SHA-256: ae77a2d1ebb86cd365fe91b04376b567c024b8c781533ae0555fc5f1cfe14f98

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
