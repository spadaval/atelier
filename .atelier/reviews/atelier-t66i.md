---
schema: atelier.review
schema_version: 1
id: atelier-t66i
title: 'crates/atelier-cli/src/human_output.rs:507: decision_brief_lines prints every one-way note on the item, its parent and its mission with no cap, and status runs this at every session start. Today it is bounded only by how many one-way notes three hand-written records accumulate; a long mission whose record gathers many one-way decisions would print them all (each a wrapped multi-line row) in every session-start status, against the strategy shape ''anything shown at session start stays bounded''. Measured cost is fine (issue show 0.06-0.08s on branch and master release builds; status within noise). Fix: cap the brief rows (e.g. newest 5) with a ''+N more: issue show <id> --full'' line, as status already caps reviews and evidence at 8.'
target: atelier-1mk4
identity: Independent reviewer (Claude subagent)
commit: 2254ca708e0ee34d8b3d2f3ba857fc6726b5f23b
created_at: 2026-10-17T18:57:57.073952637Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/human_output.rs:507: decision_brief_lines prints every one-way note on the item, its parent and its mission with no cap, and status runs this at every session start. Today it is bounded only by how many one-way notes three hand-written records accumulate; a long mission whose record gathers many one-way decisions would print them all (each a wrapped multi-line row) in every session-start status, against the strategy shape 'anything shown at session start stays bounded'. Measured cost is fine (issue show 0.06-0.08s on branch and master release builds; status within noise). Fix: cap the brief rows (e.g. newest 5) with a '+N more: issue show <id> --full' line, as status already caps reviews and evidence at 8.
