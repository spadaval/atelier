---
schema: atelier.review
schema_version: 2
id: atelier-z3z1
title: finding on atelier-8bh4 @70b45d46
target: atelier-8bh4
identity: independent reviewer subagent (claude-opus-5-5), atelier-8bh4
commit: 70b45d464142a56eba268b9270dc764ea7e13de8
created_at: 2026-11-07T09:40:07.299847170Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 83e9a3a91f34fb82d9281a01aba87137f9d86089
    recorded_at: 2026-11-07T09:46:07.205829255Z
    follow_up: atelier-ml3r
    reason: atelier-ml3r runs the migration on the other repositories and now owns this edge (master 0dd66661).
---

low: crates/atelier-cli/src/commands/migrate.rs:408 when an unfinished item's Done when holds only After merge lines with no stand-in, apply removes them all, the write guard rejects the empty section, and the whole batch fails with 'required section Done when is empty' naming no item (reproduced in a scratch repo). Nothing is written, and no record on this host hits it today (only astromech has open lines, all five with stand-ins). Fix later: name the item and keep the line as an ordinary Done when line when it is the last one, or skip it with a message.
