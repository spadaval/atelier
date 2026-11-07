---
schema: atelier.review
schema_version: 2
id: atelier-cgkt
title: '[low] .atelier/issues/atelier-zqee.md Close Reason says ''Upstream spadaval/.agents (451c0c9) ... now say'' it, but git ls-remote shows origin (git@github.com:spadaval/.agents.git) main at 07c8d1c; 2f2d0f7 and 451c0c9 exist only on the local branch agent-factory/merge-commit-strategy. No branch change is needed: the text is reviewed here, and upstream should merge only reviewed text. The landing step carries the obligation: merge agent-factory/merge-commit-strategy into upstream main and push it before or together with atelier merge, so that master never vendors text upstream lacks (ADR 0020) and the first done-when becomes true.'
target: atelier-zqee
identity: Independent reviewer (Claude subagent)
commit: 32788007725837251307c2aea2a51782353594e9
created_at: 2026-10-17T07:20:45.388278186Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude Opus 5.5)
    commit: b7e8a0ac8ebec794dbdc684ad6006b59f47427ed
    recorded_at: 2026-10-17T07:21:10.676571762Z
    reason: 'Resolved outside the branch: the reviewed text (451c0c9) is now pushed to spadaval/.agents main, so the Close Reason holds.'
    migrated_from: atelier-j0zi
---

[low] .atelier/issues/atelier-zqee.md Close Reason says 'Upstream spadaval/.agents (451c0c9) ... now say' it, but git ls-remote shows origin (git@github.com:spadaval/.agents.git) main at 07c8d1c; 2f2d0f7 and 451c0c9 exist only on the local branch agent-factory/merge-commit-strategy. No branch change is needed: the text is reviewed here, and upstream should merge only reviewed text. The landing step carries the obligation: merge agent-factory/merge-commit-strategy into upstream main and push it before or together with atelier merge, so that master never vendors text upstream lacks (ADR 0020) and the first done-when becomes true.
