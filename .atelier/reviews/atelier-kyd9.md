---
schema: atelier.review
schema_version: 2
id: atelier-kyd9
title: finding on atelier-jnrh @8798b61e
target: atelier-jnrh
identity: independent reviewer subagent (claude-opus-5-5), atelier-jnrh
commit: 8798b61ec504698e37ba7763fd2463165e3940f2
created_at: 2026-11-07T07:31:58.353637646Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/architecture/quality/agent-factory-atelier-validation.md:53 and crates/atelier-cli/src/main.rs:249 still describe the old mechanics. The doc's "Current Equivalent Workflow" tells agents to close work with `atelier issue transition <id> close` in this repository and to write the report in Close Reason, which Done when 4 rules out (no doc tells an agent to close a work item). The `merge` summary in `atelier --help` and `atelier merge --help` says "Check merge gates and fast-forward, or create only a merge commit under merge_commit", but merge now always creates a merge commit and marks work done, and the strategy names CLI help as a source a fresh agent learns the lifecycle from. Fix: reword both, for example "Check merge gates, then integrate with one merge commit that marks the owner's open work done", and the session.rs:958 doc comment that still mentions a fast-forwarded branch head.
