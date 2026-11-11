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
status: "in_progress"
title: "Sort the 147 open items and five open missions for the new tool"
updated_at: "2026-11-11T05:16:37.409905625+00:00"
---

## Goal

Decide, for every open record in this repository, what happens to it at the cutover: it converts as it is, it is rewritten for the new tool, or it is dropped as obsolete. Apply the decisions that are safe on this item's branch, and leave the rest as proposals for Sumeet. The cutover converts every open record, so anything obsolete that is still open then becomes a draft or ready item in the new tool that nobody will ever start.

Only this repository is sorted. The other four repositories' records are converted as they are, and nothing in them is read or changed by this item.

## Done when

- [x] The Report has one line per open record (every item and mission whose status is not `done`, `declined` or `superseded` when the branch starts), giving its ID, its fate (`as is`, `rewrite`, `drop`, or `left for Sumeet`) and a reason of one sentence. It gives the count per fate, and the counts add up to the number of open records, which it states with the commit they were counted at.
- [x] A record is dropped only for one of these reasons, and its line names which: (a) it changes, or exists to work around, something the product specification's "What is removed" table removes, or the tracker's site, which goes with the old workspace (`docs/strategies/redesign-3-cutover.md`, step E); (b) the new binary already does what it asks, shown by a line of a specification or by a command run on a binary built from `next/`; (c) it duplicates another open record, which is named. A record whose need survives in another form is not dropped: it is rewritten, or left as it is.
- [x] Each drop is applied with `atelier issue transition <id> supersede --reason "..."` when a specification or another record replaces it, and `decline --reason "..."` otherwise, with the reason from its line. A dropped site item whose need the Desk should meet names mission atelier-z4cr in its reason.
- [x] A record is rewritten when its need survives and its Goal or Done when names a removed command, flag or record path. The rewrite changes only those names and the sentences that depend on them, to the commands the specifications give, and keeps the need as stated. The Report lists each rewritten record with what changed.
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

## Report

