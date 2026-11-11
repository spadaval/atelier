---
created_at: "2026-11-11T09:50:30.016649080+00:00"
id: "atelier-ngib"
evidence_type: "validation"
commit: "b22a03359013ee4d041d0acf14a13a608b152806"
result: "pass"
captured_at: "2026-11-11T09:50:29.840587440+00:00"
command: "bash -c '! grep -rnE '\"'\"'atelier (land|issue|work|evidence|brief|debrief|doctor|bundle|migrate|prune|serve|init)|atelier review (request|approve|finding|defer|disposition|trivial|list|show|withdraw|reopen)|review request|merge --push|--wait|--sync|--check|--timeout|--json|workflow\\.yaml|\\.atelier/(issues|reviews|evidence)|docs/strategies|docs/adr|loose end|validation item|decision record|light path|debrief|/site|site page|\\bsupersede\\b|\\bdecline\\b|\\bland\\b'\"'\"' skills/agent-factory-next && wc -l skills/agent-factory-next/SKILL.md skills/agent-factory-next/roles/*.md skills/agent-factory-next/procedures/*.md'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t747"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t747"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "atelier-orb2's removed-name search finds nothing in the cut skill"
updated_at: "2026-11-11T09:50:30.185042307+00:00"
---

## Summary

atelier-orb2's removed-name search finds nothing in the cut skill

## Command

```console
bash -c '! grep -rnE '"'"'atelier (land|issue|work|evidence|brief|debrief|doctor|bundle|migrate|prune|serve|init)|atelier review (request|approve|finding|defer|disposition|trivial|list|show|withdraw|reopen)|review request|merge --push|--wait|--sync|--check|--timeout|--json|workflow\.yaml|\.atelier/(issues|reviews|evidence)|docs/strategies|docs/adr|loose end|validation item|decision record|light path|debrief|/site|site page|\bsupersede\b|\bdecline\b|\bland\b'"'"' skills/agent-factory-next && wc -l skills/agent-factory-next/SKILL.md skills/agent-factory-next/roles/*.md skills/agent-factory-next/procedures/*.md'
```

Exit status: 0

## Stdout

Bytes: 457
SHA-256: d28dede79aa81976f2b009fb59af6a5426c750aa7a7ccf4144066e1d9053a1a6

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
