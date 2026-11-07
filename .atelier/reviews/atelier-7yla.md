---
schema: atelier.review
schema_version: 2
id: atelier-7yla
title: 'low: crates/atelier-cli/src/commands/evidence.rs:946 print_stream_summary prints ''(not kept; passing output is summarized)'' for any stream with bytes>0, a digest and empty summary, including a failing capture whose stream was whitespace only (trim_line_ends empties it). Misleading wording on a failed receipt; say ''(not kept)'' or key on the receipt result.'
target: atelier-dg3h
identity: Independent reviewer (Claude Sonnet subagent)
commit: 745baadda963713742a83eb26140afafb667a867
created_at: 2026-10-23T14:30:14.794452085Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 0598e60fd907755f650a830ef27af7199cce377b
    recorded_at: 2026-10-23T14:35:35.463326204Z
    resolution_commit: 4cdaf39f9ed01ecdc89fabff4e53a320cc03a372
    reason: Wrong message on a failed receipt; one-line fix in the same view.
    migrated_from: atelier-46mq
---

low: crates/atelier-cli/src/commands/evidence.rs:946 print_stream_summary prints '(not kept; passing output is summarized)' for any stream with bytes>0, a digest and empty summary, including a failing capture whose stream was whitespace only (trim_line_ends empties it). Misleading wording on a failed receipt; say '(not kept)' or key on the receipt result.
