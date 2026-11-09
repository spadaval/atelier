---
created_at: "2026-11-09T07:58:23.755852734+00:00"
id: "atelier-vhps"
evidence_type: "validation"
commit: "3cb961ba9a85378701549135f55c71fdc3238fdd"
result: "pass"
captured_at: "2026-11-09T07:58:23.616917410+00:00"
command: "bash -c '! git grep -n -E \"gh pr create|gh pr merge|git push\" AGENTS.md skills/'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7o4r"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7o4r"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "AGENTS.md and skills/ tell agents to run no gh pr create, gh pr merge or git push"
updated_at: "2026-11-09T07:58:23.886050387+00:00"
---

## Summary

AGENTS.md and skills/ tell agents to run no gh pr create, gh pr merge or git push

## Command

```console
bash -c '! git grep -n -E "gh pr create|gh pr merge|git push" AGENTS.md skills/'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
