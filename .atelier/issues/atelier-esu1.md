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
closed_at: "2026-11-05T00:22:52.831287844+00:00"
status: "done"
title: "Treat a human's request for a specific release as approval to push its tag"
updated_at: "2026-11-05T00:22:52.831287844+00:00"
---

## Goal

The release strategy's Authority section says pushing a release tag comes back to the human. When the human has already asked for a specific release, the orchestrator should push that tag rather than defer it: v0.3.1 was merged and green but never tagged because the agent closed atelier-g8cn with tagging as an open item and nobody asked.

## Done when

- docs/strategies/release-distribution.md Authority says a human's request for a specific release authorizes pushing its tag once master CI passes on the release commit; other tag pushes still come back to the human.

## Close Reason

The Authority section of docs/strategies/release-distribution.md now says a human's request for a specific release authorizes pushing its tag once master CI passes on the release commit, and the agent reports the published release instead of leaving the tag as an open item. Other tag pushes, the first release's version, new platforms or channels, and secrets still come back to the human. Independently approved (atelier-64zi, no findings); PR #19 CI green on Linux and macOS.

### Deviations
None.

### Decisions
Did not add a `last_changed` entry to the strategy front matter, so the approved commit stays as reviewed.

### Surprises
atelier-l7gl merged edits to the same strategy file and closed mission atelier-vt8q while this was in review; the merge was clean and required a second sync and CI run.

### Open items
None.
