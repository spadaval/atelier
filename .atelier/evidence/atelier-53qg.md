---
created_at: "2026-06-23T23:39:21.475513939+00:00"
id: "atelier-53qg"
evidence_type: "validation"
captured_at: "2026-06-23T23:39:12.259617357+00:00"
command: "sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli -E '\"'\"'test(mission_status_verbose_reference_targets_subcommand_help) or test(root_help_parser_includes_missions_section)'\"'\"' && target/debug/atelier lint atelier-c0qc && target/debug/atelier issue transition atelier-c0qc --options | grep -q '\"'\"'pass  command_surface_current'\"'\"' && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-3js3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-3js3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission command-surface drift parser validation"
updated_at: "2026-06-23T23:39:26.746897644+00:00"
---

## Summary

Mission command-surface drift parser validation

## Command

```console
sh -c 'cargo fmt -- --check && cargo check -p atelier-cli && cargo nextest run -p atelier-cli -E '"'"'test(mission_status_verbose_reference_targets_subcommand_help) or test(root_help_parser_includes_missions_section)'"'"' && target/debug/atelier lint atelier-c0qc && target/debug/atelier issue transition atelier-c0qc --options | grep -q '"'"'pass  command_surface_current'"'"' && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 71
Truncated: no

```text
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 789
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.99s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
────────────
 Nextest run ID ea34eeb3-6b7f-49de-b163-4910525e3a58 with nextest profile: default
    Starting 2 tests across 4 binaries (474 tests skipped)
        PASS [   0.010s] (1/2) atelier-cli command_surface::tests::mission_status_verbose_reference_targets_subcommand_help
        PASS [   0.011s] (2/2) atelier-cli command_surface::tests::root_help_parser_includes_missions_section
────────────
     Summary [   0.012s] 2 tests run: 2 passed, 474 skipped
```

