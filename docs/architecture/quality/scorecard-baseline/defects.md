# Defects After Merge, And Where Defects Were Caught

Across the five repositories, 33 defects were found after the merge that
caused them, from 780 items merged between 2026-10-14 and the cutoff: 15 in
atelier and 18 in astromech, about 5 per 100 merged items in each. Demiurge,
life-on-rails and cosplay-studio show none, because they filed one bug fix
between them in the window, and the method sees only defects filed as bug
items. Review caught far more than escaped: by the review-yield rates,
atelier's fix-now findings held about 191 product defects and astromech's
112. CI caught 22 failures on work branches that a later commit fixed.

Every bug fix in the window was traced, not a sample: 130 in all (77 in
atelier, 52 in astromech, 1 in demiurge), by the method in
[defect-method.md](defect-method.md), with one stated change to step 6
([What changed](#what-changed-from-the-method)). The 30 astromech rows
traced there are reused; the other 100 rows are below. Inputs are the
repositories at their [manifest.json](manifest.json) commits, plus one
GitHub query for CI ([CI](#caught-in-ci)). The scorecard row is the "After
merge" column, counted against the item that caused each defect; the other
outcomes are reported beside it.

## Summary

| Repository | Window from | Items merged | Bug fixes | After merge | Before merge | Before adoption | No item | Not traceable | After merge per 100 merged | Open bug items |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| atelier | 2026-10-14 | 277 | 77 | 15 | 13 | 2 | 12 | 35 | 5.4 | 54 |
| astromech | 2026-10-15 | 361 | 52 | 18 | 12 | 18 | 0 | 4 | 5.0 | 5 |
| demiurge | 2026-10-17 | 86 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 2 |
| life-on-rails | 2026-11-06 | 38 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5 |
| cosplay-studio | 2026-10-14 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| **Total** | | **780** | **130** | **33** | **25** | **20** | **12** | **40** | **4.2** | **68** |

- **Window.** 2026-10-14 to the cutoff, 2026-11-10T05:14:58Z, or from the
  day a repository adopted Atelier if later. Adoption is the date of its
  first `.atelier/` commit: atelier 2026-06-11 (99b10205), astromech
  2026-10-15 (41f5a8cd), demiurge 2026-10-17 (8f8a86f), life-on-rails
  2026-11-06 (4f48754), cosplay-studio 2026-07-11 (5acc831).
- **Items merged** counts records at the manifest commit with `status:
  "done"`, `closed_at` on or after the window's start, and `issue_type` task,
  feature, bug or spike: the types that merge work. Decisions, missions,
  validations and loose ends are left out (atelier 39, astromech 22, demiurge
  22, life-on-rails 14, cosplay-studio 5). The count reads `status` and
  `closed_at` the way the bug-fix rule does, with the same loop.
- **Open bug items** are bug records at the manifest commit whose status is
  not done, superseded or declined. Atelier's 54 are 53 in backlog and 1 to
  do; it also superseded 14.
- **Three of atelier's 15** have causes merged before the window (atelier-59vp,
  6jap, p2wk, in June and July). Counting only causes merged in the window,
  atelier has 12, or 4.3 per 100.
- **Life-on-rails** adopted Atelier four days before the cutoff. It has six
  bug items: five in backlog and one superseded, none fixed.
  **Cosplay-studio** has four: two fixed in July, before the window, and two
  open. **Demiurge** has three: one fixed (a dependency advisory, not
  traceable) and two in backlog. Zero here means no bug item was fixed, not
  that no defect escaped.

## Where Defects Were Caught

| Repository | In review: fix-now findings × review-yield rate | In CI: failures fixed on a work branch | After merge |
| --- | --- | --- | --- |
| atelier | 478 × 8/20 = 191 | 6 (CI from 2026-10-28) | 15 |
| astromech | 372 × 6/20 = 112 | 16 | 18 |
| demiurge | 72 × 8/20 = 29 | no CI | 0 |
| life-on-rails | 76 × 6/20 = 23 | no CI | 0 |
| cosplay-studio | 58 × 18/20 = 52 | no CI | 0 |
| **Total** | **407** | **22** | **33** |

**Defects that an author's own tests caught are not observable before the
redesign.** Nothing records a test that failed and was fixed before the
first commit or review, so that row of the scorecard starts empty and can
only be counted once the redesign records it.

### Caught in review

The fix-now finding counts and the rates are
[classified.md](classified.md#review-yield)'s: findings recorded in the
window, and the share of a seeded sample of 20 per repository that the first
rater labelled a user or tooling bug. This is arithmetic, not new
classification. Each rate rests on 20 findings, so each product is rough;
8 of 20 has a 95% interval of about 19% to 64%. classified.md records one
disputed atelier label (entry 38, atelier-l8sl): with it relabelled,
atelier's rate is 7 of 20 and the product is 167.

The traces add a second, smaller view of review: the "Before merge" column
counts bugs filed as items before their cause merged, nearly all from a
finding in the cause's own review. Those 25 may overlap the review products
above, since a finding deferred to a bug item can also be a fix-now finding.

### Caught in CI

Atelier and astromech have a CI workflow (`.github/workflows/ci.yml`) at
their manifest commits; the other three have none. Atelier's was added on
2026-10-27 (b205449c), so its count covers 2026-10-28 to the cutoff only.
Run history is on GitHub, not in Git. It was read once, at
2026-11-10T15:21:29Z, with:

```sh
gh run list -R "$OWNER_REPO" --workflow ci.yml --created "$FROM..2026-11-10T05:14:58Z" \
  --limit 2000 --json databaseId,headBranch,headSha,event,status,conclusion,createdAt,attempt
```

`OWNER_REPO` is the repository's own `origin` remote (`git -C "$R" remote
get-url origin`) and `FROM` its window start. A later rerun may see fewer
runs, since GitHub expires old run history. Only counts were taken; no log
was read.

A failed run opens an episode on its branch. Runs that were cancelled or
are still running are ignored. The episode ends at the branch's next
successful run: on a new commit it is "fixed", on the same commit "passed on
rerun". These lines give the counts from the saved JSON:

```python
import json, collections
runs = sorted((r for r in json.load(open("runs.json")) if r["conclusion"] in ("success", "failure")), key=lambda r: r["createdAt"])
open_, out = {}, collections.Counter()
for r in runs:
    b = r["headBranch"]
    if r["conclusion"] == "failure": open_.setdefault(b, set()).add(r["headSha"])
    elif b in open_:
        out[("rerun" if r["headSha"] in open_.pop(b) else "fixed", "base" if b in ("master", "main") else "work")] += 1
print(out, "still failing:", list(open_))
```

| Repository | Runs (all) | Success or failure | Failed runs | Fixed on a work branch | Fixed on the base | Passed on rerun | Still failing |
| --- | --- | --- | --- | --- | --- | --- | --- |
| atelier | 403 | 340 | 15 | 6 | 6 | 0 | 0 |
| astromech | 827 | 528 | 28 | 16 | 4 | 0 | 2 |

The work-branch column is the one in the table above. A failure fixed on the
base had already merged, so it belongs with defects after merge; some of
those may also be bug items traced below, such as the flaky tests in
astromech rows 24 and 39. A failed run is not always a code defect: a
runner or network fault also fails a run. Counting cannot tell them apart
without the logs.

## What Changed From The Method

The rule for a bug fix, the outcomes and the steps are defect-method.md's.
The seed was not used, because every candidate was traced. Four things
needed a decision; only the first changes the method.

1. **Step 6 reads when the cause reached the base, not only `closed_at`.**
   Step 6 takes `closed_at` as the merge time. In astromech's first days and
   in much of atelier's October work, items were closed on their branch before
   their review, and reviewers filed bugs from that review before the
   branch merged. By `closed_at` alone, 18 such bugs read as after merge,
   though the defect was caught before its code reached the base. The
   outcome now compares the bug's `created_at` with the time the cause
   commit reached the base, read from the history:
   - Atelier merges into `master` with merge commits, so the time is that
     of the first `master` commit that contains the cause (`git merge-base
     --is-ancestor <cause> <commit>`): a commit on the manifest commit's
     first-parent line, or one a "Merge master into ..." commit records as
     `master`.
   - Astromech fast-forwards `main`, so its first-parent line does not show
     when `main` moved. Each "Merge main into ..." commit records `main`'s
     tip at that moment as its second parent. A bug counts as before merge
     when the first such record after the bug was filed does not contain the
     cause.
   - Where neither shows it, as in atelier's fast-forwarded history of
     2026-10-15, a bug filed from the cause item's own review before that
     review's last approval counts as before merge (atelier-lwdu only).
   - Otherwise `closed_at` is used, as written.

   The notes name the base commit behind each changed row. Under step 6 as
   written, the totals would be atelier 26 after merge and 2 before,
   astromech 25 and 5. The change moves one of defect-method.md's 30 rows:
   row 25, astromech-5t34, was filed from astromech-ipb5's review at
   20:38:24Z, and merge a10d13c5 at 20:38:44Z records `main` as f6eaf59a,
   without the cause bcbf16aa. It is before merge here.
2. **Step 1, for a fix made inside a parent's commit.** In atelier, 15 bug
   items were fixed inside one commit made for their parent item, whose
   subject names only the parent (atelier-0uaj, phfo, buqu, dg3h, h1nc).
   Step 1's walk stops at the sibling or parent commits before reaching it,
   so these are "Not traceable" as written. One helper also traced four of
   them by blaming the hunks the commit message assigns to the child; that
   reading would make atelier-bphy after merge and three others no item.
   The other 11 were not traced that way.
3. **Step 5, for atelier's rebuild.** Code written for the tracker rebuild
   on 2026-10-01 (a3675840, f072b36e, 3012fdce, 67b83d00, bfcecd13 and
   24af0ee5, the causes of 11 rows) reached `master` through "Integrate ..."
   merges whose lines name no item, so these causes are "No item", as step 5
   says. Review records at the manifest commit target several of these
   commits to atelier-1ioo, the rebuild's last item. Credited to it, the 12
   no-item rows would become after merge against that one item, 11 if
   atelier-ip9e, filed during the closeout review, counts as before merge.
4. **Demiurge's one fix was committed before its bug item existed.**
   Step 1's walk stops at commits from before the item was created, so it
   finds none. The fix reached the bug's branch through a merge of `master`
   (dd9cf39), whose second parent holds the two commits; both change only
   the package manifest and lockfile, so the row is "Not traceable" either
   way.

## Drawing The Candidates

Run from any directory outside the repositories. Each line gives a
repository, its manifest commit and its window start; the loop is
defect-method.md's, run for each repository, and prints one candidate per
line.

```sh
P=/home/supadava/projects
while read -r R C FROM; do
  git -C "$P/$R" ls-tree --name-only "$C" .atelier/issues/ | while read -r f; do
    rec=$(git -C "$P/$R" show "$C:$f")
    closed=$(sed -n 's/^closed_at: "\(.\{10\}\).*/\1/p' <<<"$rec")
    grep -q '^issue_type: "bug"' <<<"$rec" && grep -q '^status: "done"' <<<"$rec" &&
      ! [[ "$closed" < "$FROM" ]] && echo "$R $(basename "$f" .md)"
  done
done <<'EOF2'
atelier 2ff78f214301f5d8d096f088ced4b97636f04700 2026-10-14
astromech ae4d18bac93f4165d9c2efd520e1747644ce0f74 2026-10-15
demiurge 9fcc3bd97c49b4023d9d75f80a474325cf5be0de 2026-10-17
life-on-rails ac6136093f1ef02edad9ce67ca1f6988a3120813 2026-11-06
cosplay-studio c288d44f024eb98697dd8b5e46017ce1c887d4ba 2026-10-14
EOF2
```

It gives 77, 52, 1, 0 and 0. Testing `issue_type` for task, feature, bug
or spike instead of bug gives the items merged. Keeping the bug test and
replacing the other two with a status other than done, superseded or
declined gives the open bug items. All five
repositories keep records in the same layout, so the three fields are read
the same way. Cosplay-studio's items share atelier's `atelier-` prefix
(newer ones use `cosplay-studio-`); its two done bug items, atelier-2s9c and
atelier-zi2f, closed on 2026-07-15.

## Traces

Columns are defect-method.md's. "Fix" lists the commits step 1 finds that
change a file step 2 blames; "Blamed" gives the commits step 2 found, with
lines blamed, "lead" marking the neighbours of an add-only hunk; "How" says
whether blame alone named the cause, or step 3 or a reading of the fix was
needed. Rows for the four private repositories give process facts only, and
areas are paraphrased.

### Atelier

At `2ff78f21`. Rows were traced in two batches by helper agents and checked
against the method; rows 1 to 77 are in ID order.

| # | Bug item | Area | Fix | Blamed | Cause | Item or merge | Outcome | How |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | atelier-0349 | land speed | 58ab082a | 195a353d (99, moved epic migration), 584f98e1 (58), 67b83d00 (25), 6f5188dd (16), ecf099f5 (11), a7a23c55 (10), bfcecd13 (9), a3675840 (8), six more 1 to 4 each | none | none | Not traceable: slowness built up from per-object Git calls written across many rebuild commits, no single faulty line | none |
| 2 | atelier-0sia | parent view hints | eff128ae | e27fd01b (12), 090920dd (5), 5490a8be (2), 315d6f23 (1), 699c9983 (1), d1de5ae1 (1) | none | none | Not traceable: one bug item bundles three separate defects (parent proof gaps, issue link footer, brief Close Reason) written by different changes | none |
| 3 | atelier-1hjj | merge retry report | e86baad3 | a3675840 (12), 2c18ab44 (7), f94ba14a (5), 4b300310 (4), leads a3675840, 4b300310, f94ba14a | f94ba14a | atelier-p189 | Before merge | read |
| 4 | atelier-2hh8 | macOS SIGPIPE | 855d2f6d | lead only: cc2d967c (add-only hunk) | cc2d967c | atelier-bsyn | Before merge | read |
| 5 | atelier-2oor | record ID slug | 960d3c76 | 77125863 (25 + 6 lead, moved code), 5df9fa56 (13), 356d5898 (7 + 4 lead), 0bfc7a02 (4), 12 more 1 to 2 each | fbeee3a7 | commit fbeee3a7 (2026-06-09, no PR) | Before adoption | step 3 |
| 6 | atelier-3u2u | review show view | none | none | none | none | Not traceable: fixed inside atelier-phfo's commit e7e76fde, which step 1's walk does not reach | none |
| 7 | atelier-3xq4 | merge sync receipts | 9f4d7ee0 | 3ae56002 (1), lead 857b7c80 | 3ae56002 | atelier-s2ei | After merge | blame |
| 8 | atelier-40i6 | install commit landing | 6a32bb81 | 85346c57 (6 + lead), 02fb5681 (3 + lead), lead da8a01b7/e37f556b | f072b36e | none (rebuild line, 2026-10-01) | No item | step 3 |
| 9 | atelier-467y | Stop hook refusal | none | none | none | none | Not traceable: fixed inside atelier-buqu's commit d5a4bebc, which step 1's walk does not reach | none |
| 10 | atelier-4775 | transition readiness views | 923dccf9 | c86e45e8 (8), 22d38fa2 (3), 9b2a5a74 (2), 4b300310 (1), bebca71b (1), 99150241 (1), e36a6a88 (1), leads | c86e45e8 | atelier-7u4o | After merge | read |
| 11 | atelier-4xyr | advances direction | 5918de7b | 960d3c76 (10, rustfmt rewraps only), b38c536b (7), f2f6813e (3), dbd963aa (2), 356d5898 (1), 16f01101 (1), leads | b38c536b | atelier-59vp | After merge | read |
| 12 | atelier-5qcs | init slug | 44c42def | 960d3c76 (12 + leads), 090920dd (1), leads 356d5898, 1920c7d4/729a4679 | 960d3c76 | atelier-2oor | After merge | blame |
| 13 | atelier-5rs0 | broken pipe exit | e8402f1e | leads only: 2aa911f6, 13a5da2d (add-only hunks in main) | none | none | Not traceable: an omission since the CLI's first main (Rust ignores SIGPIPE, so every multi-line view panics on a closed pipe); the add-only fix has no faulty line and its leads are a code move | none |
| 14 | atelier-6wyv | work mission readiness | 258d78be | none (no code line changed) | none | none | Not traceable: closed as not reproducible; the fix only added a test and a docs line | none |
| 15 | atelier-6xxb | placeholder IDs in hints | a8e0b6d0, 7a55dcce | a8e0b6d0: 7a55dcce (17, the bug's own first fix); 7a55dcce: many commits across 15 files | none | none | Not traceable: one fix for the same mistake (placeholders where the ID is known) in hints written by many changes | none |
| 16 | atelier-749l | stale proof message | e05ab103 | bfcecd13 (5 + lead), a3675840 (2 + lead), 0bfc7a02 (1), 3012fdce (1), 6f5188dd (1 + lead), 25a5eb21 (1) | bfcecd13 | none (rebuild line, 2026-10-01) | No item | read |
| 17 | atelier-7iyw | UTC dates | 53e954d6 | 52429f07 (5), 81a56041 (3), 11b62c89 (4), a958bda2 (5, imports and format), 24af0ee5 (2), 4002c55b (2), a3675840 (1), leads | none | none | Not traceable: two defects (notes dated by the UTC day, prune cutoffs by UTC) written by different changes over months | none |
| 18 | atelier-7n5u | ready in-flight marks | 01c42117, 21a43200 | f072b36e (68), 01c42117 (32, the first fix), 8af12372 (9), 8b17c196 (9), 81058da9 (7), seven more 1 to 3 each | none | none | Not traceable: a missing check, not a faulty line; work ready never read other local branches, and the fix reshaped branch-reading code from many commits to share it | none |
| 19 | atelier-7ts2 | land dirty advice | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 20 | atelier-976s | land --push base check | 6fbdc569 | leads only: d6c662c6, d8351d87 (add-only hunks) | d6c662c6 | atelier-ecah | Before merge | read |
| 21 | atelier-9co2 | bundle create status | a11793fc | f95cff85 (20, atelier-ha6k's create check), dca230ea (17, moved code), 2f25040c (2), lead 3be1073f/5918de7b | none | none | Not traceable: two defects (any workflow status accepted; closed_at set only for a `closed` status), each from older bundle code (9163e0a5, b7c20462) that later policy changes left wrong | none |
| 22 | atelier-9jr8 | land gate report | none | none | none | none | Not traceable: fixed inside atelier-h1nc's commit eb8c7846, which step 1's walk does not reach | none |
| 23 | atelier-as7i | review origin lookup | faed7efe | a3675840 (2 + lead), 6f5188dd (1), 58ab082a (1, a doc comment + lead), lead 67b83d00 | 67b83d00 | none (rebuild line, 2026-10-01) | No item | step 3 |
| 24 | atelier-auca | evidence trailing newline | none | none | none | none | Not traceable: fixed inside atelier-dg3h's commit fc115d5b, which step 1's walk does not reach | none |
| 25 | atelier-bibd | install detection | c2f02115, ca38c4b2 | c2f02115: 6a32bb81 (2), ca38c4b2 (1, the bug's own first fix); ca38c4b2: f072b36e (6 + lead), 6a32bb81 (1 + leads) | 6a32bb81 | atelier-40i6 | Before merge | read |
| 26 | atelier-bphy | evidence output tail | none | none | none | none | Not traceable: fixed inside atelier-dg3h's commit fc115d5b, which step 1's walk does not reach | none |
| 27 | atelier-bsyn | cat-file SIGPIPE | cc2d967c | 58ab082a (8 + lead), lead 96b98896 | e8402f1e | atelier-5rs0 | Before merge | read |
| 28 | atelier-cej5 | slow parity test | c64cb4bd | 98f4ffb5 (7 + lead, a test inside path_log.rs) | 98f4ffb5 | atelier-7b6e | After merge | blame |
| 29 | atelier-d8ku | invalid value form | none | none | none | none | Not traceable: fixed inside atelier-phfo's commit e7e76fde, which step 1's walk does not reach | none |
| 30 | atelier-e4c5 | parent blockers | 70526e53 | 24af0ee5 (3, refactor call sites), ace2d681 (2, test assertions), lead 95da2cff/d2c0d1cf | none | none | Not traceable: a missing rule, not a faulty line; readiness never applied a parent's blockers to its children, and blame finds only a refactor's call sites | none |
| 31 | atelier-f4ko | load error naming | 9649c971 | 24af0ee5 (15 + lead), fdf8fd68 (9 + lead), a3675840 (7), 0f221708 (3), eight more 1 to 2 each | none | none | Not traceable: two separate defects (a policy error that names workflow.yaml instead of the record; a review-target error that calls the review an Issue) in lines from many changes | none |
| 32 | atelier-fj1t | base done claims | 4402d727 | leads only: e37f556b, 0e3ffcd5 (add-only hunks) | 74077b1a | atelier-3mjv | After merge | read |
| 33 | atelier-fnak | quiet refusals | 05342b37 | 5a006247 (11 + leads), lead ee1efe66 | 4b300310 | atelier-pr4k | After merge | read |
| 34 | atelier-ft88 | hook payload errors | none | none | none | none | Not traceable: fixed inside atelier-buqu's commit d5a4bebc, which step 1's walk does not reach | none |
| 35 | atelier-ftqn | status branch rows | 8ef20969 | a958bda2 (15), 21a43200 (14), 01c42117 (10), 5927bc00 (6), 93b34476 (4), 8b17c196 (3) | none | none | Not traceable: five presentation faults in branch rows written by several changes; the fix redrew the section | none |
| 36 | atelier-fxbg | unowned branch land | 85346c57 | f072b36e (3 + leads), lead 808ee9b2 | f072b36e | none (rebuild line, 2026-10-01) | No item | blame |
| 37 | atelier-g0zm | git 2.53 test fixtures | e2c03600 | bfa3b9b5 (1, a test inside git.rs) | none | none | Not traceable: Git 2.53 began refusing `git tag HEAD`, a platform change, not a faulty line | none |
| 38 | atelier-grsi | uncommitted records message | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 39 | atelier-h6o4 | review list view | none | none | none | none | Not traceable: fixed inside atelier-phfo's commit e7e76fde, which step 1's walk does not reach | none |
| 40 | atelier-iaas | approval currency | bbd0f996 | eae0d4b4 (7), ffabdab9 (3), 2f1af2e6 (2), 047155d4 (2), 6f5188dd (1), ee88dcdf (1), plus leads | ffabdab9 | atelier-cafy | After merge | read |
| 41 | atelier-ip9e | record validation | e9f7ef92 | add-only hunks; leads 24af0ee5, fdf8fd68 | fdf8fd68 | none (rebuild closeout, 2026-10-15) | No item | read |
| 42 | atelier-j48s | merge advice | 89a05c5b, 1d1e3732 | a3675840 (1), 584f98e1 (1) | a3675840 | none (rebuild line, 2026-10-01) | No item | step 3 |
| 43 | atelier-jiz0 | disposition errors | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 44 | atelier-jnec | debrief history | 4d8556a4, 79a5cf61 | 3b431a9e (20), 4d8556a4 (26, the first fix), 7cc6d1f1 (6), leads | 7cc6d1f1 | atelier-rx22 | Before merge | step 3 |
| 45 | atelier-jpok | priority parsing | 39ca34a9 | 4aee6436 (9, plus leads), b7c20462 (4), 77125863 (3), 2aa911f6 (2), a2798818 (2), 13a5da2d (1), 090920dd (1), c4f3d459 (1) | 4aee6436 | atelier-6jap | After merge | blame |
| 46 | atelier-jw1n | review listing | none | none | none | none | Not traceable: fixed inside atelier-phfo's commit e7e76fde, which step 1's walk does not reach | none |
| 47 | atelier-jx7g | error grammar | c7eac73a | ae6c48d6 (12, plus leads), 7a55dcce (1) | ae6c48d6 | atelier-qrwr | Before merge | blame |
| 48 | atelier-kqt9 | land claim retention | bd7dcd49 | 6f5188dd (9), 3012fdce (8, plus lead) | 3012fdce | none (rebuild line, 2026-10-01) | No item | step 3 |
| 49 | atelier-ksqc | blocker messages | 16f01101 | f2f6813e (3, plus lead), 24af0ee5 (1) | f2f6813e | atelier-p2wk | After merge | blame |
| 50 | atelier-ksx6 | detail-view footers | 315d6f23 | 26 commits; largest 090920dd (18), 13a5da2d (13), e27fd01b (12), bebca71b (9) | none | none | Not traceable: one fix for six separate footer and readiness defects in lines from many changes | none |
| 51 | atelier-lq42 | land blocker gate | 62c27770 | 3012fdce (1) | 3012fdce | none (rebuild line, 2026-10-01) | No item | blame |
| 52 | atelier-lr8o | bundle ID resolution | 026fd1cf | dca230ea (10, plus lead), ae6c48d6 (4, plus leads), 24af0ee5 (3, plus leads) | ae6c48d6 | atelier-qrwr | Before merge | read |
| 53 | atelier-lwdu | merged label after prune | 5199506c | 0e3ffcd5 (11) | 0e3ffcd5 | atelier-9k7o | Before merge | blame |
| 54 | atelier-mgl0 | debrief rendering | 31e56669, 3b6957b6, 19210197 | ae669d13 (8, plus leads), 3b6957b6 (13, the second fix), 1a2df4a1 (3, plus leads), b646d9d4 (2), ff8838b6 (1), 8151a566 (1) | none | none | Not traceable: one item for four separate debrief defects (missing Decisions part, planned work shown as filed, cut criteria, contrast) from changes in several items | none |
| 55 | atelier-mhx0 | start base recording | fac1da31 | f072b36e (10), 645efb8f (5, plus leads), 4b300310 (2), 5a006247 (1, plus leads) | f072b36e | none (rebuild line, 2026-10-01) | No item | read |
| 56 | atelier-ns04 | land guidance | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 57 | atelier-ov97 | debrief child reports | b646d9d4 | 1a2df4a1 (13, plus lead), ae669d13 (12, plus leads), 8151a566 (3), 60b995d1 (1) | 1a2df4a1 | atelier-nexd | After merge | step 3 |
| 58 | atelier-pr4k | quiet output | 4b300310 | 23 commits; largest 960d3c76 (17), 89f237fb (13), a3675840 (10), 24af0ee5 (10) | none | none | Not traceable: one fix for the same mistake (no documented --quiet token) in a dozen commands written by many changes | none |
| 59 | atelier-pyfr | invalid-value errors | efee9e95 | a8389f10 (3, pyfr's own first fix), ae6c48d6 (2), dbd963aa (2, plus lead) | none | none | Not traceable: the item lists several error shapes from different commands, and the subject-named fix only repairs four review findings on pyfr's own first fix | none |
| 60 | atelier-qrwr | not-found errors | 20f796fa, ae6c48d6 | 21 commits; largest 24af0ee5 (93), 2aa911f6 (44), dbd963aa (21), 164ebd77 (18) | none | none | Not traceable: one fix for inconsistent not-found and invalid-value wording across every command, written by many changes | none |
| 61 | atelier-qz9x | land behind base | 03219e94 | f072b36e (3, plus lead), 3012fdce (3) | f072b36e | none (rebuild line, 2026-10-01) | No item | read |
| 62 | atelier-s5x4 | decision status without policy | e223e881 | 43eefb4b (3), 7a371ee7 (2), 3b431a9e (1), lead | 7a371ee7 | atelier-fip8 | Before merge | read |
| 63 | atelier-secy | after-merge tick on base | 2206fb88 | 6f5188dd (4, plus lead), 3012fdce (3, plus lead), 2148b5a5 (2, plus lead), 58ab082a (2, plus lead), 09880419 (1), bfcecd13 (1), bd912504 (1) | 902a5a63 | atelier-n0qh | After merge | read |
| 64 | atelier-ss25 | land touched claims | 700e102e | 6f742c79 (13, plus leads), 42d20797 (11, plus lead), ebdc2ad5 (2, plus leads), ece2a8e4 (2, plus leads) | 6f742c79 | atelier-37uq | After merge | blame |
| 65 | atelier-teld | item-line wrapping | 24209d7d | a958bda2 (37, plus leads) | a958bda2 | atelier-i2qq | Before merge | blame |
| 66 | atelier-tqru | duplicate YAML keys | 02b8ace6 | 77125863 (2) | 5569b244 | commit 5569b244 (no PR) | Before adoption | step 3 |
| 67 | atelier-u3vj | status output rules | 371ecf05 | 5490a8be (9), e27fd01b (5), 07b3dea3 (1), e5b714b6 (1) | none | none | Not traceable: the item lists three separate contract breaks (parenthesised counts, section order, Scope/Work labels), and the subject-named fix only repairs review findings on u3vj's own change | none |
| 68 | atelier-uw4x | land done-claim check | 047155d4 | 4402d727 (7), 5e071c9c (2), 5178ce05 (1), 249ce829 (1), leads e37f556b, da8a01b7, 191c8574 | 4402d727 | atelier-fj1t | After merge | read |
| 69 | atelier-vo0z | release targets | 3c486736, a523e50c | f62a2a3b (1) | f62a2a3b | atelier-7v9q | After merge | blame |
| 70 | atelier-x5j3 | session lease | 93698dad | 0ca727ef (3, plus lead), 24af0ee5 (1, plus lead) | 0ca727ef | atelier-81e5 | Before merge | blame |
| 71 | atelier-x68o | record-load errors | 4a02534f | 24af0ee5 (6, plus leads), 8e8702a5 (1) | 24af0ee5 | none (rebuild line, 2026-10-01) | No item | blame |
| 72 | atelier-xe43 | branch summary output | 8b17c196 | f072b36e (4, plus lead) | f072b36e | none (rebuild line, 2026-10-01) | No item | blame |
| 73 | atelier-y0en | policy error output | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 74 | atelier-ymmb | ready classification | 6257cb3c | 3c98ae3c (6, plus leads), 711b0fb8 (5, plus leads), 488c1b62 (3), 07b3dea3 (3), e43da8bc (1) | none | none | Not traceable: the fix adds a missing classification for missions and parents, plus a bundled parked-status finding; no faulty line, and the gap grew from several changes | none |
| 75 | atelier-ypks | Close Reason reuse | 14293fdd | workflow.rs only: b4fcc9c4 (5, plus leads), a3675840 (5, moved block), 9b2a5a74 (2), 24af0ee5 (1), 315d6f23 (1), ee1efe66 (1), f80def96 (1) | b4fcc9c4 | atelier-sfsq | Before merge | blame |
| 76 | atelier-yrlj | merge recovery advice | none | none | none | none | Not traceable: fixed inside atelier-0uaj's commit 1f28862b, which step 1's walk does not reach | none |
| 77 | atelier-ytxg | truncated list hints | e5b714b6 | 11b62c89 (8, plus leads), 965dfc79 (2), bebca71b (2, plus lead), ace2d681 (2, plus lead), 5490a8be (2), 56a619dc (1), 315d6f23 (1), 2124dcd7 (1) | none | none | Not traceable: two separate lists (issue show's Impact list and the unassigned groups) truncated without a command, from different changes | none |

Notes on atelier rows:

- **Before merge by when the cause reached `master`** (the change to step
  6). Each pair is the first `master` commit holding the cause, then the
  bug's filing time, both UTC: 1hjj 97d61a04 21:01:42 after 20:59:30 (from
  a finding on atelier-c3kh, p189's parent); 2hh8 filed in b81a5a89 on
  atelier-bsyn's branch, the commit `master` later moved to; 976s and x5j3
  c90eb4d1 21:18:35 after 21:18:32; bibd 8742bfdd 00:12:57 after 00:01:32;
  bsyn c42c2470 07:13:17 after 06:31:32; jnec bf738f49 19:45:47 after
  19:45:46; jx7g and lr8o 27d9fd7b 07:29:59 after 07:17:24; s5x4 80542968
  17:53:18 after 17:51:42; lwdu, fast-forwarded, filed as the deferral of
  atelier-9k7o's finding (5129f0aa) before 9k7o's last approval (ceb58264).
  teld and ypks were filed before their cause's `closed_at`, as written.
  fj1t stays after merge: it was filed in 246aff02 on `master`, which
  already held 74077b1a.
- **Rebuild causes, no item** (8, 16, 23, 36, 42, 48, 51, 55, 61, 71, 72):
  step 5's walk from each cause ends at ecf099f5, whose next commit is an "Integrate ..." merge naming no item. Row 41's cause
  fdf8fd68 is from the rebuild closeout on 2026-10-15; its walk reaches
  4d2d3868, which names atelier-8ud1 only because it records evidence for
  that item (closed 2026-10-04), so reading rejects it.
- 8 (40i6): blame names atelier-fxbg's rewrite; the refusal of unowned
  branches was written by f072b36e (step 3).
- 5 (2oor): 77125863 moved the slug minting; `-S` finds fbeee3a7
  (2026-06-09), before atelier's first `.atelier/` commit. By the first
  convention a second tracer could instead name 19e94428, an adoption-day
  commit that added the slug setting without making minting read it; it
  names no item, so the row would be no item.
- 66 (tqru): `-S` finds 5569b244, dated 2026-06-11 12:00 -0400 and an
  ancestor of the first `.atelier/` commit (99b10205, 19:22).
- 11 (4xyr): the walk listed a batch of other fixes made on `master`
  (d1cb7d90 and eight more), dropped. The walk from b38c536b reaches a
  validation item; b38c536b's subject is atelier-59vp's title, so reading
  names 59vp. 59vp, 6jap (45) and p2wk (49) were archived on 2026-09-30
  (5c4a38ef); their `closed_at` is read at `5c4a38ef^`.
- 12 (5qcs): the walk from 960d3c76 reaches atelier-8ud1; 960d3c76 is
  atelier-2oor's fix (its subject is 2oor's title, and d1cb7d90 records 2oor
  done), so reading names 2oor.
- First convention, the later change is the cause: 10 (7u4o's brief view
  left the full view on the old test), 25 (40i6's fix met the base lookup),
  27 (5rs0's default SIGPIPE met older pipe writes), 32 (3mjv let
  record-only commits land on the base), 33 (pr4k's quiet rule missed two
  commands' older refusals), 52 (qrwr's short IDs left bundles out), 63
  (n0qh made an after-merge tick a validation step without teaching land),
  68 (fj1t's refusal met merge's older approval rule), 20 (ecah's land
  --push met an older skip).
- Second convention, the commit that wrote the faulty logic: 3 (f94ba14a
  wrote the retry; 2c18ab44, the same item's review fix, kept it), 16
  (bfcecd13 wrote newest_failure), 62 (7a371ee7 and 43eefb4b hold the same
  test; the finding is about 7a371ee7's).
- Step 3 by `-S`: 23 (`--diff-filter=A` finds 67b83d00), 44
  (`--diff-merges=first-parent` finds 7cc6d1f1), 57 (finds 1a2df4a1 under
  atelier-8bil's move).
- 40 (iaas): blame's largest share is doc comments the fix rewrote for
  another finding, dropped as bundled work. 64 (ss25): 6f742c79 is a review
  fix on atelier-37uq's branch; 42d20797 later rewrote the lines, keeping
  the rule.
- 28 (cej5): a slow test inside a source file's test module, counted like
  defect-method.md's row 24.
- **Not traceable, 35 rows:** 15 fixed inside a parent's commit (change 2
  above); 10 items that hold several separate defects with separate causes
  (2, 17, 21, 31, 35, 50, 54, 59, 67, 77); 4 fixes of one mistake repeated
  across many changes (1, 15, 58, 60); 4 omissions with no faulty line,
  where blame finds only neighbours or refactors (13, 18, 30, 74); one not
  reproducible (14) and one platform change (37).

### Astromech

At `ae4d18ba`. Rows 1 to 30 are defect-method.md's
[30 traces](defect-method.md#the-30-traces), reused unchanged except row
25, whose outcome is before merge here (change 1). They hold 10 after
merge, 6 before merge, 11 before adoption and 3 not traceable. Rows 31 to
52 are the rest of the 52, in ID order.

| # | Bug item | Area | Fix | Blamed | Cause | Item or merge | Outcome | How |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 31 | astromech-2hml | flaky test | 48299681 | 93687e5d (2, test file) | 93687e5d | PR #1 | Before adoption | blame |
| 32 | astromech-4ed7 | session summary | d5ba1939 | c79dec45 (2, and a lead) | f6e429d2 | astromech-627q | After merge | step 3 |
| 33 | astromech-6h7t | route jump count | 4ba62832 | 562d5f3a (1), lead a2be550e | 562d5f3a | astromech-94j4 | After merge | blame |
| 34 | astromech-782v | journal liveness | 76ca4118 | 5dc189c5 (1), three leads | none | none | Not traceable: the fix added a test and a test seam; behaviour was already right | none |
| 35 | astromech-8ozh | upload queue ids | 03b6a951 | leads only: e3cd7deb, 5e61b010, d78c25f8 | e3cd7deb | astromech-6xyo | Before merge | read |
| 36 | astromech-b0tn | journal following | bf39b03b | 51ce0c0d (6), e95aaa8a (2), 5b1bfc68 (1) | e95aaa8a | astromech-lpsk | Before merge | read |
| 37 | astromech-dr5j | merit totals | ff91d631 | lead 51c7785f | 51c7785f | astromech-ny26 | After merge | read |
| 38 | astromech-fg9l | material counts | 2f261a25, c43873f0 | 34b8f5e9 (3), 1e5fd333 (1), the first fix (14) | 1e5fd333 | PR #8 | Before adoption | read |
| 39 | astromech-gp9x | flaky test | c9848180 | 1d55799b (3, test file) | 1d55799b | astromech-1c66 | After merge | blame |
| 40 | astromech-k20i | sample config | db710854 | 93687e5d (2), ab6ad12c (2), 4a43d080 (1) | ab6ad12c | PR #3 | Before adoption | read |
| 41 | astromech-kktx | data cache | d7aab952 | f02beb56 (3), a2be550e (2) | f02beb56 | astromech-zs2q | Before merge | blame |
| 42 | astromech-lnzj | status values | a4a4d017 | ed880d67 (2), 90bd0e76 (1) | ed880d67 | none (direct to main, 2026-10-11) | Before adoption | blame |
| 43 | astromech-lpsk | journal following | e95aaa8a | 12dfbfe7 (10), 74fbb4ba (5), 6fda4a5f (3), 47cf61a0 (2), three more | 12dfbfe7 | astromech-ki3j | Before merge | step 3 |
| 44 | astromech-nw2d | docked state | 8383c2ac | leads only: e71c2a37, ed880d67, f09a5d81 | 8d842926 | none (initial scaffold, 2026-10-10) | Before adoption | step 3 |
| 45 | astromech-o7jl | flaky test | 3d203bb6 | c06cdd41 (12, test module) | c06cdd41 | astromech-0bkq | After merge | blame |
| 46 | astromech-pb3o | settings carry-over | 41d05c14 | dfacc276 (5) | dfacc276 | astromech-7w5d | Before merge | blame |
| 47 | astromech-rhfn | keybinds page | da4caae5 | 79612349 (3), lead e1f29831 | 79612349 | astromech-kbcm | Before merge | blame |
| 48 | astromech-rjhy | journal liveness | df0c707b | 36f1203f (3), b5404b19 (2), five leads | d2f40e46 | astromech-nrhv | After merge | step 3 |
| 49 | astromech-vett | ships ledger | 7cf75990 | 8069aac3 (13), 1f5596b0 (9), c11b2a20 (5), e1a22ac9 (1) | c11b2a20 | astromech-aonb | After merge | step 3 |
| 50 | astromech-xtq1 | model install | b96aad0f | d8b2724a (3), ab6ad12c (1), 93687e5d (1) | 93687e5d | PR #1 | Before adoption | step 3 |
| 51 | astromech-zc0v | upload queue schema | 1e71807e | d78c25f8 (6), 03b6a951 (4) | d78c25f8 | astromech-el1v | After merge | blame |
| 52 | astromech-ztrb | builds page refresh | 8bab27e1, cfd5a07f, 54184a26 | 1e5fd333 (24), d242981a (10), 8976ee14 (7), 0012a201 (3), more | d242981a | PR #12 | Before adoption | step 3 |

Notes on astromech rows:

- **Before merge by when the cause reached `main`** (change 1). For each,
  the first "Merge main into ..." commit after the bug was filed records a
  `main` without the cause: 35 (8ozh) ed8af59e records cde239ae; 36 (b0tn)
  7cf362cd records ea9d3fd0; 41 (kktx) d0897d1a records 5ecaf2d2; 43
  (lpsk) a74c66c3 records a61ca2b9; 46 (pb3o) 825c4bfc records fe9d9b3a; 47
  (rhfn) 40521f78 records 5ba7eae5. All but 47 were filed from a finding in
  the review of the branch carrying the cause.
- 32 (4ed7): blame names astromech-hk7e, whose reviewer called the fault
  older; `-S` on the reset finds f6e429d2, which wrote the replay
  reset that forgets a summary was said.
- 34 (782v): the fix adds a test and a test seam, and its message says an
  earlier change already handled the case.
- 35 (8ozh): the fix only adds lines. First convention: e3cd7deb
  (astromech-6xyo) added the purge that let a new message take a purged
  message's id; the table without ever-increasing ids is d78c25f8's.
- 36 (b0tn): step 1's walk finds bf39b03b and stops at dc7ea2a0
  (astromech-t34c). Second convention: e95aaa8a wrote the rule the fix
  narrows; 5b1bfc68 made it sticky and does not count.
- 37 (dr5j): the fix only adds lines; `-S` on the summing expression finds
  51c7785f.
- 38 (fg9l): the fault is the ledger's uncapped count (1e5fd333); the
  34b8f5e9 lines are the cap rule the fix rewrote to match.
- 40 (k20i): first convention: the config opener (ab6ad12c, PR #3) writes
  PR #1's sample, which sets a value, as the user's config. Both predate
  adoption.
- 41 (kktx): `-S` on the table replacement finds f02beb56.
- 42 (lnzj): 90bd0e76 (astromech-08w8) copied the wrong values into a
  script (second convention; not counted).
- 43 (lpsk): 6fda4a5f moved the header-order rule; `-S` on its earlier form
  finds 12dfbfe7, astromech-ki3j's fix.
- 44 (nw2d): the fix only adds a line. The undock handler that keeps the
  station dates from the initial scaffold 8d842926; e71c2a37
  (astromech-tpdb) rewrote it and kept the omission.
- 31, 39, 45: flaky tests, counted like row 24. 45's test is in a source
  file's test module; `-S` on its name finds c06cdd41.
- 47 (rhfn): the walk finds da4caae5 (its Materials change is bundled work,
  dropped) and 7b5ab437, which changes only a test. First convention:
  astromech-kbcm's open-on-press met the window's own clicks.
- 48 (rjhy): `-S` on the latest-journal-only rule finds d2f40e46
  (astromech-nrhv's fix); 36f1203f extended it and does not count.
- 49 (vett): `-S` on the ships table's key finds c11b2a20, which keyed ships by
  id; 1f5596b0 added the commander to the key.
- 50 (xtq1): `-S` on the files-present check finds 93687e5d; d8b2724a fixed
  the same fault for the other model only.
- 51 (zc0v): second convention: the racy migration loop is d78c25f8's.
  Read by the first convention, 03b6a951 (astromech-8ozh's fix) added the
  migration that exposed it, and the row would be before merge, since the
  bug was filed from 8ozh's review.
- 52 (ztrb): first convention: the builds page (d242981a, PR #12) took
  over PR #8's folder signature; 0012a201 later moved another page off it.
- The walk in step 1 found the fix for 8ozh, b0tn, nw2d, o7jl, pb3o, rhfn,
  xtq1 and zc0v; no subject names those items.

### Demiurge

At `9fcc3bd`.

| # | Bug item | Area | Fix | Blamed | Cause | Item or merge | Outcome | How |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | worldcrafter-ta77 | dependency advisory | ff6f45e, ed1282f (change 4) | lockfile and manifest only | none | none | Not traceable: a security advisory against a dependency, no faulty line | none |

### Life-on-rails and cosplay-studio

No bug item was fixed in either window, so there is nothing to trace.

## The Items That Caused Defects After Merge

The scorecard row counts each defect against the item that caused it. The
merge date beside each shows its age at the cutoff; a recent merge has had
less time for its defects to be found.

| Repository | Item (merged) | Defects after merge |
| --- | --- | --- |
| astromech | astromech-xvjs (2026-10-16) | 3: rows 5, 20, 29 |
| astromech | astromech-c9em (2026-10-17) | 2: rows 4, 13 |
| astromech | 13 items, one each: proz, 36g1, wu30, bh0c, 627q, 94j4 (2026-10-17); 09s3, ny26, 0bkq, aonb, el1v (2026-10-16); 1c66 (2026-10-18); nrhv (2026-10-22) | 13 |
| atelier | 15 items, one each: 59vp (2026-06-23), 6jap (2026-06-17), p2wk (2026-07-16), 2oor, 3mjv (2026-10-15), cafy, 37uq, fj1t (2026-10-16), 7u4o, pr4k, nexd (2026-10-17), s2ei, 7b6e (2026-10-22), 7v9q (2026-10-28), n0qh (2026-11-07) | 15 |

Every astromech cause merged between 2026-10-16 and 2026-10-22, in the
first week after adoption; nothing merged later has a defect found yet.
Five of the 33 causes are themselves bug fixes (astromech-1c66, 36g1, nrhv;
atelier-2oor, fj1t).

## Limits

[defect-method.md](defect-method.md#what-the-method-cannot-see) lists what
the method cannot see; all of it applies here. Three choices move the
numbers most:

- **Step 6.** As written, it would count 18 more defects after merge
  (change 1). Those bugs were caught in review of the branch that carried
  the cause.
- **Atelier's not-traceable share.** 35 of 77 atelier rows are not
  traceable, against 4 of 52 in astromech, mostly because atelier's bug
  items are fixed in shared commits or hold several defects each. Its count
  after merge is a floor.
- **Attribution conventions.** Rows whose cause rests on step 4's
  conventions are named in the notes. Two that would change outcome:
  astromech 51 (before merge by the first convention) and the rebuild rows
  (after merge if credited to atelier-1ioo).

The 77 atelier rows were traced by two helper agents given the method and a
list each; every row was checked against the method before it went in,
three were repeated in full, and the conventions were made the same across
both batches (changes 2 and 3).
