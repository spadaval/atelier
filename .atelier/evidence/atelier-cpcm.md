---
created_at: "2026-07-06T18:46:53.139197429+00:00"
id: "atelier-cpcm"
evidence_type: "validation"
captured_at: "2026-07-06T18:46:53.139189183+00:00"
agent_identity: "independent-validator"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "FAIL (in-scope stale-guidance finding, owner atelier-p0am): docs/spec/agent-factory/tracker-replacement-mvp.md was partially refreshed by atelier-durs but still presents removed commands as current required Agent Factory equivalents, including atelier search, issue close, dep add/remove, lint, doctor, export, rebuild, standalone import-beads/import, and claim paths. The same document says Atelier supports this agent-facing command surface, so the rows are active guidance rather than a bounded historical transcript. Root help rejects or hides these surfaces and docs/product/cli-surface.md removes them. atelier check passes despite the drift, so the stale-guidance search claim fails until a separate implementer corrects the active MVP guidance and adds regression proof. Other dated audit hits are historical evidence and not this finding."
updated_at: "2026-07-06T18:46:59.464217614+00:00"
---

FAIL (in-scope stale-guidance finding, owner atelier-p0am): docs/spec/agent-factory/tracker-replacement-mvp.md was partially refreshed by atelier-durs but still presents removed commands as current required Agent Factory equivalents, including atelier search, issue close, dep add/remove, lint, doctor, export, rebuild, standalone import-beads/import, and claim paths. The same document says Atelier supports this agent-facing command surface, so the rows are active guidance rather than a bounded historical transcript. Root help rejects or hides these surfaces and docs/product/cli-surface.md removes them. atelier check passes despite the drift, so the stale-guidance search claim fails until a separate implementer corrects the active MVP guidance and adds regression proof. Other dated audit hits are historical evidence and not this finding.
