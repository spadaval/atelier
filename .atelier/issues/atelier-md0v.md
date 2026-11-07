---
created_at: "2026-11-07T05:01:00.802679334+00:00"
id: "atelier-md0v"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-md0v
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ilba"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Cut the records and gates that review does not need"
updated_at: "2026-11-07T08:34:54.029205352+00:00"
---

## Goal

Keep findings, decisions on deferred and dismissed findings, approvals and the merge gate; remove the bookkeeping and gates around them that the transcripts show cost turns and caught nothing.

## Done when

- [x] A finding whose reviewer recommended `fix_now` and that has no disposition is resolved by an independent approval that is both pinned to a later commit and recorded after the finding (`created_at`), so an approval that never saw the finding cannot resolve it. A finding recommended `defer` or `no_action` still needs a recorded decision, even if it was fixed instead. Old `fix_now` dispositions, with or without a resolution commit, still count, and `review disposition --decision fix_now` stays accepted but is never asked for. Tests: finding, fix commit, approval, merge with no disposition command; and an approval recorded before a finding on an earlier commit does not resolve it.
- [x] No gate reads evidence receipts: the built-in `evidence.no_newer_failure` gate and the `evidence.attached` validator are removed (a policy naming `evidence.attached` still parses and the name is ignored), and a fail receipt blocks neither merge nor land. Receipts remain as notes for what nothing else records.
- [x] `land` and the Stop hook report and exit successfully while work is in progress on a branch; they refuse only what would put unreviewed code on the base. A hook run from a directory outside any Atelier repository reports that and exits successfully (the Stop half of atelier-gg0m; its SessionStart wording stays with gg0m). Tests for both.
- [x] Removed: the rule that an approval's identity, or a `defer`/`no_action` decision's identity, differ from the work's Git authors (identities stay recorded); ADR 0025 decision 4 goes too, since atelier-ilba already accepts that a finding can be deleted before merge, so a non-author rule on decisions guards nothing. Also removed: the source rule on deferral follow-ups (`review_follow_up_source_mismatch`).
- [x] The merge base is the `branch_policy.base_branch` of the branch's committed policy, checked against the base ref's own policy as merge already does; `land_base_mismatch` and every read of `branch_base` go (merge.rs, workflow_policy.rs `action_policy`, review_request.rs, git.rs, status.rs, session.rs). New ownership records keep writing `branch_base`, set to the policy base and ignored when read, so older pinned binaries still parse them, until atelier-ml3r moves astromech's CI pin and drops it.
- [x] `start` uses no type prefix: on the base it creates a branch named with the bare item id; if the owner already has a recorded branch it switches to it as today; if the current branch is owned by another item that is not this item's parent it refuses and names that item; from a detached HEAD it refuses as today; otherwise it takes the current branch. `--base` is removed. Tests for each case.
- [x] A unique short commit id is accepted wherever a command takes a commit (`--commit`, `--resolution-commit`, `review trivial --base`, evidence commands), resolved on input and stored as the full id; an ambiguous or unknown prefix is refused. Test.
- [x] Merge, `land` on the base and `doctor` judge findings, decisions and approvals by the same rules: `findings.disposed`, `approval_accepted` (used by `land`'s base coverage) and doctor's historical paths (`original_independent`, `disposition_independent`, the `historical` branch of `evaluate_scope`) all apply the changes above.
- [x] The lifecycle success criterion's scenario test extends crates/atelier-cli/tests/lifecycle_cost.rs: in a fixture repository with the `atelier init` starter policy, one issue goes through start; notes, links and ticked Done when; a review with one `fix_now` and one `defer` finding; a fix commit; re-review and approval; a note and a link edited after approval; merge; then `atelier land` on the base and `atelier doctor`. It finishes with no refusal and no approval beyond the fix's re-review, the issue is done in the merge commit, and the test prints how many `atelier` commands the author and reviewer ran (replacing the fixed count constants).
- [x] ADR 0028 (decision 5, and the Supersession entries left open for this item: ADR 0016's review targets and mission integration branches, ADR 0025 decision 4), PRODUCT_INTENT.md and docs/product describe the rules as built.

## Scope

Gates in merge, land and doctor; the Stop hook path; `start`; review and evidence commands; workflow policy validators; tests; docs; ADR 0028. Must not change: merge refuses without a current independent approval of the code; a fix after review is re-reviewed (its approval is what resolves the finding). Other repositories keep working with the shared binary: old policies and records still parse.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 5), docs/strategies/simpler-lifecycle.md (Shape: Fewer records and gates), crates/atelier-app/src/review_gate.rs (`independent`, `findings.disposed`, `approval_accepted`), crates/atelier-cli/src/commands/start.rs and git.rs, crates/atelier-app/src/record_index.rs, crates/atelier-core/src/lib.rs (`WorkflowBranch`). atelier-jnrh (merge marks done) and atelier-ilba (write-time validation) land first. A branch with no issue is atelier-4nsl's light path; `start` taking the current branch must not stand in its way.

## Report

Built as planned. A `fix_now` finding is resolved by an approval of a later commit recorded after it, with no disposition; only `defer` and `no_action` record a decision, by any identity, and old `fix_now` dispositions still count. No gate reads receipts. On a work branch `land` and the Stop hook report what is open and exit successfully; on the base a refusal stands. Identities are recorded and not compared with the Git authors, and the deferral source rule is gone. The base is the policy's `base_branch` everywhere; `branch_base` is still written and never read. `start` takes no type prefix, and commands that take a commit accept a unique short ID. The lifecycle scenario (crates/atelier-cli/tests/lifecycle_cost.rs) takes one issue from start through merge, `land` on the base and `doctor` with no refusal and one approval: the author ran 12 `atelier` commands, the reviewer 3 (15 together), plus 3 for planning. Proof after the last code change: `cargo nextest run -j 4 --workspace` 995 passed, and 1001 with `--features test-hooks`, plus `cargo fmt -- --check` and `git diff --check`. This binary's `atelier check` and `atelier doctor` pass in astromech (on feature/astromech-v6r3), demiurge and life-on-rails; in cosplay-studio `check` passes and `doctor` fails with the same "review needs YAML front matter" the installed binary reports.

### Deviations

- `land --push` on a work branch with checks open pushes the commit `land` began at to the branch's own upstream, and says so in its verdict (`land: work in progress — … , pushed to origin/<branch>`). Pushing a work branch cannot put code on the base, and a push still refuses when a check moved HEAD.
- `land` on the base still refuses uncommitted records, and `doctor` still refuses every failure. The Done when covers work in progress on a branch; the hook already did not block for uncommitted records on the base.
- `start` also finds an owner's branch that only a local branch's own record names, as `land` and `merge` already did (`owning_branches`). Once `start` takes a branch of any name, "switches to it as today" needed that: without it, `start` on the base would create a second branch for the same owner.
- Evidence commands take no commit argument, so short IDs apply to `--commit`, `--resolution-commit` and `review trivial --base`. `--base` also still takes a revision such as `HEAD`, which tests and other repositories use.
- The close-transition check that refused a done transition on a newer fail receipt went too, along with the receipt gate machinery in `evidence_freshness` (`issue_gate`). Views still show freshness. The mark "⚠ blocks completion" became "⚠ no pass since", and the fail verdict now says the receipt gates nothing.
- Tests: the session and support `cli` helpers read land's `work in progress` verdict as not passed, so gate tests still read the report; the exit status itself is tested directly. Tests built on the removed rules were rewritten or removed: author identity, disposition tracing, evidence gates, `--base`, the prefix mismatch, `land_base_mismatch` and the deferral source rule.

### Decisions

- An approval of a later commit, recorded after a `fix_now` finding, resolves it even when the later commit changed only records. The rule is the reviewer's judgement, as the Done when states it.
- An old `fix_now` disposition that names a resolution commit keeps its rule (an approval at or after that commit). One without a resolution commit is judged by the new rule.
- From history (`land` claims, `doctor`), an approval still counts only as first committed (`original_intact`). Tracing a disposition to the commit that set it, and checking a migrated disposition against its event, existed only to judge the setter's identity, so they went with it.
- `evidence.attached` is dropped at parse with any parameters, even ones the old parser refused, since the name is ignored.
- The shape of a record's `workflow_branch.branch_base` is still validated: older pinned binaries validate it too.
- `land -q` prints `work in progress — <n> checks open (…)` for work in progress.
- On a work branch, the fix printed for an open blocker outside the owner's scope is `git switch <base> && atelier start <id>`, because `start` refuses another item's branch. A blocker filed only on the work branch cannot start from the base until that branch merges; its error says so, and the unlink fix remains.
- The docs commit also updates CONTEXT.md's definitions of approval, finding, disposition and land, which would otherwise contradict the rules. docs/product/cli-surface.md still says "Atelier never pushes", which was already wrong before this item (`land --push` pushes); it is left as it was.

### Surprises

- Once `start` takes the current branch, an owner's branch can have any name. That made the `owning_branches` lookup and the blocker fix above necessary.
- 122 existing tests failed at first, almost all because they read land's exit status or pinned removed messages.

### Open items

- Review, CI and merge are outstanding.
- atelier-ml3r: drop the `branch_base` write once astromech's CI pin moves. Update the Agent Factory skill, which still describes dispositions for every finding, the identity rule and type-prefixed branches, and tell the other repositories what changed.
- atelier-gg0m: the SessionStart wording outside an Atelier repository.
