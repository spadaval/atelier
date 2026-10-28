---
created_at: "2026-10-28T00:35:18.197859557+00:00"
id: "atelier-vy5i"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-vy5i
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5tl2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-vt8q"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-28T04:29:36.657064742+00:00"
status: "done"
title: "Cut Atelier's first GitHub release"
updated_at: "2026-10-28T04:29:36.657064742+00:00"
---

## Goal

Prepare the first release, v0.3.0 (the version Sumeet chose), so it can be tagged from `master`. The tag push follows the merge.

## Done when

The workspace version is 0.3.0, and `CHANGELOG.md`'s first-release entry is headed `[0.3.0] - <date>`. `dist manifest --tag=v0.3.0` then announces 0.3.0 with that section as its notes and does not mark it as a prerelease, since `latest/download` skips prereleases. After merge, `v0.3.0` is pushed only on a `master` commit whose CI workflow run (the test workflow, not the Release PR run) passed, because the release workflow publishes whether or not CI passed. atelier-g6a1 checks the published release.

## Context

Strategy Authority: pushing a release tag comes back to the human; Sumeet authorized v0.3.0 on 2026-10-28. The tag must equal the workspace version (`dist plan --tag=v<x>` refuses a mismatch). The CI-gate gap was found in review of atelier-7v9q (R1), and the heading and prerelease points in review of atelier-5tl2.

## Notes

- 2026-10-28 [note] 2026-10-28 [decision] Sumeet chose v0.3.0 for the first release and authorized pushing the tag: 0.2.0 has been the version since before the Markdown-record rebuild, so older source builds also report it.

## Close Reason

The workspace is at 0.3.0, the changelog entry is headed `[0.3.0] - 2026-10-28`, and `dist manifest --tag=v0.3.0` announces "0.3.0 - 2026-10-28" with that section as notes, not a prerelease. The full suite passes at 0.3.0.

### Deviations
Done when was narrowed from "the release exists" to "ready to tag". The tag can only go on a master commit that carries this bump and passed CI, so publishing follows the merge, and atelier-g6a1 checks the result.

### Decisions
Sumeet chose v0.3.0 over v0.2.0 because older source builds from before the rebuild also report 0.2.0.

### Surprises
None.

### Open items
- Push `v0.3.0` on the merge commit once its master CI run passes, then confirm the release workflow published the release.
