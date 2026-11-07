# ADR 0025: Findings Carry Their Disposition

## Status

Accepted (2026-11-06, within the Shape of mission atelier-n4mj, which
Sumeet approved: "One record per finding"; atelier-hwxm).

Partially supersedes [ADR 0019](0019-verified-repo-native-tracker.md)
decision 6: findings and their dispositions are no longer two append-only
events. Every other review event stays append-only, and the rest of ADR
0019 stands.

## Context

ADR 0019 made reviews append-only Markdown events, including a finding's
disposition: `review disposition` wrote a new file naming the finding, and
changing a decision meant writing a successor that named the one it
replaced. On 2026-11-06 atelier held 577 findings and 580 disposition
events; astromech 1054 and 1077. A defer or no_action proposed by the author
and then accepted by the reviewer took three files. The disposition events
carried no fact the finding could not: what was decided, by whom, at which
commit, and why.

The append-only rule protected one property the gates rely on: who set a
defer or no_action is judged against the work's authors at the commit that
introduced the event (`original_independent`), so a self-declared identity
cannot be backdated past work its owner authored later.

## Decision

1. **A finding carries its disposition.** The finding's record holds a
   `disposition` field: the decision (`fix_now` with a resolution commit,
   `defer` with a follow-up issue, or `no_action`), the identity that set
   it, the commit the branch was at, the time and the reason.
   `review disposition` sets it, replacing any earlier one. Review records
   move to schema version 2; version 1 records without separate
   dispositions still load unchanged.
2. **Only the disposition changes.** Every other review event, and every
   other field of a finding, stays append-only. The append-only check and
   approval freshness share one rule: a change to a review record is
   allowed when it adds an event, changes only a finding's disposition, or
   removes a retired disposition event.
3. **Recording a disposition keeps an approval current**, as adding a
   disposition event did. Editing anything else in an existing review record
   still makes it stale, and the append-only check refuses it.
4. **Who set it is traced per field.** A defer or no_action needs an
   identity independent of the work's authors, judged at the commit that set
   the field: the oldest commit since which the finding's file has held this
   disposition. The commit and time the disposition names are not trusted for
   this, so setting it again, or restoring earlier bytes by hand, is judged
   at the later commit.
5. **A changed disposition is audited through Git.** The record keeps only
   the current disposition. Earlier ones, such as a `fix_now` later changed to
   `defer`, or an author's proposal the reviewer replaced, stay in the Git
   history of the finding's file (`git log -p -- .atelier/reviews/<id>.md`,
   which `review show` names). The gates read only the current one, as they
   read only the latest event before. A second record per change was
   considered and rejected: it is the record this decision removes, and Git
   already keeps every version, each pinned to a commit and author.
6. **Migration.** `atelier migrate dispositions --apply` folds each finding's
   latest disposition event into the finding, with `migrated_from` naming the
   event, and removes every disposition event, superseded ones included.
   History keeps them. A migrated disposition is traced to the commit that
   added its event and must match that event's fields. A separate
   disposition event in the checkout fails to load with a pointer to the
   command; history that holds them still reads.

## Consequences

- A finding is one review file for its whole life; the lifecycle scenario
  (crates/atelier-cli/tests/lifecycle_cost.rs) counts one file per finding
  where it counted two or three.
- `review disposition` prints and, under `--quiet`, returns the finding's ID;
  `review list` has no disposition rows, and each finding's line shows its
  disposition.
- A fix still needs a fresh independent approval after its resolution, as
  before. That re-review may cover only the fix's diff, and the reviewer who
  raised the finding may do it.
- Repositories with disposition events run the migration once, on the base
  and on each open work branch, and commit the result. The migration commit
  keeps approvals current.
- Agent Factory's text that described two events (references/atelier.md,
  references/finding-disposition.md) changes with this, per ADR 0023.
