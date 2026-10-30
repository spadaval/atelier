---
created_at: "2026-10-30T06:40:58.020419843+00:00"
id: "atelier-hzuk"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Document installing a pinned Atelier release into a chosen directory"
updated_at: "2026-10-30T06:40:58.020419843+00:00"
---

## Goal

The Agent Factory eval suite (spadaval/.agents, evals/agent-factory) builds throwaway Atelier repositories and grades the state they end in, so a run is only comparable with another when both used the same binary. Today its fixtures use whatever `atelier` is on PATH (its 2026-10-30 baseline ran 0.2.0+5abb3651). The README documents only the `latest` installer, which also edits the shell profile. The release already supports pinning (`releases/download/v0.3.0/atelier-cli-installer.sh` returns 200) and the installer honours `ATELIER_CLI_INSTALL_DIR` and `ATELIER_CLI_NO_MODIFY_PATH`; say so.

## Done when

README's Install section shows the versioned installer URL and how to install into a chosen directory without changing the shell profile. Following those lines in a clean HOME installs v0.3.0 into the chosen directory, `<dir>/atelier --version` prints `atelier 0.3.0+…`, and no profile file is created or changed; the receipt records the commands.

## Scope

README.md only.

## Context

Checked 2026-10-30: fixture setup takes 0.1 to 0.2 s per scenario with the debug binary (done-when-rewrite 3.9 s, mostly its own benchmark), and `-q` on `issue create`, `evidence record`, `review finding` and `issue note` prints the new record's ID, so fixtures and graders need neither a faster builder nor deterministic IDs or timestamps.
