---
schema: atelier.review
schema_version: 1
id: atelier-qit8
title: $HOME/.cache/atelier-tmp/scripts/pipe_check.sh (receipt atelier-8bl7) — the scenario cannot tell the fixed and unfixed builds apart. Its fresh-fixture outputs fit in one pipe buffer, so all seven Done-when commands exit 0 with empty stderr even when restore_default_sigpipe() is commented out (I ran it against such a build). No action, because the receipt itself names the integration test as the durable proof, and that test does discriminate. On the real repo, review list | head -1 gives 101 plus a panic without the fix and 141 with it.
target: atelier-5rs0
identity: Independent reviewer (Claude subagent)
commit: e3ac5198f39683addf8584c26576f99f55871ea4
created_at: 2026-10-17T06:30:54.579841237Z
event:
  event: finding
  recommendation: no_action
---

$HOME/.cache/atelier-tmp/scripts/pipe_check.sh (receipt atelier-8bl7) — the scenario cannot tell the fixed and unfixed builds apart. Its fresh-fixture outputs fit in one pipe buffer, so all seven Done-when commands exit 0 with empty stderr even when restore_default_sigpipe() is commented out (I ran it against such a build). No action, because the receipt itself names the integration test as the durable proof, and that test does discriminate. On the real repo, review list | head -1 gives 101 plus a panic without the fix and 141 with it.
