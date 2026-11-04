---
created_at: "2026-11-04T23:19:16.861854392+00:00"
id: "atelier-esu1"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-esu1
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Treat a human's request for a specific release as approval to push its tag"
updated_at: "2026-11-04T23:19:26.906407890+00:00"
---

## Goal

The release strategy's Authority section says pushing a release tag comes back to the human. When the human has already asked for a specific release, the orchestrator should push that tag rather than defer it: v0.3.1 was merged and green but never tagged because the agent closed atelier-g8cn with tagging as an open item and nobody asked.

## Done when

- docs/strategies/release-distribution.md Authority says a human's request for a specific release authorizes pushing its tag once master CI passes on the release commit; other tag pushes still come back to the human.
