---
schema: atelier.review
schema_version: 2
id: atelier-5107
title: 'docs/product/work-view-ordering.md:59-61 — the added clause ''so every work view offers to start only work that start accepts'' is false on a current path. start reads the records on the owner''s existing branch (crates/atelier-cli/src/commands/start.rs:52-66), but the views read the current checkout. Repro: parent P with children C1 (low) and C2 (critical), C2 blocked_by B. Run start C1, which creates task/P, then on main supersede B and commit. work mission and work parent then list C2 as Ready, and the footer says ''atelier start C2''. start C2 refuses: ''direct blocker path C2 -> B''. start also refuses ready rows for a dirty checkout or unreviewed base commits. Fix: limit the sentence to the transitive blocker rule on the records the view reads, or drop the ''so every work view...'' clause.'
target: atelier-6wyv
identity: Independent reviewer (Claude subagent)
commit: 1380e97a1504d0cb1859aaaee518e7d616ea02ea
created_at: 2026-10-17T08:53:38.404486587Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 7f23f3420c9f83a4d40cb552a0319201fcaf68ce
    recorded_at: 2026-10-17T08:54:24.066758805Z
    resolution_commit: 7f23f3420c9f83a4d40cb552a0319201fcaf68ce
    reason: The ready definition now claims only the shared blocker rule on the records each reads.
    migrated_from: atelier-nddt
---

docs/product/work-view-ordering.md:59-61 — the added clause 'so every work view offers to start only work that start accepts' is false on a current path. start reads the records on the owner's existing branch (crates/atelier-cli/src/commands/start.rs:52-66), but the views read the current checkout. Repro: parent P with children C1 (low) and C2 (critical), C2 blocked_by B. Run start C1, which creates task/P, then on main supersede B and commit. work mission and work parent then list C2 as Ready, and the footer says 'atelier start C2'. start C2 refuses: 'direct blocker path C2 -> B'. start also refuses ready rows for a dirty checkout or unreviewed base commits. Fix: limit the sentence to the transitive blocker rule on the records the view reads, or drop the 'so every work view...' clause.
