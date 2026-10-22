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
closed_at: "2026-10-22T20:54:37.599894951+00:00"
status: "done"
title: "Add land --push so a refused land can never be pushed behind a pipe"
updated_at: "2026-10-22T21:09:54.022972412+00:00"
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
- 2026-10-22 [decision] Dropped the 'set -o pipefail; ' prefix from setup-generated hooks, deviating from the Done when's 'hooks use set -o pipefail'
  Reason: The hook command has no pipe so the prefix guards nothing, and under a /bin/sh that is dash older than 0.5.12-7 'set -o pipefail' is an illegal option that exits 2, which blocks every Stop. The pipeline guard is land --push plus the verdict line (review finding atelier-ztla)
  Rejected: guarded prefix '(set -o pipefail) 2>/dev/null && set -o pipefail;' (still protects nothing)

## Close Reason

`atelier land --push` pushes the checked HEAD to the branch's configured upstream only after land passes, and every non-hook land now ends with a verdict line; a refusal or rejected push pushes nothing and exits non-zero. Setup-generated hooks are prefixed `set -o pipefail; `.

### Deviations
The verdict uses the documented form (`land: passed` / `land: refused — 1 check (code)`) rather than literal PASS/FAIL, per human-cli-output.md and ADR 0022. A refusal always counts one check (the one that stopped land); atelier-rc8j/bvl3 will list all failed checks. Hook mode (`--hook`) is unchanged and does not combine with `--push`.

### Decisions
Two decision notes on this item: explicit-refspec push to the configured upstream, and the verdict wording/Reported-error mechanism in main.

### Surprises
No verdict line existed in code, only in the docs. The hook pipefail prefix has no effect on the hook's own command (it has no pipe); it protects pipelines users append, as the Done-when asked.

### Open items
None.
