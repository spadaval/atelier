---
schema: atelier.review
schema_version: 2
id: atelier-41mc
title: 'issue.rs:286 maps any issue_transition_options error to ''Transitions: none'' for done items (including repo_root/action_policy failures), and no test asserts the closed-item brief view; test it and narrow the match to the no-transitions case.'
target: atelier-7u4o
identity: Independent reviewer (Claude subagent)
commit: 96654ae0441c66cbe31a04363578bfa207a9a058
created_at: 2026-10-17T02:42:49.001819620Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: e06d0456e69cd5113dfcaa16352192cbbd8e9ea1
    recorded_at: 2026-10-17T02:49:59.685687745Z
    resolution_commit: 5c4498a13656369cc60c7abf97434c5a5287551f
    reason: Typed NoTransitions error; only it renders 'none'; closed-item brief test added.
    migrated_from: atelier-dl5w
---

issue.rs:286 maps any issue_transition_options error to 'Transitions: none' for done items (including repo_root/action_policy failures), and no test asserts the closed-item brief view; test it and narrow the match to the no-transitions case.
