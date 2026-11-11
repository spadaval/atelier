---
created_at: "2026-10-17T01:10:57.963324057+00:00"
id: "atelier-dmh5"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "issue"
    id: "atelier-ihss"
    type: "related"
  - kind: "review"
    id: "atelier-3ewf"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:42.992600863+00:00"
status: "declined"
title: "Treat a reversed directional link as a different link"
updated_at: "2026-11-11T03:55:42.992600863+00:00"
---

## Goal

Since atelier-ihss each new link is stored only on the record that declares it, so a directional role such as `supersedes` keeps its direction. Two places still treat a link in the reverse direction as the same link (finding atelier-3ewf, note below): the duplicate check in `RecordStore::add_issue_relation` (crates/atelier-records/src/lib.rs) prints 'already exists' for `issue link A B --role supersedes` when B already declares `supersedes A`, and the two-sided dedupe in `index_issue_graph` (crates/atelier-app/src/record_index.rs) indexes only one of two records that declare the role toward each other. Records written before ihss that store the same directional type on both sides leave the direction ambiguous. The same applies to `duplicates`, `implements`, `derived_from`, `part_of`, `falsifies` and directional custom roles.

## Done when

- [ ] `issue link A B --role supersedes` succeeds when B already declares `supersedes A`, and both links are indexed and shown with the right direction on each side (`supersedes` and `superseded by`).
- [ ] Records that store the same directional type on both sides are read with a defined direction, or `atelier check` names them with the edit that fixes them.
- [ ] Symmetric roles (`related`) and `advances` are unchanged; tests link in each direction.

## Context

Narrowed in decision atelier-mzpn to what ihss left.

## Unknowns

Whether custom roles from `.atelier/config.toml` need a declared direction.

## Notes

- 2026-10-17 [note] 2026-10-17, from review of atelier-ihss (finding atelier-3ewf, deferred here): after ihss, links are stored only on the declaring record, but two places still treat a link in the reverse direction as the same link: the duplicate check in RecordStore::add_issue_relation (crates/atelier-records/src/lib.rs ~509-515) and the two-sided dedupe in index_issue_graph (crates/atelier-app/src/record_index.rs ~569-576). So `issue link A B --role supersedes` prints 'already exists' when B already declares 'supersedes A', and if both declare it toward each other only one is indexed. In the index a Relation's issue_id_1 is the declaring record; for legacy two-sided links it is whichever id sorts first, so direction is ambiguous there. blocked_by is now a directional entry in relates; blocks remains the blocker-side form.

## Report

Dropped in the cutover sort (atelier-5zre), reason (b). The new records hold no directional link roles, only `blocked_by` and `caused_by` (product specification, Records), so there is no reversed link to misread.
