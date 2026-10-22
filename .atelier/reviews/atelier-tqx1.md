---
schema: atelier.review
schema_version: 1
id: atelier-tqx1
title: 'medium: crates/atelier-cli/src/commands/session.rs:160-162 and 123-125: the land_state_changed guard compares HEAD with a head read after land_text returns, so it only covers the microseconds between two reads. A commit made in the checkout while land''s checks run (another agent session sharing the checkout, or a hook) is pushed without having been checked, contradicting the ecah decision note ''pushes exactly the commit land checked''. Fix: have land_text return the head it pinned and its captured snapshot head, and push that commit (refspec <checked head>:refs/heads/<branch>) after confirming HEAD still equals it.'
target: atelier-yim7
identity: Independent reviewer (Claude subagent)
commit: 6b8b1ccb31042d1a3a7b167903f6a43b4a9acdf7
created_at: 2026-10-22T21:06:42.301586146Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/session.rs:160-162 and 123-125: the land_state_changed guard compares HEAD with a head read after land_text returns, so it only covers the microseconds between two reads. A commit made in the checkout while land's checks run (another agent session sharing the checkout, or a hook) is pushed without having been checked, contradicting the ecah decision note 'pushes exactly the commit land checked'. Fix: have land_text return the head it pinned and its captured snapshot head, and push that commit (refspec <checked head>:refs/heads/<branch>) after confirming HEAD still equals it.
