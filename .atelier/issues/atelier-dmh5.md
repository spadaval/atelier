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
    id: "atelier-ihss"
    type: "related"
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
  - kind: "review"
    id: "atelier-3ewf"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Store the direction of directional context links"
updated_at: "2026-10-17T06:57:31.852566483+00:00"
---

## Goal

`atelier issue link xmtr qkmr --role supersedes` writes `type: "supersedes"` onto both records (`add_issue_relation`, `crates/atelier-records/src/lib.rs:469-498`). `issue show qkmr` then prints `supersedes atelier-xmtr`, which is backwards: xmtr supersedes qkmr. Once stored, the records can't say which side is which. The same applies to `duplicates`, `implements`, `derived_from`, `part_of`, `falsifies` and directional custom roles. This repo only uses `related` and `advances` (22 each), so adopters are the ones affected now.

## Done when

- Directional roles keep their direction: the target stores an inverse (`superseded_by`, …) or a direction marker, and each side renders correctly (`superseded by atelier-xmtr`).
- Symmetric roles (`related`) and `advances` are unchanged.
- `atelier check` flags existing records that store the same directional type on both sides, or a migration rewrites them.
- Tests link in each direction and check both `issue show` outputs, and unlink removes both sides.

## Unknowns

Whether custom roles from `.atelier/config.toml` need a declared inverse or direction flag.

## Notes

- 2026-10-17 [note] 2026-10-17, from review of atelier-ihss (finding atelier-3ewf, deferred here): after ihss, links are stored only on the declaring record, but two places still treat a link in the reverse direction as the same link: the duplicate check in RecordStore::add_issue_relation (crates/atelier-records/src/lib.rs ~509-515) and the two-sided dedupe in index_issue_graph (crates/atelier-app/src/record_index.rs ~569-576). So `issue link A B --role supersedes` prints 'already exists' when B already declares 'supersedes A', and if both declare it toward each other only one is indexed. In the index a Relation's issue_id_1 is the declaring record; for legacy two-sided links it is whichever id sorts first, so direction is ambiguous there. blocked_by is now a directional entry in relates; blocks remains the blocker-side form.
