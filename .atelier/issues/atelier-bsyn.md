---
created_at: "2026-10-17T06:31:32.769643923+00:00"
id: "atelier-bsyn"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
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
title: "Report a dead git cat-file child instead of ending on SIGPIPE"
updated_at: "2026-10-17T08:41:51.157406002+00:00"
---

## Goal

atelier-5rs0 restored the default SIGPIPE disposition at start-up, so an early-closing stdout reader ends atelier quietly. The disposition covers the whole process, including `ObjectReader` writes to the long-running `git cat-file --batch` child (`crates/atelier-app/src/git_freshness.rs`, about line 75). If that child has exited, atelier is now killed with status 141 and no message, where before the write failed and views degraded but still printed. Reproduced by the atelier-5rs0 reviewer with a fake `git` whose `cat-file` exits 128 (finding atelier-spnf); not reproduced with real git.

## Done when

- A dead `cat-file` child is reported as an error naming git, not a silent 141. For example, check `try_wait` before writing, or run the reader with SIGPIPE ignored around its writes.
- A test with a `cat-file` that exits at once shows the error and no SIGPIPE death.
