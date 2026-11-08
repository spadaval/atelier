---
created_at: "2026-11-08T05:35:31.731292457+00:00"
id: "atelier-edaj"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: atelier-edaj
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Have atelier check catch plan records with no Done when lines or broken links"
updated_at: "2026-11-08T06:12:17.587947150+00:00"
---

## Goal

Plan review spends reviewer time on mechanical problems a script can find. Have `atelier check` report them so reviewers can spend their time on vagueness and sequencing.

## Done when

- [x] `atelier check` reports an open work item with no Done when lines, and a Done when line with no text.
- [x] `atelier check` reports a repository-relative Markdown link in an issue record or a strategy file that a mission links when the target file does not exist (anchors need not be checked).
- [x] Each report names the record, the line and what to do instead.
- [x] The new problems print as warnings and do not change `atelier check`'s exit code, so on the current base of each local Atelier repository (atelier, astromech, life-on-rails, cosplay-studio, demiurge, worldcrafter) it exits as it does with the installed binary. The Report lists what the new checks find in each repository today.
- [x] Tests cover each new report and the existing-records case.

## Scope

Atelier's code, `docs/product` and this item's records. Fixing the problems the check finds in other repositories is out of scope; list them.

## Context

The 2026-11-08 review audit found that plan review mostly caught criteria that could not pass, false facts, ordering and ownership problems, which need judgment; broken links and missing criteria are the mechanical share. Lauren Tan's pstack checks plan structure with a script (check-plan.mjs) for the same reason.

## Notes

- 2026-11-08 [decision] A Done when counts as having lines only when it has a `- [ ]` box line; prose or plain bullets get the warning.
  Reason: review request checks only boxed lines, so an item without them can be sent to review with nothing proved; the existing required-section rule already refuses an empty Done when.
  Rejected: Count plain bullets as lines: they pass review request unticked, the gap the check exists to show.
- 2026-11-08 [decision] A link resolves if its target exists beside the file or from the repository root; a leading / means the root.
  Reason: Records use both forms today: atelier writes ../../docs/... from .atelier/issues, astromech writes docs/... from the root. Either reader-friendly form should pass.
  Rejected: File-relative only, as Markdown renders: would warn on all 18 working astromech record links.
- 2026-11-08 [decision] The Done when warning applies to open work items only; link warnings apply to every issue record, open or closed, and to every strategy a mission links.
  Reason: The Done when line says open work item and an issue record; a closed item's finish line no longer matters, but its links are still followed by readers.
- 2026-11-08 [decision] Warnings are computed only for atelier check, not for the write guard (record_invalid), and print on stderr with -q.
  Reason: The write guard compares findings; adding warnings there would refuse writes and change exit codes, which the plan forbids. Quiet stdout stays the one-token contract.

## Report

`atelier check` now warns, at `<file>:<line>` with a `Next:` fix, about an open work item whose Done when has no `- [ ]` line, a `- [ ]` line with no text, and a Markdown link in an issue record or a mission's strategy whose repository-relative target is missing. Warnings print whether the check passes or fails (stderr with `-q`) and never change the exit code. Proof, all run after the last code change: `cargo nextest run -p atelier-cli plan_checks` (`check_warns_about_plan_problems_without_changing_its_exit_code`) covers each warning with its line, the existing-records case (sound records print exactly `Check passed.`), scoped and quiet output, closed items, and a failing check that still prints every warning with the same failure; it failed when the warnings were removed from the failing path. `cargo nextest run -p atelier-app plan_checks` covers the parsing cases (front matter, code spans and fences, URLs, anchors, both link bases). `cargo nextest run -p atelier-cli -p atelier-app`: 919 passed. `cargo fmt -- --check` and `git diff --check` are clean.

On scratch clones (`/var/tmp/atelier-edaj-scratch`) checked out at each repository's base, the installed binary (0.3.1+e03d6b71) and the new one both exit 0, and their output is identical apart from the added `Warning:` lines. What the checks find today:

| Repository (base) | No `- [ ]` lines | Empty line | Broken links |
| --- | --- | --- | --- |
| atelier (master 1c3b9145) | 65 (64 backlog, 1 todo) | 0 | 0 |
| astromech (main 015bcc85) | 188 (129 backlog, 59 todo) | 0 | 0 |
| cosplay-studio (main 1034f62) | 36 (7 backlog, 29 todo) | 0 | 0 |
| demiurge (master c8a096a) | 17 (backlog) | 0 | 0 |
| life-on-rails (main 6a5db78) | 7 (2 backlog, 5 todo) | 0 | 0 |
| worldcrafter (master c8a096a) | 17 (backlog) | 0 | 0 |

The Done when warnings are items written before the checklist form, with prose or plain bullets, plus items created without a body ("Done when was not specified."). Every local link in these repositories resolves, so none is reported. Fixing them is out of scope.

### Decisions

See the four decision notes: what counts as a Done when line, how a link resolves (beside the file or from the root), which records each check covers, and why warnings stay out of the write guard.

### Surprises

Three existing tests asserted exact `Check passed.` output on fixtures created without a Done when (`loose_ends::migrate_after_merge_turns_open_lines_into_loose_ends_once`, `session::migrating_dispositions_folds_events_into_findings_and_passes_doctor`, `session::a_migrated_author_no_action_is_accepted`). The new warning fired correctly on them, so the fixtures now get a `- [ ]` line and keep asserting what they were written for.

### Open items

- Reference-style links (`[text][ref]` with `[ref]: path`) and percent-encoding other than `%20` are not checked.
- The 330 Done when warnings across the six repositories are unfixed, by scope. A follow-up could convert pre-checklist items in bulk.
- Linux only locally; the full suite with `test-hooks` and macOS runs in CI.
