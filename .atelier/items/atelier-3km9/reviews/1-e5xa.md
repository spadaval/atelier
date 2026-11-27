---
schema: atelier.review
schema_version: 3
round: 1
commit: 22dbda5c5269126f1aa69e3d2e540b71f57dd7a4
reviewers:
- thread:delegated-task:command%3Amcp%3Aead99f40-03ed-4356-863e-d863c7bf227a%3Adelegate-task%3Ac9854b11-16d1-4dfb-bc48-fb018214f834
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-27T19:43:31.641034714Z
---

## Summary

Complexity: medium. A small authorization change crosses action checks, repository facts and views, alongside a larger update to agent operating instructions.
Value: high. Leads can finish approved work without taking it over, and technical mission records can track implementation without reopening the human contract.
Risk of merging: medium. The code is reversible and covered by tests, but the role skills guide work in other repositories.

I read the complete diff against master at 22dbda5c5269126f1aa69e3d2e540b71f57dd7a4 and traced lead lookup, merge authorization, next-action selection, status rendering and plan-file classification. The code preserves the restrictions on submit and merge --trivial, and the tests cover the new merge permission and both living-document paths. The scope is proportionate to the value.

Validation: cargo fmt -- --check and git diff --check passed. cargo nextest run -p atelier-core -p atelier-cli passed all 414 tests, with 1 skipped, using CARGO_TARGET_DIR=/home/supadava/.cache/atelier-review-3km9-target and CARGO_BUILD_JOBS=2. The first build in the checkout failed with disk quota exceeded in /tmp; I removed only that run's target directory and rebuilt outside /tmp. I did not independently rerun clippy or inspect forge CI.

## Finding 1: Important: Create the mission branch before asking contributors to write its decisions
Recommendation: fix now

In skills/agent-factory/roles/planner.md:32-42, Explore tells the planner and Contributors to write .atelier/missions/<id>/decisions.md, but atelier start --mission is first introduced in Check, after Explore and Decide. That command allocates the ID and creates the mission record and branch (crates/atelier-cli/src/commands.rs:446-461). A planner following this sequence has neither the directory's ID nor the required planning branch when the first write is requested. The old instructions created the mission before any plan writing. Move the start command before Explore and make Check commit and submit the already-created mission branch.

## Finding 2: Critical: Document the lead's approved-item merge permission in the product contract
Recommendation: fix now

The final Done when line requires both docs/product.md and docs/design.md to document lead-merge and the living mission files. The product document's changes only describe the living files; its merge command description (docs/product.md:324), status description (393-397), and Merge section (622-627) never explain that a mission's lead can merge an approved item without holding it or using ATELIER_SKIP. The existing description of merging past review with a reason describes a different permission. Add the ordinary approved-item lead permission and its limits to the product contract, including the status offer, so readers can discover the behavior this item introduces.
