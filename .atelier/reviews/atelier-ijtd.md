---
schema: atelier.review
schema_version: 1
id: atelier-ijtd
title: 'crates/atelier-app/src/record_history.rs:193: scope added is decided by comparing committer timestamps at one-second resolution, not by history position. A link committed in the same second as the start is not flagged; the fixture needs a 1.1 s sleep for this reason (cli_integration/debrief.rs:78). A link made on a branch before the start but merged into the base after it is also not flagged, and clock skew or rebased commits can misorder either way. Fix: compare positions in the walk (or use ancestry: the joining commit is not an ancestor of the start commit) instead of times.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:16.431898867Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_history.rs:193: scope added is decided by comparing committer timestamps at one-second resolution, not by history position. A link committed in the same second as the start is not flagged; the fixture needs a 1.1 s sleep for this reason (cli_integration/debrief.rs:78). A link made on a branch before the start but merged into the base after it is also not flagged, and clock skew or rebased commits can misorder either way. Fix: compare positions in the walk (or use ancestry: the joining commit is not an ancestor of the start commit) instead of times.
