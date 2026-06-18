---
created_at: "2026-06-17T19:37:02.008688939+00:00"
id: "atelier-0v3f"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2y4u"
    type: "advances"
  - kind: "issue"
    id: "atelier-58n9"
    type: "advances"
  - kind: "issue"
    id: "atelier-6375"
    type: "advances"
  - kind: "issue"
    id: "atelier-6jap"
    type: "advances"
  - kind: "issue"
    id: "atelier-7g43"
    type: "advances"
  - kind: "issue"
    id: "atelier-7qsr"
    type: "advances"
  - kind: "issue"
    id: "atelier-98mo"
    type: "advances"
  - kind: "issue"
    id: "atelier-c5oz"
    type: "advances"
  - kind: "issue"
    id: "atelier-hw9t"
    type: "advances"
  - kind: "issue"
    id: "atelier-j75d"
    type: "advances"
  - kind: "issue"
    id: "atelier-mwup"
    type: "advances"
  - kind: "issue"
    id: "atelier-ngb2"
    type: "advances"
  - kind: "issue"
    id: "atelier-tovs"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Close architecture audit gaps and readiness graph"
updated_at: "2026-06-18T16:44:20.749787388+00:00"
---

## Intent

Make the code, architecture docs, tracker claims, and operator health surfaces agree after the app/CLI split, RecordStore extraction, canonical record normalization, session/PR field work, corrective session-as-issue-events work, and readiness validation.

## Constraints

- Reuse the existing open graph instead of creating parallel duplicate epics.
- Do not reopen closed epics; create follow-up work where live code contradicts closed claims.
- Contract/artifact decisions must land before dependent implementation starts.
- The session-as-issue-events model wins over durable optional sessions: worker/reviewer/validator attempts are derived from canonical issue activity, and session commands are inspection-only rather than workflow drivers.
- The current atelier status branch-lifecycle failure is mission-blocking until fixed or explicitly documented as expected stale data.

## Risks

- Closed tracker claims for the app/CLI split and RecordStore extraction currently overstate the live code boundary.
- Existing dirty tracker/code changes may already be partway through this graph; preserve them and do not revert unrelated work.

## Validation

- Mission status maps all linked epics and child issues to proof.
- `atelier-ngb2` proof shows the corrective session-as-issue-events model replaces durable-session workflow assumptions before final readiness validation closes.
- Final evidence includes cargo fmt -- --check, focused nextest suites, atelier lint, atelier export --check, atelier doctor, and git diff --check.
- A status transcript either passes or documents the branch-lifecycle graph failure as an intentional stale-data condition with follow-up ownership.
