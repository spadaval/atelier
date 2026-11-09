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
  disposition:
    decision: fix_now
    identity: codex-plan-rereviewer (gpt-6.1-sol)
    commit: 50824282d5eb20545e826281d02aa0f93e83cac3
    recorded_at: 2026-11-09T02:12:41.882059152Z
    resolution_commit: 6715d534869f0ae784e7e08b8aefd3c8280e24fb
    reason: 'The valid finding was fixed in 6715d534 rather than deferred: strategy lines 96-101 and atelier-vqoy now verify GitHub merged state, report a published base with an open pull request honestly, and never close it as a substitute for merging. Official GitHub indirect-merge documentation supports the normal merge-commit push route. Record the actual fix with fix_now and its resolution commit; no follow-up, no_action or wont_fix would describe what happened. This repair is accepted, though the mission approval still awaits the separate new bootstrap blocker on atelier-7tul.'
---

medium: docs/strategies/forge-delivery.md:93 The fallback of closing the pull request if a pushed local merge does not mark it merged fails the Outcome, which requires the pull request to show as merged. gh pr close closes without performing a merge; closed is not merged. GitHub documents indirect merges when the head commits become reachable from the base, so the normal merge-commit push route is supported: https://docs.github.com/en/pull-requests/reference/pull-request-merges#indirect-merges and https://cli.github.com/manual/gh_pr_close . Before atelier-vqoy becomes ready, replace this fallback with verification of the merged state and an honest refusal or an evidenced route preserving the Atelier merge commit. Deferred because the affected implementation is still backlog, not part of the ready CI-check increment.
