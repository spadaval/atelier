---
created_at: "2026-10-23T14:33:05.270779022+00:00"
id: "atelier-kyyq"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:33:02.984149061+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks new_review_titles_do_not_repeat_the_body"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-hulj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-hulj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "New finding and approval records carry a generated title (kind, owner, short pin) and store the text once"
updated_at: "2026-10-23T14:33:05.271171363+00:00"
---

## Summary

New finding and approval records carry a generated title (kind, owner, short pin) and store the text once

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks new_review_titles_do_not_repeat_the_body
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 564
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.87s
────────────
 Nextest run ID 88249633-8583-4960-a557-94f76091e680 with nextest profile: default
    Starting 1 test across 10 binaries (899 tests skipped)
        PASS [   0.134s] (1/1) atelier-cli::cli_integration review_views::new_review_titles_do_not_repeat_the_body
────────────
     Summary [   0.136s] 1 test run: 1 passed, 899 skipped
```

