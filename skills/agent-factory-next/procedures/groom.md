# Groom

Use this subskill to keep a repository's backlog worth working from: sort and
sequence its open items into and across missions, and find the work we should
not do. You do the reading and the plain record fixes; the human makes the
planning calls from one digest. Judging merged work is the
[reviewer](../roles/reviewer.md)'s verdict, and
external queues such as dependency PRs are out of scope. Run it when a mission
closes or the human asks, on the whole backlog unless an area is named, and
before planning a mission, on its area.

## Inputs

A run covers one repository: its whole open backlog, or one area (a mission, a
label or a set of items) and the items around it that could duplicate, block or
belong with it. Read the open items and missions (`atelier list`, `atelier
show <id>`) with their missions' documents, the deferred findings of done items
(`atelier list --deferred`), and earlier digests (label `groom`, and `atelier
list --done --label groom`). Check
first what most often moves: parked items whose pick-up condition may now hold,
and items naming code, commands or decisions (search the code and `git log`).
On a large backlog, cheaper agents may do the bulk reading; check each claim
yourself before it becomes an entry.

## What to Propose

An entry earns its place by changing what gets worked on, or when. Each rests
on evidence the human can check in a minute: a commit, a path and line, a search
that finds nothing, another item's ID. Report a claim you could not check with
what would settle it, not as an entry.

- **Sequence.** Move items into, out of or across missions, by
  [roles/planner.md](../roles/planner.md)'s tests; form a mission from a
  cluster that shares one theme; split or
  merge items; plan a parked item whose condition holds; reorder work, fix
  blockers and change priorities.
- **Not doing.** Flag items that are stale (done by other work, or naming code,
  commands or decisions that no longer exist), duplicated, or not worth their
  cost on the [three scales](../roles/reviewer.md#size-and-verdict). Propose
  `atelier drop`, naming the work that replaces it, or why it is real work we
  will not do; the reason is recorded with it.
- **Shape.** Flag items that cannot be started as written: no Done when line
  that could fail, too big for one review, partly done, or waiting on a human
  decision no item tracks. Propose the rewrite, the split, or a `needs-human`
  item for the decision.

## Authority

Make only record fixes whose correctness is evident from the records, such as a
missing label, a link both items' text already states, or a format fix `atelier
check` asks for that keeps the wording; commit them on the base and list them.
Everything else goes in the digest. A started item's record changes only on its
work branch, so its owner applies a fix or accepted entry for it there.

## The Digest

File one digest per run and repository (`atelier new "Groom <area>: <n>
proposals"`, then `atelier set <id> --labels needs-human,groom`), or add to the
open one while any entry is unanswered; a run with no entries files none. Its
Goal asks which go ahead and numbers the entries by kind, with any
that has a deadline first, saying so:

```text
G1 atelier-abcd: drop, not worth its cost.
   Evidence: what the human can check, with paths, commits or IDs.
   Change: the command or rewrite that applies it.
   Size: complexity, value and risk, each with a reason.
```

An entry may cover several items, such as a split or a new mission; size what
it does to the work as a whole, and give a new body as an outline. The human
answers each entry in conversation or in the digest's body, and the agent
writes the answer under its entry: yes, yes with a change, or no with a reason.

## Applying Answers

Start each run with earlier digests' answers. Recheck each accepted entry
against the records now; if its evidence or size has changed, put it back to
the human. Apply the rest with `atelier set`, `atelier drop` or an edit to the
record's body. A new item body is planning: write it and have it reviewed as
[roles/planner.md](../roles/planner.md) says. Propose a rejected
entry again only with new evidence, and name it. Once every entry is answered
and applied, close the digest with `atelier close <id> "<outcome>"`. Report
the scope, the record fixes, the digest's new entries, the entries applied, and
the claims you could not check.
