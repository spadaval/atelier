---
created_at: "2026-06-12T21:18:48.516520254+00:00"
id: "atelier-cyo8"
data: "{\"captured_at\":\"2026-06-12T21:18:48.285925893+00:00\",\"command\":\"target/debug/atelier export --check\",\"exit_code\":0,\"exit_status\":\"0\",\"kind\":\"validation\",\"output\":{\"limit_bytes_per_stream\":4096,\"stderr\":{\"bytes\":0,\"summary\":\"\",\"truncated\":false},\"stdout\":{\"bytes\":90,\"summary\":\"Canonical export is current\\nState: /root/atelier/.atelier-worktrees/atelier-7yen/.atelier\\n\",\"truncated\":false}},\"path\":null,\"producer\":null,\"result\":\"pass\",\"spawn_error\":null,\"success\":true,\"target\":{\"id\":\"atelier-7yen\",\"kind\":\"issue\",\"role\":\"validates\"},\"uri\":null}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7yen"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "atelier export --check passed"
updated_at: "2026-06-12T21:18:49.563720349+00:00"
---

atelier export --check passed

Command: target/debug/atelier export --check
Exit status: 0

Stdout summary:
Canonical export is current
State: /root/atelier/.atelier-worktrees/atelier-7yen/.atelier

Stderr summary:
(none)

