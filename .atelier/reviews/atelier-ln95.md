---
schema: atelier.review
schema_version: 2
id: atelier-ln95
title: finding on atelier-a4rs @ddec1a4d
target: atelier-a4rs
identity: GPT-6.1 Sol independent planning reviewer
commit: ddec1a4d5d163a01c63c2711ff3c77494c01a148
created_at: 2026-11-11T04:46:17.441573258Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-a4rs.md:53-56 at ddec1a4d: medium, FIX NOW. The agent cap still does not cover every launch path, so atelier-9x7a and atelier-1hf0 remain partly unresolved. Two non-threaded Codex roots are allowed concurrently, but the proposed Codex config permits one live subagent per root instead of disabling spawn_agent for those runs: one such spawn already makes three eval agents alive. Claude Code's native Agent tool is another uncovered path, even though its child shares an Atelier identity and is not a new thread. The current root and delegate commands leave native tools available (harness/run.py:110-137,320-332; results/2026-11-10-next-loop.md:69-71), and the revised plan restricts only delegate launches under Claude. A root plus its delegate plus either one's native Agent child exceeds two. State that non-threaded runs cannot start any child, and disable Claude native Agent/Task launches in both roots and delegates; retain the one-child/depth bound for threaded Codex runs. Require non-agent tests of these denied launch paths as well as the planned delegate/config checks, for smoke runs and the t747 matrix.
