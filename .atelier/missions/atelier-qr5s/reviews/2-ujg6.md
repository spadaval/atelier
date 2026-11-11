---
schema: atelier.review
schema_version: 3
round: 2
commit: e3014370d4c790969b80ef042250ddb6a988bb9b
reviewers:
- thread:delegated-task:command%3Amcp%3A2074a88e-920b-49f9-bd17-caf5903cc60f%3Adelegate-task%3A023a04f3-d206-400d-8f5e-40b2da4c8c67
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T15:30:45.725320561Z
---

## Summary

Complexity: high. The mission replaces the shared binary, record layout and
skill across five repositories; the remaining plan spans first use, CI,
command guidance, eval fixtures and the product and architecture docs.

Value: high. The installed tool needs proof of its real workflow and current
documentation before the cutover mission is complete.

Risk of merging it: medium. Incorrect instructions can stop work in several
repositories, but the failures are visible and the changes can be reverted.

The value justifies the work. I found no substantially smaller change that
would achieve the remaining outcomes.

I reviewed the plan at e301437: mission.md, design.md, notes.md and all four
member items, including the changes since the previous review. I checked the
three carried findings against the revised items, submit's actual gate and
forge ordering, the product and architecture specifications, and the cutover
runbook. I also inspected CI and its job selector, the current skill and eval
fixture dependencies, the retained standards and documentation paths, and the
worldcrafter-s9lp plan in demiurge's checkout. atelier check passed for 133
items, seven missions and config.toml; it reported only this uncommitted
review. Cargo metadata without dependency resolution confirmed the workspace
manifests without building anything.

This was a plan review. I did not rerun the cutover, rollback trials, eval
matrix or first-use items, run a build or test suite, or request anything
from the forge. The remaining CI-ordering finding is established from the
submission code and configured workflow triggers, not a live GitHub run.

## Finding 1: Move the first-use item's post-submit and post-merge proof out of its submission checklist
Recommendation: fix now
Was: 1-vl9u.1
Now: fixed

.atelier/items/atelier-2mlh/item.md:26 now requires a local run of the
workflow commands. Line 28 explicitly limits the submission checklist to
what can be known before submit, then assigns the later push, pull request,
CI, review and merge transcript to the lead's mission Result. It no longer
requires ticking evidence of the item's own future submission or merge.

## Finding 2: Give the first-use item the limited scope exception its blocking-fix requirement needs
Recommendation: fix now
Was: 1-vl9u.2
Now: fixed

.atelier/items/atelier-2mlh/item.md:29 takes the alternative offered by the
finding: defects outside Scope are reported, blocking ones are escalated to
the lead, and none is fixed inside this item. Its restricted Scope at line
34 is now consistent with that requirement.

## Finding 3: Allow the docs move to update the retained adoption runbook's reference
Recommendation: fix now
Was: 1-vl9u.3
Now: fixed

.atelier/items/atelier-6w8d/item.md:36 permits the documentation-only update
to scripts/cutover/README.md and explicitly requires its protocol reference
to point to the design record's adoption section. Line 32 includes that
runbook in the relative-link checks. No converter or adoption behaviour has
to change to repair the reference.

## Finding 4: Keep the docs item's CI result out of its submission checklist
Recommendation: fix now

.atelier/items/atelier-6w8d/item.md:32 now requires that CI pass as part of a
Done when line. This introduces the same submission-order problem that the
revision fixes for atelier-2mlh: crates/atelier-app/src/acts.rs:912 checks
all Done when lines before submit can proceed, and
crates/atelier-cli/src/commands.rs:204-215 performs that check before sharing
the branch and opening its pull request. .github/workflows/ci.yml runs on
pull requests and pushes to master, and AGENTS.md forbids pushing the branch
or opening its pull request by hand around submit.

The docs item's first submission therefore cannot truthfully tick this
line based on CI for its change. A green run on unchanged master would not
validate its possible Rust comment edits or fixture deletions. Keep its
local checks and any required local execution of workflow commands in Done
when; leave successful pull-request CI to the existing merge gate, or assign
its later proof outside the submission checklist as atelier-2mlh now does.
