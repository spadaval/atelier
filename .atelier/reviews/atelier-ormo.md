---
schema: atelier.review
schema_version: 2
id: atelier-ormo
title: finding on atelier-nafc @f317ac78
target: atelier-nafc
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:49.695122373Z
event:
  event: finding
  recommendation: fix_now
---

R5 .atelier/issues/atelier-nafc.md:51: medium. Fresh ordinary clones do not retain the source repository local branch set: only the checked-out branch is local, with other fetched heads represented as remote-tracking refs. The new reader enumerates refs/heads only (next/crates/atelier-cli/src/git.rs:110 and repo.rs:114), and the converter deliberately converts no branch work. Timing status/list on these converted bases can therefore report fast results without testing the strategy risk, reading started items from their branches in the repository with the most branches. Merely retaining old-format branches would not exercise reads of new-format started items either. Specify a representative scratch-only branch population, including unmerged item-named branches holding new-format started records, and report the number actually read as well as total local branches. This is rehearsal setup, not permission to extend the converter to migrate in-flight work or add a stored index.
