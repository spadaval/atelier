---
acceptance: []
created_at: "2026-06-12T00:57:45.632734590+00:00"
evidence_required: []
id: "atelier-o2a4"
issue_type: "epic"
labels:
- "cli"
- "ergonomics"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4x33"
  - kind: "issue"
    id: "atelier-dx73"
  - kind: "issue"
    id: "atelier-zjua"
  children:
  - kind: "issue"
    id: "atelier-46lg"
  - kind: "issue"
    id: "atelier-51xk"
  - kind: "issue"
    id: "atelier-8vfc"
  - kind: "issue"
    id: "atelier-9jbu"
  - kind: "issue"
    id: "atelier-o3w3"
  - kind: "issue"
    id: "atelier-ob49"
  - kind: "issue"
    id: "atelier-vr9g"
  - kind: "issue"
    id: "atelier-xenr"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Reshape command hierarchy for workflow ergonomics"
updated_at: "2026-06-12T03:36:04.665698736+00:00"
---

Plan intuitive command placement and remove redundant command surfaces that make agent workflows harder to explain. Acceptance: command placement is documented with old-to-new mappings, explicit staged-deprecation or alias decisions, and validation expectations for issue lifecycle, dependencies, links, search, notes/activity, work/start, setup/integrations, rebuild, maintenance, and state commands. Prefer compatibility aliases or staged deprecation for moved commands; direct removal is allowed only when a specific issue proves the replacement is clear, tested, documented, and safe for Agent Factory guidance and existing operator habits.
