---
schema: atelier.review
schema_version: 2
id: atelier-rrcf
title: finding on atelier-ilba @73613b72
target: atelier-ilba
identity: independent reviewer subagent (claude-opus-5-5), atelier-ilba
commit: 73613b7224fe1b07d00e1d7ee336d278298c8795
created_at: 2026-11-07T07:42:14.274343365Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-app/src/review_gate.rs:969 The approval guard reads only the record before the change (final_once_committed on edge.parent's blob), so a finding not yet on the base can still be rewritten into an approval: verified with target/debug/atelier at 73613b72, editing a finding's event block to 'event: approval' and committing passes check, land and merge --check. That is not a wider hole than today's: an implementer can already run 'atelier review approve --identity <anyone>', since identity is self-declared, and the honest correction path finding atelier-oyh0 named (re-pinning an approval) is now refused. Optional one-line hardening: also treat the change as final when the after blob is an approval or exemption, which would make the docs' 'approvals cannot be edited' hold for this case too.
