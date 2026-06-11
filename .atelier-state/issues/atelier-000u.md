---
acceptance: []
created_at: "2026-06-08T17:33:27+00:00"
evidence_required: []
id: "atelier-000u"
issue_type: "task"
labels:
- "domain-model"
- "feature"
- "spec"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0005"
  - kind: "issue"
    id: "atelier-0006"
  - kind: "issue"
    id: "atelier-000l"
  children:
  - kind: "issue"
    id: "atelier-001h"
  - kind: "issue"
    id: "atelier-001i"
  - kind: "issue"
    id: "atelier-001j"
  - kind: "issue"
    id: "atelier-001k"
  - kind: "issue"
    id: "atelier-001l"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Add first-class mission, milestone checkpoint, plan, evidence, and workflow validator records"
updated_at: "2026-06-10T14:51:59.670497926+00:00"
---

Integrate first-class mission, milestone checkpoint, plan, evidence, and workflow validator records across storage and command layers after the project-scoped random ID cutover. Keep this issue as the integration parent for persistence, export/rebuild, and post-cutover record behavior after the focused command slices land.

The mission-centered model is the organizing target: missions hold objective intent, constraints, risks, validation expectations, and links to checkpoint milestones, durable plans, evidence, epics, and issues. Epics and issues remain accountable work units; milestones remain validated checkpoint states; plans remain durable execution intent; evidence remains validation proof; links connect these concepts explicitly.

Direct agent-run/session records are intentionally deferred from the first domain-model slice; workflow and evidence records should leave room for later run metadata without requiring it now.

Acceptance:
Focused child issues provide mission, milestone checkpoint, plan, evidence, and workflow validator command surfaces. Schema and models represent the target records or accepted staged subset using the single project-scoped random record ID form. Create/show/list JSON exists for agent-facing records. Records export and rebuild deterministically. Mission records are first-class objective records, not generic issues or labels. Milestones are checkpoint states with validation criteria, not work containers. Typed links express mission-to-milestone, mission-to-plan, mission-to-work, and evidence validation relationships without overloading dependencies. Migration behavior is tested as cutover, not long-lived compatibility. Docs name deferred run/session fields explicitly. Workflow validators enforce transitions and are not milestone fields.
