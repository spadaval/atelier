---
created_at: "2026-11-07T05:00:32.018996016+00:00"
id: "atelier-mzpn"
issue_type: "decision"
labels:
- "needs-human"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Which items of the six draft missions does the simpler lifecycle supersede, revise or keep?"
updated_at: "2026-11-07T05:52:22.228799928+00:00"
---

## Context

docs/strategies/simpler-lifecycle.md (Scope and Authority: superseding draft-mission items is Sumeet's call). Until this is decided, none of those missions should start.

## Question

Six draft missions (atelier-2mky, atelier-4pzj, atelier-gg6b, atelier-l63c, atelier-ll3p, atelier-uf97) hold over a hundred backlog items planned under the old model. Some contradict the simpler lifecycle: atelier-jzh9 records implementers to strengthen the reviewer-identity check this mission removes, atelier-bywq warns about base edits to branch-owned records that will no longer make approvals stale, atelier-4ppu reports what a done claim needs at close, a step that goes away. Which items should be superseded, revised to fit, or kept as they are, and should any of the six missions be superseded as a whole?

## Options

- The orchestrator prepares a triage table as a note on this record: each item marked supersede, revise (with the change) or keep, with a one-line reason. Sumeet decides from the table.
- Supersede the six missions wholesale and re-plan later from scratch.

## Notes

- 2026-11-07 [result] Triage table prepared by an independent audit agent against the simpler-lifecycle strategy (master bc8bee34), for Sumeet to decide from.

  Prepared 2026-11-07 against master `bc8bee34` and `docs/strategies/simpler-lifecycle.md`. Read-only: no record was changed.

  **109 open items: 23 supersede, 33 revise, 53 keep.**

  | Mission | Open | Supersede | Revise | Keep | Recommendation |
  |---|---|---|---|---|---|
  | atelier-2mky adoption and upkeep | 18 | 2 | 7 | 9 | Merge its check items into uf97; adoption goes to mmpv's l457 |
  | atelier-4pzj choosing work | 19 | 5 | 4 | 10 | Keep with fewer items, and take in l63c |
  | atelier-gg6b debrief | 14 | 4 | 3 | 7 | Supersede as a whole |
  | atelier-l63c where things stand | 24 | 3 | 11 | 10 | Merge into 4pzj |
  | atelier-ll3p gate shortcuts | 7 | 4 | 3 | 0 | Supersede as a whole |
  | atelier-uf97 writing records | 27 | 5 | 5 | 17 | Keep with fewer items |

  ## How I judged

  - **supersede**: mmpv removes the item's premise, one of mmpv's items does the work, or the item adds a view, record or command that fails the strategy's earn-its-place test. The premises mmpv removes are: the close step, evidence gates, approvals made stale by record edits, the recorded base and type-named branches, the reviewer-identity rule, `fix_now` dispositions, the deferral source rule and after-merge checks.
  - **revise**: still wanted, but it names a removed concept or belongs folded into another item.
  - **keep**: output that exists today is wrong or misleading, or a view disagrees with what a command accepts, and mmpv does not touch it. Keep says nothing about priority: most keeps are P2/P3 polish. Additive features had to earn their place.

  The mmpv items named below are: 3jdm (approvals cover code), jnrh (one order, merge marks done, no close), 8bh4 (loose ends), ilba (invalid records refused at write, authors fix their own), md0v (records and gates review doesn't need are cut), 4nsl (light path), l457 (adopt and upgrade), ml3r (say the lifecycle once).

  I spot-checked these against master:

  - `issue list` still sorts by ID with done records first (pu2i).
  - `work missions` already prints "More info" and omits zero buckets (lkuy is partly done), but still prints "Open blockers: 8" for mmpv.
  - Needs-human rows no longer say "shown for context" (46nq is partly done).
  - `decide` still requires close_reason beside the Decision section (49eo still holds).
  - The starter and repo policies have no `blocked` status (the examples in e6fl and bywq are stale).
  - `start` on a backlog item still prints "(no start transition…)" (hugc).
  - `work.rs:158` still pushes "readiness gate" as a blocker (tf9n, b17k).
  - Init does no branch detection (0vp3).

  ## atelier-2mky: adopting and maintaining Atelier (18)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-0vp3 | Detect the base branch at init; fix land_base_mismatch | supersede | md0v drops the recorded base, so land_base_mismatch goes; l457 makes the init policy merge as written. Carry "detect master vs main" into l457's Done when. |
  | atelier-gg0m | Hook cwd outside a repo is a misconfiguration | supersede | md0v makes the Stop hook report, refusing only unreviewed code on the base; add the outside-repo payload to md0v's hook tests. |
  | atelier-6tmk | Handle repos without Git or without commits | revise | Fold into l457, since an unborn HEAD right after init is the first step of adoption; drop `prime` (sdb0 retired it). |
  | atelier-t3ko | Init reports what it did and the real next steps | revise | Next steps should land the install by the light path (4nsl) and state the new order, not `land --install`; do it inside l457. |
  | atelier-hfze | A view of the workflow policy | revise | Wait until jnrh, md0v and 8bh4 shrink the policy, then make it part of ml3r's CLI help (`atelier help workflow`), or drop it if the smaller YAML reads plainly. |
  | atelier-w4ef | check and check \<id\> agree on missing sections | revise | Fold into ilba: write-time validation needs one rule set, so the placeholder rule that now runs only for `check <id>` must run everywhere. |
  | atelier-sfd9 | Repair route for links to records that never existed | revise | ilba refuses such links at write. Narrow this to the repair still needed for links that arrive by merge or hand edit: unlink works on a graph that won't load, and the finding names the file and field. |
  | atelier-ry7d | Report an invalid status or type once | revise | Same defect as kset; merge into it. |
  | atelier-4t5l | check confirms what it checked | revise | Merge into kset as one check-output item. |
  | atelier-fb1i | Graph errors even when a record fails to parse | keep | Hand edits and merges still produce unparsable records that ilba can't refuse. One run that names every fault is the repair path the 2026-11-05 stall lacked. |
  | atelier-w4j2 | check calls other record kinds corrupt | keep | A review ID or an unknown ID gets a false corruption report. |
  | atelier-kset | Each check finding once, on one line, on its own record | keep | Duplicated, mislabelled findings slow every repair; absorbs ry7d and 4t5l. |
  | atelier-pxyz | Prune's Next hint only when apply would act | keep | The hint offers an apply that then refuses or does nothing. |
  | atelier-rmvb | Prune rows on one line | keep | Rows run to several paragraphs; merge with rp1c and 08rc into one prune-output item. |
  | atelier-rp1c | Prune summary: eligible vs kept, by root | keep | The reader can't tell why records are kept; part of the same prune item. |
  | atelier-08rc | prune --apply ends with the commit step and recovery | keep | The docs require a commit that apply never mentions; part of the same prune item. |
  | atelier-hzuk | Document installing a pinned release | keep | README only; makes eval runs comparable. Update the version it names. |
  | atelier-h90a | setup claude reports unchanged runs and foreign hooks | keep | Low priority: setup adds duplicate hooks without saying so. Check the wording against the sdb0 status hook. |

  ## atelier-4pzj: choosing work (19 open; 3 already superseded)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-e151 | start creates the worktree, or shows the owner branch | supersede | md0v drops type-named branches, so there is no branch name to guess. After md0v, check that a child's start is still not refused. |
  | atelier-cy58 | Mission and parent on every row | supersede | Fails earn-its-place: a suffix on every row for two observed calls, and loose ends would make "· no mission" common noise. |
  | atelier-5q5v | Parent/child structure in work mission | supersede | Fails earn-its-place: `work parent` already shows children, and jnrh drops the parent-closes-last rule that made structure matter. |
  | atelier-7emi | Show retired missions and stranded work | supersede | Rare. status already lists unassigned work, and a triage that retires a mission re-homes its items. |
  | atelier-fdxj | Blocker chains and unblock ranking | supersede | Sumeet already left it out (note of 2026-10-17); om6l covers what holds work up. |
  | atelier-mi3x | status Next Actions from checkout state | revise | Rewrite its ordered list to the new lifecycle: once Done when is ticked, `review request`; once approved, `merge`; drop the done-owner and land-handoff steps. Could go into ml3r. |
  | atelier-501y | Start success shows title, Done when, mission | revise | Drop the recorded-base warning (md0v). Keep the title and Done when, which feed the tick-every-line step. |
  | atelier-46nq | Needs a Human rows say what they hold | revise | The "shown for context" label is already gone on master; fold the remaining "holds N" into om6l. |
  | atelier-9bm2 | Outcome, blockers and next step in the mission header | revise | Keep the Outcome line; derive Next from jnrh's mission-close rule once it lands. |
  | atelier-hueh | Judge ready work on the records start checks | keep | Views offer a start that start then refuses; this is the core of ready-means-startable. |
  | atelier-hugc | Refuse start on parked items | keep | On master, start still creates a branch for a backlog item. |
  | atelier-mfyd | Ready count net of in-flight items | keep | The headline count overstated 3 of 5 in 1w8c's validation; small change. |
  | atelier-oxnw | Refused start: one line per blocker | keep | A 17-line refusal that still sends the agent to `issue show`. |
  | atelier-xw8r | Start hints point at `atelier start` | keep | The hints lead to the record-only transition (63 vs 31 uses in astromech); fixing them serves the one order directly. |
  | atelier-tf9n | Direct blockers on rows; drop the count | keep | Rows disagree with `issue show` and can show a fake "readiness gate" blocker; do with b17k. |
  | atelier-b17k | Remove or justify the second readiness gate | keep | Deletes code and a scan of every record. |
  | atelier-l9ty | Mark blocked and needs-human items in issue list | keep | Named by the ready-means-startable criterion; agents filter `issue list` for work. |
  | atelier-5qes | Blocker-first order in every work view | keep | Low priority: the views contradict the documented order. |
  | atelier-om6l | "Waiting on" section in work mission | keep | One place for what holds a mission up; absorbs 46nq's "holds N" and lkuy's Waiting on line. Close call (judgment call 3). |

  ## atelier-gg6b: post-mission review from the debrief (14)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-930x | Show every receipt in a validation run | supersede | md0v makes receipts optional notes and 8bh4 takes validation off its own branch. The Validation section gets redone, so check it then. |
  | atelier-idx3 | A receipt's full summary and source in cards | supersede | Same reason: receipt detail on cards doesn't earn its place once receipts gate nothing. |
  | atelier-r8om | Tell planning reviews from delivery reviews | supersede | Needs a new record field, which fails the no-regrowth rule; reviews of plan records become ordinary reviews under 3jdm and 4nsl. |
  | atelier-j0b9 | Planned-scope edge cases | supersede | It patches edges of mgl0's deferral rule, which md0v (deferral source rule) and 8bh4 reshape: a rule patching a rule. |
  | atelier-okfy | The mission's close report and item notes | revise | Show whatever report jnrh keeps (written before review, carried into the merge) and how missions close under jnrh. |
  | atelier-rdsa | Tidy Results and validation links | revise | Drop the records-only trivial-waiver nit (3jdm) and the receipt-chip nits (md0v); keep the link targets and header labels. |
  | atelier-ztuq | Small debrief inaccuracies | revise | Hold the "Delivered after this validation" nit until 8bh4 settles validation; keep the raw `### Outcome`, zero-count and year fixes. |
  | atelier-j79t | Keep Open items when follow-ups were filed | keep | Hides what an implementer left open; doesn't depend on the lifecycle. |
  | atelier-k7vp | Credit a criterion to work whose Done when names it | keep | Criteria survive mmpv; low priority. |
  | atelier-wg6p | No chips inside paths and branch names | keep | Rendering bug. |
  | atelier-1hbk | Name every part a collapsed entry hides | keep | Rendering bug. Its example, a tag-push check, would now be a loose end. |
  | atelier-g21n | Drop a Decisions part that only points at notes | keep | Shows the same decision twice. Re-check if jnrh changes the report parts. |
  | atelier-les9 | Accessibility beyond contrast | keep | Rendering; low priority. |
  | atelier-lozn | Verdict links and More toggles visibly interactive | keep | Rendering; low priority. |

  ## atelier-l63c: where things stand (24)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-dvnv | Warn in status when land would fail | supersede | Under md0v, land reports and refuses only unreviewed code on the base. The done-claim and uncommitted-record failures go, and the base case is step 4 of mi3x. |
  | atelier-jbwi | Planned vs built, and backlog health, in work missions | supersede | Fails earn-its-place: it adds a configurable age. Triage like this one handles backlog sprawl, and `work missions` already shows counts per state. |
  | atelier-kglz | Show docs that exist only on unmerged branches | supersede | Fails earn-its-place: one anecdote, and it adds a scan of every branch to status. |
  | atelier-lkuy | Rewrite the Mission Overview entry | revise | Partly done ("More info", zero buckets omitted). Left is Sumeet's own ask: drop "Scope" and "Open blockers" (mmpv shows 8), sort active before todo before draft, show the strategy. |
  | atelier-jcki | Ownership, review state and mission in issue show | revise | Drop the Proof line and `(base master)` (md0v); the Review line follows 3jdm's code-only freshness. |
  | atelier-yzuv | Mission issue show restates scope three times | revise | Also drop the Evidence Gates section, since receipts gate nothing (md0v). |
  | atelier-1a5k | work parent lists the parent as its own row | revise | Drop the Proof gaps bullet (md0v); keep the self-row and count fixes. |
  | atelier-dmh5 | Store the direction of directional links | revise | ihss already stores each link only on the record that declares it. Narrow this to the two places its note names that treat a reversed link as the same one, plus legacy two-sided records. |
  | atelier-t18i | Count links and name cycles the same way | revise | The count goes if yzuv removes the Relationships line; merge the cycle message with uf97's x3ru. |
  | atelier-pvo0 | Name the command for the remaining truncations | revise | Drop the land path lists, which md0v reshapes; keep issue show, the ref suggestions and evidence list. |
  | atelier-oyzb | Fix work mission and parent footers and filters | revise | Choose "summary sections never truncate" over new `--human`/`--backlog` flags; keep the label and hint fixes. |
  | atelier-d2xe | Every scope item, in plain words, in the owner block | revise | Point it at status's owner block instead of prime (sdb0 is done); its review words follow md0v's smaller vocabulary. |
  | atelier-lehs | Drop or fix Impact in issue show | revise | Pick option (a): delete the section and `downstream_impact`, which nothing reads. |
  | atelier-majk | Recent tracker activity at session start | revise | Merge into avx0, whose `--since <commit>` answers this item's open question of what "recent" means. |
  | atelier-pu2i | Open work before done work in issue list | keep | On master the default list is still in ID order with done records first, and open records past row 50 are hidden. |
  | atelier-o1bv | Ignore COLUMNS when output is redirected | keep | Breaks the one-line-per-item contract agents rely on. |
  | atelier-x0vs | Show each mission decision once in show --full | keep | Doubled decisions read as two decisions. |
  | atelier-n831 | Branch and checkout facts in words | keep | Prints git porcelain codes, doesn't split records from code, and gives no behind-base count. |
  | atelier-1ci8 | Show active work once in status | keep | Check first: master's status no longer prints `Current work: N`, so this may be partly fixed. |
  | atelier-8g9d | Name the missions behind status's count | keep | status still prints a bare `Current missions: 7`. |
  | atelier-5zta | Counts in section headings | keep | The count line disagrees with truncated rows; the fix deletes a summary line. |
  | atelier-5qep | Several values and --open in issue list filters | keep | Agents tried `-s open`; small change. |
  | atelier-d4b6 | Classify draft missions consistently | keep | Low priority; take the option where work all leaves missions to work missions. |
  | atelier-avx0 | A since-commit mission digest | keep | Re-reading context at wake-ups was the largest measured cost (about 1.4B cache-read tokens); absorbs majk. Close call (judgment call 4). |

  ## atelier-ll3p: make gate shortcuts visible (7 open; r9hb and 80qd done)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-nmvv | Show a measured fail cleared by assertion | supersede | Under md0v no gate reads receipts, so nothing is passed by assertion. |
  | atelier-jzh9 | Record the implementer for the non-author check | supersede | Contradicted: md0v removes the reviewer-differs-from-authors rule this strengthens (mzpn names it). Decision 80qd's work goes with it. |
  | atelier-efnf | Open fix-now findings in branch readiness | supersede | Under md0v the approval after a `fix_now` finding resolves it, and the extra cases (self-approval, trivial exemption, proof exemptions) test rules mmpv removes. md0v's readiness mark must agree with merge. |
  | atelier-ma4a | Review history where work is judged (parent) | supersede | A container for ll3p's criterion: r9hb is done, efnf goes, and abdp stands alone. |
  | atelier-fad5 | merge --push; stop recommending piped gates | revise | Drop the ll3p receipt clause. Build on jnrh so that `merge --push` is the last step of the lifecycle (judgment call 1). |
  | atelier-nl9a | CI check of base commits pushed without review | revise | A CI notice that costs agents nothing. Restate "reviewed" as 3jdm's approved code, accept 4nsl light-path approvals instead of trivial exemptions, and drop the ll3p receipt (judgment call 2). |
  | atelier-abdp | Count review rounds in readiness and the brief | revise | Unblock it from efnf and show the round count on its own; it shows state rather than gating (judgment call 2). |

  ## atelier-uf97: writing records safely (27)

  | ID | Title | Verdict | Reason |
  |---|---|---|---|
  | atelier-4ppu | Say at close what the done claim still needs | supersede | jnrh removes the close step: review request refuses unticked lines, and merge --check names what is missing (mzpn names it). |
  | atelier-9pa6 | Warn on a done transition on the base | supersede | Under jnrh work items have no close transition, so nobody can make a done claim on the base. |
  | atelier-bywq | Warn on base edits to branch-owned records | supersede | Under 3jdm a record-only conflict resolution no longer makes an approval stale (mzpn names it). |
  | atelier-6esg | Warn when a bundle's evidence commit is absent | supersede | Under md0v a receipt is proof of nothing, so "cannot count as proof" warns about a gate that no longer exists. |
  | atelier-rz2l | Import a drafted record file | supersede | Fails fewer-concepts: a fourth way to write records beside create, bundle and direct edits. sxcj covers the quoting problem and ilba the validation. |
  | atelier-k55o | Refuse work items without a real Done when | revise | Fold the create-time refusal into ilba. Drop the start refusal, but have jnrh treat a Done when with no checkbox lines as unticked. |
  | atelier-guew | bundle preview runs apply's graph checks | revise | Build on ilba: preview runs the same write-time validation, so preview and apply share one rule set. |
  | atelier-cj54 | issue update replaces one section | revise | Drop the jy9t completion-claim clause, since done now means merged; keep the one-section replace. |
  | atelier-e6fl | --reason on transitions that aren't done transitions | revise | The `block` example is stale (no blocked status). Take the refuse option for plan, defer and start. |
  | atelier-6gpl | Transition options, one line each | revise | Redo it after jnrh removes close: work items will have fewer transitions, and the route-finding error is mostly moot. |
  | atelier-sxcj | --body-file in create and update | keep | Matches `--reason-file`, and ends shell-quoted bodies and Python splices. |
  | atelier-7ebi | Confirm update, note and link with the values written | keep | 178 of 572 astromech mutating commands were followed by `issue show`. |
  | atelier-8bve | issue create confirms and suggests the next step | keep | The footer is generic, and its error names an ID that was never written. |
  | atelier-ceu3 | Readable create/update body and type errors | keep | ilba makes refusals more common, so their wording matters more. |
  | atelier-rh0d | Link role defaults and role errors | keep | `unlink` with no role exits 0 having removed nothing. |
  | atelier-49eo | Use the Decision section as decide's reason | keep | Still true: decide requires close_reason beside the Decision section. Removes a duplicate. |
  | atelier-sl01 | Same wording for a transition that needs an argument | keep | supersede still needs --reason; do with e6fl. |
  | atelier-hat4 | evidence --criterion on a mission without criteria | keep | A one-line message fix; criteria survive mmpv. |
  | atelier-0xcr | Bundle contract in bundle --help | keep | Serves "learn it from the CLI help"; adopting repos have no copy of the schema. |
  | atelier-woul | Field-level changes for update entries | keep | An update entry silently dropped a label. |
  | atelier-x3ru | Name bundle entries by client_ref in errors | keep | Errors name staging IDs that were thrown away; absorbs t18i's cycle message. |
  | atelier-z6vj | Bundle errors name the entry | keep | Reused parser messages with garbage tails. |
  | atelier-dpvz | Check the kind of every bundle reference | keep | Preview passes what apply rejects; low priority. |
  | atelier-h0nt | Bundle preview as a tree | keep | Lets a planner check the graph before apply; low priority. |
  | atelier-1oez | Regenerate the preview fixture and test it | keep | Do with h0nt. |
  | atelier-vsh8 | bundle apply -q lists every record written | keep | Low priority. |
  | atelier-0cal | bundle apply's next step | keep | Tiny: drops a redundant check and adds the commit step. |

  ## Per mission

  - **atelier-2mky: merge into uf97 and supersede the mission.** Its adoption half is mmpv's l457: 0vp3, 6tmk and t3ko go there, gg0m goes to md0v and hfze to ml3r. Its check items (fb1i, w4j2, kset with ry7d and 4t5l, sfd9, w4ef) join uf97 as one "write, check and repair records" mission. The prune items, merged into one, plus hzuk and h90a stay as unassigned low-priority backlog.
  - **atelier-4pzj: keep with fewer items (19 to 14) and take in l63c's survivors.** Its ready-means-startable criterion still names a real cost: views that offer starts the CLI refuses. Hold mi3x, 501y and 9bm2 until jnrh and md0v land.
  - **atelier-gg6b: supersede as a whole.** Its items were filed against vt8q's debrief under the old record model. mmpv changes nearly every input the debrief reads: reports, receipts, `fix_now` dispositions, validation and after-merge checks. Leave the 7 keep items unassigned. After mmpv's every-repo criterion is met, run one agent review of a debrief generated under the new model and file what it finds.
  - **atelier-l63c: merge into 4pzj.** The two already share renderers and had to run one after the other, and l63c has no success criteria of its own. 20 items survive once dvnv, jbwi and kglz go and majk merges into avx0. The combined mission is large (about 34 items), so consider leaving the P3 polish unassigned.
  - **atelier-ll3p: supersede as a whole.** Its premise was making shortcuts past gates visible, and mmpv removes most of those gates: staleness from record edits, evidence gates, reviewer identity and close. r9hb already delivered the Done-when part. fad5 and nl9a can stand alone (or fad5 joins mmpv; see judgment call 1), and abdp joins the views mission.
  - **atelier-uf97: keep with fewer items (27 to 22, plus 2mky's check items).** Drop the S6 criterion: it is a measurement after shipping, at most a loose end now. Narrow refuse-then-confirm to "confirms what it wrote, and preview refuses what apply refuses", because ilba now owns refusing at write.

  ## Additions to mmpv items

  None of these adds a concept, so they fall within the orchestrator's authority.

  - **l457**: detect the base branch at init (0vp3); handle a directory without Git and an unborn HEAD (6tmk); give init next steps in the new lifecycle (t3ko).
  - **ilba**: put the placeholder Goal/Done when rule in write-time validation, with one rule set for `check` and `check <id>` (k55o, w4ef); make that validation callable from bundle preview (guew).
  - **jnrh**: a Done when with no checkbox lines counts as unticked at `review request` (k55o).
  - **md0v**: a Stop hook whose cwd is outside any repository doesn't block (gg0m); the readiness ✓ agrees with merge once approvals resolve `fix_now` (efnf); after type-named branches go, a child's start on a fresh branch is not refused (e151).
  - **ml3r**: workflow help in the CLI (hfze), and optionally status Next Actions in lifecycle order (mi3x).

  ## Judgment calls to look at first

  1. **fad5 (`merge --push`): fold into mmpv now?** It makes merge the single last step and removes the piped `land; git push` chain that once pushed unreviewed docs to master. It adds a flag, not a concept, so it could pass as one of the strategy's small cuts. Against that, the strategy puts the draft missions' features out of scope. I marked it revise, standalone.
  2. **nl9a and abdp: keep ll3p's "show, don't gate" leftovers?** nl9a is CI-only, costs agents nothing and watches the one invariant mmpv keeps (incident 1620b534). abdp shows review loops of 5 to 7 rounds. Both are additions, though, and both could be superseded with their mission. I marked both revise.
  3. **om6l (Waiting on section) or tf9n alone?** tf9n already names the far end of each row's blocker chain, so keeping om6l as well partly duplicates it. I kept om6l and folded 46nq and lkuy's Waiting on line into it. The leaner choice is tf9n alone, superseding om6l and 46nq.
  4. **avx0 (with majk): is a since-commit digest Atelier's job?** It has the strongest cost evidence of any view item (wake-up re-reads), but it is a new mode of `work mission`, and `git log` or `git diff --stat` since a commit may be enough. I marked it keep.
  5. **gg6b: supersede wholesale, and how much more to invest in the debrief at all.** I recommend superseding the mission and re-reviewing a fresh debrief after mmpv. The larger question is whether the debrief keeps earning its place, given that Sumeet's own taste read (atelier-3km4) is still open.

  The next closest calls were rz2l (record import, superseded), hfze (policy view, folded into ml3r) and cy58 (row suffix, superseded).
