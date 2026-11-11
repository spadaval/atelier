---
schema: atelier.review
schema_version: 3
round: 3
commit: e70df1b74dfd94bb2ed25deb39a0e9cae461f074
reviewers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A03dd0ca7-d8ff-4ca0-a23e-d4690e64a664
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-11T15:35:00.176923564Z
---

## Summary

Complexity: high. The whole mission replaces the shared tool, records and
skill across five repositories; the remaining work spans first use, CI,
command guidance, eval fixtures and documentation.

Value: high. Real use and current instructions are needed to finish adopting
the installed tool.

Risk of merging it: medium. Incorrect guidance can stop work across these
repositories, but the failures are visible and the remaining changes can be
reverted.

The value justifies the work. I found no substantially smaller change that
would achieve the remaining outcomes.

I reviewed the plan at e70df1b: mission.md, design.md, notes.md and all four
member items. I checked the carried finding against the revised docs item,
submit's checklist gate and forge ordering, and CI's triggers. I also checked
the earlier findings' resolutions, the product and architecture
specifications, the cutover and adoption runbook, the current skill and its
recorded eval results, CI's job selector, the retained standards, and the
next-* fixtures' dependencies on old scenarios. I read the product-repository
first-use plan, worldcrafter-s9lp, in demiurge's checkout. A search of the
four product repositories' available AGENTS.md and CLAUDE.md files found no
instruction to run the removed Atelier commands searched for. atelier check
passed for 133 items, seven missions and config.toml, reporting only this
uncommitted review.

This is a plan review. I did not execute the first-use items, rerun the
cutover, rollback trials or eval matrix, run a build or test suite, or make
a forge request. The recorded cutover and eval evidence was inspected, not
independently reproduced. No additional finding remains to block the plan.

## Finding 1: Keep the docs item's CI result out of its submission checklist
Recommendation: fix now
Was: 2-ujg6.4
Now: fixed

.atelier/items/atelier-6w8d/item.md:32 now requires the local link and
reference checks, atelier check and cargo fmt before submission. It explicitly
leaves pull-request CI to merge after submit. This agrees with
crates/atelier-app/src/acts.rs:912, which checks Done when lines, and
crates/atelier-cli/src/commands.rs:204-215, which performs that check before
pushing the branch and opening its pull request. The checklist no longer
requires evidence that its own first submission has yet to create.
