---
created_at: "2026-10-17T02:56:36.227115148+00:00"
id: "atelier-uw4x"
issue_type: "bug"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make merge refuse a done claim that land will refuse"
updated_at: "2026-10-17T19:00:36.343866065+00:00"
---

## Goal

`atelier merge <id> --check` and `merge` passed atelier-sdb0 with `review.current: current independent approval recorded` although the approval (pinned at ccda8bdd) predated the commit that closed the issue. After the fast-forward, `atelier land` on master refused with `land_base_done_claim: commit c2f1f8a9 on master marks atelier-sdb0 done outside review`, because land counts only commits reachable from an approval pin as reviewed. The two gates disagree, so the refusal arrives only after integration, on the base, where the fix is a records-only approval commit.

## Done when

`merge --check` and `merge` fail, before integrating, when the owner's done claim is not reachable from a current approval or trivial exemption, naming the fix (record a fresh approval at or after the close commit). A test closes after approving and asserts merge refuses; the existing land test still passes. Alternatively, if a status-only change after approval is meant to stay covered, land accepts it the same way merge does; either way one rule, stated in `docs/product/`.

## Context

`crates/atelier-cli/src/commands/git.rs` (land coverage), the merge gate `review.current`. Seen 2026-10-16 merging atelier-sdb0 (mission atelier-1w8c).

## Notes

- 2026-10-17 [note] Retro 2026-10-17: sdb0's merge passed a done claim that land on master then refused, costing a fifth review subagent; still the top cause of the 'approve after close' rule. See also the new approval-carry item.
