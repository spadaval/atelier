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
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Document installing a pinned Atelier release into a chosen directory"
updated_at: "2026-10-30T06:40:58.020419843+00:00"
---

## Goal

The Agent Factory eval suite (spadaval/.agents, evals/agent-factory) builds throwaway Atelier repositories and grades the state they end in, so a run is only comparable with another when both used the same binary. Today its fixtures use whatever `atelier` is on PATH (its 2026-10-30 baseline ran 0.2.0+5abb3651). The README documents only the `latest` installer, which also edits the shell profile. The release already supports pinning (`releases/download/v0.3.0/atelier-cli-installer.sh` returns 200) and `ATELIER_CLI_UNMANAGED_INSTALL=<dir>` installs to `<dir>/atelier` without touching HOME or PATH; say so. (`ATELIER_CLI_INSTALL_DIR` is not the right route: it installs to `<dir>/bin/atelier` and writes an updater receipt under `~/.config/atelier-cli`, which would point the user's upgrades at the eval copy.)

## Done when

README's Install section shows the versioned installer URL with `ATELIER_CLI_UNMANAGED_INSTALL=<dir>`. Following those lines in a clean HOME installs v0.3.0, `<dir>/atelier --version` prints `atelier 0.3.0+…`, and nothing at all is created or changed under HOME (`find "$HOME" -newer <marker>` lists nothing); the receipt records the commands.

## Scope

README.md only.

## Context

Checked 2026-10-30: fixture setup takes 0.1 to 0.2 s per scenario with the debug binary (done-when-rewrite 3.9 s, mostly its own benchmark), and `-q` on `issue create`, `evidence record`, `review finding` and `issue note` prints the new record's ID, so fixtures and graders need neither a faster builder nor deterministic IDs or timestamps.