Every Done when line holds. The branch started from `master` at fc0fe395, where 181 records were open: 173 items and 8 missions (the title's 147 and five are older counts). Each has one line below. 68 were dropped on this branch with `atelier issue transition` (25 superseded, 43 declined), 27 were rewritten, 10 stay as they are, and 76 are left for Sumeet: the 72 protected records, 3 missions and atelier-w0su. 68 + 27 + 10 + 76 = 181. No protected record changed: the changed paths in `git diff --name-only fc0fe395..HEAD -- .atelier/issues` are the 95 dropped or rewritten records and this record, none of them protected. A record counts as protected when its own file's `git log --since=2026-11-09 fc0fe395` names a commit, run file by file. `atelier check` passes at the head. In a scratch clone of the branch, `scripts/cutover/convert_records.py` converts 105 items and 8 missions, `--verify` passes and the new binary's `check` passes (Conversion check below).

### Counts

- drop: 68
- rewrite: 27
- as is: 10
- left for Sumeet: 76
- total: 181, the open records at fc0fe395

### Dropped on this branch

Each line gives the reason letter of the Done when, the transition applied (`supersede` where a specification or another record replaces it, `decline` otherwise), and the reason, which is also the Report each dropped record now holds. Evidence marked "run in a scratch clone" was run with the build of `next/` at `~/.cache/atelier-qr5s/bin/atelier-next-3ba4a62f` on a converted scratch clone of this branch.


#### (a) removed by the What is removed table, or the site: 39

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
- atelier-d4b6: drop (a), superseded. It changes how `work all` and `work ready` classify missions, and the What is removed table removes the `work` views; the new `list` heads each open mission with its status.
- atelier-hueh: drop (a), declined. Its case needs a parent's branch that its children share, and the What is removed table removes parents; in the new tool each item starts its own branch.
- atelier-x0vs: drop (a), declined. It changes how `issue show --full` prints decision notes, and the What is removed table removes decision records (Commands: decision records and notes are gone).
- atelier-ceu3: drop (a), declined. It changes `issue create` and `update` errors for `--body` and `--issue-type`, and the What is removed table removes issue types; the new `new` takes only a title and agents edit the body directly.
- atelier-1hbk: drop (a), superseded. It changes the debrief page, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief.
- atelier-1nlr: drop (a), superseded. It changes `land`, which the What is removed table removes; the new pre-push hook refuses a push of the base whose new commit is not an Atelier merge and touches code (product specification, Hooks).
- atelier-42th: drop (a), declined. It removes `migrate dispositions` and the old review events' read path, and the What is removed table removes `migrate` and the old review subcommands.
- atelier-4urj: drop (a), declined. It changes evidence gates and views, and the What is removed table removes evidence records and commands.
- atelier-8ghm: drop (a), declined. It restores an assertion in a `land` test, and the What is removed table removes `land`.
- atelier-akkp: drop (a), declined. It measures the evidence receipts and refusals of the old lighter workflow, and the What is removed table removes evidence records; the redesign's scorecard (atelier-c1z3, atelier-pwhn) measures the cost from here.
- atelier-e8l6: drop (a), declined. It changes evidence body parsing, and the What is removed table removes evidence records.
- atelier-f4xz: drop (a), declined. Its case is a base policy dropping a status, and the What is removed table removes `workflow.yaml`; the new lifecycle is fixed in the binary.
- atelier-fnql: drop (a), superseded. It adds a decision to `review disposition`, and the What is removed table removes the nine `review` subcommands; the new `drop` settles a deferred finding with a reason.
- atelier-g21n: drop (a), superseded. It changes the debrief, and the What is removed table removes the `debrief` command and decision notes; the Desk (mission atelier-z4cr) shows the debrief.
- atelier-hu64: drop (a), declined. It is a container of deferred findings with no work of its own, and the What is removed table removes parents; its children atelier-9hkr and atelier-gobs stand alone.
- atelier-j458: drop (a), declined. It changes the history loading of `land` and `doctor`, which the What is removed table removes.
- atelier-j79t: drop (a), superseded. It changes the debrief, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief.
- atelier-jtsn: drop (a), declined. It is a container of deferred findings with no work of its own, and the What is removed table removes parents; its child atelier-c24v stands alone.
- atelier-lozn: drop (a), superseded. It changes the site's debrief page, which the What is removed table removes with `serve`; the Desk (mission atelier-z4cr) shows the debrief.
- atelier-muac: drop (a), declined. It bounds the update check that `doctor` and the background refresh run, and the What is removed table removes `doctor`; the new binary has no update check (`grep -rli freshness next/crates` finds only an unrelated file).
- atelier-rmvb: drop (a), declined. It changes `prune`, which the What is removed table removes.
- atelier-u0ai: drop (a), declined. It runs `review request` in `scripts/smoke-repos.sh`, and the What is removed table removes the `review` subcommands; the switch deletes the script (architecture specification, Transition part 3).
- atelier-wir7: drop (a), declined. It changes lines of `merge --check`, `merge --sync` and `review show`, and the What is removed table removes those flags and the `review` subcommands.
- atelier-wiri: drop (a), declined. It fixes links in ADRs, which the What is removed table removes, and in docs/product, which step I replaces; the links already resolve (`grep -rn "tracker-records\|ready-missions" docs/product docs/adr` finds nothing).
- atelier-wu7t: drop (a), declined. It changes evidence freshness reasons, and the What is removed table removes evidence records.

#### (b) the new binary already does it: 27

- atelier-w4j2: drop (b), declined. It changes how `check <id>` treats review and evidence IDs, and the new `check` takes no ID and validates every record (`atelier check --help` on the new binary), while evidence records are removed by the What is removed table.
- atelier-fb1i: drop (b), superseded. The new binary's `check` already reports a record that does not parse and the other records' missing references in one run (run in a scratch clone: one unparseable item, one unknown mission and one unknown blocker gave three problems in one report).
- atelier-1ci8: drop (b), superseded. The new `status` lists what the thread holds once with its status (product specification, Commands: `status`), shown in a scratch clone.
- atelier-8g9d: drop (b), superseded. The new `status` on the base lists the missions in progress by name (product specification, Commands: `status`).
- atelier-efnf: drop (b), superseded. The new `status` lists only what is not met on an item, open fix-now findings included, and `verdict approve` is refused while a fix-now finding is open (product specification, What status prints and Reviewer).
- atelier-hugc: drop (b), superseded. The new `start` already refuses a draft item before any Git operation and names `atelier ready` (run in a scratch clone: `atelier-1a5k is draft, and atelier start runs on an item that is ready. Nothing changed.`).
- atelier-mfyd: drop (b), superseded. The new `status` lists only the ready items `start` would accept now and counts the others by what stops them (run in a scratch clone: `14 other ready items cannot start yet: 5 wait on a blocker that is not done, 11 are in a mission that is not ready`).
- atelier-mi3x: drop (b), superseded. The new `status` prints the one next command for what the thread holds (product specification, Commands: `status`; `Then: atelier start atelier-1oel` in a scratch clone).
- atelier-pvo0: drop (b), superseded. The truncations it names are in `issue show`, the old `status` owner block, `evidence list` and the old not-found hint, none of which the new binary has; its not-found error names `atelier list` (run in a scratch clone).
- atelier-yzuv: drop (b), superseded. The new `show <mission>` prints the mission once on one page with its items by status, with no Relationships or Evidence Gates lines (run on atelier-4pzj in a scratch clone).
- atelier-dmh5: drop (b), declined. The new records hold no directional link roles, only `blocked_by` and `caused_by` (product specification, Records), so there is no reversed link to misread.
- atelier-l9ty: drop (b), superseded. The new `list` marks each blocked and needs-human row (`blocked by ...`, `waits on a human`), run in a scratch clone.
- atelier-lehs: drop (b), superseded. The new `show` prints no Impact section (run on atelier-1oel in a scratch clone).
- atelier-o1bv: drop (b), superseded. The new binary does not wrap redirected output (`COLUMNS=30 atelier list | cat` printed full rows in a scratch clone).
- atelier-6gpl: drop (b), superseded. The new tool has no transition options view: each status is set by the command that performs the act (product specification, Words: Status), and `status` names the next command.
- atelier-cj54: drop (b), declined. The product specification has agents edit a record's Markdown body directly (Commands: "Agents edit the Markdown body of a record directly"), so no command splices a section and the quoting problem it solves does not arise.
- atelier-e6fl: drop (b), declined. The new tool has no transitions that take `--reason` besides `drop`, which records it (`atelier drop --help`).
- atelier-rh0d: drop (b), declined. The new tool has no `issue link` or `unlink` and no link roles; `set --blocked-by` sets the only sequencing link (`atelier set --help`).
- atelier-sl01: drop (b), declined. The new tool has no transitions; `drop` takes `--reason` as a required argument (`atelier drop --help`), so a view and a run cannot disagree on it.
- atelier-sxcj: drop (b), declined. The product specification has agents edit a record's Markdown body directly, so no command takes a body and `--body-file` has nothing to feed.
- atelier-3i7x: drop (b), superseded. The new `status` reads started items without the old snapshot: atelier-nafc's rehearsal timed it at 503 ms with 1 branch and 563 ms with 31 in astromech.
- atelier-4gsb: drop (b), superseded. The new review brief asks for no severity (`grep -rni severity next/crates` prints nothing).
- atelier-67ay: drop (b), declined. The new binary has no `setup --link` (`atelier setup --help` takes only a harness), so it never replaces a skill copy.
- atelier-b99t: drop (b), declined. The new workspace's `atelier-cli` has no `build.rs` (`ls next/crates/atelier-cli`), so no stale commit is stamped; the old one goes with `crates/` at the switch.
- atelier-hfme: drop (b), declined. The new records hold no branch field: an item's branch is named for its ID (product specification, Records), and the new `start` has no `--base`.
- atelier-taxm: drop (b), declined. The new binary has no `setup --link` and no update check (`atelier setup --help`), so there is no second copy of the rule.
- atelier-xz5c: drop (b), superseded. Its lines are output of `land`, `merge --check`, `review request` and evidence freshness, none of which the new binary has; the specification's Words table gives each word one meaning in the new CLI.

#### (c) duplicates another open record: 2

- atelier-abdp: drop (c), superseded. It duplicates atelier-1oel, whose Done when makes `status` show the round of an item in review; the new `show` already names each review's round.
- atelier-avx0: drop (c), superseded. It duplicates atelier-8b0a, which adds `atelier log` printing every act since a time, the form the specification gives a since-commit digest (`log`, product specification Commands).

### Rewritten on this branch

- atelier-x3ru: rewrite. Its cycle diagnostic survives: two items blocked by each other pass the new `check` (run in a scratch clone). What changed: Goal and Done when: narrowed to `check` reporting a `blocked_by` cycle with the items in it; the bundle staging IDs and the `relationships.blocks` wording went with `bundle` and the old storage.
- atelier-kset: rewrite. It two of its faults survive in the new `check`: YAML line numbers one less than the file line, and a last line that names no problem (run in a scratch clone). What changed: Goal and Done when: narrowed to the two faults the new `check` still has, file-relative YAML line numbers and a last line that names the first problem, stated against `.atelier/items/<id>/item.md` and the new binary's output; the parts the new `check` already does left.
- atelier-sfd9: rewrite. Its need survives in part: the new `check` names the file and field of a missing link but not the edit that removes it (run in a scratch clone). What changed: Goal and Done when: narrowed to the finding naming the edit that removes a missing link; `issue unlink` became `set --blocked-by`; the file-and-field and shared-advice parts left, since the new `check` already does them.
- atelier-5qes: rewrite. Its need survives: the new `list` orders by priority and can print a blocked row far above its blocker (run in a scratch clone). What changed: Goal, Done when and Scope: `work blocked`, `work mission` and `work all` became `list --blocked`, `show <mission>` and `list`; the old function names and the shared order doc went; the blocker-first order stays, stated in the Done when, and so do the tests from atelier-31l2 (a mission's items in several states, in `show <mission>`) and atelier-8zxh (more than 20 ready rows, in `list --ready`).
- atelier-5zta: rewrite. Its need survives: the new `list` headings carry no counts (run in a scratch clone). What changed: Goal and Done when: the `work` views' section headings became `list`'s mission and label headings and `show <mission>`'s status groups, each with its count, and the agreement with `status` stays; `work mission`'s summary line went with that view.
- atelier-9bm2: rewrite. Its need survives in part: the new `show <mission>` names no next step for the mission itself (run in a scratch clone). What changed: Goal and Done when: `work mission` became `show <mission>`; narrowed to the mission's own next step, since `show` already prints the Outcome and criteria and missions have no blockers in the new records; a todo mission's `start` became `lead`, still noting items already started.
- atelier-lkuy: rewrite. Its need survives in part: the new `list` orders missions by ID, not by state, and gives no item counts (run in a scratch clone). What changed: Goal, Done when and Scope: `work missions` became `list`'s mission headings; ordering missions by state and counting items by state stay; the "More info" line stays as a pointer to `show <mission>`, the strategy link became the path of the mission's documents in its directory, and the doc line names the design record; the three-line entry went, since `list` already uses one plain line per mission.
- atelier-n831: rewrite. Its need survives in part: the new `status` gives no behind-base count and does not name uncommitted record changes apart from code (review finding atelier-qptk). What changed: Goal and Done when: narrowed to the behind-base count on an item's branch with the `git merge` that catches up, the base's ahead-of-upstream count, the detached commit ID and record changes named apart from code; the branch-in-words and porcelain parts left, since the new `status` already does them; the record path became `.atelier/items/<id>/item.md`.
- atelier-om6l: rewrite. Its need to see the end of each blocker chain and what it holds survives: the new `list` and `show <mission>` name only the direct blocker and its status (run in a scratch clone). What changed: Goal, Done when and Scope: `work mission`, `work human`, `work ready` and `work missions` became `show <mission>`, `list --needs-human`, `status` and `list`; superseded became dropped; the decision type, the `+3 children` count and the parent-inherited blocker test went with types and parents; the Goal quotes the new binary; Scope names the new commands instead of old source files. The Waiting on section, what blockers hold and the mission's Waiting on line stay.
- atelier-oxnw: rewrite. Its need survives in part: the new `start` refusal lists each blocker's ID and status but not its title (run in a scratch clone). What changed: Goal and Done when: narrowed to naming each blocker's title in a refused `start`, since the new refusal already prints one line per condition; `--verbose`, `issue transition` and the parent case went.
- atelier-pu2i: rewrite. Its need survives in part: the new `list` shows only open work but never says how many done items it hid or how to see them (review finding atelier-p4v4, checked in a scratch clone). What changed: Goal, Done when and Scope: `issue list` became `list`; narrowed to a last line counting the done and dropped items `list` leaves out and naming `list --done`, since `list` already shows open work only and has no padding block; the truncation line went, since `list` does not cut rows.
- atelier-tf9n: rewrite. Its need survives in part: the new `list` names each row's direct blockers but not where a longer chain ends (run in a scratch clone). What changed: Goal and Done when: narrowed to naming a longer chain's far end on `list` and `show` rows, since the new `list` already names direct blockers and prints no count or "readiness gate"; `work mission`, `work blocked` and `issue show` became `list` and `show`; parents went.
- atelier-xw8r: rewrite. Its need survives in part: the new `start <mission>` points at `status` without saying it is a mission or naming its items (run in a scratch clone). What changed: Goal and Done when: narrowed to `start <mission>` saying it is a mission and naming the items that can start; the `issue transition` hint and warning lines went with that command.
- atelier-501y: rewrite. Its need survives in part: the new `start` prints the title and Done when but not the mission (run in a scratch clone). What changed: Goal and Done when: narrowed to naming the mission, since the new `start` already prints the title and Done when; `-q` and the parent case went, since the new tool has no quiet form and no parents.
- atelier-5qep: rewrite. Its need for several values in one filter survives: the new `list --label` takes one value, and a comma list matches nothing (run in a scratch clone). What changed: Goal and Done when: `issue list` became `list`; the Goal quotes the new binary; repeated or comma-separated `--label` values ORed and an empty result that names the filters stay; `--open`, `--status`, `--category` and `--issue-type` and their errors went, since `list` shows open work by default and has none of those flags.
- atelier-jcki: rewrite. Its need survives in part: the new `show` prints each review's verdict but no stale mark on an approval the head has outgrown (review finding atelier-1d9u). What changed: Goal and Done when: `issue show` became `show`; narrowed to a stale mark on an approval the head has outgrown, judged as `merge` judges it, since `show` already prints the mission and each review; the branch line and the child-of-a-parent case went with the branch field and parents.
- atelier-8bve: rewrite. Its need survives in part: the new `new` names the status and next step but not the title or priority it wrote (run in a scratch clone). What changed: Goal and Done when: `issue create` became `new`; narrowed to naming the title and priority in the confirmation and to a next step that puts an item with no mission into one (`set --mission`, once `issue link --role advances`), since `new` already names the status and the `ready` step; `--parent`, the type and the unpersisted-ID error went with parents, types and the old command.
- atelier-eb71: rewrite. Its need survives in part: the new binary prints no backtrace, but CI still sets `RUST_BACKTRACE: 0` (`.github/workflows/ci.yml`). What changed: Goal and Done when: narrowed to running the new workspace's tests under `RUST_BACKTRACE=1` and removing CI's `RUST_BACKTRACE: 0`, since the new binary already prints no backtrace.
- atelier-gg0m: rewrite. Its need survives in part: the new `status` outside an Atelier repository prints one plain line but exits 2 (run in a scratch directory). What changed: Goal and Done when: `status --hook claude` became the hook's plain `atelier status`; narrowed to one plain line and exit 0, since the double period and the `check` advice are gone.
- atelier-h90a: rewrite. Its need survives in part: the new `setup claude` says it keeps other hooks but not which, and gives no commit command (run in a scratch clone). What changed: Goal and Done when: narrowed to listing the kept handlers and ending with the commit command; the unchanged-run line left, since the new `setup claude` already says `nothing changed`; `prime` and `land` became the new hooks' `status` and `check`.
- atelier-jjvc: rewrite. Its need survives in another form: the new binary has no `prime`, but `check` says nothing of old hooks that call removed commands (run in a scratch clone). What changed: Goal and Done when: the alias's removal left, since the new binary has no `prime`; what remains is `check` naming a `.claude/settings.json` hook that calls a removed command or flag, now including `status --hook` and `land --hook`; the CHANGELOG line stays, naming `status` as the replacement; the plan condition went.
- atelier-k55o: rewrite. Its need survives: the new `new` writes empty Goal and Done when sections and `ready` and `check` accept them (run in a scratch clone), so the refusal moves from `issue create` to `ready`. What changed: Goal and Done when: `issue create` became `atelier ready`, which must refuse an item with empty Goal or Done when, since `new` writes empty sections by design (shown in a scratch clone: `new`, then `ready`, then `check` all pass on an empty item); the decision type and `--body` example went with the removed command. A sentence in Context says so.
- atelier-7ebi: rewrite. Its need survives for `set`, which confirms `Set the fields of item <id>` without the values (run in a scratch clone); `issue note` and `issue link` do not exist in the new tool. What changed: Goal and Done when: `issue update` became `set`, with the observed output of `set` as the new evidence; the `issue note` and `issue link` bullets and lines went, since the new tool has neither; removing a label that is not set became giving a field its current value, since `--labels` replaces the set. A new Context says so.
- atelier-bp3s: rewrite. Its need, a merge check that runs each repository's suite, survives; `workflow.yaml`'s `merge_checks` becomes `config.toml`'s merge checks. What changed: Goal, Done when and Context: `.atelier/workflow.yaml`'s `merge_checks` `shell.check` became a merge check named in `.atelier/config.toml`; the sentence about `merge --sync` rerunning suite receipts went; its pick-up point became the cutover (atelier-wd1i).
- atelier-e95f: rewrite. Its need, slices of work a repository defines, survives; the built-in views it names become `list` and `show`, and evidence freshness and merge readiness leave its list of derived states. What changed: Goal and Done when: work views became views of `atelier list`; `work ready` and `work mission` became `list --ready` and `show <mission>`; the ADR became the design record; evidence freshness and merge readiness left the derived states.
- atelier-fpxc: rewrite. Its need, finding an item by text, survives; `issue list` becomes `list`. What changed: Goal, Done when and Scope: `issue list --search` became `atelier list --search`, and the test's `--status` became `--label`, since `list` has no status filter.
- atelier-nl9a: rewrite. Its need, CI noticing an unreviewed commit already on the base, survives; `land --since` becomes the pre-push hook's base rule run over a range. What changed: Goal, Done when, Scope, Context and Unknowns: `atelier land --since <commit>` became the pre-push hook's base rule (`atelier check --hook pre-push`) run in CI over the pushed range; a passing range is planning commits and Atelier merges, `--trivial` ones included; the light path became `start --new`; the unknown about `land_work_untraceable` went with `land`.

### Kept as they are

- atelier-9hkr: as is. It a deferred finding on the live skill's opening, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-c24v: as is. It a deferred finding on the live skill's description, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-gobs: as is. It a deferred finding on the live skill's opening and install procedure, which names no removed command; step A (atelier-orb2) writes the new skill and may absorb it.
- atelier-hzuk: as is. It README install documentation for pinned releases, whose Goal and Done when name no removed command.
- atelier-jxce: as is. It an eval of the skill that names no removed command; the new tool keeps fix-now findings and their exits.
- atelier-jy8k: as is. It scheduling the gardener survives the cutover; mission atelier-295u rewrites the gardener it will run.
- atelier-pju0: as is. It rebuilding an eval fixture names no removed command; it is jxce's work.
- atelier-t3ko: as is. The product specification keeps `init` ("plus `init` and `setup`") and the new binary has none yet, so its need waits for that `init`.
- atelier-wl6u: as is. It porting two eval fixtures names no removed command; step B (atelier-t747) ports the scenarios to the new binary and may take it.
- atelier-zjgy: as is. It why tracker-only commits are so many is still a question under the new tool, and its Goal and Done when name no removed command.

### Left for Sumeet

Each line gives the fate this sort proposes. The 72 protected records come first, grouped so they can be read in a few minutes; none was changed.


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
- atelier-73vx: left for Sumeet; proposed rewrite, moved to atelier-z4cr. A metrics page by model and role is the Desk's ("a scorecard page is the Desk's", product specification, Out of scope); its Goal names "the site".
- atelier-wo22: left for Sumeet; proposed rewrite, moved to atelier-z4cr. Its check of who-did-what runs on the Desk's page, not the old site, and its receipt line names evidence records, which are removed.
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

#### Old-tool records, debrief and site: drop

- atelier-jdpf: left for Sumeet; proposed drop (a). It reviews the old `debrief` command and site page, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-k7vp: left for Sumeet; proposed drop (a). It changes how the debrief credits criteria, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-les9: left for Sumeet; proposed drop (a). It changes the accessibility of the site's debrief page, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-okfy: left for Sumeet; proposed drop (a). It adds the mission Report and item notes to the debrief, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-rdsa: left for Sumeet; proposed drop (a). It fixes links and labels on the site's debrief page, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-wg6p: left for Sumeet; proposed drop (a). It changes the debrief's chips, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-ztuq: left for Sumeet; proposed drop (a). It tidies small debrief inaccuracies, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief. Protected because commit 775afd05 of 2026-11-09 changed its body.
- atelier-w4ef: left for Sumeet; proposed drop (c). `check <id>` is gone, and its placeholder rule duplicates the rewritten atelier-k55o, which makes `ready` refuse empty sections.
- atelier-2yef: left for Sumeet; proposed drop (b). Where a declined started item's record goes is settled by the specification: dropping is final and happens on the base, and every command reads the base's record first (product specification, Lifecycle: When a thread dies).
- atelier-pg2b: left for Sumeet; proposed drop. Already done: atelier-nz6a made install.md give the install lines (atelier-8i86, G14), and the skill no longer lives in spadaval/.agents; none of (a) to (c) names this case.
- atelier-c6g2: left for Sumeet; proposed drop (a). It teaches the skill `work mission --since` and the old references/atelier.md Administration section, and its blockers atelier-avx0 and atelier-fb1i were dropped in this sort.
- atelier-5lv9: left for Sumeet; proposed drop (a). It changes the light-path review brief, and the What is removed table removes the light path.
- atelier-ayrg: left for Sumeet; proposed drop (a). It changes the debrief's counts, and the What is removed table removes the `debrief` command and the site; the Desk (mission atelier-z4cr) shows the debrief.
- atelier-lr08: left for Sumeet; proposed drop (a). It speeds up the `debrief` command, which the What is removed table removes.
- atelier-3km4: left for Sumeet; proposed drop (a). A taste read of the old site's debrief page, which the What is removed table removes; the Desk (mission atelier-z4cr) gets its own.
- atelier-pywh: left for Sumeet; proposed drop (a). A taste read of the old fleet site, which the What is removed table removes with `serve`; the Desk (mission atelier-z4cr) gets its own.
- atelier-dd3a: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its children atelier-lkf5 and atelier-m2v1 stand alone.
- atelier-rqzu: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its open child atelier-2yef stands alone.
- atelier-zmei: left for Sumeet; proposed drop (a). A container of deferred findings with no work of its own, and the What is removed table removes parents; its child atelier-c1sr stands alone.

#### Questions for Sumeet: close with this Report

- atelier-8i86: left for Sumeet; proposed close with this Report. Most of its 21 proposals concern records this sort dropped or rewrote; answer it by pointing at this Report and close it.
- atelier-j86z: left for Sumeet; proposed close with this Report. Eight of its ten recommendations change old commands or old files that the cutover removes; carry R6 (grep for stale docs as a proof step, for step A) and R8 (`check` warns on trailing whitespace) into new items if still wanted, and close it.

#### Not protected, left for Sumeet

- atelier-w0su: left for Sumeet. Its Design section is built on the old model throughout (`workflow_branch`, `branch_base`, `land`, `prime`), so a rewrite of names alone would leave it incoherent; proposal: drop, and refile against the new `start` and `merge` if parallel missions still need it.
- atelier-g6rp: left for Sumeet. It an open mission, which this item does not change; see Missions.
- atelier-4pzj: left for Sumeet. It an open mission, which this item does not change; see Missions.
- atelier-uf97: left for Sumeet. It an open mission, which this item does not change; see Missions.

### Missions

Missions are not dropped or rewritten here. Open items are counted at fc0fe395 by this sort's fate at the head; protected items are counted by the fate proposed for them. An item of a draft mission cannot start under the new tool.

- atelier-qr5s (todo, converts as ready): keep. 11 open items, all protected and proposed as is. At fc0fe395, 6 were ready (todo): atelier-1oel, atelier-5zre, atelier-mlwx, atelier-oci2, atelier-orb2 and atelier-yudj. Under the new tool, 4 of them could start (atelier-1oel, atelier-mlwx, atelier-orb2, atelier-yudj); atelier-oci2 waits on its blocker atelier-mlwx, and atelier-5zre is this item, now in progress. A ready mission is needed for its items to start, not enough.
- atelier-295u (draft): keep. 4 open items, all protected and proposed as is; none is ready.
- atelier-z4cr (draft): keep, and take in atelier-aab1, atelier-73vx and atelier-wo22 from atelier-bnek. 7 open items, all protected and proposed as is. Its 2 ready items (atelier-1ie1, atelier-psjw) cannot start until the mission is made ready.
- atelier-bnek (draft): fold into atelier-z4cr. 6 open items, all protected: 3 proposed to move to atelier-z4cr (atelier-aab1 as is; atelier-73vx and atelier-wo22 rewritten for the Desk) and 3 proposed to drop (atelier-7yy9, atelier-00hl, atelier-5kwk). Its 4 ready items (atelier-00hl, atelier-5kwk, atelier-7yy9, atelier-aab1) cannot start while it is a draft.
- atelier-icv4 (draft): drop. 5 open items, all protected and proposed to drop: each is met by the new commands or names removed ones. Its 5 ready items cannot start while it is a draft. Its loose end atelier-zno7 is proposed to drop with it.
- atelier-4pzj (draft): keep, with its 13 rewritten items. 33 open items: 20 dropped on this branch and 13 rewritten (atelier-5qep, atelier-om6l, atelier-501y, atelier-xw8r, atelier-9bm2, atelier-tf9n, atelier-jcki, atelier-oxnw, atelier-n831, atelier-5qes, atelier-5zta, atelier-lkuy, atelier-pu2i), all about what the new `status`, `list`, `show` and `start` still leave out. None is ready. Its outcome, seeing where things stand, still holds for the new commands. If Sumeet drops it instead, take the 13 rewritten items out of it first (`set --mission ""` after the cutover), or they stay drafts in a dropped mission.
- atelier-uf97 (draft): drop. 27 open items: 20 dropped on this branch, 6 rewritten (atelier-k55o, atelier-7ebi, atelier-kset, atelier-8bve, atelier-x3ru, atelier-sfd9) and 1 protected (atelier-w4ef, proposed to drop as a duplicate of atelier-k55o). None is ready. Its bundle and record-writing half is gone; the rewritten items are small `check`, `set`, `new` and `ready` fixes. If it is dropped, take the 6 rewritten items out of it first (`set --mission ""` after the cutover), or they stay drafts in a dropped mission.
- atelier-g6rp (todo, converts as ready): keep only to close it. It has no open items: all 8 are done and its last wait, atelier-tee8, is done. Close it with a Result before the cutover; if it is still open then, drop it.

### Counts for the cutover

Facts at the branch head, changing nothing (`~/.cache/atelier-qr5s/5zre/counts.py` and `deferred.py`):

- Ready items that belong to a draft mission: 11 of 28 ready items: atelier-00hl, atelier-06jc, atelier-1ie1, atelier-5kwk, atelier-5xvl, atelier-7yy9, atelier-aab1, atelier-jqzl, atelier-psjw, atelier-tvfd and atelier-zzwi. The new binary's `status` in the scratch clone agrees: "11 are in a mission that is not ready".
- Ready items whose Done when has no checkbox line, which the converter will write as one unticked line (atelier-yudj): 3: atelier-3km4 (prose Done when), atelier-8i86 and atelier-j86z (decisions with no Done when section).
- Deferred findings on done records with no follow-up item: 0. Of the 126 deferred findings on done records, every one names a follow-up: 73 are done, 11 are still in the backlog, and 42 were superseded or declined, including follow-ups this sort dropped.

### Links to dropped records

`atelier check` passes at the head. Four open records link to a dropped one, and each is right:

- atelier-c6g2 (protected) is blocked by atelier-avx0 and atelier-fb1i. It exists only to teach the skill what those two add, so it is proposed to drop with them.
- atelier-9hkr and atelier-gobs are children of atelier-hu64, and atelier-c24v of atelier-jtsn. The parents were containers for deferred findings with no work of their own; parents are removed in the new tool, and the converter makes each child a standalone item.

### Conversion check

Run by `~/.cache/atelier-qr5s/5zre/convcheck.sh` at the branch head, in a scratch clone under `~/.cache/atelier-qr5s/5zre/conv`, with `TMPDIR` under `~/.cache`:

```text
git clone -b atelier-5zre <worktree> conv && git branch -m master
python3 scripts/cutover/convert_records.py
git add -A && git commit -m "Convert Atelier records to the new layout"
python3 scripts/cutover/convert_records.py --verify <pre-conversion head>
atelier-next-3ba4a62f check
```

Output at 8a46b3c9. Its parent 1a9cb68a is the last commit that changes a record other than this one; later commits change only this record:

```text
Records converted, by kind and new status:
  item draft: 76
  item from decision ready: 2
  item ready: 27
  mission draft: 6
  mission ready: 2
Made ready from in_progress; each branch's work is not converted:
  atelier-5zre
This tree's .atelier/ and moved files match a fresh conversion of 8a46b3c9fbcb74c190cdbe8ac8d8e5d4610a5b2a: 127 files.
Every record is valid: 105 items, 8 missions and config.toml.
```

The converter printed 105 items (76 draft, 27 ready, 2 ready from decisions) and 8 missions (6 draft, 2 ready); `--verify` and `check` each exited 0. 105 items and 8 missions are the 181 open records less the 68 dropped. The full output is in `~/.cache/atelier-qr5s/5zre/convout.txt`; it also lists 21 `blocked_by` entries naming a closed or missing record, which the converter leaves out.

### Deviations

- The first pass was not delegated to GPT-6 Luna. I read all 181 records myself, since every drop and rewrite needed checking by hand anyway.
- Claims that the new binary already does what a record asks were checked by running the build of `next/` at `~/.cache/atelier-qr5s/bin/atelier-next-3ba4a62f` on a converted scratch clone (`~/.cache/atelier-qr5s/5zre/exp`), never in a real repository.

### Decisions

- A drop is `supersede` where the specification names a replacement (the `work` views, `land`, the debrief, an existing command) or another record covers it, and `decline` where the thing is simply removed (`bundle`, evidence, `prune`, `migrate`, parents).
- atelier-w0su is left for Sumeet, not rewritten: its Design section is built on the old model throughout, so changing names alone would leave it incoherent.
- atelier-t3ko stays as is although the new binary has no `init`: the product specification keeps `init`, so its need waits for one.

### Open items

- Proposals for the 72 protected records, the three unprotected missions and atelier-w0su are Sumeet's to decide.
- Master moved after fc0fe395; records created there since are not in this list.

### Review

Round 1 (GPT-6.1 Sol, at 2393521a) recorded four `fix_now` findings, each fixed in its own commit:

- atelier-ax6k: atelier-5qep was dropped under (b) though the new `list` takes one `--label`. Fixed: restored from fc0fe395 and rewritten to the part that survives.
- atelier-zs8v: atelier-om6l was dropped under (b) though the new views name only direct blockers. Fixed: restored and rewritten for `show <mission>` and `list`.
- atelier-h72a: atelier-kset was dropped under (b) though two of its faults remain. Fixed: restored and rewritten to those two.
- atelier-shpk: the qr5s proposal said all five ready items could start, though atelier-oci2 waits on atelier-mlwx. Fixed in Missions above, which now counts at fc0fe395.

The same flaw, a (b) drop of a record the new binary meets only in part, was looked for in the other (b) drops by running each claim again. Six more were found and restored the same way: atelier-8bve, atelier-h90a, atelier-501y, atelier-xw8r, atelier-9bm2 and atelier-tf9n. Restoring a record is the recovery the item's Context names: its file is checked out from fc0fe395 on this branch, then its Goal and Done when are rewritten.

Round 2 (GPT-6.1 Sol, at b8f5a1ea) recorded five `fix_now` findings, each fixed in its own commit:

- atelier-1d9u: atelier-jcki was dropped under (b) though the new `show` marks no stale approval. Fixed: restored and rewritten to that part.
- atelier-3ww2: atelier-x3ru was dropped under (a) though a `blocked_by` cycle still passes the new `check`. Fixed: restored and rewritten to the cycle diagnostic.
- atelier-s2h3: atelier-oxnw was dropped under (b) though a refused `start` names no blocker title. Fixed: restored and rewritten to that part.
- atelier-qptk: atelier-n831 was dropped under (b) though `status` gives no behind-base count and does not name record changes apart from code. Fixed: restored and rewritten to those parts.
- atelier-k6yq: the deferred follow-up breakdown was stale. Fixed in Counts for the cutover, recounted at the head.

Every remaining drop was then checked again, one Done when line at a time, against the new binary or a specification line: a drop stands only where each line is met, or its subject does not exist in the new tool. Seven more were restored and rewritten the same way: atelier-5qes, atelier-5zta and atelier-lkuy (the `list` order and headings), atelier-gg0m (exit status outside a repository), atelier-eb71 (CI's backtrace workaround), atelier-sfd9 (the edit that removes a missing link) and atelier-jjvc (old hooks that call removed commands).

Round 3 (GPT-6.1 Sol, at 8c68c66c) recorded one `fix_now` finding and otherwise found the sort fit to merge:

- atelier-p4v4: atelier-pu2i was dropped under (b) though `list` never counts the done items it hides. Fixed: restored and rewritten to that last line.

Three rounds ended without an approval, so the fix went to the orchestrator, who decided on a fourth round (decision note of 2026-11-11).

Round 4 (GPT-6.1 Sol, at 073ac412) recorded one `fix_now` finding:

- atelier-brmd: seven records this branch dropped were protected, since commit 775afd05 of 2026-11-09 changed each one's body: atelier-jdpf, atelier-k7vp, atelier-les9, atelier-okfy, atelier-rdsa, atelier-wg6p and atelier-ztuq. The sort's first protected-set check ran one `git log --since` over the whole `.atelier/issues/` directory, which did not list that commit; run file by file, it names these seven and no others. Fixed: each file restored byte for byte from fc0fe395, and each is now left for Sumeet with its proposed drop.

Round 5 (GPT-6.1 Sol, at 24b518dd) recorded one `fix_now` finding:

- atelier-864b: the rewrite of atelier-n831 lost two of its surviving requirements, the base's ahead-of-upstream count and a detached HEAD's commit ID, which the new `status` does not show. Fixed: both are back in its Done when, with their test cases. Counts are unchanged.

Round 5 did not approve, so under the orchestrator's decision this fix goes back to the orchestrator unreviewed.

Before round 6, each of the 27 rewrites' Done when was compared line by line with its text at fc0fe395, as the drops were after round 2: every requirement had to be kept, met by the new binary, or about something the new tool does not have. Seven had lost one, now put back: atelier-8bve (a next step that puts an item with no mission into one), atelier-9bm2 (noting items already started), atelier-n831 (the `git merge` that catches up), atelier-5qes (the tests from atelier-31l2 and atelier-8zxh), atelier-5zta (counts on `show <mission>`'s status groups), atelier-lkuy (the pointer to `show <mission>`, and the design record describing the heading) and atelier-jjvc (the CHANGELOG line). The other 20 keep every requirement the new tool does not meet.

Round 6 (GPT-6.1 Sol, at f24fefd6) recorded one `fix_now` finding:

- atelier-w8mt: the rewrite of atelier-lkuy dropped its requirement to show the mission's strategy, on the reasoning that the documents moved into the mission's directory; the new `list` and `show <mission>` print no path to them. Fixed: the heading now names the path of the mission's documents, such as `.atelier/missions/atelier-qr5s/design.md`. Counts are unchanged.
