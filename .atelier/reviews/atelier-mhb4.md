---
schema: atelier.review
schema_version: 2
id: atelier-mhb4
title: finding on atelier-yfqu @afdcfda0
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: afdcfda0c4ab99e6c2f020c0773a9a446803fe4b
created_at: 2026-11-09T01:08:02.340020987Z
event:
  event: finding
  recommendation: fix_now
---

medium: Caching every rendered page only by the base commit hides branch and review changes.
.atelier/issues/atelier-zpmb.md:36; docs/strategies/fleet-dashboard.md:45-47; .atelier/issues/atelier-ahpf.md:32,35.
The load-bound fix requires a page to be computed once per base commit, but the repository page must match status for in-progress branches and current/stale approvals. Starting work, committing a finding or approval on its branch, or changing code after approval moves a work-branch ref without moving master. A previously rendered repository page then keeps its old branch/review state indefinitely even though status on master has changed. The existing status path reads current refs/heads and branch snapshots (crates/atelier-cli/src/commands/git.rs:982 onward), so these inputs are not determined by the base commit. The last-14-days section also changes as time passes without any commit. Retain the render semaphore and resource limits, but cache only immutable base-derived fragments by base SHA; invalidate branch-derived views on their ref inputs or give them a stated bounded refresh, and account for time-derived sections. Add a warm-cache fixture that moves only an owned work branch from unreviewed to approved to stale while master stays fixed, and shows the page updates to match status.
Recommendation: FIX NOW. This is a new integration defect introduced by the load fix, contradicting the current-and-correct site outcome and the ready repository-page acceptance.
