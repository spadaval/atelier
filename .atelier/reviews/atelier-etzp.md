---
schema: atelier.review
schema_version: 2
id: atelier-etzp
title: approval on atelier-md0v @18e7c2bc
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: 18e7c2bccc9da2fae8c697c047676977243106c3
created_at: 2026-11-07T10:21:18.965557553Z
event:
  event: approval
---

Re-review of the master merge 18e7c2bc (parents 4dc2debd, 371f67a0) by its remerge diff. evidence_freshness.rs: proof_matches, used only by the removed receipt gate, stays gone, and claim_endpoint keeps atelier-8bh4's unused _index and its comment; the test binding removed was left unused by 8bh4's evidence_is_fresh signature. start.rs: 8bh4's refuse_done_on_base now reads the checkout's index, since md0v removed the --base snapshot it used, and runs before branch selection; md0v's branch rules follow unchanged. The auto-merged files hold no read of branch_base or evidence gates, and 8bh4's merge.rs refusal sits beside md0v's policy base. Checked with this binary in a scratch repo: start of a loose_end refuses with start_done_on_base; on the base start makes the bare-id branch and ignores --base with its note; another item's branch refuses with start_branch_owned; an unowned handmade branch is taken; a fail receipt leaves merge, land on the base and doctor passing. The needs-human advice is left to 8bh4's tests, which pass. Proof at 18e7c2bc: cargo build with no warnings; cargo nextest run -j 4, 997 passed; with --features test-hooks, 1003 passed; cargo fmt --check, cargo metadata --locked and git diff --check clean. CI on PR 40 was still pending when recorded.
