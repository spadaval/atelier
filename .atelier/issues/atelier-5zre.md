---
created_at: "2026-11-10T00:56:09.647724857+00:00"
id: "atelier-5zre"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Sort the 147 open items and five open missions for the new tool"
updated_at: "2026-11-11T02:46:33.362163646+00:00"
---

## Goal

Decide, for every open record in this repository, what happens to it at the cutover: it converts as it is, it is rewritten for the new tool, or it is dropped as obsolete. Apply the decisions that are safe on this item's branch, and leave the rest as proposals for Sumeet. The cutover converts every open record, so anything obsolete that is still open then becomes a draft or ready item in the new tool that nobody will ever start.

Only this repository is sorted. The other four repositories' records are converted as they are, and nothing in them is read or changed by this item.

## Done when

- [ ] The Report has one line per open record (every item and mission whose status is not `done`, `declined` or `superseded` when the branch starts), giving its ID, its fate (`as is`, `rewrite`, `drop`, or `left for Sumeet`) and a reason of one sentence. It gives the count per fate, and the counts add up to the number of open records, which it states with the commit they were counted at.
- [ ] A record is dropped only for one of these reasons, and its line names which: (a) it changes, or exists to work around, something the product specification's "What is removed" table removes, or the tracker's site, which goes with the old workspace (`docs/strategies/redesign-3-cutover.md`, step E); (b) the new binary already does what it asks, shown by a line of a specification or by a command run on a binary built from `next/`; (c) it duplicates another open record, which is named. A record whose need survives in another form is not dropped: it is rewritten, or left as it is.
- [ ] Each drop is applied with `atelier issue transition <id> supersede --reason "..."` when a specification or another record replaces it, and `decline --reason "..."` otherwise, with the reason from its line. A dropped site item whose need the Desk should meet names mission atelier-z4cr in its reason.
- [ ] A record is rewritten when its need survives and its Goal or Done when names a removed command, flag or record path. The rewrite changes only those names and the sentences that depend on them, to the commands the specifications give, and keeps the need as stated. The Report lists each rewritten record with what changed.
- [ ] Missions are not dropped or rewritten by this item. Each open mission has a proposal in the Report for Sumeet: keep, drop, or fold into another, with the count of its open items by fate and whether its ready items could start under the new tool (an item of a draft mission cannot).
- [ ] Nothing in the protected set (Scope) is changed. Each protected record still has its line, with fate `left for Sumeet` and the fate this sort would propose for it.
- [ ] The Report gives three counts for this repository, as facts for the cutover, and changes nothing for them: ready items that belong to a draft mission; ready items whose Done when has no checkbox line, which the converter will write as one unticked line (atelier-yudj); and deferred findings on done records that have no follow-up item, which the conversion will not carry over.
- [ ] After the changes, `atelier check` passes on the branch, and no open record is blocked by, or is a child of, a dropped one unless the Report says why that is right.
- [ ] The sorted records still convert: in a scratch clone of this branch, `scripts/cutover/convert_records.py` converts, `--verify` passes and the new binary's `check` passes. The Report gives the commands and the item and mission counts the converter prints.

## Scope

Records under `.atelier/issues/` in this repository, and this item's record. No code, no doc, no strategy file, no skill text, and nothing under `.atelier/reviews/` or `.atelier/evidence/` except what `atelier issue transition` itself writes.

The protected set, which this item reads and does not change, because other threads are planning live work in it or it is Sumeet's:

- any record created or changed on or after 2026-11-09 (by its `created_at` or `updated_at`, or by `git log --since=2026-11-09` on its file);
- any record labelled `needs-human`;
- mission atelier-z4cr (the Desk) and every item that advances it, and mission atelier-295u and every item that advances it;
- atelier-8ep4, atelier-y0qe, atelier-t6o6, atelier-pwhn, atelier-c1z3 and atelier-7w9s;
- mission atelier-qr5s and every item that advances it, which are this mission's own work.

If `master` changes a record this branch also changed, master's version wins when the branch is brought up to date (`atelier merge --sync`), and the record joins the protected set.

The four product repositories under `/home/supadava/projects/` are not read for this item. The scratch clone and anything built for the conversion check go under `$HOME/.cache/atelier-qr5s/5zre/`, never in `/tmp`.

## Context

Mission atelier-qr5s, step C of `docs/strategies/redesign-3-cutover.md`. The new tool's commands and what it removes are in `docs/strategies/redesign-product.md` (Commands, Records, What is removed); the conversion is `docs/strategies/redesign-architecture.md`, Transition, part 2, and `scripts/cutover/convert_records.py`. Open work is listed by `atelier work missions --all`; a record's file is `.atelier/issues/<id>.md`.

About 180 records are open: about 150 in the backlog, 25 ready and 8 missions. Most backlog items came from review findings deferred against the old tool's commands, and many of those commands are gone. A first pass that reads each record and proposes a fate may be given to a cheaper model (GPT-6 Luna, through `delegate_task`); the implementer checks every proposed drop and rewrite itself before applying it, since a drop is a judgment.

A dropped record stays in Git and can be reopened, so a wrong drop costs one command to undo. A wrong keep costs a stale item in the new tool. When in doubt between drop and keep, keep and say why.

The branch changes only records, so it needs no CI run of its own, and its review is of the list: a reviewer should be able to pick any line and agree from the record and the cited reason.

## Unknowns

- How many of the `cli-output` items the new commands make moot.
