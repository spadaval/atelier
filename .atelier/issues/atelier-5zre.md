---
created_at: "2026-11-10T00:56:09.647724857+00:00"
id: "atelier-5zre"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-5zre
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
closed_at: "2026-11-11T06:24:32.870378979+00:00"
status: "done"
title: "Sort the 147 open items and five open missions for the new tool"
updated_at: "2026-11-11T06:24:32.870378979+00:00"
---

## Goal

Decide, for every open record in this repository, what happens to it at the cutover: it converts as it is, it is rewritten for the new tool, or it is dropped as obsolete. Apply the decisions that are safe on this item's branch, and leave the rest as proposals for Sumeet. The cutover converts every open record, so anything obsolete that is still open then becomes a draft or ready item in the new tool that nobody will ever start.

Only this repository is sorted. The other four repositories' records are converted as they are, and nothing in them is read or changed by this item.

## Done when

- [x] The Report has one line per open record (every item and mission whose status is not `done`, `declined` or `superseded` when the branch starts), giving its ID, its fate (`as is`, `drop`, or `left for Sumeet`) and a reason of one sentence. It gives the count per fate, and the counts add up to the number of open records, which it states with the commit they were counted at.
- [x] A record is dropped only for one of these reasons, and its line names which: (a) it changes, or exists to work around, something the product specification's "What is removed" table removes, or the tracker's site, which goes with the old workspace (`docs/strategies/redesign-3-cutover.md`, step E); (b) the new binary already does what it asks, shown by a line of a specification or by a command run on a binary built from `next/`; (c) it duplicates another open record, which is named. A record whose need survives in another form is not dropped: it is rewritten, or left as it is.
- [x] Each drop is applied with `atelier issue transition <id> supersede --reason "..."` when a specification or another record replaces it, and `decline --reason "..."` otherwise, with the reason from its line. A dropped site item whose need the Desk should meet names mission atelier-z4cr in its reason.
- [x] No record is rewritten by this item. A record whose need survives and whose Goal or Done when names a removed command, flag or record path converts as it is, and its line in the Report says what it names; it is rewritten when it is planned. (Changed by the orchestrator of atelier-qr5s on 2026-11-11, after four review rounds each found a rewrite that had lost a requirement.)
- [x] Missions are not dropped or rewritten by this item. Each open mission has a proposal in the Report for Sumeet: keep, drop, or fold into another, with the count of its open items by fate and whether its ready items could start under the new tool (an item of a draft mission cannot).
- [x] Nothing in the protected set (Scope) is changed, apart from this item's own record, which `atelier start`, the ticks and the Report change as on any item. Each protected record still has its line, with fate `left for Sumeet` and the fate this sort would propose for it.
- [x] The Report gives three counts for this repository, as facts for the cutover, and changes nothing for them: ready items that belong to a draft mission; ready items whose Done when has no checkbox line, which the converter will write as one unticked line (atelier-yudj); and deferred findings on done records that have no follow-up item, which the conversion will not carry over.
- [x] After the changes, `atelier check` passes on the branch, and no open record is blocked by, or is a child of, a dropped one unless the Report says why that is right.
- [x] The sorted records still convert: in a scratch clone of this branch, `scripts/cutover/convert_records.py` converts, `--verify` passes and the new binary's `check` passes. The Report gives the commands and the item and mission counts the converter prints.

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

A drop is not undone by a command. The old tool has no transition out of `declined` or `superseded`, the converter leaves both out of the converted tree, and in the new tool `dropped` is final. A wrongly dropped record is recovered from Git: before the cutover by reverting the commit that dropped it, or its part of that commit, on a reviewed branch; after the cutover by writing a new item from the old record's text in Git history. A wrong keep costs a stale draft item in the new tool, which one `atelier drop` ends. So when in doubt between drop and keep, keep and say why.

The branch changes only records, so it needs no CI run of its own, and its review is of the list: a reviewer should be able to pick any line and agree from the record and the cited reason.

## Unknowns

- How many of the `cli-output` items the new commands make moot.

## Notes

- 2026-11-10 [decision] Run a fourth review round with a fresh GPT-6.1 Sol reviewer (xhigh) after three rounds without an approval. If it finds only more partial (b) drops, restore each as a narrowed rewrite or as it is, fix, and run one further round; if that round does not approve, or a finding asks for anything beyond Scope, stop and report. Master is not brought in.
  Reason: Orchestrator's decision on the escalation: the rounds converged (4, then 5, then 1 wrongly dropped record, all of one kind), and a wrong keep costs one atelier drop while a wrong drop costs digging in Git.
  Rejected: Have Sumeet review the list directly
- 2026-11-11 [decision] Run a sixth review round with a fresh GPT-6.1 Sol reviewer (xhigh) at the branch head, after rechecking each of the 27 rewrites' Done when against its text at fc0fe395. If round 6 finds one more record wrong in the same way (a drop that should be a keep, or a rewrite that lost a requirement), fix it and run one more round, the last authorised; anything else stops for the orchestrator.
  Reason: Orchestrator's decision after round 5: rounds 4 and 5 each found one real problem and nothing else, and round 5's was a rewrite that lost requirements, so the rewrites get the same line-by-line recheck the drops had after round 2.
