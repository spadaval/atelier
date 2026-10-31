---
created_at: "2026-10-30T06:52:06.059708538+00:00"
id: "atelier-nl9a"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ll3p"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Check base commits that reached the upstream without review, in CI"
updated_at: "2026-10-30T06:52:06.059708538+00:00"
---

## Goal

`land` on the base checks only `rev-list <tip> --not <upstream>` (crates/atelier-cli/src/commands/git.rs:230), so once an unreviewed commit is pushed, past a gate whose refusal was masked by `| tail` or a `;` chain, nothing ever checks it again (incident 1620b534 on 2026-10-17 was found only because an agent noticed). Add `atelier land --since <commit>` that applies land's base rules to `<commit>..HEAD` instead of the unpushed range, and run it in this repository's CI on each push to master with the push's previous head, so a slipped push turns CI red within minutes. The red run is a notice, not a block: the commit is already on the base, and the fix is a reviewed follow-up or an after-the-fact `review trivial --base`.

## Done when

Integration tests with a bare origin: an unreviewed non-record commit pushed to the base passes plain `land` (unchanged behaviour) but `land --since <previous head>` refuses with `land_base_unreviewed` naming it; a range holding only record commits, gated merge commits and trivially exempted commits passes. The CI job runs `atelier land --since <before>` on pushes to master and also accepts a `since` input under `workflow_dispatch`; a dispatched run on a scratch branch whose range holds an unreviewed commit is shown failing, and one on master passing (run links in the receipt). A receipt on atelier-ll3p judges criterion slipped-push-caught with those runs.

## Scope

`land` in commands/session.rs and git.rs, the CI workflow, docs/product/sessions-and-hooks.md. Prevention stays with atelier-fad5.

## Unknowns

- `BaseReviews::new(..., &upstream)` (git.rs:229) also takes the upstream as its boundary; `--since` replaces both uses.
- CI checks out a detached HEAD; `land --since` may need to accept one on the base commit.
- A first push or force push has no usable previous head (`github.event.before` is all zeros or unreachable); decide whether CI then checks from the last gated merge or skips with a visible note.
