---
created_at: "2026-06-19T04:46:38.673385896+00:00"
id: "atelier-hi7z"
evidence_type: "validation"
captured_at: "2026-06-19T04:46:38.673379255+00:00"
target:
  kind: "issue"
  id: "atelier-unwz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-unwz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Old pr surface removed. target/debug/atelier pr --help is rejected as an unknown subcommand; root help exposes review; target/debug/atelier review --help exposes the renamed commands. rg over active CLI source, CLI tests, product/spec docs, AGENTS.md, and agent-factory found no active atelier pr guidance; remaining pull_request hits are provider/internal data fields. Proof: target/debug/atelier pr --help failed, target/debug/atelier --help passed, target/debug/atelier review --help passed, atelier lint atelier-unwz passed, git diff --check passed."
updated_at: "2026-06-19T04:46:41.521231422+00:00"
---

Old pr surface removed. target/debug/atelier pr --help is rejected as an unknown subcommand; root help exposes review; target/debug/atelier review --help exposes the renamed commands. rg over active CLI source, CLI tests, product/spec docs, AGENTS.md, and agent-factory found no active atelier pr guidance; remaining pull_request hits are provider/internal data fields. Proof: target/debug/atelier pr --help failed, target/debug/atelier --help passed, target/debug/atelier review --help passed, atelier lint atelier-unwz passed, git diff --check passed.
