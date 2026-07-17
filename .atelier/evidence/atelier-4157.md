---
created_at: "2026-07-16T23:43:46.993485441+00:00"
id: "atelier-4157"
evidence_type: "validation"
captured_at: "2026-07-16T23:43:35.263275390+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo check --workspace --all-targets && cargo nextest run --profile extended --run-ignored=only --no-tests=pass && git diff --check && git diff 6399b635..b720ee05944d081089da7712f1b60f68049fe0ab --check && ./target/debug/atelier check atelier-p2wk && ./target/debug/atelier check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p2wk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p2wk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && cargo check --workspace --all-targets && cargo nextest run --profile extended --run-ignored=only --no-tests=pass && git diff --check && git diff 6399b635..b720ee05944d081089da7712f1b60f68049fe0ab --check && ./target/debug/atelier check atelier-p2wk && ./target/debug/atelier check'"
updated_at: "2026-07-16T23:43:46.995460094+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && cargo check --workspace --all-targets && cargo nextest run --profile extended --run-ignored=only --no-tests=pass && git diff --check && git diff 6399b635..b720ee05944d081089da7712f1b60f68049fe0ab --check && ./target/debug/atelier check atelier-p2wk && ./target/debug/atelier check'

## Command

```console
bash -lc 'cargo fmt -- --check && cargo check --workspace --all-targets && cargo nextest run --profile extended --run-ignored=only --no-tests=pass && git diff --check && git diff 6399b635..b720ee05944d081089da7712f1b60f68049fe0ab --check && ./target/debug/atelier check atelier-p2wk && ./target/debug/atelier check'
```

Exit status: 0

## Stdout

Bytes: 26
Truncated: no

```text
Lint passed.
Lint passed.
```

## Stderr

Bytes: 602
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p2wk/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.58s
────────────
 Nextest run ID 8b94f25b-9139-4d71-94c2-8aa7b964b55a with nextest profile: extended
    Starting 0 tests across 9 binaries (781 tests skipped)
────────────
     Summary [   0.001s] 0 tests run: 0 passed, 781 skipped
```
