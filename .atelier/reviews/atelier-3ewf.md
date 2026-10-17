---
schema: atelier.review
schema_version: 1
id: atelier-3ewf
title: 'Low, for atelier-dmh5. Direction is still conflated in two places: RecordStore::add_issue_relation (crates/atelier-records/src/lib.rs ~509-515) treats the opposite-direction entry on the other record as the same link, so ''issue link A B --role supersedes'' prints ''already exists'' when B declares ''supersedes A''; and index_issue_graph (crates/atelier-app/src/record_index.rs ~569-576) dedups typed links by unordered pair and type, so if A and B each declare a directional link to the other, only the first by id is indexed. Same observable behaviour as master under two-sided storage, so no regression, and one-sided storage actually makes the declarer the natural source for dmh5. Note both spots on atelier-dmh5 so it makes them direction-aware for directional types.'
target: atelier-ihss
identity: Independent reviewer (Claude subagent)
commit: af8645378791a3ff66fe5d8ab2a5db7d9cd010bd
created_at: 2026-10-17T06:27:54.540910365Z
event:
  event: finding
  recommendation: defer
---

Low, for atelier-dmh5. Direction is still conflated in two places: RecordStore::add_issue_relation (crates/atelier-records/src/lib.rs ~509-515) treats the opposite-direction entry on the other record as the same link, so 'issue link A B --role supersedes' prints 'already exists' when B declares 'supersedes A'; and index_issue_graph (crates/atelier-app/src/record_index.rs ~569-576) dedups typed links by unordered pair and type, so if A and B each declare a directional link to the other, only the first by id is indexed. Same observable behaviour as master under two-sided storage, so no regression, and one-sided storage actually makes the declarer the natural source for dmh5. Note both spots on atelier-dmh5 so it makes them direction-aware for directional types.
