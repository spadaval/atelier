# Backlog

The planner's pass over a repository's whole backlog, or one area of it (a
mission, a label or a set of items, with the items that could duplicate,
block or belong with it). Run it when Sumeet asks, when a mission closes, and
on an area before planning a mission there. You drop what you can prove is
stale and bring him the calls that are his.

## Reading

Read the open items and missions (`atelier list`, `atelier show <id>`) with
their missions' documents, and the deferred findings of done items (`atelier
list --deferred`). Check first what most often moves: items waiting on a
condition that may now hold, and items naming code, commands or decisions.
Cheaper agents may do bulk reading; check each claim yourself before acting.

## Dropping on Proof

Run `atelier drop <id> --reason "<proof>"` yourself once you can show,
against the item's Goal and each unticked Done when line, that nothing is
left to do:

- **The need is gone.** The feature or rule it serves was removed; name the
  commit.
- **It is already there.** A test on the installed binary shows the
  behaviour, or a named commit did it.
- **Another item covers it.** Name the item that holds all that remains.

A search that finds nothing the item names shows only that its wording is
old. Trace what replaced that code and test the outcome there. If the work is
still wanted, rewrite the item against today's code; if part is done, keep
the rest; if you cannot tell, it is a call for Sumeet.

You may also make record fixes evident from the records alone, such as a
missing label, a link both items' text already states, or a format fix
`atelier check` asks for that keeps the wording, and clear a blocker that
names a dropped item. A pass's drops and fixes go on one branch, `atelier
start --backlog <slug>`: commit them there, then `atelier submit` and
`atelier merge`, which merges records alone on `atelier check`.

## Never Without Sumeet

- An item he asked for or set aside himself, even with complete proof: its
  `ready.by` names him, or its body says he asked for it or gives his
  condition for picking it up.
- Real work we could do but may not want.
- A change of priority, or of order between missions.
- Forming a mission, or dropping one that is not wholly stale.

A started item, and an item in a mission another thread leads, are left to
their owners; tell them what you found.

## The Calls

Put the calls to Sumeet in the conversation as one numbered list of at most
seven, each a sentence of evidence he can check in a minute (a commit, a path
and line, an item ID) and your recommendation. Further calls wait for the
next pass. Apply each answer with `atelier set`, `atelier drop` or an edit to
the body; a new item body is planning, reviewed as
[roles/planner.md](../roles/planner.md) says. Only when he is not in the
conversation, file the list as one item labelled `needs-human`, and close it
with `atelier close` once every call is answered and applied.

## Report

End with a report in the order of the pass:

1. Open items and missions, before and after.
2. What was dropped, by kind, each with its proof.
3. What changed on kept items: rewrites, record fixes, cleared blockers.
4. The calls, and how many wait for the next pass.
5. A suggested order for what remains.
