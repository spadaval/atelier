---
schema: atelier.review
schema_version: 2
id: atelier-jalb
title: 'crates/atelier-cli/src/commands/issue.rs:762-773 — `issue show --full` now adds one "Execute <name>" line per runnable transition to Next, and every one repeats verbatim a `Command:` line printed about fifteen lines above in Transition Readiness (render_transition_readiness prints Command for each option without a validator hint, which is exactly the set the footer filter keeps). On atelier-jpok in this repo the footer grows from 5 to 8 lines, all four new lines duplicating block/defer/start/supersede commands already on screen; issues with more transitions grow it further. ADR 0022 Decision 3 asks for dense output with no repeated tokens, and the Done when only asks that the hint name real transition names. Simpler constructions that meet it: drop the Execute lines, since Transition Readiness already names each runnable transition with its exact command, or keep a single line. If a single line is kept, choose it deliberately: the first option in policy order is `block`, which is rarely the next step.'
target: atelier-6xxb
identity: Independent reviewer (Claude subagent)
commit: b6ffbc65aaed76df1da64ae0f4046e7d307e1a4e
created_at: 2026-10-17T07:38:01.568083005Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: b501d94b057b54fbf20b21200aa37e23d6bb62f3
    recorded_at: 2026-10-17T07:45:59.460839326Z
    resolution_commit: a8e0b6d0d29ab7c236fc761517bbaf684348a950
    reason: Execute footer lines removed; Transition Readiness names each runnable command once.
    migrated_from: atelier-xgcm
---

crates/atelier-cli/src/commands/issue.rs:762-773 — `issue show --full` now adds one "Execute <name>" line per runnable transition to Next, and every one repeats verbatim a `Command:` line printed about fifteen lines above in Transition Readiness (render_transition_readiness prints Command for each option without a validator hint, which is exactly the set the footer filter keeps). On atelier-jpok in this repo the footer grows from 5 to 8 lines, all four new lines duplicating block/defer/start/supersede commands already on screen; issues with more transitions grow it further. ADR 0022 Decision 3 asks for dense output with no repeated tokens, and the Done when only asks that the hint name real transition names. Simpler constructions that meet it: drop the Execute lines, since Transition Readiness already names each runnable transition with its exact command, or keep a single line. If a single line is kept, choose it deliberately: the first option in policy order is `block`, which is rarely the next step.
