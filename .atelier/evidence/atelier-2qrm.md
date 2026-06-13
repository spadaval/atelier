---
created_at: "2026-06-13T23:02:16.526059573+00:00"
id: "atelier-2qrm"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T23:02:16.471448308+00:00\",\"command\":\"cargo udeps\",\"exit_code\":101,\"exit_status\":\"101\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":159,\"summary\":\"error: no such command: `udeps`\\n\\nhelp: view all installed commands with `cargo --list`\\nhelp: find a package to install `udeps` with `cargo search cargo-udeps`\\n\",\"truncated\":false},\"stdout\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false}},\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"deferred\",\"spawn_error\":null,\"success\":false,\"target\":{\"id\":\"atelier-e723\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e723"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "deferred"
title: "cargo udeps"
updated_at: "2026-06-13T23:02:19.216151919+00:00"
---

cargo udeps

Command: cargo udeps
Exit status: 101

Stdout summary:
(none)

Stderr summary:
error: no such command: `udeps`

help: view all installed commands with `cargo --list`
help: find a package to install `udeps` with `cargo search cargo-udeps`

