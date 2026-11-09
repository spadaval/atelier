---
created_at: "2026-11-09T02:01:20.277399423+00:00"
id: "atelier-vmq2"
issue_type: "decision"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T02:01:20.594187627+00:00"
status: "done"
title: "Let Atelier push branches, open pull requests and read CI through a forge, with review staying in the repository"
updated_at: "2026-11-09T02:01:20.594187627+00:00"
---

## Context

The merged-tree rule (`docs/product/reviews-and-merge.md`) was written for local `shell.check`s, which run on the merge commit. A CI run only ever saw the branch head, so the script fakes the rule by refusing whenever the base gained code, docs included.

## Question

May Atelier talk to a forge (GitHub) to push work branches, open pull requests and read CI results? ADR 0019 removed the Forgejo provider, and `PRODUCT_INTENT.md` keeps Atelier a local CLI with no provider. Today the merge policy is a hand-run routine (push the branch, open a PR, wait for CI, `atelier merge`, `atelier land`, push the base), and the CI gate is a repository script, `scripts/merge-ci-passed.sh`, run as a `shell.check`.

## Options

- **Forge for transport and CI.** Atelier pushes branches, makes sure each has a pull request, reads CI status for a commit, and pushes the base after merging. Reviews, findings and approvals stay Markdown records in the repository. GitHub first, through the `gh` CLI.
- **Hooks.** Keep Atelier forge-free and let each repository plug scripts into merge. Rejected: Atelier cannot tell what a script proves, so it cannot explain refusals or decide when a base change needs a new CI run, and each repository copies and drifts its own script. ADRs 0012 and 0013 already rejected generic hooks.
- **No change.** Keep the routine and the script. Rejected: on 2026-11-08 atelier-3sk9 needed three syncs and CI reruns because other threads committed strategy docs and a skill change to master while CI ran.

## Decision

Allow a forge for transport and CI. The Forgejo provider was cut in the rebuild to shed complexity, not because Atelier should not reach a forge. What stays from ADR 0019: reviews, findings and approvals are records in the repository (Decision 6), there is no external review provider, and the guards decide from Git and records. A forge only carries branches and reports CI. GitHub first, through `gh`; add another forge only when a repository moves to one. A CI result proves the branch head, not the merged tree; a base that moved since that run needs no new run, and CI on the base after the push is the backstop. Decided by Sumeet on 2026-11-08 in a design discussion; ADR 0031 records it (mission forge-delivery's first item).

## Report

Allowed: a forge for pushing branches, pull requests and CI status, GitHub first through gh; reviews stay repository records. Decided by Sumeet on 2026-11-08; ADR 0031 records it.
