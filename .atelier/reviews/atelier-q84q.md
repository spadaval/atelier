---
schema: atelier.review
schema_version: 1
id: atelier-q84q
title: 'crates/atelier-app/src/lint.rs:293-312: check no longer ties a receipt''s stored criteria_mission to its target at all. The CLI only writes a mission it derived from the target, but a hand-edited receipt passes check with criteria_mission changed to another mission that defines the slug, or with its target removed (the old ''Evidence names criteria but has no issue target'' finding was dropped). Reproduced in a scratch repo: both edits left ''check'' passing and moved the proof to mission B. This is the same trust class as hand-editing result: fail to pass, which check also cannot catch, and the author''s choice not to report a moved target is sound (a receipt states what passed for mission A at its commit; moving the item does not falsify it, and an append-only record could not fix such a finding). No action unless records stop being reviewed in git; a cheap partial guard would be to restore the ''no issue target'' finding.'
target: atelier-ho7o
identity: Independent reviewer 3 (Claude subagent)
commit: 8f3e78189c614d14110a199469ca8441cc7f7293
created_at: 2026-10-17T18:58:42.461902224Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/lint.rs:293-312: check no longer ties a receipt's stored criteria_mission to its target at all. The CLI only writes a mission it derived from the target, but a hand-edited receipt passes check with criteria_mission changed to another mission that defines the slug, or with its target removed (the old 'Evidence names criteria but has no issue target' finding was dropped). Reproduced in a scratch repo: both edits left 'check' passing and moved the proof to mission B. This is the same trust class as hand-editing result: fail to pass, which check also cannot catch, and the author's choice not to report a moved target is sound (a receipt states what passed for mission A at its commit; moving the item does not falsify it, and an append-only record could not fix such a finding). No action unless records stop being reviewed in git; a cheap partial guard would be to restore the 'no issue target' finding.
