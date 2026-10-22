---
schema: atelier.review
schema_version: 1
id: atelier-zhpk
title: 'Reviewed dc4ce18d..2239abec. plu8: ss25 test now runs under fast_forward_only and merge_commit, and merge --check runs merge_claim_nodes under both, so its merge assertion can fail. bu66: merge_claim_nodes now runs on the would-be base commit under both strategies (branch head under fast-forward, trusting base_head); new test merge_refuses_a_done_mission_the_branch_gives_open_work covers both. Scratch repos under both strategies (done issue newly blocked by a branch issue, new open child of a done base issue, side branch carrying an approved and a broken done claim, approve-after-close, clean control): merge --check, land on the branch and land on the base after integration agree every time. g0mh: widening documented and tested (base_issue_edited cases); it cannot grow the scope or loosen in-scope readiness; one low doc-precision finding recorded (defer). git merge-tree --write-tree master HEAD is clean (b9b0cf46). Proof rerun: nextest --workspace --features test-hooks 863/863, without features 859/859, fmt, diff --check, cargo metadata --locked all pass.'
target: atelier-r28p
identity: Independent reviewer (Claude subagent)
commit: 2239abecc5796d58309d9ba151bae100f131537b
created_at: 2026-10-22T22:48:25.557513851Z
event:
  event: approval
---

Reviewed dc4ce18d..2239abec. plu8: ss25 test now runs under fast_forward_only and merge_commit, and merge --check runs merge_claim_nodes under both, so its merge assertion can fail. bu66: merge_claim_nodes now runs on the would-be base commit under both strategies (branch head under fast-forward, trusting base_head); new test merge_refuses_a_done_mission_the_branch_gives_open_work covers both. Scratch repos under both strategies (done issue newly blocked by a branch issue, new open child of a done base issue, side branch carrying an approved and a broken done claim, approve-after-close, clean control): merge --check, land on the branch and land on the base after integration agree every time. g0mh: widening documented and tested (base_issue_edited cases); it cannot grow the scope or loosen in-scope readiness; one low doc-precision finding recorded (defer). git merge-tree --write-tree master HEAD is clean (b9b0cf46). Proof rerun: nextest --workspace --features test-hooks 863/863, without features 859/859, fmt, diff --check, cargo metadata --locked all pass.
