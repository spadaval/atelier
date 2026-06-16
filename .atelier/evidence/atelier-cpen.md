---
created_at: "2026-06-16T17:47:44.912504641+00:00"
id: "atelier-cpen"
evidence_type: "validation"
captured_at: "2026-06-16T17:47:44.511125189+00:00"
command: "bash -lc 'rg -n \"doctor --fix|Refresh canonical export|failed to run atelier export --check\" crates/atelier-app/src crates/atelier-cli/src crates/atelier-cli/tests -g \"*.rs\"; ! rg -n \"Refresh canonical export|failed to run atelier export --check\" crates/atelier-app/src crates/atelier-cli/src crates/atelier-cli/tests -g \"*.rs\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-a7gd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-a7gd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "normal recovery text no longer routes through export or rebuild"
updated_at: "2026-06-16T17:47:48.332098897+00:00"
---

## Summary

normal recovery text no longer routes through export or rebuild

## Command

```console
bash -lc 'rg -n "doctor --fix|Refresh canonical export|failed to run atelier export --check" crates/atelier-app/src crates/atelier-cli/src crates/atelier-cli/tests -g "*.rs"; ! rg -n "Refresh canonical export|failed to run atelier export --check" crates/atelier-app/src crates/atelier-cli/src crates/atelier-cli/tests -g "*.rs"'
```

Exit status: 0

## Stdout

Bytes: 3540
Truncated: no

```text
crates/atelier-cli/src/main.rs:72:  atelier doctor --fix
crates/atelier-cli/src/main.rs:151:    /// Advanced projection diagnostic; normal local repair uses doctor --fix
crates/atelier-cli/tests/smoke/cli_data.rs:128:            && result.stderr.contains("3. run `atelier doctor --fix`")
crates/atelier-cli/src/commands/status.rs:255:            "  Repair ignored projection state ({} stale record(s)): atelier doctor --fix",
crates/atelier-cli/src/commands/man.rs:209:            println!("  3. atelier doctor --fix - Repair ignored local state when safe.");
crates/atelier-cli/src/commands/man.rs:244:            println!("  atelier doctor --fix");
crates/atelier-app/src/health.rs:47:            "doctor --fix refused to edit tracked `.atelier/` canonical records; \
crates/atelier-app/src/health.rs:48:             run `atelier lint`, fix the named canonical Markdown record, then rerun `atelier doctor --fix`"
crates/atelier-app/src/health.rs:53:                    "doctor --fix failed while repairing ignored local projection state at {}",
crates/atelier-app/src/export.rs:63:            "Refusing to write canonical tracker records from the local projection:\n{}\nrecovery: 1. run `atelier lint`; 2. fix any named canonical Markdown records; 3. run `atelier doctor --fix` for ignored local projection/runtime state; 4. rerun the blocked command",
crates/atelier-app/src/export.rs:270:            "invalid: canonical tracker Markdown is invalid while running a deterministic export diagnostic: {error:#}\nrecovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command"
crates/atelier-app/src/command_storage.rs:129:                    tracing::warn!("Recovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command before closing or mutating work.");
crates/atelier-app/src/command_storage.rs:170:            "{prefix}; recovery: 1. run `atelier lint`; 2. fix the named canonical Markdown record; 3. run `atelier doctor --fix`; 4. rerun the blocked command."
crates/atelier-cli/tests/cli_integration/mission_projection_worktree.rs:1840:            && stderr.contains("3. run `atelier doctor --fix`")
crates/atelier-cli/tests/cli_integration/mission_projection_worktree.rs:2099:            && stderr.contains("3. run `atelier doctor --fix`")
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:185:    assert!(success, "doctor --fix failed for missing db: {stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:198:        "doctor --fix failed for stale projection: {stderr}"
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:204:    assert!(success, "issue show failed after doctor --fix: {stderr}");
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:225:        "doctor --fix must fail on malformed canonical Markdown"
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:229:        "doctor --fix should not print repair success"
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:232:        stderr.contains("doctor --fix refused to edit tracked `.atelier/` canonical records")
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:236:        "unexpected doctor --fix refusal: {stderr}"
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:241:        "doctor --fix must not rewrite malformed tracked canonical Markdown"
crates/atelier-cli/tests/cli_integration/setup_guidance.rs:677:        "atelier doctor --fix",
```

## Stderr

Bytes: 0
Truncated: no

```text
```
