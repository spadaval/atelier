---
schema: atelier.review
schema_version: 1
id: atelier-5nk6
title: 'low: crates/atelier-cli/src/gate_report.rs close_command (fallback) For an open blocker or open scoped item in todo status no terminal transition exists, so the fix printed is the bare ''atelier issue transition <id>'', which only lists transitions and changes nothing (exit 0). Repro: owner blocked_by a todo issue; ''Next: atelier issue transition atelier-ojiz''. It is not a fix; suggest naming the first runnable transition toward done (start, then close --reason) or ''atelier start <id>''.'
target: atelier-h1nc
identity: Independent reviewer (Claude Sonnet subagent)
commit: 342d93ff699a891b98de38b3ab5c4b932a1d303b
created_at: 2026-10-23T14:52:35.445136373Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/gate_report.rs close_command (fallback) For an open blocker or open scoped item in todo status no terminal transition exists, so the fix printed is the bare 'atelier issue transition <id>', which only lists transitions and changes nothing (exit 0). Repro: owner blocked_by a todo issue; 'Next: atelier issue transition atelier-ojiz'. It is not a fix; suggest naming the first runnable transition toward done (start, then close --reason) or 'atelier start <id>'.
