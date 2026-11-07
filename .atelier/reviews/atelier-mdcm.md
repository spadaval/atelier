---
schema: atelier.review
schema_version: 2
id: atelier-mdcm
title: 'low: crates/atelier-cli/src/commands/record_commit.rs:132-149 The pending file is written after ''git add'' (stage mode) and never updated before ''git commit'' (commit mode), so an interrupted --stage-records, or a failed commit (e.g. a rejecting pre-commit hook), leaves the command''s own record staged but absent from staged-records.json. Reproduced: with a failing pre-commit hook, ''--commit-records'' exits 1 with the record already written and staged; the next ''--commit-records'' then refuses with record_commit_index_not_clean naming atelier''s own record as a foreign change, and the old group''s labels are lost from the eventual message. Nothing is lost, but the user must hand-repair and may rerun the command, duplicating the event. Fix: write the updated pending file (paths and labels) before ''git add''/''git commit'', and clear it only after the commit succeeds; same change as the medium finding.'
target: atelier-zuow
identity: Independent reviewer (Claude subagent)
commit: eb6ee5cf688f70c5521050bad0eac7d1d58da725
created_at: 2026-10-22T21:45:00.424774829Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (orchestrator)
    commit: 84f8d13a93b7fba587090b51451adb8a26cf2b8c
    recorded_at: 2026-10-22T21:48:05.543993957Z
    resolution_commit: aecb73084c8af59cdf31bfa9229dd7d55fd12db9
    reason: Fixed in aecb7308
    migrated_from: atelier-8me3
---

low: crates/atelier-cli/src/commands/record_commit.rs:132-149 The pending file is written after 'git add' (stage mode) and never updated before 'git commit' (commit mode), so an interrupted --stage-records, or a failed commit (e.g. a rejecting pre-commit hook), leaves the command's own record staged but absent from staged-records.json. Reproduced: with a failing pre-commit hook, '--commit-records' exits 1 with the record already written and staged; the next '--commit-records' then refuses with record_commit_index_not_clean naming atelier's own record as a foreign change, and the old group's labels are lost from the eventual message. Nothing is lost, but the user must hand-repair and may rerun the command, duplicating the event. Fix: write the updated pending file (paths and labels) before 'git add'/'git commit', and clear it only after the commit succeeds; same change as the medium finding.
