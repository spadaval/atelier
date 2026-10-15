---
schema: atelier.review
schema_version: 1
id: atelier-6j93
title: 'Re-review after 0f489f2f (and merge 0e6363e9, records fe87109e). Rebuilt and reproduced in a scratch repo: update --body now refuses dropping Proof on a ready epic, dropping Success criteria on a mission, --issue-type epic plus a Proof-less body on a todo record, bodies over 64 KiB, and any body change on a done record (title edits still allowed). Draft epics without Proof and placeholder text pass, as atelier check (unscoped) also allows. The old depends_on bundle fails before mutation with the rename hint. Help for create/update --body and check, work-model.md (land refuses only active work; gap tracked in atelier-fj1t), and cli-surface.md:60 now match behaviour. The extended test covers the ready-epic, size, done-record and bundle-hint cases. vlo3 defer to atelier-92mz is accepted: ADR 0020 decision 2 forbids local edits to the vendored skill, and 92mz names the update --body and blocked_by changes with a verifiable resync. Residual: until 92mz lands, the vendored skill still tells agents update --body is missions-only. No mission-body case in the test (verified manually). The suite was not re-run by me.'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: fe87109ef1c147a1eda7197aa643f9eb1ec22716
created_at: 2026-10-15T23:51:11.427135608Z
event:
  event: approval
---

Re-review after 0f489f2f (and merge 0e6363e9, records fe87109e). Rebuilt and reproduced in a scratch repo: update --body now refuses dropping Proof on a ready epic, dropping Success criteria on a mission, --issue-type epic plus a Proof-less body on a todo record, bodies over 64 KiB, and any body change on a done record (title edits still allowed). Draft epics without Proof and placeholder text pass, as atelier check (unscoped) also allows. The old depends_on bundle fails before mutation with the rename hint. Help for create/update --body and check, work-model.md (land refuses only active work; gap tracked in atelier-fj1t), and cli-surface.md:60 now match behaviour. The extended test covers the ready-epic, size, done-record and bundle-hint cases. vlo3 defer to atelier-92mz is accepted: ADR 0020 decision 2 forbids local edits to the vendored skill, and 92mz names the update --body and blocked_by changes with a verifiable resync. Residual: until 92mz lands, the vendored skill still tells agents update --body is missions-only. No mission-body case in the test (verified manually). The suite was not re-run by me.
