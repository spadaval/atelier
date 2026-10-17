---
schema: atelier.review
schema_version: 1
id: atelier-v72d
title: 'docs/architecture/quality/agent-factory-atelier-validation.md:55 still says agents close with ''atelier issue transition <id> close --reason "..." in this repository'', which now fails here unless the report is already in the record body; crates/atelier-cli/src/commands/issue.rs:1242 tells agents to add close reasons ''with --reason on a transition'' and omits the record body and --reason-file routes. Agents following either get a refusal before they find the report guidance. Fix: name the report and --reason-file in both (the main.rs after_help example can stay since it still works with a body-written report).'
target: atelier-sfsq
identity: Independent reviewer (Claude subagent)
commit: 184469851584ee81ba2b163fb54071bd9ccabd7b
created_at: 2026-10-17T18:04:30.141125978Z
event:
  event: finding
  recommendation: fix_now
---

docs/architecture/quality/agent-factory-atelier-validation.md:55 still says agents close with 'atelier issue transition <id> close --reason "..." in this repository', which now fails here unless the report is already in the record body; crates/atelier-cli/src/commands/issue.rs:1242 tells agents to add close reasons 'with --reason on a transition' and omits the record body and --reason-file routes. Agents following either get a refusal before they find the report guidance. Fix: name the report and --reason-file in both (the main.rs after_help example can stay since it still works with a body-written report).
