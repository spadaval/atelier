---
schema: atelier.review
schema_version: 2
id: atelier-wgan
title: finding on atelier-45gr @3d8be7a4
target: atelier-45gr
identity: reviewer-45gr
commit: 3d8be7a4ca8ff9e250ebd16032468977b2a430df
created_at: 2026-11-08T07:07:15.379631668Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/freshness.rs:562-579 run() bounds only the wait on the direct child. Once the child exits, it joins the stdout/stderr reader threads with no deadline, so a grandchild that outlives its parent while holding the pipe (as in `sh -c 'sleep 30 &'`) would hang doctor, or leave the detached refresh running, past the 5 s and 10 s timeouts. I found no current path that does this: skills 1.7.6's ls and check spawn nothing, and git ls-remote waits on its remote helper. Suggested fix: call kill_group after a normal exit too (the group lingers only while members remain), or join the readers against the same deadline. A unit test like a_command_that_outlives_its_timeout_is_stopped with 'sleep 30 &' alone would cover it.
