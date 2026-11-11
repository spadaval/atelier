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
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Check base commits that reached the upstream without review, in CI"
updated_at: "2026-11-07T21:03:32.204039302+00:00"
---

## Goal

The pre-push hook (`atelier check --hook pre-push`) checks the base's new commits only in a clone that has the hooks and only when nobody skips them (`--no-verify`, or `ATELIER_SKIP`), so once an unreviewed commit is pushed past it, nothing checks it again (incident 1620b534 on 2026-10-17, under the old `land`, was found only because an agent noticed). Apply the hook's base rule to `<previous head>..HEAD` in this repository's CI on each push to master, so a slipped push turns CI red within minutes. The red run is a notice, not a block: the commit is already on the base, and the fix is a reviewed follow-up, through an item or `atelier start --new`.

## Done when

- [ ] Integration tests with a bare origin: an unreviewed code commit pushed to the base with the hook skipped is refused when the base rule runs over the range from the previous head, naming the commit.
- [ ] A range holding only planning commits (`.atelier/missions/`, `.atelier/items/`) and Atelier merge commits, `--trivial` ones included, passes.
- [ ] The CI job runs the rule on pushes to master with the push's previous head and accepts a `since` input under `workflow_dispatch`; a dispatched run on a scratch branch whose range holds an unreviewed commit is shown failing, and one on master passing, with the run links in the Report.

## Scope

The pre-push hook's base rule and the command that runs it, and the CI workflow. Prevention is the hook's.

## Context

Kept in decision atelier-mzpn from the superseded atelier-ll3p: a CI notice that costs agents nothing and watches the one invariant the review rule keeps, that no unreviewed code reaches the base.

Rewritten for the new tool in the cutover sort (atelier-5zre): `land --since <commit>` became the pre-push hook's base rule run over a range, since the new tool has no `land`; the light path became `start --new`.

## Unknowns

- CI checks out a detached HEAD; running the rule may need to accept one on the base commit.
- A first push or force push has no usable previous head (`github.event.before` is all zeros or unreachable); decide whether CI then checks from the last merge commit or skips with a visible note.
