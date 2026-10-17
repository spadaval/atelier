---
created_at: "2026-10-17T01:10:57.988977601+00:00"
id: "atelier-5rs0"
issue_type: "bug"
labels:
- "cli-output"
- "cross-cutting"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Exit cleanly when piped output is closed early"
updated_at: "2026-10-17T06:14:10.544747886+00:00"
---

## Goal

Any command that prints in several `println!` calls panics when the reader closes the pipe early, which `head` and `sed -n Np` do. `atelier issue show atelier-1w8c | head -1` prints the line and then:
```
thread 'main' (95740) panicked at .../library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
The exit status is 101. Four auditors hit this, on `issue show`, `review list`, `issue transition` and `evidence` commands. The panic text appears 62 times across 22 transcript files. Agents pipe to head and sed all the time, so they see a Rust backtrace hint plus a failing status. Single-string views (`work all`, `issue list`) don't panic, so the behaviour depends on how each view happens to print.

## Done when

- `atelier issue show <id> | head -1; echo ${PIPESTATUS[0]}` prints one line and `0` (or the conventional SIGPIPE status 141), with empty stderr.
- The same holds for `review list`, `issue transition <id>`, `evidence show <id>`, `evidence record ... -- true`, `issue create`, and `work mission <id>`.
- One integration test spawns the binary, closes its stdout after the first line, and asserts there is no `panicked` on stderr and the status isn't 101.

## Scope

Fix it once in `crates/atelier-cli/src/main.rs`: restore the default SIGPIPE disposition on Unix at start-up, or map `io::ErrorKind::BrokenPipe` to a clean exit. Don't patch individual commands.
