---
schema: atelier.review
schema_version: 1
id: atelier-86qv
title: 'docs/product/human-cli-output.md:354-356 — the contract now says --full''s Transition Readiness and `issue transition <id>` both report an argument-only transition ''as `supersede: allowed - to superseded, needs --reason`''. Only --full prints that line. `issue transition <id>` prints `supersede: allowed` and then a separate `  Requirements: needs --reason` line (workflow.rs:406-426; pinned by issues.rs test_argument_only_transition_is_allowed_in_every_issue_view). Anyone who writes a test or a parser from the governing doc will expect a line the options view never prints. The misstatement comes from this change. Fix: state the options-view form (`supersede: allowed` with `Requirements: needs --reason`), or say only that both views report it as allowed with its flag.'
target: atelier-4775
identity: Independent reviewer (Claude subagent)
commit: aaf9777b531162d180c2105f0d404deb55432398
created_at: 2026-10-17T09:52:56.203845858Z
event:
  event: finding
  recommendation: fix_now
---

docs/product/human-cli-output.md:354-356 — the contract now says --full's Transition Readiness and `issue transition <id>` both report an argument-only transition 'as `supersede: allowed - to superseded, needs --reason`'. Only --full prints that line. `issue transition <id>` prints `supersede: allowed` and then a separate `  Requirements: needs --reason` line (workflow.rs:406-426; pinned by issues.rs test_argument_only_transition_is_allowed_in_every_issue_view). Anyone who writes a test or a parser from the governing doc will expect a line the options view never prints. The misstatement comes from this change. Fix: state the options-view form (`supersede: allowed` with `Requirements: needs --reason`), or say only that both views report it as allowed with its flag.
