---
schema: atelier.review
schema_version: 1
id: atelier-x5pe
title: 'crates/atelier-app/src/epic_migration.rs:117-132 split_sections (follow-up to atelier-gf77): body text before the first ''## '' heading belongs to no section and is still dropped silently. Reproduced: the pre-migration atelier-1ioo (commit 3f836ef0) with ''Preamble: also requires a security review.'' inserted before ''## Outcome'', then ''atelier migrate epics --to feature --apply'', gives a record with no Preamble text and no refusal. Because review_gate issue_status_only accepts an exact migration as status only, an approved legacy epic can lose that acceptance text and keep its approval. The 9e34b745 refusals (Outcome+Goal, Done when+Proof/Delivers, duplicate headings) are correct and close the rest of gf77. Suggested fix: bail with ''migrate it by hand'' when non-blank text comes before the first section, and add that case to migration_refuses_to_drop_sections.'
target: atelier-0349
identity: Independent reviewer (Claude subagent)
commit: cbaa24eae445ffadca4ebc3507fba161a678e0b8
created_at: 2026-10-16T02:53:09.585159108Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/epic_migration.rs:117-132 split_sections (follow-up to atelier-gf77): body text before the first '## ' heading belongs to no section and is still dropped silently. Reproduced: the pre-migration atelier-1ioo (commit 3f836ef0) with 'Preamble: also requires a security review.' inserted before '## Outcome', then 'atelier migrate epics --to feature --apply', gives a record with no Preamble text and no refusal. Because review_gate issue_status_only accepts an exact migration as status only, an approved legacy epic can lose that acceptance text and keep its approval. The 9e34b745 refusals (Outcome+Goal, Done when+Proof/Delivers, duplicate headings) are correct and close the rest of gf77. Suggested fix: bail with 'migrate it by hand' when non-blank text comes before the first section, and add that case to migration_refuses_to_drop_sections.
