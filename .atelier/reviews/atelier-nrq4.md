---
schema: atelier.review
schema_version: 2
id: atelier-nrq4
title: finding on atelier-jnrh @8798b61e
target: atelier-jnrh
identity: independent reviewer subagent (claude-opus-5-5), atelier-jnrh
commit: 8798b61ec504698e37ba7763fd2463165e3940f2
created_at: 2026-11-07T07:31:52.636430953Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/commands/merge.rs:850 merge marks done every open work item in scope without checking or showing whether its Done when is ticked and its Report written. Review request is the only check, and record edits after it never make the approval stale. Unticking a line after approval is no longer reported (record_history.rs:559 compares both sides without ticks, and the docs say so), and a child created after approval has no start, so neither shows in merge --check. Reproduced with the starter policy: approve, then untick one line and add a child with `- [ ] Never done.` and no Report. merge --check prints `merge: passed` with no note, and the merge commit puts both items on main as done with unticked lines. Any approval from a per-child review (previous finding) leads to the same result. The strategy's risk section relies on merge --check to show a Done when that changed after approval. Fix: in merge and merge --check, print a Note for each item merge marks done that has an unticked box (other than an After merge line) or an empty Report, reusing refuse_unfinished's gap listing. Make it a note, not a refusal, like the Done when edit notes.
