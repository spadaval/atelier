---
created_at: "2026-06-12T17:07:36.333322708+00:00"
id: "atelier-qb7m"
issue_type: "task"
labels:
- "diagnostics"
- "reliability"
- "repair"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T23:49:22.961652574+00:00"
status: "done"
title: "Keep orientation commands usable with malformed records"
updated_at: "2026-06-12T23:49:22.961652574+00:00"
---

## Description

Malformed canonical tracker records will exist during migrations, conflict
resolution, and manual edits. Atelier should not let one bad issue make normal
orientation and repair surfaces unusable. The tracker should enter an explicit
degraded state that names malformed records, preserves safe read orientation,
and keeps completion gates strict.

This work defines and implements the recovery behavior for malformed canonical
records. It complements projection freshness diagnostics: stale projections and
invalid records are related failure modes, but a malformed record should produce
bounded repair guidance rather than collapsing unrelated mission, status,
doctor, lint, and show workflows.
- `atelier status`, `atelier doctor`, `atelier lint`, and focused record repair
  commands remain usable when one issue record is malformed.
- Mission and issue orientation surfaces can show valid records from canonical
  Markdown or a clearly labeled degraded fallback without silently hiding the
  malformed record.
- Degraded output names each malformed record, the failing file, the diagnostic,
  and the safest next repair command.
- `atelier lint <id>` works for a malformed record even when the projection is
  stale or cannot rebuild.
- Safe automated repairs exist for unambiguous migrations such as legacy issue
  front matter moved into body sections, or the command prints a patch-oriented
  repair instruction when automation would invent product meaning.
- Mission closeout, issue closeout, evidence satisfaction, and workflow gates
  still fail closed while linked malformed records remain unresolved.
- Tests distinguish recoverable orientation behavior from strict completion
  behavior so future projection changes cannot reintroduce all-or-nothing
  failure.
- CLI transcript tests prove a malformed issue does not prevent `atelier status`,
  `atelier doctor`, global `atelier lint`, or `atelier lint <id>` from reporting
  bounded, actionable repair guidance.

- CLI transcript tests prove mission or issue orientation can still identify the
  active mission and valid linked work while clearly reporting malformed linked
  records as degraded-state blockers.

- Negative tests prove mission closeout, issue closeout, and workflow gates fail
  while linked malformed records remain unresolved.

- Repair-command tests cover at least one safe legacy-frontmatter migration and
  one ambiguous malformed record that requires manual repair guidance.

- Run focused projection, lint, doctor, status, and mission orientation tests.
Do not make degraded mode a silent best-effort query path. Any answer produced
from fallback state must say the tracker is degraded and must not be accepted as
closeout proof until the malformed records are repaired.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
