---
schema: atelier.review
schema_version: 2
id: atelier-irzs
title: finding on atelier-psjw @ef737fde
target: atelier-psjw
identity: GPT-6.1 Sol independent planning reviewer
commit: ef737fdeeee66301b00a990de3a7caf81570b035
created_at: 2026-11-11T03:33:40.673392241Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-psjw.md:33-34: medium severity. Settle the shared refusal and bypass boundary for repeated library calls before implementation. ready currently imports commands::{gate,gated,mission_gate,refusal,Outcome} and output; decide does too. The moved git::commit_paths calls with_skips, which reads skip::PASSED, a process-global accumulator explicitly justified by one command being one process (next/crates/atelier-cli/src/skip.rs:8-19, git.rs:218-234). Passing ATELIER_SKIP as an argument removes the environment read but does not settle who owns that accumulator, its lifetime, or how the CLI commands that stay behind share the one refusal/skip policy. A long-lived Desk process can otherwise carry one request\u0027s skipped rules into a later request\u0027s commit or output. Nor can all of output.rs move unchanged: it imports CLI forge, merge and setup types. This is a shared library boundary, not an implementation detail to choose alone. State where the common gate/refusal formatting and bypass code live and how CLI adapters call it; specify that reason and passed rules belong to one act invocation, reach that act\u0027s commit and result, and never leak to another call or repository. Add library proof that two calls in one process, one skipped and one ordinary, keep their outcomes and trailers separate, while existing CLI skip behavior stays intact.
