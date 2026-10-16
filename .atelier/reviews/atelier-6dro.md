---
schema: atelier.review
schema_version: 1
id: atelier-6dro
title: 'Reviewed git diff master...HEAD (base_done_claim in crates/atelier-cli/src/commands/git.rs and the new session test). Ran the session base_* tests and the epic-migration test: they pass. Mutations in a scratch copy: judging only the first parent fails the merge half (land refused); making the fallback say ''already done'' fails the broken-record half; removing migrate_epic_text fails migrating_an_already_done_epic_on_the_base_is_not_a_done_claim, so that test is still valid and now depends on the migration path. The ''feature'' target type does not matter: migrate_epic_text only rewrites the type and draft->draft, the status is kept, and the commit''s own type is only checked against mission/decision, which an epic is not. Two deferred findings: an unpushed reopen can be hidden behind a broken copy, and the fallback''s ''already done'' branch has no test.'
target: atelier-hyew
identity: Independent reviewer (Claude subagent)
commit: 387dbfad150d05d24d840cfefcc70e4154673ba4
created_at: 2026-10-16T20:46:11.328322235Z
event:
  event: approval
---

Reviewed git diff master...HEAD (base_done_claim in crates/atelier-cli/src/commands/git.rs and the new session test). Ran the session base_* tests and the epic-migration test: they pass. Mutations in a scratch copy: judging only the first parent fails the merge half (land refused); making the fallback say 'already done' fails the broken-record half; removing migrate_epic_text fails migrating_an_already_done_epic_on_the_base_is_not_a_done_claim, so that test is still valid and now depends on the migration path. The 'feature' target type does not matter: migrate_epic_text only rewrites the type and draft->draft, the status is kept, and the commit's own type is only checked against mission/decision, which an epic is not. Two deferred findings: an unpushed reopen can be hidden behind a broken copy, and the fallback's 'already done' branch has no test.
