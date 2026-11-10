---
created_at: "2026-11-10T19:18:01.696087670+00:00"
id: "atelier-kffb"
evidence_type: "test"
commit: "5b6f8fb21e9acd5c5d0d38254d8f37b4654c8860"
result: "pass"
captured_at: "2026-11-10T19:18:01.695971664+00:00"
target:
  kind: "issue"
  id: "atelier-xtvh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-xtvh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "On 5b6f8fb2 (the fixes of atelier-bywd and atelier-fvc4), in next/: cargo fmt --check and clippy --all-targets -D warnings clean; cargo nextest run 123 of 123 pass. Without the branch check, a_checkout_switched_to_another_item_during_the_wait_merges_neither fails (merge exits 0); with forge.rs as at f1ff5ad4, a_process_gh_leaves_holding_its_output_does_not_hold_up_the_bound fails after 60 s. The CI job's next steps, run as written in a scratch clone at 5b6f8fb2, all pass (123 passed)."
updated_at: "2026-11-10T19:18:01.913508283+00:00"
---

On 5b6f8fb2 (the fixes of atelier-bywd and atelier-fvc4), in next/: cargo fmt --check and clippy --all-targets -D warnings clean; cargo nextest run 123 of 123 pass. Without the branch check, a_checkout_switched_to_another_item_during_the_wait_merges_neither fails (merge exits 0); with forge.rs as at f1ff5ad4, a_process_gh_leaves_holding_its_output_does_not_hold_up_the_bound fails after 60 s. The CI job's next steps, run as written in a scratch clone at 5b6f8fb2, all pass (123 passed).
