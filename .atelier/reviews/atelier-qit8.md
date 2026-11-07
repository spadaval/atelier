---
schema: atelier.review
schema_version: 2
id: atelier-qit8
title: $HOME/.cache/atelier-tmp/scripts/pipe_check.sh (receipt atelier-8bl7) — the scenario cannot tell the fixed and unfixed builds apart. Its fresh-fixture outputs fit in one pipe buffer, so all seven Done-when commands exit 0 with empty stderr even when restore_default_sigpipe() is commented out (I ran it against such a build). No action, because the receipt itself names the integration test as the durable proof, and that test does discriminate. On the real repo, review list | head -1 gives 101 plus a panic without the fix and 141 with it.
target: atelier-5rs0
identity: Independent reviewer (Claude subagent)
commit: e3ac5198f39683addf8584c26576f99f55871ea4
created_at: 2026-10-17T06:30:54.579841237Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: e3ac5198f39683addf8584c26576f99f55871ea4
    recorded_at: 2026-10-17T06:31:33.858087657Z
    reason: 'Accepted: the scenario script''s outputs fit one pipe buffer, so the closed-stdout integration test is the proof that discriminates; the receipt says so.'
    migrated_from: atelier-zkg0
---

$HOME/.cache/atelier-tmp/scripts/pipe_check.sh (receipt atelier-8bl7) — the scenario cannot tell the fixed and unfixed builds apart. Its fresh-fixture outputs fit in one pipe buffer, so all seven Done-when commands exit 0 with empty stderr even when restore_default_sigpipe() is commented out (I ran it against such a build). No action, because the receipt itself names the integration test as the durable proof, and that test does discriminate. On the real repo, review list | head -1 gives 101 plus a panic without the fix and 141 with it.
