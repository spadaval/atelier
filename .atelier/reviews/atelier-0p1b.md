---
schema: atelier.review
schema_version: 1
id: atelier-0p1b
title: 'crates/atelier-cli/src/commands/bundle.rs:98 and :747-761: a bundle update revises the body of a done-category record, bypassing the issue_body_done guard that atelier issue update --body enforces (commands/issue.rs update_lifecycle). Reproduced in a scratch repo: superseded epic; ''issue update --body'' fails with issue_body_done, but a bundle entry {operation:update,id,body} applies, rewrites Outcome/Proof, and reports ''(updated)''. Failure: acceptance can change after a done claim and its proof, which the constraints forbid (no weakened validation or workflow bypass). Fix: in fill_update_from_record (or validate_bundle), reject a given body when the record''s status is in a done category, with the same issue_body_done message; add a test.'
target: atelier-4n7v
identity: Independent reviewer (Claude subagent)
commit: a9234343ba772ab7e86fa682752f752f73f4af7e
created_at: 2026-10-16T00:40:13.149059161Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/bundle.rs:98 and :747-761: a bundle update revises the body of a done-category record, bypassing the issue_body_done guard that atelier issue update --body enforces (commands/issue.rs update_lifecycle). Reproduced in a scratch repo: superseded epic; 'issue update --body' fails with issue_body_done, but a bundle entry {operation:update,id,body} applies, rewrites Outcome/Proof, and reports '(updated)'. Failure: acceptance can change after a done claim and its proof, which the constraints forbid (no weakened validation or workflow bypass). Fix: in fill_update_from_record (or validate_bundle), reject a given body when the record's status is in a done category, with the same issue_body_done message; add a test.
