---
schema: atelier.review
schema_version: 1
id: atelier-ooay
title: 'medium: crates/atelier-cli/src/commands/session.rs:645 (Stop hook path, land_text/read_hook_payload branch) The Stop-hook JSON reason built from GatesRefused::fmt::Display carries only the failure lines: no verdict line (land: refused — N checks (...)) and no passes, so the ''ends with a verdict'' requirement does not hold for the hook surface. The wrapper also appends ''. Fix the named check or hand off...'' directly after the last line, so the last fix command reads ''Next: atelier review request atelier-gy85. Fix the named check'', gluing a period and prose onto a command to copy. Repro: echo ''{"hook_event_name":"Stop","cwd":"<repo>"}'' | atelier land --hook claude in a repo with an unapproved done owner. Fix: have the hook reason end with the verdict (crate::gate_report::verdict("land", &report.failed)) on its own line and put the boilerplate before the gate lines or on its own line.'
target: atelier-h1nc
identity: Independent reviewer (Claude Sonnet subagent)
commit: 342d93ff699a891b98de38b3ab5c4b932a1d303b
created_at: 2026-10-23T14:52:34.956328944Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/session.rs:645 (Stop hook path, land_text/read_hook_payload branch) The Stop-hook JSON reason built from GatesRefused::fmt::Display carries only the failure lines: no verdict line (land: refused — N checks (...)) and no passes, so the 'ends with a verdict' requirement does not hold for the hook surface. The wrapper also appends '. Fix the named check or hand off...' directly after the last line, so the last fix command reads 'Next: atelier review request atelier-gy85. Fix the named check', gluing a period and prose onto a command to copy. Repro: echo '{"hook_event_name":"Stop","cwd":"<repo>"}' | atelier land --hook claude in a repo with an unapproved done owner. Fix: have the hook reason end with the verdict (crate::gate_report::verdict("land", &report.failed)) on its own line and put the boilerplate before the gate lines or on its own line.
