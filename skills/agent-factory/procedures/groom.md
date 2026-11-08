# Groom

Use this subskill to keep a repository's backlog worth working from: sort and
sequence its open items into and across missions, and find the work we should
not do. You do the reading and the plain record fixes; the human makes the
planning calls from one digest. Judging merged work is `review`'s verdict, and
external queues such as dependency PRs are out of scope. Run it when a mission
closes or the human asks, on the whole backlog unless an area is named, and
before planning a mission, on its area (`plan` runs it).

## Inputs

A run covers one repository: its whole open backlog, or one area (a mission, a
label or a set of items) and the items around it that could duplicate, block or
belong with it. Read the open items and missions (`atelier work missions
--all`, `issue show <id>`) with their strategies, the findings `review defer`
filed (label `deferred-findings`), and earlier digests (label `groom`). Check
first what most often moves: parked items whose pick-up condition may now hold,
and items naming code, commands or decisions (search the code and `git log`).
On a large backlog, cheaper agents may do the bulk reading; check each claim
yourself before it becomes an entry.

## What to Propose

An entry earns its place by changing what gets worked on, or when. Each rests
on evidence the human can check in a minute: a commit, a path and line, a search
that finds nothing, another item's ID. Report a claim you could not check with
what would settle it, not as an entry.

- **Sequence.** Move items into, out of or across missions, by `plan`'s
  shaping tests; form a mission from a cluster that shares one theme; split or
  merge items; plan a parked item whose condition holds; reorder work, fix
  blockers and change priorities.
- **Not doing.** Flag items that are stale (done by other work, or naming code,
  commands or decisions that no longer exist), duplicated, or not worth their
  cost on the [three scales](../references/sizing.md). Propose `supersede`
  when other work replaces one, and `decline` when it is real work we will not
  do; the reason becomes its Report.
- **Shape.** Flag items that cannot be started as written: no Done when line
  that could fail, too big for one review, partly done, or waiting on a human
  decision no item tracks. Propose the rewrite, split or decision item.

## Authority

Make only record fixes whose correctness is evident from the records, such as a
missing label, a link both items' text already states, or a format fix `atelier
check` asks for that keeps the wording; commit them on the base and list them.
Everything else goes in the digest. A started item's record changes only on its
work branch, so its owner applies a fix or accepted entry for it there.

## The Digest

File one digest per run and repository (`atelier issue create "Groom <area>:
<n> proposals" --issue-type decision -l needs-human -l groom`), or add to the
open one while any entry is unanswered; a run with no entries files none. Its
Question asks which go ahead. Its Options number the entries by kind, with any
that has a deadline first, saying so:

```text
G1 atelier-abcd: decline, not worth its cost.
   Evidence: what the human can check, with paths, commits or IDs.
   Change: the command or rewrite that applies it.
   Size: complexity, value and risk, each with a reason.
```

An entry may cover several items, such as a split or a new mission; size what
it does to the work as a whole, and give a new body as an outline. The human
answers each entry in a note: yes, yes with a change, or no with a reason.

## Applying Answers

Start each run with earlier digests' answers. Recheck each accepted entry
against the records now; if its evidence or size has changed, put it back to
the human. Apply the rest with `atelier issue transition`, `issue update` and
`issue link` or `unlink` (`advances` for missions). A new item body is planning:
write it as `plan` does and have it reviewed as [Planning on the
Base](../references/atelier.md#planning-on-the-base) says. Propose a rejected
entry again only with new evidence, and name it. Once every entry is answered
and applied, write the outcome in the digest's Decision and `decide` it. Report
the scope, the record fixes, the digest's new entries, the entries applied, and
the claims you could not check.
