---
created_at: "2026-10-17T01:10:57.979319063+00:00"
id: "atelier-t3ko"
issue_type: "bug"
labels:
- "cli-output"
- "setup"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nn9y"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make init report what it did and the real next steps"
updated_at: "2026-10-17T01:10:57.979319063+00:00"
---

## Goal

`atelier init` output (crates/atelier-cli/src/commands/init.rs:10-24) has several problems:
- The `Created` list omits `.gitignore`, `.atelier/issues`, `.atelier/evidence` and `.atelier/reviews`, which init creates.
- A re-run prints `Atelier initialized successfully!` again, even when nothing changed.
- `-q` is accepted but ignored.
- The comment columns are misaligned (`atelier issue create "Task"     #`).
- Next steps are only `atelier check` and `atelier issue create`. They leave out committing the install, `atelier setup claude`, and how the installing commit lands (`atelier land --install`, then normal review). The cross-cutting audit raised the same next-steps gap.

Status straight after init says `No specific next action is available from checkout state.` while three untracked install files sit in the checkout.

## Done when

- Every created path is listed.
- A re-run says `Already initialized; nothing changed` or lists what was reconciled.
- `-q` prints only the slug and base branch.
- Next steps read:
  ```
  Commit the install: git add .atelier .gitignore && git commit
  Install hooks:      atelier setup claude
  Create work:        atelier issue create "…"
  ```
  with a link to how the install lands.
- Tests cover a first run, a re-run and `-q`.
