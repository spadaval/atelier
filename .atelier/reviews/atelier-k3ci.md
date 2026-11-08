---
schema: atelier.review
schema_version: 3
id: atelier-k3ci
title: approval on branch docs/no-draft-prs @c4aed6be
branch: docs/no-draft-prs
identity: reviewer-nodraft
commit: c4aed6be336892b1c0746e6ca98c94b50790d772
created_at: 2026-11-08T15:14:00.315722941Z
event:
  event: approval
---

Read the AGENTS.md diff: the merge-policy sentence still reads correctly and keeps push, open PR, wait for CI, merge, land and push master in order. Grepped the whole repo (AGENTS.md, CONTEXT.md, docs/, scripts/, .github/, crates/atelier-cli, .atelier workflow.yaml and config.toml) and the installed Agent Factory skill: nothing else tells agents to open draft PRs, and scripts/merge-ci-passed.sh queries gh run list by commit, not draft state. Remaining draft hits are mission/epic statuses or dated tracker notes (finding atelier-2wrn). git diff --check, cargo fmt --check and cargo metadata --locked pass. merge-ci-passed.sh runs only in the merge checkout; CI on PR #52 at c4aed6be is in progress, pending.
