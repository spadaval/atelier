---
created_at: "2026-06-13T23:02:16.462297470+00:00"
id: "atelier-gutu"
data: "{\"agent_identity\":null,\"captured_at\":\"2026-06-13T23:02:16.403854660+00:00\",\"command\":\"cargo machete\",\"exit_code\":101,\"exit_status\":\"101\",\"follow_up_ids\":[],\"independence_level\":\"unspecified\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":165,\"summary\":\"error: no such command: `machete`\\n\\nhelp: view all installed commands with `cargo --list`\\nhelp: find a package to install `machete` with `cargo search cargo-machete`\\n\",\"truncated\":false},\"stdout\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false}},\"path\":null,\"producer\":null,\"proof_scope\":\"scoped to the attached target or summary\",\"residual_risks\":[],\"result\":\"deferred\",\"spawn_error\":null,\"success\":false,\"target\":{\"id\":\"atelier-e723\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
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
title: "cargo machete"
updated_at: "2026-06-13T23:02:19.108212073+00:00"
---

cargo machete

Command: cargo machete
Exit status: 101

Stdout summary:
(none)

Stderr summary:
error: no such command: `machete`

help: view all installed commands with `cargo --list`
help: find a package to install `machete` with `cargo search cargo-machete`