- 2026-11-11 [decision] Rescope class (b): each of the 26 remaining (b) drops stays dropped only if the Report shows every Done when line at fc0fe395 met, by a specification line or a command run on atelier-next-7b06ed12 in a scratch repository; otherwise the record is restored byte for byte from fc0fe395 with fate as is and a note of what was not shown, never narrowed into a rewrite. Then round 8 with a fresh GPT-6.1 Sol reviewer (xhigh). If it finds another (b) drop wrong, restore every remaining (b) drop without further checking and run one last round; if that does not approve, or a finding is of another kind, stop.
  Reason: Orchestrator's decision after round 7: rounds 4 to 7 each found one record dropped or narrowed as already done by the new binary where it does only part; that judgment is not safe to apply on a reading, the item applies only what is safe, and a wrong keep costs one atelier drop later.
  Rejected: Narrowing such records into rewrites
- 2026-11-11 [decision] Second rescope: no record is rewritten by this item; every rewritten record is restored byte for byte from fc0fe395 as is, its Report line naming what it names that the new tool removed. The orchestrator changes the item's Done when to match. atelier-abdp stays dropped only if atelier-1oel as merged meets each of its Done when lines, shown. Each class (a) drop is reread, and restored where its need would survive the removal in another form. Then round 10; a wrong (a) drop gets one last round, anything else stops.
  Reason: Orchestrator's decision after round 9: rounds 5, 6, 8 and 9 each found a rewrite that had lost a requirement; rewriting backlog items for a tool nobody uses yet is not safe on a reading and buys little, since an item is rewritten when it is planned.
- 2026-11-11 [decision] Run round 11 with a fresh GPT-6.1 Sol reviewer (xhigh) at the head, after reading the Report top to bottom for sentences the two rescopes made untrue. If it finds another stale sentence or a wrong count, fix it and run one more round; anything else stops.
  Reason: Orchestrator's decision after round 10, which passed everything but one stale sentence, now fixed.

## Report

