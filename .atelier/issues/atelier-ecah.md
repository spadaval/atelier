---
created_at: "2026-10-17T19:00:36.349556286+00:00"
id: "atelier-ecah"
issue_type: "feature"
labels:
- "land"
- "retro-2026-10-17"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Add land --push so a refused land can never be pushed behind a pipe"
updated_at: "2026-10-22T20:53:42.440556963+00:00"
---

## Goal

Twice a refused `land` was hidden by `| tail` and the push still ran: broken master after 749l (b32c39e6) and unreviewed docs pushed from d81e0801. The guard is now only a memory rule.

## Done when

`atelier land --push` pushes the current branch only when land passes and says so on its last line; on failure it pushes nothing and exits non-zero. `land` prints its verdict (PASS/FAIL with exit code) as its last line, and the setup-generated hooks use `set -o pipefail`. Tests cover pass, fail, and a failing push.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Related: mission atelier-rc8j (every gate ends in a verdict).

## Notes

- 2026-10-22 [decision] land --push pushes the checked HEAD to the branch's configured upstream (branch.<name>.remote/.merge) by explicit refspec, and refuses with no upstream
  Reason: A bare 'git push' depends on push.default and could publish other branches; an explicit refspec pushes exactly the commit land checked
  Rejected: plain git push
- 2026-10-22 [decision] Verdict line follows human-cli-output.md ('land: passed' / 'land: refused — 1 check (code)'), with ', pushed to <remote>/<branch>' or ', nothing pushed' suffixes; a rejected push ends 'land: push failed, nothing pushed'; failures print Error then the verdict on stderr and main exits 1 via a Reported marker
  Reason: Done-when asks for PASS/FAIL, but the documented verdict form is the repository convention (ADR 0022); rc8j will extend the failure count later
