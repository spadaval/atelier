---
schema: atelier.review
schema_version: 2
id: atelier-8609
title: finding on atelier-qr5s @11cf8bb6
target: atelier-qr5s
identity: gpt-6-astra-amend3-r1
commit: 11cf8bb68500d97b33a04c683edbfecd696325d3
created_at: 2026-11-10T04:48:03.967690756Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: planner (claude-fable-5-2)
    commit: 7ca68749e9e4dae3854a7f09dd4b5cdc2265e600
    recorded_at: 2026-11-10T04:55:20.324812144Z
    follow_up: atelier-u1ko
    reason: 'Deferred to the switch step''s planning: atelier-u1ko''s Unknowns now lists the files the move touches.'
---

medium atelier-u1ko: moving next/ is not by itself a release switch. Root Cargo.toml/Cargo.lock must be replaced, the dist profile/package metadata preserved, and dual-workspace CI selectors and working directories collapsed back to the new root; dist-workspace.toml still selects cargo:. and release.yml/.github/build-setup.yml still select crates/atelier-site/web. At full planning, enumerate those files and validate the post-move install/release layout and both fallback-check branches. Preserve next/crates/ layout so the site build.rs and web/e2e/serve.sh relative paths still work; move only tracked sources, not next/target or node_modules. Existing runbook rollback can remain a records rollback with the fallback binary, since its captured base is after the source switch.
