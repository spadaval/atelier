---
schema: atelier.review
schema_version: 1
id: atelier-wff1
title: 'Re-reviewed a89e76fd, 6aa8ac7c, and the b0b45d7c merge, and confirmed all five prior findings fixed in a scratch repo with the HEAD binary. A body update on a superseded record fails with issue_body_done; a title change and a note on it still apply, matching issue update. check_bundle_record refuses a mission body without Success criteria and a missing strategy file, for updates and for creates. A duplicate update id given as a suffix and a full id is refused. Moving a record to a different parent is refused with a pointer to issue update --parent, and restating the same parent by full id is accepted and deduplicated. The create-only wording is gone. A no-op update round trip (fenced heading in the body, P3 priority, suffix ids, mission) changes only updated_at. Merged one-level-parent hierarchy checks reject a grandchild create or update. The schema.md ''Updating records'' section matches the behavior. cargo nextest run -p atelier-cli bundle: 27 passed. The remaining parent-ref gaps are filed as a defer finding: they fail closed and lose no data.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: 12cd9e849b50fda58f3ab0887c995a6e81a33999
created_at: 2026-10-16T00:45:10.261845372Z
event:
  event: approval
---

Re-reviewed a89e76fd, 6aa8ac7c, and the b0b45d7c merge, and confirmed all five prior findings fixed in a scratch repo with the HEAD binary. A body update on a superseded record fails with issue_body_done; a title change and a note on it still apply, matching issue update. check_bundle_record refuses a mission body without Success criteria and a missing strategy file, for updates and for creates. A duplicate update id given as a suffix and a full id is refused. Moving a record to a different parent is refused with a pointer to issue update --parent, and restating the same parent by full id is accepted and deduplicated. The create-only wording is gone. A no-op update round trip (fenced heading in the body, P3 priority, suffix ids, mission) changes only updated_at. Merged one-level-parent hierarchy checks reject a grandchild create or update. The schema.md 'Updating records' section matches the behavior. cargo nextest run -p atelier-cli bundle: 27 passed. The remaining parent-ref gaps are filed as a defer finding: they fail closed and lose no data.
