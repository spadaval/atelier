---
created_at: "2026-11-11T03:48:31.746278282+00:00"
id: "atelier-p0pc"
evidence_type: "test"
commit: "ff6b56fe471851cb286e77f7ebe24e5ee975fae0"
result: "pass"
captured_at: "2026-11-11T03:48:31.509004433+00:00"
command: "bash -c '! grep -rnE '\"'\"'atelier (land|issue|work|evidence|brief|debrief|doctor|bundle|migrate|prune|serve|init)|atelier review (request|approve|finding|defer|disposition|trivial|list|show|withdraw|reopen)|review request|merge --push|--wait|--sync|--check|--timeout|--json|workflow\\.yaml|\\.atelier/(issues|reviews|evidence)|docs/strategies|docs/adr|loose end|validation item|decision record|light path|debrief|/site|site page|\\bsupersede\\b|\\bdecline\\b|\\bland\\b'\"'\"' skills/agent-factory-next'"
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
title: "No file in the new skill names a removed command or record path (grep exits 1)"
updated_at: "2026-11-11T03:48:31.946061349+00:00"
---

## Summary

No file in the new skill names a removed command or record path (grep exits 1)

## Command

```console
bash -c '! grep -rnE '"'"'atelier (land|issue|work|evidence|brief|debrief|doctor|bundle|migrate|prune|serve|init)|atelier review (request|approve|finding|defer|disposition|trivial|list|show|withdraw|reopen)|review request|merge --push|--wait|--sync|--check|--timeout|--json|workflow\.yaml|\.atelier/(issues|reviews|evidence)|docs/strategies|docs/adr|loose end|validation item|decision record|light path|debrief|/site|site page|\bsupersede\b|\bdecline\b|\bland\b'"'"' skills/agent-factory-next'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
