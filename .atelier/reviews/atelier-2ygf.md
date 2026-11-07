---
schema: atelier.review
schema_version: 2
id: atelier-2ygf
title: finding on atelier-jnrh @8798b61e
target: atelier-jnrh
identity: independent reviewer subagent (claude-opus-5-5), atelier-jnrh
commit: 8798b61ec504698e37ba7763fd2463165e3940f2
created_at: 2026-11-07T07:31:46.325006057Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: f9634a225d4d6a556f4bc6ce3ad6b63d2c1e8ad3
    recorded_at: 2026-11-07T07:39:39.397146897Z
    resolution_commit: f9634a225d4d6a556f4bc6ce3ad6b63d2c1e8ad3
    reason: Fixed in f9634a22.
---

high: crates/atelier-cli/src/commands/review_request.rs:140 review request refuses any review of a parent branch while any child is still open, because it always covers the owner's whole scope (a child id resolves to the owner). astromech reviews children one at a time on the parent branch today: astromech-bp23 (todo, 7 open children) carries approval astromech-vimx @b42b7f92 for child 6siv, and ndzx was just started. With this binary, in a scratch clone of astromech on feature/astromech-bp23, `atelier review request astromech-ndzx` refuses with review_request_unfinished listing 8 items and their unticked lines. The next per-child brief cannot be produced until every child is implemented, and the only path offered is ticking lines that do not hold yet. The strategy puts review practice out of scope and needs the shared binary to keep other repositories working. Fix: scope the refusal to what is being asked about, for example the named item when a child is named, or only open items in an active status (unstarted children are not claimed implemented), together with the merge note in the next finding so nothing merge marks done goes unshown. Or, if per-child review is meant to end with this item, decide that in ADR 0028 decision 2 and have atelier-ml3r tell astromech before the binary is installed.
