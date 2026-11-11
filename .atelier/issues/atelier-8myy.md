---
created_at: "2026-11-11T01:39:45.504060362+00:00"
id: "atelier-8myy"
issue_type: "bug"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Fix what the validator found: a plan review of text it does not record, a broken branch that hides a drop, and nine smaller gaps"
updated_at: "2026-11-11T01:39:45.504060362+00:00"
---

## Goal

Close the gaps an independent validator found between the two specifications and the new binary, before the next mission switches real repositories to it. The validator built `master` at `0e11b9bb` and ran 216 commands against it. Two of what it found are defects that matter at once:

- A plan review shows the reviewer the checkout's text and records `HEAD`. With an uncommitted edit to a mission, `review <mission>` printed "Review the plan as commit 6c02006 holds it" above the uncommitted text, and `verdict approve` then recorded that commit. An approval can name a plan the reviewer never saw.
- A terminal status on the base does not win when the item's branch record is broken. An item was dropped on the base and its surviving branch then held a record with `status: broken`; `status`, `list`, `show`, `submit`, `merge` and `start` all exited 2 with a parse error, and neither orientation nor the drop's reason appeared. The branch record is read before the base's terminal record is looked at.

The rest are smaller: two places where the binary prints less than the specifications promise, and several where a document describes something the binary, by decision, does not do.

## Done when

- [ ] `review <mission>` shows the plan as the commit it records holds it: the mission's sections and the paths of its documents and items come from that commit. When the checkout holds uncommitted changes to the mission's directory or its items, the brief says so in one line that names them and says the review does not cover them. A test commits a plan, edits it without committing, and checks the brief holds the committed text and that line.
- [ ] A terminal status on the base wins whatever the item's branch holds, including a record that does not parse or validate. A test drops a started item on the base, commits a broken record on its branch, and checks that none of these exits 2: `status` from the base and from the branch, `show`, `submit`, `merge` and `start` each say the item was dropped and why; plain `list` succeeds and leaves the item out, as it does every finished item; and `list --done` shows it as dropped. For an item that is done on the base while a branch of its name holds a broken record, the same commands succeed and say it is done, with no reason to give.
- [ ] A mission lead's `status` says what each started item of its mission needs: the unmet conditions the item's holder would see, a question that waits on a decider, and since when an approved item has been approved. A test with a building item that has an unticked Done when line and no Report checks the lead sees both.
- [ ] `review <id>` run where the item's branch is not checked out names the checkout that has it, as `decide` does.
- [ ] `docs/strategies/redesign-product.md` says what the binary does, outside its Words, Lifecycle and Review sections: the `status` example under "What `status` prints" lists only unmet conditions and names the base's merge checks instead of when they last passed; the `show` row says an item's reviews are summarised by reviewer, verdict and finding, with each finding's text in its review file; the Merge section's rule for which CI run decides includes the case of a failed run with a newer one still running, as the architecture specification states it; and the sentence on an approved item that has not merged says it shows since when, from the verdict's commit.
- [ ] `docs/strategies/redesign-architecture.md` gains a short account, under Status, of which rules of the product specification's Review section the binary enforces and which it only shows and leaves to the skill. Enforced: an author never reviews or decides its own item; a reviewer of a finding, in any round that carried it, never decides it; `verdict approve` is refused while a fix-now finding is open; a decided finding cannot be reopened. Shown and not enforced: that a rebuttal goes to a fresh reviewer and gets one try, that escalation is required after three rounds without an approval, and that a mission's lead is the one who decides. It also says the review brief carries no merge check results, since nothing records them.

## Scope

The four changes to `next/` above and the two documents. The product specification's Words, Lifecycle and Review sections are not edited: whether the binary should enforce the Review section's three rules and the lead as decider, and whether the brief should carry merge check results, are the owner's to decide, and this item changes neither the documents' Review text nor the binary's behaviour on them. No flag, config option, record field or dependency. No file under `crates/` is changed.

## Context

The validator's report is in the mission's notes (`atelier issue show atelier-fcb7`), with the commands it ran for each finding. Plan reviews are in `next/crates/atelier-cli/src/review.rs`; where an item's record is chosen between the base and its branch is `next/crates/atelier-cli/src/repo.rs` and `atelier-app`'s `acts.rs`; the lead's view is `atelier-app`'s `view.rs` and `atelier-cli`'s `view_output.rs`. The commit reader that reads records at a commit without a checkout is `atelier-records`' `at_commit.rs`. Design: `docs/strategies/redesign-architecture.md`, "Where records are read and written" ("A terminal status on the base wins").