Every Done when line holds, as the orchestrator changed them on 2026-11-11. The branch started from `master` at fc0fe395, where 181 records were open: 173 items and 8 missions (the title's 147 and five are older counts). Each has one line below. 28 were dropped on this branch with `atelier issue transition ... decline`, all under reason (a); 77 stay as they are; and 76 are left for Sumeet: the 72 protected records, 3 missions and atelier-w0su. 28 + 77 + 76 = 181. No record is rewritten. No protected record changed: `git diff --name-only fc0fe395..HEAD -- .atelier/issues` lists the 28 dropped records and this one, and a record counts as protected when its own file's `git log --since=2026-11-09 fc0fe395` names a commit, run file by file. `atelier check` passes at the head. In a scratch clone of the branch, `scripts/cutover/convert_records.py` converts 145 items and 8 missions, `--verify` passes and the new binary's `check` passes (Conversion check below).

### Counts

- drop: 28
- as is: 77
- left for Sumeet: 76
- total: 181, the open records at fc0fe395

### Dropped on this branch

All 28 are reason (a): each changes, or exists to work around, something the product specification's What is removed table removes. Each was declined, since nothing replaces it, and its reason below is also the Report the record now holds. No drop stands under (b) or (c): those were checked again under the orchestrator's rescopes and restored, as Kept as they are says.

- atelier-0cal: drop (a), declined. It changes the output of `bundle apply`, and the product specification's What is removed table removes `bundle`.
- atelier-0xcr: drop (a), declined. It documents `bundle` in its help, and the What is removed table removes `bundle`.
- atelier-1oez: drop (a), declined. It tests the output of `bundle preview`, and the What is removed table removes `bundle`.
- atelier-dpvz: drop (a), declined. It changes reference checks in `bundle` preview and apply and evidence IDs, and the What is removed table removes `bundle` and evidence records.
- atelier-guew: drop (a), declined. It changes `bundle preview` validation, and the What is removed table removes `bundle`.
- atelier-h0nt: drop (a), declined. It changes `bundle preview` output, and the What is removed table removes `bundle`.
- atelier-vsh8: drop (a), declined. It changes `bundle apply -q` output, and the What is removed table removes `bundle`.
- atelier-woul: drop (a), declined. It changes `bundle preview` output, and the What is removed table removes `bundle`.
- atelier-z6vj: drop (a), declined. It changes `bundle` error messages, and the What is removed table removes `bundle`.
- atelier-49eo: drop (a), declined. It changes the close reason of decision records, and the What is removed table removes decision records.
- atelier-hat4: drop (a), declined. It changes a refusal of `evidence record --criterion`, and the What is removed table removes evidence records and commands.
- atelier-1a5k: drop (a), declined. It changes `work parent`, and the What is removed table removes the nine `work` views and parents.
- atelier-b17k: drop (a), declined. It removes a gate inside `work ready`, and the What is removed table removes the nine `work` views.
- atelier-d2xe: drop (a), declined. It changes evidence and review rows in the old `status` owner block, and the What is removed table removes evidence records.
- atelier-x0vs: drop (a), declined. It changes how `issue show --full` prints decision notes, and the What is removed table removes decision records (Commands: decision records and notes are gone).
- atelier-42th: drop (a), declined. It removes `migrate dispositions` and the old review events' read path, and the What is removed table removes `migrate` and the old review subcommands.
- atelier-4urj: drop (a), declined. It changes evidence gates and views, and the What is removed table removes evidence records and commands.
- atelier-8ghm: drop (a), declined. It restores an assertion in a `land` test, and the What is removed table removes `land`.
- atelier-e8l6: drop (a), declined. It changes evidence body parsing, and the What is removed table removes evidence records.
- atelier-f4xz: drop (a), declined. Its case is a base policy dropping a status, and the What is removed table removes `workflow.yaml`; the new lifecycle is fixed in the binary.
- atelier-hu64: drop (a), declined. It is a container of deferred findings with no work of its own, and the What is removed table removes parents; its children atelier-9hkr and atelier-gobs stand alone.
- atelier-jtsn: drop (a), declined. It is a container of deferred findings with no work of its own, and the What is removed table removes parents; its child atelier-c24v stands alone.
- atelier-muac: drop (a), declined. It bounds the update check that `doctor` and the background refresh run, and the What is removed table removes `doctor`; the new binary has no update check (`grep -rli freshness next/crates` finds only an unrelated file).
- atelier-rmvb: drop (a), declined. It changes `prune`, which the What is removed table removes.
- atelier-u0ai: drop (a), declined. It runs `review request` in `scripts/smoke-repos.sh`, and the What is removed table removes the `review` subcommands; the switch deletes the script (architecture specification, Transition part 3).
- atelier-wir7: drop (a), declined. It changes lines of `merge --check`, `merge --sync` and `review show`, and the What is removed table removes those flags and the `review` subcommands.
- atelier-wiri: drop (a), declined. It fixes links in ADRs, which the What is removed table removes, and in docs/product, which step I replaces; the links already resolve (`grep -rn "tracker-records\|ready-missions" docs/product docs/adr` finds nothing).
- atelier-wu7t: drop (a), declined. It changes evidence freshness reasons, and the What is removed table removes evidence records.

### Kept as they are

Each of these converts as it stands at fc0fe395. Where its Goal or Done when names a command, flag or record path the new tool removed, its line says which; it is rewritten when it is planned.

- atelier-x3ru: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `bundle` and its staging IDs, which the new tool removed, and is rewritten when it is planned.
- atelier-w4j2: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its line that review and evidence IDs are checked or rejected naming their kind, and its test line, since the new binary has no such IDs and `check` takes no ID. It names `check <id>` and review and evidence IDs, which the new tool removed, and is rewritten when it is planned.
- atelier-fb1i: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its test line: a run of the binary shows both faults reported in one run, not that a test covers the fixture. It names the `advances` link, which the new tool removed, and is rewritten when it is planned.
- atelier-kset: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `.atelier/issues/` paths, issue types and `check <id>`, which the new tool removed, and is rewritten when it is planned.
- atelier-sfd9: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue unlink` and `relationships.relates`, which the new tool removed, and is rewritten when it is planned.
- atelier-1ci8: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its line that the owner is marked inline and not repeated in Local State, and its test line. It names the old `status` panels (Current Work, Local State), which the new tool removed, and is rewritten when it is planned.
- atelier-5qes: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work blocked`, `work all`, `work mission`, `work parent` and `work ready`, which the new tool removed, and is rewritten when it is planned.
- atelier-5zta: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work mission`, `work parent` and `work ready`, which the new tool removed, and is rewritten when it is planned.
- atelier-8g9d: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its `--quiet` line (the new binary has no `--quiet`) and its counts per mission. It names `status --quiet` and `work missions`, which the new tool removed, and is rewritten when it is planned.
- atelier-9bm2: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work mission`, mission blockers and `start` on a todo mission, which the new tool removed, and is rewritten when it is planned.
- atelier-abdp: as is. Restored unchanged from fc0fe395: a drop under (c) as a duplicate of atelier-1oel was considered, and not shown met by atelier-1oel as merged was its line that docs/product/human-cli-output.md defines a round; it names `review request` and the `status` Branches row, which the new tool removed.
- atelier-avx0: as is. Restored unchanged from fc0fe395 after review finding atelier-7f68: a drop under (c) as a duplicate of atelier-8b0a was considered, and not shown was that `log` covers items filed and the next step, which its Done when asks for and the architecture specification's log leaves out. It names `work mission --since` and loose ends, which the new tool removed, and is rewritten when it is planned.
- atelier-d4b6: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, missions of every status shown alike and no empty panels, survives in `list` and `status`. It names `work all` and `work ready`, which the new tool removed, and is rewritten when it is planned.
- atelier-efnf: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its docs line for docs/product/human-cli-output.md and its tests in the old session tests. It names evidence facts and docs/product/human-cli-output.md, which the new tool removed, and is rewritten when it is planned.
- atelier-hueh: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, views offering as ready only what `start` accepts, survives in `status` and `list`. It names `work mission`, `work parent`, `work ready` and parents, which the new tool removed, and is rewritten when it is planned.
- atelier-hugc: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its `already in_progress on <branch>` message and its test line. It names `issue transition ... plan` and the backlog status, which the new tool removed, and is rewritten when it is planned.
- atelier-lkuy: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work missions`, `work mission` and strategy files in `docs/strategies/`, which the new tool removed, and is rewritten when it is planned.
- atelier-mfyd: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its count of ready items in flight next to the ready count, and its session test. It names `work ready` and `work all`, which the new tool removed, and is rewritten when it is planned.
- atelier-mi3x: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its seven cases, which name old commands (`work parent`, `review request`, `land`, `issue create`), and their tests. It names `work parent`, `review request`, `land` and `issue create`, which the new tool removed, and is rewritten when it is planned.
- atelier-n831: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `land` and `.atelier/issues/` paths, which the new tool removed, and is rewritten when it is planned.
- atelier-om6l: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work mission`, `work human`, `work ready`, `work missions`, the decision type and parents, which the new tool removed, and is rewritten when it is planned.
- atelier-oxnw: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue transition`, `--verbose` and parents, which the new tool removed, and is rewritten when it is planned.
- atelier-pu2i: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue list`, `--category` and `--limit`, which the new tool removed, and is rewritten when it is planned.
- atelier-pvo0: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its rendering test line, and the windows it names, which the new binary does not have. It names `issue show` and `evidence list`, which the new tool removed, and is rewritten when it is planned.
- atelier-tf9n: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work mission`, `work parent`, `work blocked`, `work all` and parents, which the new tool removed, and is rewritten when it is planned.
- atelier-xw8r: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue transition` and `issue show`'s Transition Readiness, which the new tool removed, and is rewritten when it is planned.
- atelier-yzuv: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue show --full`, `work mission`, evidence gates and parents, which the new tool removed, and is rewritten when it is planned.
- atelier-501y: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `-q`, parents and the old `start` output, which the new tool removed, and is rewritten when it is planned.
- atelier-5qep: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue list` and its `--status`, `--category`, `--issue-type` and `--open`, which the new tool removed, and is rewritten when it is planned.
- atelier-dmh5: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no `issue link` or link roles to show them met. It names `issue link` and directional link roles, which the new tool removed, and is rewritten when it is planned.
- atelier-jcki: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue show`, `workflow_branch`, parents and `merge --check`, which the new tool removed, and is rewritten when it is planned.
- atelier-l9ty: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its test line and the `blocked by 2` form for several blockers. It names `issue list`, which the new tool removed, and is rewritten when it is planned.
- atelier-lehs: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its test of a done parent's `--full` view, since parents and `--full` are gone. It names `issue show --full`, which the new tool removed, and is rewritten when it is planned.
- atelier-o1bv: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was that a terminal still uses `COLUMNS`, and its test line. It names `work ready`, which the new tool removed, and is rewritten when it is planned.
- atelier-6gpl: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no transition options view. It names `issue transition`, which the new tool removed, and is rewritten when it is planned.
- atelier-8bve: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue create`, `--parent` and `issue link --role advances`, which the new tool removed, and is rewritten when it is planned.
- atelier-ceu3: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, readable errors about a record's sections, survives in `check` on hand-edited bodies. It names `issue create`, `issue update`, `--body` and `--issue-type`, which the new tool removed, and is rewritten when it is planned.
- atelier-cj54: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no `issue update --section`. It names `issue update --section`, which the new tool removed, and is rewritten when it is planned.
- atelier-e6fl: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no `issue transition` or `issue note`. It names `issue transition` and `issue note`, which the new tool removed, and is rewritten when it is planned.
- atelier-rh0d: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no `issue link` or `unlink`. It names `issue link` and `unlink`, which the new tool removed, and is rewritten when it is planned.
- atelier-sl01: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was every line: the new binary has no transitions, and the doc line is not shown. It names `issue transition`, which the new tool removed, and is rewritten when it is planned.
- atelier-sxcj: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its `--body-file` line: the new binary has no body-writing command. It names `issue create` and `issue update`, which the new tool removed, and is rewritten when it is planned.
- atelier-1hbk: as is. Restored unchanged from fc0fe395 on rereading class (a): its need survives in the Desk's debrief (mission atelier-z4cr). It names the `debrief` command, which the new tool removed, and is rewritten when it is planned.
- atelier-1nlr: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, no plain merge bringing unreviewed code onto the base, survives in the pre-push hook. It names `land`, which the new tool removed, and is rewritten when it is planned.
- atelier-3i7x: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its timing of `work ready` against `work blocked` and its snapshot rule, which name old views and code. It names `work ready`, `work blocked` and `work all`, which the new tool removed, and is rewritten when it is planned.
- atelier-4gsb: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its named test and its `git grep` of the old `review_request.rs`; a run shows only that the new brief asks for no severity. It names `review request`, which the new tool removed, and is rewritten when it is planned.
- atelier-67ay: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its line: the new binary has no `setup --link`. It names `setup --link`, which the new tool removed, and is rewritten when it is planned.
- atelier-akkp: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, checking that a process change cut agents' bookkeeping in real use, survives as the scorecard's comparison after the cutover. It names evidence receipts, which the new tool removed, and is rewritten when it is planned.
- atelier-b99t: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its line: the new binary has no `--version` and no `build.rs`. It names `--version`, which the new tool removed, and is rewritten when it is planned.
- atelier-eb71: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue list --status`, which the new tool removed, and is rewritten when it is planned.
- atelier-fnql: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, settling a deferred finding fixed in passing, survives in `drop` on a deferred finding. It names `review disposition`, which the new tool removed, and is rewritten when it is planned.
- atelier-g21n: as is. Restored unchanged from fc0fe395 on rereading class (a): its need survives in the Desk's debrief (mission atelier-z4cr). It names the `debrief` command and decision notes, which the new tool removed, and is rewritten when it is planned.
- atelier-gg0m: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `status --hook claude` and `land --hook claude`, which the new tool removed, and is rewritten when it is planned.
- atelier-h90a: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names the `prime` and `land` hooks, which the new tool removed, and is rewritten when it is planned.
- atelier-hfme: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its CI pin line and its ADR 0028 and docs line. It names `start --base` and `branch_base`, which the new tool removed, and is rewritten when it is planned.
- atelier-j458: as is. Restored unchanged from fc0fe395 on rereading class (a): its need, history reads that survive a non-UTF-8 path, survives in the new tool's reads of records at a commit. It names `land` and `doctor`, which the new tool removed, and is rewritten when it is planned.
- atelier-j79t: as is. Restored unchanged from fc0fe395 on rereading class (a): its need survives in the Desk's debrief (mission atelier-z4cr). It names the `debrief` command, which the new tool removed, and is rewritten when it is planned.
- atelier-jjvc: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `prime` and `land`, which the new tool removed, and is rewritten when it is planned.
- atelier-lozn: as is. Restored unchanged from fc0fe395 on rereading class (a): its need survives in the Desk's debrief page (mission atelier-z4cr). It names the site's debrief page, which the new tool removed, and is rewritten when it is planned.
- atelier-taxm: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its line: the new binary has no `setup --link` or update check. It names `setup --link` and the update check, which the new tool removed, and is rewritten when it is planned.
- atelier-xz5c: as is. Restored unchanged from fc0fe395 under the orchestrator's rescope of (b): a drop under (b) was considered, and not shown met by a specification line or a run of atelier-next-7b06ed12 was its lines on the old verdict and gate output, the old brief's template, and the docs and CONTEXT.md. It names `land`, `merge --check`, `review request` and evidence freshness, which the new tool removed, and is rewritten when it is planned.
- atelier-k55o: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue create`, its `--body` and the decision type, which the new tool removed, and is rewritten when it is planned.
- atelier-7ebi: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue update`, `issue note`, `issue link` and `-q`, which the new tool removed, and is rewritten when it is planned.
- atelier-bp3s: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `.atelier/workflow.yaml`'s `merge_checks` and `merge --sync` rerunning suite receipts, which the new tool removed, and is rewritten when it is planned.
- atelier-e95f: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `work ready`, `work mission`, an ADR and evidence freshness, which the new tool removed, and is rewritten when it is planned.
- atelier-fpxc: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `issue list` and its `--status`, which the new tool removed, and is rewritten when it is planned.
- atelier-nl9a: as is. Restored unchanged from fc0fe395 after a rewrite was withdrawn (orchestrator's second rescope); it names `land --since`, `land_base_unreviewed` and the light path, which the new tool removed, and is rewritten when it is planned.
- atelier-9hkr: as is. A deferred finding on the live skill's opening, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-c24v: as is. A deferred finding on the live skill's description, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-gobs: as is. A deferred finding on the live skill's opening and install procedure, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-hzuk: as is. README install documentation for pinned releases, whose Goal and Done when name no removed command.
- atelier-jxce: as is. An eval of the skill that names no removed command; the new tool keeps fix-now findings and their exits.
- atelier-jy8k: as is. Scheduling the gardener survives the cutover; mission atelier-295u rewrites the gardener it will run.
- atelier-pju0: as is. Rebuilding an eval fixture names no removed command; it is jxce's work.
- atelier-t3ko: as is. The product specification keeps `init` ("plus `init` and `setup`") and the new binary has none yet, so its need waits for that `init`. It names `init -q`'s `-q`, which the new tool removed, and is rewritten when it is planned.
- atelier-wl6u: as is. Porting two eval fixtures names no removed command; step B (atelier-t747) ports the scenarios to the new binary and may take it.
- atelier-zjgy: as is. Why tracker-only commits are so many is still a question under the new tool, and its Goal and Done when name no removed command.

### Left for Sumeet

Each line gives the fate this sort proposes. The 72 protected records come first, grouped so they can be read in a few minutes; none was changed. A proposed drop under (b) or (c) is a reading, not checked line by line as an applied drop would now be.


#### Mission atelier-qr5s and its items: keep as they are

- atelier-qr5s: left for Sumeet; proposed as is. Keep: the cutover mission, live work in progress.
- atelier-1oel: left for Sumeet; proposed as is. Live work of mission atelier-qr5s on the new binary.
- atelier-2mlh: left for Sumeet; proposed as is. Step H of mission atelier-qr5s, planned at its boundary.
- atelier-5zre: left for Sumeet; proposed as is. This item.
- atelier-6w8d: left for Sumeet; proposed as is. Step I of mission atelier-qr5s.
- atelier-mlwx: left for Sumeet; proposed as is. Step D of mission atelier-qr5s, in progress on its own branch.
- atelier-oci2: left for Sumeet; proposed as is. Step E of mission atelier-qr5s.
- atelier-orb2: left for Sumeet; proposed as is. Step A of mission atelier-qr5s.
- atelier-t747: left for Sumeet; proposed as is. Step B of mission atelier-qr5s.
- atelier-u1ko: left for Sumeet; proposed as is. Step F of mission atelier-qr5s.
- atelier-wd1i: left for Sumeet; proposed as is. Step G of mission atelier-qr5s.
- atelier-yudj: left for Sumeet; proposed as is. Converter work for mission atelier-qr5s.
- atelier-7w9s: left for Sumeet; proposed as is. The work host's cutover, which is Sumeet's.

#### Missions atelier-295u and atelier-z4cr, their items and the scorecard: keep as they are

- atelier-295u: left for Sumeet; proposed as is. Keep: mission 4, written for the new tool.
- atelier-613a: left for Sumeet; proposed as is. Mission atelier-295u's work, written for the new tool.
- atelier-8b0a: left for Sumeet; proposed as is. Mission atelier-295u's work, written for the new tool.
- atelier-9dku: left for Sumeet; proposed as is. Mission atelier-295u's work, written for the new tool.
- atelier-vl5n: left for Sumeet; proposed as is. Mission atelier-295u's work, written for the new tool.
- atelier-c1z3: left for Sumeet; proposed as is. The scorecard rerun after the cutover, written for the new tool.
- atelier-pwhn: left for Sumeet; proposed as is. The scorecard comparison after the cutover, written for the new tool.
- atelier-z4cr: left for Sumeet; proposed as is. Keep: the Desk, written for the new records; take in atelier-aab1, atelier-73vx and atelier-wo22 from atelier-bnek.
- atelier-1ie1: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-94pv: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-e05n: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-h3hl: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-psjw: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-uwxy: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-va99: left for Sumeet; proposed as is. Mission atelier-z4cr's work, written for the new core.
- atelier-qxmr: left for Sumeet; proposed as is. Points the Desk at the converted repositories after the cutover.

#### Other work written for the new tool or the new skill: keep as they are

- atelier-42b4: left for Sumeet; proposed as is. An eval of the new loop's untested paths, which step B may take.
- atelier-o91z: left for Sumeet; proposed as is. A bug in the new binary's Responses parsing.
- atelier-lkf5: left for Sumeet; proposed as is. A deferred finding against the new binary (`next/crates/atelier-cli/src/plan.rs`); its empty Done when becomes one unticked line (atelier-yudj), so give it a real one before it is made ready.
- atelier-m2v1: left for Sumeet; proposed as is. A deferred finding against the new binary's `drop`; same note on its Done when as atelier-lkf5.
- atelier-8ep4: left for Sumeet; proposed as is. Named in step A's plan, which says whether the new skill starts from it.
- atelier-y0qe: left for Sumeet; proposed as is. Named in step A's plan, which says whether the new skill absorbs it.
- atelier-bg7w: left for Sumeet; proposed as is. A skill readability budget, blocked by atelier-8ep4; step A may absorb it.
- atelier-c1sr: left for Sumeet; proposed as is. A deferred finding on the live skill's orchestrate.md, like atelier-9hkr; step A may absorb it.

#### Mission atelier-bnek: fold into atelier-z4cr

- atelier-bnek: left for Sumeet; proposed fold into atelier-z4cr. Its live parts are the Desk's: atelier-aab1, atelier-73vx and atelier-wo22 move to atelier-z4cr, and the rest drop as below.
- atelier-aab1: left for Sumeet; proposed as is, moved to atelier-z4cr. How an outside process reads T3 threads is the Desk's question.
- atelier-73vx: left for Sumeet; proposed as is, moved to atelier-z4cr. A metrics page by model and role is the Desk's ("a scorecard page is the Desk's", product specification, Out of scope); it names "the site", and is rewritten when it is planned.
- atelier-wo22: left for Sumeet; proposed as is, moved to atelier-z4cr. Its check of who-did-what runs on the Desk's page; it names the old site and evidence receipts, and is rewritten when it is planned.
- atelier-7yy9: left for Sumeet; proposed drop (b). The specification records attribution as fields the act writes (implementer and reviewer threads, the reviewer's model) and puts a session database out of scope ("A session database ... the Desk's").
- atelier-00hl: left for Sumeet; proposed drop (b). A review file's front matter already holds the reviewer's threads and model (product specification, Records).
- atelier-5kwk: left for Sumeet; proposed drop (a). Its reminder keeps the light path apart from item work, and the What is removed table removes the light path; `status` on a branch no item owns says so (`which is no item's`).

#### Mission atelier-icv4: drop with its items

- atelier-icv4: left for Sumeet; proposed drop. Its items are met by the new commands or name removed ones; see Missions.
- atelier-06jc: left for Sumeet; proposed drop (b). The new brief carries the item's definition and earlier rounds' open findings with the author's responses (product specification, Review: What the reviewer is given).
- atelier-5xvl: left for Sumeet; proposed drop (b). The new `review <mission>` opens a plan review and prints its brief (product specification, Commands: Reviewer).
- atelier-jqzl: left for Sumeet; proposed drop (b). The new `merge` waits for CI on the head itself (product specification, Merge), so no script is run by hand.
- atelier-tvfd: left for Sumeet; proposed drop (a). The write commands whose forms it lists (`issue create`, `link`, `note`, `evidence record`, `review disposition`) are removed by the What is removed table; the new tool's 18 commands each print their own flags.
- atelier-zzwi: left for Sumeet; proposed drop (b). The new binary binds each thread to a role and answers another role's command with one line (product specification, What status prints), and a reviewer runs only `review` and `verdict`; the skill part is step A's.
- atelier-zno7: left for Sumeet; proposed drop (c). Duplicates atelier-pwhn, whose scorecard comparison counts confusion per 100 `atelier` calls, the measure it would re-count.

#### Debrief records, which the Desk takes over: keep as they are

- atelier-jdpf: left for Sumeet; proposed as is. Its need, an independent review of a debrief generated under the current records, survives for the Desk's debrief (mission atelier-z4cr); protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-k7vp: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-les9: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-okfy: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-rdsa: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-wg6p: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-ztuq: left for Sumeet; proposed as is. Its need survives in the Desk's debrief (mission atelier-z4cr), the form the specification gives the debrief, so it stays for the Desk's planning; protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-ayrg: left for Sumeet; proposed as is. Its need, counting findings resolved by a later approval as fixed, survives in the Desk's debrief (mission atelier-z4cr).

#### Old-tool records and the old site: drop, except atelier-w4ef

- atelier-w4ef: left for Sumeet; proposed as is. Its need, `check` applying one rule set to placeholder sections, survives, and atelier-k55o, kept as it is, does not cover `check`.
- atelier-2yef: left for Sumeet; proposed drop (b). Where a declined started item's record goes is settled by the specification: dropping is final and happens on the base, and every command reads the base's record first (product specification, Lifecycle: When a thread dies).
- atelier-pg2b: left for Sumeet; proposed drop. Already done: atelier-nz6a made install.md give the install lines (atelier-8i86, G14), and the skill no longer lives in spadaval/.agents; none of (a) to (c) names this case.
- atelier-c6g2: left for Sumeet; proposed drop (a). It teaches the skill `work mission --since` and the old references/atelier.md Administration section, both gone in the new tool; its blockers atelier-avx0 and atelier-fb1i stay open as they are.
- atelier-5lv9: left for Sumeet; proposed drop (a). It changes the light-path review brief, and the What is removed table removes the light path.
- atelier-lr08: left for Sumeet; proposed drop (a). It speeds up the `debrief` command, which the What is removed table removes.
- atelier-3km4: left for Sumeet; proposed drop (a). A taste read of the old site's debrief page, which the What is removed table removes; the Desk (mission atelier-z4cr) gets its own.
- atelier-pywh: left for Sumeet; proposed drop (a). A taste read of the old fleet site, which the What is removed table removes with `serve`; the Desk (mission atelier-z4cr) gets its own.
- atelier-dd3a: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its children atelier-lkf5 and atelier-m2v1 stand alone.
- atelier-rqzu: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its open child atelier-2yef stands alone.
- atelier-zmei: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its child atelier-c1sr stands alone.

#### Questions for Sumeet: close with this Report

- atelier-8i86: left for Sumeet; proposed close with this Report. Most of its 21 proposals concern records this sort dropped or kept as they are; answer it by pointing at this Report and close it.
- atelier-j86z: left for Sumeet; proposed close with this Report. Eight of its ten recommendations change old commands or old files that the cutover removes; carry R6 (grep for stale docs as a proof step, for step A) and R8 (`check` warns on trailing whitespace) into new items if still wanted, and close it.

#### Not protected, left for Sumeet

- atelier-w0su: left for Sumeet. Its Design section is built on the old model throughout (`workflow_branch`, `branch_base`, `land`, `prime`), so a rewrite of names alone would leave it incoherent; proposal: drop, and refile against the new `start` and `merge` if parallel missions still need it.
- atelier-g6rp: left for Sumeet. An open mission, which this item does not change; see Missions.
- atelier-4pzj: left for Sumeet. An open mission, which this item does not change; see Missions.
- atelier-uf97: left for Sumeet. An open mission, which this item does not change; see Missions.

### Missions

Missions are not dropped or rewritten here. Open items are counted at fc0fe395 by this sort's fate at the head; protected items are counted by the fate proposed for them. An item of a draft mission cannot start under the new tool.

- atelier-qr5s (todo, converts as ready): keep. 11 open items, all protected and proposed as is. At fc0fe395, 6 were ready (todo): atelier-1oel, atelier-5zre, atelier-mlwx, atelier-oci2, atelier-orb2 and atelier-yudj. Under the new tool, 4 of them could start (atelier-1oel, atelier-mlwx, atelier-orb2, atelier-yudj); atelier-oci2 waits on its blocker atelier-mlwx, and atelier-5zre is this item, now in progress. A ready mission is needed for its items to start, not enough.
- atelier-295u (draft): keep. 4 open items, all protected and proposed as is; none is ready.
- atelier-z4cr (draft): keep, and take in atelier-aab1, atelier-73vx and atelier-wo22 from atelier-bnek, and the debrief records kept as they are when the Desk's debrief is planned. 7 open items, all protected and proposed as is. Its 2 ready items (atelier-1ie1, atelier-psjw) cannot start until the mission is made ready.
- atelier-bnek (draft): fold into atelier-z4cr. 6 open items, all protected: 3 proposed to move to atelier-z4cr as they are (atelier-aab1, atelier-73vx, atelier-wo22) and 3 proposed to drop (atelier-7yy9, atelier-00hl, atelier-5kwk). Its 4 ready items (atelier-00hl, atelier-5kwk, atelier-7yy9, atelier-aab1) cannot start while it is a draft.
- atelier-icv4 (draft): drop. 5 open items, all protected and proposed to drop, since each names old commands the new ones replace. Its 5 ready items cannot start while it is a draft. Its loose end atelier-zno7 is proposed to drop with it.
- atelier-4pzj (draft): keep. 33 open items: 4 dropped on this branch (atelier-1a5k, atelier-b17k, atelier-d2xe, atelier-x0vs) and 29 as they are. None is ready. Its outcome, seeing where things stand, still holds for the new `status`, `list`, `show` and `start`; most of its items name the old views and are rewritten, or dropped, when the mission is planned.
- atelier-uf97 (draft): keep, as the mission for the record-writing and `check` items. 27 open items: 11 dropped on this branch (its `bundle` items, atelier-49eo and atelier-hat4), 15 as they are and 1 protected (atelier-w4ef, proposed as is). None is ready. Its Outcome names `bundle`, which is removed, and is rewritten when the mission is planned.
- atelier-g6rp (todo, converts as ready): keep only to close it. It has no open items: all 8 are done and its last wait, atelier-tee8, is done. Close it with a Result before the cutover; if it is still open then, drop it.

### Counts for the cutover

Facts at the branch head, changing nothing (`~/.cache/atelier-qr5s/5zre/counts.py` and `deferred.py`):

- Ready items that belong to a draft mission: 11 of 28 ready items: atelier-00hl, atelier-06jc, atelier-1ie1, atelier-5kwk, atelier-5xvl, atelier-7yy9, atelier-aab1, atelier-jqzl, atelier-psjw, atelier-tvfd and atelier-zzwi.
- Ready items whose Done when has no checkbox line, which the converter will write as one unticked line (atelier-yudj): 3: atelier-3km4 (prose Done when), atelier-8i86 and atelier-j86z (decisions with no Done when section).
- Deferred findings on done records with no follow-up item: 0. Of the 126 deferred findings on done records, every one names a follow-up: 73 are done, 27 are still in the backlog, and 26 were superseded or declined, including follow-ups this sort dropped.

### Links to dropped records

`atelier check` passes at the head. Three open records are children of a dropped one, and each is right: atelier-9hkr and atelier-gobs are children of atelier-hu64, and atelier-c24v of atelier-jtsn. The parents were containers for deferred findings with no work of their own; parents are removed in the new tool, and the converter makes each child a standalone item. No open record is blocked by a dropped one.

### Conversion check

Run by `~/.cache/atelier-qr5s/5zre/convcheck.sh` in a scratch clone under `~/.cache/atelier-qr5s/5zre/conv`, with `TMPDIR` under `~/.cache`:

```text
git clone -b atelier-5zre <worktree> conv && git branch -m master
python3 scripts/cutover/convert_records.py
git add -A && git commit -m "Convert Atelier records to the new layout"
python3 scripts/cutover/convert_records.py --verify <pre-conversion head>
atelier-next-3ba4a62f check
atelier-next-7b06ed12 check
```

Output at d88fb94e, which is also the last commit that changes a record other than this one; later commits change only this record:

```text
Records converted, by kind and new status:
  item draft: 116
  item from decision ready: 2
  item ready: 27
  mission draft: 6
  mission ready: 2
Made ready from in_progress; each branch's work is not converted:
  atelier-5zre
This tree's .atelier/ and moved files match a fresh conversion of d88fb94e73e34c8c2c0215e0977e5e0d48f3d347: 167 files.
Every record is valid: 145 items, 8 missions and config.toml.
```

The converter printed 145 items (116 draft, 27 ready, 2 ready from decisions) and 8 missions (6 draft, 2 ready); `--verify` and `check` each exited 0, `check` on both atelier-next-3ba4a62f and atelier-next-7b06ed12. 145 items and 8 missions are the 181 open records less the 28 dropped. The full output is in `~/.cache/atelier-qr5s/5zre/convout.txt`; it also lists 22 `blocked_by` entries naming a closed or missing record, which the converter leaves out.

### Review

Ten rounds of a fresh GPT-6.1 Sol reviewer each recorded `fix_now` findings, all fixed in their own commits and none disputed: atelier-ax6k, atelier-zs8v, atelier-h72a and atelier-shpk (round 1); atelier-1d9u, atelier-3ww2, atelier-s2h3, atelier-qptk and atelier-k6yq (round 2); atelier-p4v4 (round 3); atelier-brmd (round 4, seven protected records dropped, restored); atelier-864b (round 5); atelier-w8mt (round 6); atelier-jaa4 (round 7); atelier-sc6q (round 8); atelier-7f68 and atelier-5yj2 (round 9); atelier-oxvm (round 10, a sentence the rescopes had left untrue). All but atelier-shpk, atelier-k6yq, atelier-brmd and atelier-oxvm found a record dropped, or rewritten, where a requirement was lost.

The orchestrator then took both judgments out of the item (decision notes of 2026-11-11):

- After round 7, every drop under (b) was rechecked against each Done when line at fc0fe395. None had every line shown met by a specification line or a run of atelier-next-7b06ed12, so all 26 were restored unchanged. atelier-avx0, dropped under (c), was restored after round 9.
- After round 9, no record is rewritten. All 28 rewritten records were restored unchanged from fc0fe395, and the rewritten text is kept nowhere. atelier-abdp, the last drop under (c), was kept only if atelier-1oel as merged met each of its Done when lines; its line that docs/product/human-cli-output.md defines a round is not met by atelier-1oel, so it was restored unchanged. Each drop under (a) was read again against reason (a) alone, and 11 whose need survives the removal in another form were restored unchanged: atelier-akkp, atelier-d4b6, atelier-hueh, atelier-ceu3, atelier-1hbk, atelier-g21n, atelier-j79t, atelier-lozn, atelier-1nlr, atelier-fnql and atelier-j458. The protected debrief records' proposals changed to match: the Desk's debrief takes them over.

Before round 11 the Report was read from top to bottom for sentences the two rescopes had made untrue. Fixed then: atelier-8i86's proposal, which spoke of rewrites; nine record lines that began with a stray "It" (atelier-hzuk, atelier-jxce, atelier-jy8k, atelier-pju0, atelier-wl6u, atelier-zjgy, atelier-g6rp, atelier-4pzj and atelier-uf97); and round 10's place in this section.

### Deviations

- The fourth Done when line was changed by the orchestrator on 2026-11-11: no record is rewritten by this item, and a kept record's line says what it names that the new tool removed.
- The first pass was not delegated to GPT-6 Luna. I read all 181 records myself, since every drop needed checking by hand anyway.
- Commit ac1a558a holds all 40 restores of the second rescope (the 28 rewritten records, atelier-abdp and the 11 class (a) records), not only the 28 its message names: `git checkout fc0fe395 -- <file>` had already staged each one.
- Runs of the new binary were made only on converted scratch clones under `~/.cache/atelier-qr5s/5zre/`, never in a real repository.

### Decisions

- Every remaining drop is `decline`: each removes something with no replacement the specification names.
- atelier-w0su is left for Sumeet: its Design section is built on the old model throughout.
- atelier-t3ko stays as it is although the new binary has no `init`: the product specification keeps `init`, so its need waits for one.

### Open items

- Proposals for the 72 protected records, the three unprotected missions and atelier-w0su are Sumeet's to decide.
- Master moved after fc0fe395; records created there since are not in this list.
