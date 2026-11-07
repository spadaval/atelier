# ADR 0027: Proof Has a Time

## Status

Accepted (2026-11-06, within the authority of mission atelier-n4mj's strategy,
docs/strategies/lighter-workflow.md, on Sumeet's direction for the model;
atelier-n0qh).

Superseded by [ADR 0028](0028-simpler-lifecycle.md#supersession) when
atelier-8bh4 lands: loose ends replace after-merge checks, so the After merge
marker, stand-ins, `carried to`, the awaiting-validation views and the
mission-close warning go. Until then this ADR applies as written.

## Context

Some proof can only come from real use: a device or place no agent can reach,
the deployed or merged system, the human's judgment, or time and real use.
Atelier had no honest place for it. Such a check was either forced in before
the merge or split off as a separate validation issue, sometimes a
`needs-human` child that held the owner's branch off the base
(cosplay-studio-ojg1 holds phone steps its parent closed blocked).

Sumeet's rules for the model: validate everything possible before merge; a
check goes after merge only with one of those reasons, and its nearest
stand-in still runs before merge; the split is fixed when the work is
planned; the reviewer sees each after-merge check and flags ones that could
run earlier; validation is recorded on the base as a planning change with no
reviewer identity, and a failure files a follow-up bug; merged work awaiting
validation is visible; and nothing new refuses.

The counts of how repositories use each status and type (atelier-rkxm, its
2026-11-06 result note) bear on where after-merge state lives. All four
repositories keep the starter policy's statuses unchanged. The retired
`validation` status left 197 records on branches and none at rest on a base;
`review` and `blocked` almost never sit on a base and are read by no code by
name; `in_progress` never rests on a base outside atelier. Statuses that
describe a branch's progress do not survive at rest, and atelier-bqbz will cut
the ones nobody uses. A new status would also be a policy change in every
repository on this host, which share one installed `atelier`.

## Decision

An after-merge check is a marked line of the item's Done when, and its state
is the line's own:

```text
- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.
```

- A line counts when, after an optional list bullet and checkbox, it starts
  with `After merge`, an optional `(<reason>)`, and a colon. Text after
  `Stand-in:` is the stand-in. Every other line is checked before merge, as
  before.
- `[ ]` (or no box) is open. Ticking the box, `[x]`, records the validation.
  `carried to <id>` on the line moves the check to another item, which then
  holds it; it counts only when `<id>` is an existing issue, so prose such as
  "carried to the server" leaves the check open. Every check that can be
  validated now is validated, not carried. Carrying is for a failed
  validation, which files a follow-up bug with `issue create
  --discovered-from <item>` and carries the check to it rather than
  reopening the merged item, and for a check that needs time and real use
  beyond the mission; the mission's close report names each carried check.
- The item closes on its work branch and merges as today. Recording the
  validation is an edit to a done record on the base, a planning change: it
  needs no reviewer identity, and `land` accepts it, since the status does not
  change and so it makes no done claim (`land_base_done_claim`).
- Shown, never enforced:
  - the review request brief lists each after-merge check with its reason
    and stand-in, saying `reason missing` or `stand-in missing`, marks one
    `ticked before merge` (validation belongs on the base), and asks the
    reviewer to record a finding for any that could run before merge or
    lacks a reason or a stand-in;
  - `status`, `work all`, and `work mission` and `work parent` list done work
    with an open after-merge check under Awaiting Validation, and the debrief
    says `Awaiting validation after merge: <check>` under the item;
  - closing a mission whose scope still has one warns and names each item,
    and closes anyway; a carried check does not count.
- Moving a check to after merge once the item has started is a Done when edit,
  shown to the reviewer by atelier-r9hb in `merge --check` and now in the
  brief. Ticking a box or adding `carried to <id>` (an existing issue) to an
  open check is not counted as an edit, so recording validation does not
  read as a changed criterion; any other change to the line, such as a
  rewritten check, stand-in or reason, still is.

Options rejected:

- **A status** such as `validating` after `done`. The counts show statuses
  that describe progress do not rest on a base; it would add a workflow
  state and a transition to every policy, would make a merged item not done,
  and would need the transition run on the base, where `land` refuses done
  claims made outside review.
- **A front-matter field** listing after-merge checks and their state. It
  would copy Done when text into a second place, and it changes the record
  format, so the shared installed binary would need old-format reads and an
  `atelier migrate` step.

## Consequences

- No record format, policy or migration changes. Every installed Atelier
  reads these records; an older one shows the line as plain Done when text.
- The marker is text, so a mistyped one is just a before-merge line. Nothing
  checks that a reason is one of the four or that a stand-in ran; the brief
  shows what is missing and the reviewer judges.
- No receipt is required per check. A device or deployed check leaves no
  other record, so validators record one as [what gets a
  receipt](../product/validation.md#what-gets-a-receipt) says.
- Awaiting Validation reads the checkout's records, so on a work branch a
  done owner with an open check shows there too, before it is merged.
- A `needs-human` check that runs after the merge may be a marked line of the
  owner's Done when, when it meets the reasons above and is decided at
  planning; a standalone validation item still works, and atelier-bqbz
  decides which types stay.
