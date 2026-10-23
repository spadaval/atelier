---
schema: atelier.review
schema_version: 1
id: atelier-7yla
title: 'low: crates/atelier-cli/src/commands/evidence.rs:946 print_stream_summary prints ''(not kept; passing output is summarized)'' for any stream with bytes>0, a digest and empty summary, including a failing capture whose stream was whitespace only (trim_line_ends empties it). Misleading wording on a failed receipt; say ''(not kept)'' or key on the receipt result.'
target: atelier-dg3h
identity: Independent reviewer (Claude Sonnet subagent)
commit: 745baadda963713742a83eb26140afafb667a867
created_at: 2026-10-23T14:30:14.794452085Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/evidence.rs:946 print_stream_summary prints '(not kept; passing output is summarized)' for any stream with bytes>0, a digest and empty summary, including a failing capture whose stream was whitespace only (trim_line_ends empties it). Misleading wording on a failed receipt; say '(not kept)' or key on the receipt result.
