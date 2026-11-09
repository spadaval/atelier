---
created_at: "2026-11-09T16:43:58.464994642+00:00"
id: "atelier-pqyi"
evidence_type: "test"
commit: "b7c839c1b46c70f668c05c810bb0bdc3127944d8"
result: "pass"
captured_at: "2026-11-09T16:43:56.319480401+00:00"
command: "cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve api::"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-zpmb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-zpmb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "API tests: fixture JSON is current, brief and debrief counts and titles match atelier brief -q and debrief -q on four fixture missions (draft, closed, with and without strategy, proposed initial status with ready in the same category), review choice, JSON 404s, a broken repository answers 500 on its own routes while others answer, views built once per base commit"
updated_at: "2026-11-09T16:43:58.594207442+00:00"
---

## Summary

API tests: fixture JSON is current, brief and debrief counts and titles match atelier brief -q and debrief -q on four fixture missions (draft, closed, with and without strategy, proposed initial status with ready in the same category), review choice, JSON 404s, a broken repository answers 500 on its own routes while others answer, views built once per base commit

## Command

```console
cargo nextest run -j 4 --build-jobs 4 -p atelier-cli --test serve api::
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 1148
SHA-256: 31806007552bbdb723f18bd8d8e8d56542d285772f5991c4d4bbd3a5439e31d8
Test summary: Summary [   0.471s] 6 tests run: 6 passed, 9 skipped
