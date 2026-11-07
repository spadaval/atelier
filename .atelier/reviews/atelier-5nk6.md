---
schema: atelier.review
schema_version: 2
id: atelier-5nk6
title: 'low: crates/atelier-cli/src/gate_report.rs close_command (fallback) For an open blocker or open scoped item in todo status no terminal transition exists, so the fix printed is the bare ''atelier issue transition <id>'', which only lists transitions and changes nothing (exit 0). Repro: owner blocked_by a todo issue; ''Next: atelier issue transition atelier-ojiz''. It is not a fix; suggest naming the first runnable transition toward done (start, then close --reason) or ''atelier start <id>''.'
target: atelier-h1nc
identity: Independent reviewer (Claude Sonnet subagent)
commit: 342d93ff699a891b98de38b3ab5c4b932a1d303b
created_at: 2026-10-23T14:52:35.445136373Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: d4a52ac942571049b35accfa2c5e9b1e928682dd
    recorded_at: 2026-10-23T14:58:29.906291261Z
    resolution_commit: 06272b9d4b6401c8d7fe6c7561257da0b2c9446d
    reason: S1 needs a runnable fix that moves the check; a transition listing changes nothing.
    migrated_from: atelier-ofsb
---

low: crates/atelier-cli/src/gate_report.rs close_command (fallback) For an open blocker or open scoped item in todo status no terminal transition exists, so the fix printed is the bare 'atelier issue transition <id>', which only lists transitions and changes nothing (exit 0). Repro: owner blocked_by a todo issue; 'Next: atelier issue transition atelier-ojiz'. It is not a fix; suggest naming the first runnable transition toward done (start, then close --reason) or 'atelier start <id>'.
