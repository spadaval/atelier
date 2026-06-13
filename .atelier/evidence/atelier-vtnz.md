---
created_at: "2026-06-13T22:41:23.616718781+00:00"
id: "atelier-vtnz"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T22:41:23.616679563+00:00\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"pass\",\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2rf7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Updated docs/product/cli-surface.md with a visible command-group contract covering job, default output, quiet output, drill-down path, and compatibility classification. Help review covered atelier --help, atelier mission --help, atelier evidence --help, atelier issue --help, atelier worktree --help, atelier status --help, plus focused subcommand help. Match: the docs now account for all help-visible root groups, including export/rebuild/import-beads/integrations/maintenance/diagnostics as specialized but visible surfaces. Remaining implementation drift: atelier evidence --help still exposes predecessor add/capture and peer attach, and mission-facing guidance still needs to stop teaching predecessor evidence flows. Follow-up implementation owner: atelier-u08r."
updated_at: "2026-06-13T22:41:25.282057167+00:00"
---

Updated docs/product/cli-surface.md with a visible command-group contract covering job, default output, quiet output, drill-down path, and compatibility classification. Help review covered atelier --help, atelier mission --help, atelier evidence --help, atelier issue --help, atelier worktree --help, atelier status --help, plus focused subcommand help. Match: the docs now account for all help-visible root groups, including export/rebuild/import-beads/integrations/maintenance/diagnostics as specialized but visible surfaces. Remaining implementation drift: atelier evidence --help still exposes predecessor add/capture and peer attach, and mission-facing guidance still needs to stop teaching predecessor evidence flows. Follow-up implementation owner: atelier-u08r.
