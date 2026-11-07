---
created_at: "2026-11-07T15:34:13.581353903+00:00"
id: "atelier-oh52"
evidence_type: "validation"
commit: "72d05b0303f8a1f547c9e994ad6438ee44902f0c"
result: "pass"
captured_at: "2026-11-07T15:34:13.581240+00:00"
agent_identity: "independent validator (claude-opus-5-5), atelier-2bho"
target:
  kind: "issue"
  id: "atelier-mmpv"
  role: "validates"
criteria:
- slug: "lifecycle"
  result: "pass"
criteria_mission: "atelier-mmpv"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mmpv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "lifecycle holds on master 72d05b03. `cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost --no-capture` (fresh CARGO_TARGET_DIR, TMPDIR under ~/.cache): 2 tests run, 2 passed, 0 skipped, one fixture from `git init` on main and one on master. Read the test: `atelier init` with its starter policy unedited and committed as init prints; then start; issue note; issue link; tick the Done when (plus the Report); review request; one fix_now and one defer finding; a fix commit; a defer disposition naming a filed follow-up; review request; approve; a note and a link after approval, committed; merge; then land and doctor. That is the criterion's order. Every atelier call goes through `ok()`, which fails the test on a non-zero exit or a land `work in progress` verdict, so nothing was refused. It asserts exactly one approval record, no disposition on the fix_now finding, `<base>^2` equal to the pre-merge tip, status in_progress at the tip and done at the merge commit. It prints the count: on both bases the author ran 12 atelier commands and the reviewer 3 (15 together), plus 3 for planning. Limits: the count is printed, not pinned; approvals are counted by the `event: approval` text in review records; the Done when tick is a direct file edit, as an author would make it."
updated_at: "2026-11-07T15:34:13.747813843+00:00"
---

lifecycle holds on master 72d05b03. `cargo nextest run -j 4 -p atelier-cli --test lifecycle_cost --no-capture` (fresh CARGO_TARGET_DIR, TMPDIR under ~/.cache): 2 tests run, 2 passed, 0 skipped, one fixture from `git init` on main and one on master. Read the test: `atelier init` with its starter policy unedited and committed as init prints; then start; issue note; issue link; tick the Done when (plus the Report); review request; one fix_now and one defer finding; a fix commit; a defer disposition naming a filed follow-up; review request; approve; a note and a link after approval, committed; merge; then land and doctor. That is the criterion's order. Every atelier call goes through `ok()`, which fails the test on a non-zero exit or a land `work in progress` verdict, so nothing was refused. It asserts exactly one approval record, no disposition on the fix_now finding, `<base>^2` equal to the pre-merge tip, status in_progress at the tip and done at the merge commit. It prints the count: on both bases the author ran 12 atelier commands and the reviewer 3 (15 together), plus 3 for planning. Limits: the count is printed, not pinned; approvals are counted by the `event: approval` text in review records; the Done when tick is a direct file edit, as an author would make it.
