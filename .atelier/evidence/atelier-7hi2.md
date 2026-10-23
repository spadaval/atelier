---
created_at: "2026-10-23T17:31:23.799749072+00:00"
id: "atelier-7hi2"
evidence_type: "test"
commit: "d5a4bebcd353188a3cc4880716c0732a5f494519"
result: "pass"
captured_at: "2026-10-23T17:31:21.364477112+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(hook_payload_errors_are_reported_apart_from_landing_failures) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'"
exit_status: "0"
agent_identity: "claude-implementer-buqu"
target:
  kind: "issue"
  id: "atelier-ft88"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ft88"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Hook payload errors are a non-blocking misconfigured systemMessage (Stop and SessionStart); a real failure blocks with land's refusal verbatim"
updated_at: "2026-10-23T17:31:23.800054492+00:00"
---

## Summary

Hook payload errors are a non-blocking misconfigured systemMessage (Stop and SessionStart); a real failure blocks with land's refusal verbatim

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(hook_payload_errors_are_reported_apart_from_landing_failures) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 678
SHA-256: b48ba4b0089709b08684c18be9c81f306f79930e8e01deb1085e2841860619c9
Test summary: Summary [   0.410s] 2 tests run: 2 passed, 742 skipped
