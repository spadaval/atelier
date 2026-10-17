---
schema: atelier.review
schema_version: 1
id: atelier-41mc
title: 'issue.rs:286 maps any issue_transition_options error to ''Transitions: none'' for done items (including repo_root/action_policy failures), and no test asserts the closed-item brief view; test it and narrow the match to the no-transitions case.'
target: atelier-7u4o
identity: Independent reviewer (Claude subagent)
commit: 96654ae0441c66cbe31a04363578bfa207a9a058
created_at: 2026-10-17T02:42:49.001819620Z
event:
  event: finding
  recommendation: fix_now
---

issue.rs:286 maps any issue_transition_options error to 'Transitions: none' for done items (including repo_root/action_policy failures), and no test asserts the closed-item brief view; test it and narrow the match to the no-transitions case.
