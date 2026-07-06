---
created_at: "2026-06-25T00:55:46.637651841+00:00"
id: "atelier-48ro"
evidence_type: "test"
captured_at: "2026-06-25T00:55:45.033701678+00:00"
command: "cargo test -p atelier-cli transition_options --lib"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3llx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3llx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli transition_options --lib"
updated_at: "2026-06-25T00:55:50.026896923+00:00"
---

## Summary

cargo test -p atelier-cli transition_options --lib

## Command

```console
cargo test -p atelier-cli transition_options --lib
```

Exit status: 0

## Stdout

Bytes: 408
Truncated: no

```text

running 3 tests
test commands::workflow::tests::transition_options_use_color_when_interactive_context_allows_it ... ok
test commands::workflow::tests::transition_options_stay_plain_when_stdout_is_not_interactive ... ok
test commands::workflow::tests::transition_options_stay_plain_when_no_color_is_set ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 150 filtered out; finished in 0.15s
```

## Stderr

Bytes: 219
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.37s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
```

