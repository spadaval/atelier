---
schema: atelier.review
schema_version: 2
id: atelier-q0hg
title: finding on atelier-1gb3 @8095332a
target: atelier-1gb3
identity: codex-plan-reviewer (gpt-6.1-sol)
commit: 8095332a5c72f40efe4006ed20d26b13e84f456b
created_at: 2026-11-09T02:07:16.648537815Z
event:
  event: finding
  recommendation: defer
---

medium: docs/strategies/forge-delivery.md:93 The fallback of closing the pull request if a pushed local merge does not mark it merged fails the Outcome, which requires the pull request to show as merged. gh pr close closes without performing a merge; closed is not merged. GitHub documents indirect merges when the head commits become reachable from the base, so the normal merge-commit push route is supported: https://docs.github.com/en/pull-requests/reference/pull-request-merges#indirect-merges and https://cli.github.com/manual/gh_pr_close . Before atelier-vqoy becomes ready, replace this fallback with verification of the merged state and an honest refusal or an evidenced route preserving the Atelier merge commit. Deferred because the affected implementation is still backlog, not part of the ready CI-check increment.
