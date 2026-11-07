---
schema: atelier.review
schema_version: 2
id: atelier-5ywv
title: 'Consequences (ADR:60-63) and atelier-sdb0''s Done when list only sessions-and-hooks.md and Agent Factory references/atelier.md as prime docs to change. Other current references would go stale when prime is deprecated: PRODUCT_INTENT.md:43, docs/product/work-model.md:67,76,152, docs/product/index.md:18, AGENTS.md:20 (''Start a session with target/debug/atelier prime''), Agent Factory procedures/install.md:34 (tells adopters to write ''atelier prime to start'' into AGENTS.md; upstream per ADR 0020), the setup claude success message (crates/atelier-cli/src/commands/setup.rs:99 ''Hooks run prime''), and root help (main.rs:20). Fix: extend sdb0''s Done when (or the ADR consequence) to cover them.'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:54.066687790Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: a485ef023556583b5731cad57ec897adc820948c
    recorded_at: 2026-10-16T23:44:12.210087151Z
    resolution_commit: a485ef023556583b5731cad57ec897adc820948c
    reason: atelier-sdb0 Done when lists every current prime reference; ADR Consequences name them
    migrated_from: atelier-6ec6
---

Consequences (ADR:60-63) and atelier-sdb0's Done when list only sessions-and-hooks.md and Agent Factory references/atelier.md as prime docs to change. Other current references would go stale when prime is deprecated: PRODUCT_INTENT.md:43, docs/product/work-model.md:67,76,152, docs/product/index.md:18, AGENTS.md:20 ('Start a session with target/debug/atelier prime'), Agent Factory procedures/install.md:34 (tells adopters to write 'atelier prime to start' into AGENTS.md; upstream per ADR 0020), the setup claude success message (crates/atelier-cli/src/commands/setup.rs:99 'Hooks run prime'), and root help (main.rs:20). Fix: extend sdb0's Done when (or the ADR consequence) to cover them.
