# Groom

Use this subskill to keep a repository's backlog worth working from: sort and
sequence its open items into and across missions, and find the work we should
not do. You do the reading and the plain record fixes; the human makes the
planning calls from one digest. Judging merged work is `review`'s verdict, and
external queues such as dependency PRs are out of scope.

Run it when a mission closes or the human asks, on the whole backlog unless an
area is named, and before planning a mission, on its area (`plan` runs it).

## Inputs

A run covers one repository: its whole open backlog, or one area (a mission, a
label or a set of items). Read its open items and missions (`atelier work
missions --all`, `issue list`, `issue show <id> --full`) with their strategy
files, the deferred review findings `review defer` filed (label
`deferred-findings`), earlier digests and their answers (label `groom`), and
enough of the code and `git log` to tell whether each item still makes sense.
On an area run, also read the items around it that could duplicate, block or
belong with it.

## What to Propose

An entry earns its place by changing what gets worked on, or when; leave the
rest alone. Each rests on evidence the human can check in a minute: a commit, a
path and line, a search that finds nothing, another item's ID. A claim you
could not check is not an entry; report it with what would settle it.

- **Sequence.** Move items into, out of or across missions, by `plan`'s
  shaping tests; form a mission from a cluster of unassigned items that share
  one theme, stating it; split or merge items; reorder work, fix blockers and
  change priorities.
- **Not doing.** Flag items that are stale (done by other work, or naming code,
  commands or decisions that no longer exist), duplicated, or not worth their
  cost, sized on the [three scales](../references/sizing.md). Propose
  `supersede` when other work or another item replaces one, and `decline` when
  it is real work we will not do; the reason becomes its Report.
- **Shape.** Flag items that cannot be started as written: no Done when line
  that could fail, too big for one review (high complexity), or waiting on a
  human decision no item tracks. Propose the rewrite, the split or the decision
  item.

## Authority

Make only record fixes whose correctness is evident from the records, such as a
missing label or a duplicate link both items' text already states; commit them
on the base and list them in your report. Everything else goes in the digest:
declines, supersedes, priorities, blockers, mission moves, splits and rewrites.
A started item's record changes only on its work branch, so its owner applies
a fix or accepted entry for it there.

## The Digest

File one digest per run and repository (`atelier issue create "Groom <area>:
<n> proposals" --issue-type decision -l needs-human -l groom`), or add to the
open one while any of its entries is unanswered. A run with no entries files
none. Its Question asks which
proposals go ahead; its Options number the entries, grouped by kind:

```text
G1 atelier-abcd: decline, not worth its cost.
   Evidence: what the human can check, with paths, commits or IDs.
   Change: the command or rewrite that applies it.
   Size: the item's complexity, value and risk, each with a reason.
```

The human answers each entry in a note on the digest: yes, yes with a change,
or no with a reason.

## Applying Answers

Start each run with the answers on earlier digests. Recheck each accepted entry
against the records as they now are; if its evidence or size has changed, put
it back to the human instead. Apply the rest as answered, with `atelier issue
transition`, `issue update` and `issue link` or `unlink` (`advances` for
missions). A new item body, such as a split, a
rewrite or a mission, is planning: write it as `plan` does and have it reviewed
as [Planning on the Base](../references/atelier.md#planning-on-the-base) says.
Propose a rejected entry again only with new evidence, and name it. Once every
entry is answered and applied, write the outcome in the digest's Decision
section and `decide` it. Report the scope, the record fixes made, the digest
and its new entries, the entries applied, and the claims you could not check.
