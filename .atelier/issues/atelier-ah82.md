---
created_at: "2026-11-09T02:02:04.388927238+00:00"
id: "atelier-ah82"
issue_type: "task"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record ADR 0031 and document the forge, ci.passed and the one-command merge before building them"
updated_at: "2026-11-09T02:02:04.388927238+00:00"
---

## Goal

Write decision atelier-vmq2 down as ADR 0031 and describe the new interface in the product docs, so the forge policy keys, the `ci.passed` check and the merge flow are reviewed on paper before any code is written (strategy `docs/strategies/forge-delivery.md`, Shape).

## Done when

- [ ] `docs/adr/0031-*.md` states the decision: a forge carries branches and reports CI; reviews, findings and approvals stay repository records (ADR 0019 Decision 6 stands); GitHub first, through `gh`; a CI result proves the branch head, so a moved base needs no new run, and CI on the base after the push is the backstop; a failing base CI run is shown, never a gate. It is marked as scoping ADR 0019 where 0019 removed the provider, and ADR 0019 links to it.
- [ ] `PRODUCT_INTENT.md`'s Boundaries no longer read as ruling out a forge: Atelier is still not a review provider, and may push branches, open pull requests and read CI when a repository's policy names a forge.
- [ ] `docs/product/workflow-configuration.md` gives the policy keys that name the forge and the trusted CI workflow, with an example, and says a policy without them uses no network.
- [ ] `docs/product/reviews-and-merge.md` describes `ci.passed` (what it proves, when it passes, its refusals and their Next lines), the merged-tree rule's scope (local `shell.check`s prove the merged tree; `ci.passed` proves the branch head), and `merge --push` and `--wait` step by step, including the pull request and what happens when `gh` is missing or signed out.
- [ ] No other current doc contradicts these (`git grep -n -i "merge-ci-passed\|review provider\|forge" docs PRODUCT_INTENT.md CONTEXT.md` shows each hit updated or historical).

## Scope

Docs and the ADR only; no code. Strategy files stay as they are.

## Context

The interface these docs set is what atelier-7tul builds. Name policy keys and flags here; the strategy's Authority lets the orchestrator choose them.
