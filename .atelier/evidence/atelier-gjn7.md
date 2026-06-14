---
created_at: "2026-06-14T07:32:47.649978893+00:00"
id: "atelier-gjn7"
evidence_type: "validation"
captured_at: "2026-06-14T07:32:47.649855938+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-oqtz"
  role: "validates"
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-oqtz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Stale projection and invalid canonical recovery diagnostics are bounded and actionable: messages preserve the blocked command or record/parser context and provide one ordered recovery path through lint, record repair, doctor/doctor --fix, and rerunning the blocked command. Proof: focused projection/invalid-markdown tests, smoke export check, cargo fmt -- --check, git diff --check, atelier lint, atelier lint atelier-oqtz, atelier export --check, atelier doctor."
updated_at: "2026-06-14T07:32:50.019246188+00:00"
---

Stale projection and invalid canonical recovery diagnostics are bounded and actionable: messages preserve the blocked command or record/parser context and provide one ordered recovery path through lint, record repair, doctor/doctor --fix, and rerunning the blocked command. Proof: focused projection/invalid-markdown tests, smoke export check, cargo fmt -- --check, git diff --check, atelier lint, atelier lint atelier-oqtz, atelier export --check, atelier doctor.
