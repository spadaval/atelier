---
schema: atelier.review
schema_version: 2
id: atelier-ncen
title: finding on atelier-qr5s @9394bb27
target: atelier-qr5s
identity: grok-4.6-amend3-r1
commit: 9394bb27f77fc5da005bab5e28496dbf8e6a0a27
created_at: 2026-11-10T04:53:44.877926370Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: 6858374bac03365545b89809e165633d5d2efd0b
    recorded_at: 2026-11-10T04:55:20.637070195Z
    follow_up: atelier-u1ko
    reason: 'Deferred to the switch step''s planning: atelier-u1ko''s Unknowns now lists the files the move touches.'
---

medium atelier-u1ko and redesign-architecture.md:474: the switch deletes crates/ and moves next/ to the root, and does not say to collapse the dual-workspace CI mal9 adds or to keep root [profile.dist] (Cargo.toml:37) when next/Cargo.toml overwrites it. dist-workspace.toml stays at the root as cargo:. Consequence: after the move, a ci.yml job with working-directory next fails, and cargo-dist loses profile.dist unless next/ already has it. Smallest fix: u1ko Done when: ci.yml and ci-jobs.sh become one workspace again; next/Cargo.toml includes [profile.dist] or the switch rewrites root Cargo.toml without dropping it.
