---
schema: atelier.review
schema_version: 1
id: atelier-vfgr
title: 'The upstream fallback can say ''already done'' for a fresh local claim. Probe: item done and pushed; locally hand-edit it to in_progress (commit), make it unparsable (commit), restore it as done (commit). land does not raise land_base_done_claim, while the same sequence without the broken commit is refused. The fallback skips the unpushed parseable copy (in_progress) and reads the pushed copy (done). The impact is small because the pushed status is unchanged, but this is the break-and-restore pattern atelier-2rse is about. Fix: before falling back to upstream, walk back to the nearest parseable copy in the unpushed range, as Done when item 1 offers.'
target: atelier-hyew
identity: Independent reviewer (Claude subagent)
commit: 387dbfad150d05d24d840cfefcc70e4154673ba4
created_at: 2026-10-16T20:46:11.248401887Z
event:
  event: finding
  recommendation: defer
---

The upstream fallback can say 'already done' for a fresh local claim. Probe: item done and pushed; locally hand-edit it to in_progress (commit), make it unparsable (commit), restore it as done (commit). land does not raise land_base_done_claim, while the same sequence without the broken commit is refused. The fallback skips the unpushed parseable copy (in_progress) and reads the pushed copy (done). The impact is small because the pushed status is unchanged, but this is the break-and-restore pattern atelier-2rse is about. Fix: before falling back to upstream, walk back to the nearest parseable copy in the unpushed range, as Done when item 1 offers.
