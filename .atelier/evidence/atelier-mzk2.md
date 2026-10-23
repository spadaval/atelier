---
created_at: "2026-10-23T14:41:23.649313242+00:00"
id: "atelier-mzk2"
evidence_type: "test"
commit: "fa47f83a4dbb588e03d70cbfeed54763ddd47cbc"
result: "pass"
captured_at: "2026-10-23T14:41:20.461266505+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_check_)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-6rh7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-6rh7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "merge --check: failures first with fixes, 21 resolved fix_now findings collapse under a stale approval, exemption-only wording, per-finding and open-item lines, passing check lists passes and Next, verdict counts checks; quiet and real merge refusals"
updated_at: "2026-10-23T14:41:23.649803005+00:00"
---

## Summary

merge --check: failures first with fixes, 21 resolved fix_now findings collapse under a stale approval, exemption-only wording, per-finding and open-item lines, passing check lists passes and Next, verdict counts checks; quiet and real merge refusals

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(merge_check_)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 725
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.47s
────────────
 Nextest run ID 98c74607-0c5e-4b65-b57e-7ff6912e1eb1 with nextest profile: default
    Starting 2 tests across 10 binaries (896 tests skipped)
        PASS [   0.302s] (1/2) atelier-cli::merge_gates merge_check_names_each_finding_and_open_item_with_the_command_that_settles_it
        PASS [   1.463s] (2/2) atelier-cli::merge_gates merge_check_lists_failures_first_with_fixes_and_collapses_resolved_fix_now_findings
────────────
     Summary [   1.464s] 2 tests run: 2 passed, 896 skipped
```

