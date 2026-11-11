# Defects After Merge: The Method

The scorecard's outcome row is "defects found after merge, traced to the item
that caused them" ([product.md](../../../product.md#scorecard)).
This document fixes how the baseline counts it. It was tested on a seeded
sample of 30 bug fixes in astromech (atelier-z5bp): 27 of the 30 were traced
to the merge that introduced the defect, so **tracing is the measure**, not
the fallback of bug items filed per merged item.

The method is written for someone applying it to a repository its author has
not seen. Run every command at the repository's commit in
[manifest.json](manifest.json) and read no history after it.

## What Counts As A Bug Fix

A bug fix is an Atelier item that, at the manifest commit:

- has `issue_type: "bug"`;
- has `status: "done"`;
- has `closed_at` on or after the window's start (for astromech, 2026-10-15,
  the day it adopted Atelier).

Nothing else counts: not commit messages that say "fix", not tasks or
features whose work happened to repair something. The rule reads only
tracker records, so it gives the same list to anyone, and the fallback
measure counts the same population. What it misses is under
[What The Method Cannot See](#what-the-method-cannot-see).

If a repository's records use another layout, adapt how the three fields are
read, not the rule.

## Drawing The Sample

The sample is the 30 candidates whose `sha256("<seed>:<id>")` sorts lowest.
The seed is `z5bp`. Hashing makes the draw independent of any language's
random number generator. These commands, run from any directory outside the
repository, write the candidates and the sample:

```sh
R=/home/supadava/projects/astromech C=ae4d18bac93f4165d9c2efd520e1747644ce0f74 FROM=2026-10-15 SEED=z5bp
git -C "$R" ls-tree --name-only "$C" .atelier/issues/ | while read -r f; do
  rec=$(git -C "$R" show "$C:$f")
  closed=$(sed -n 's/^closed_at: "\(.\{10\}\).*/\1/p' <<<"$rec")
  grep -q '^issue_type: "bug"' <<<"$rec" && grep -q '^status: "done"' <<<"$rec" &&
    ! [[ "$closed" < "$FROM" ]] && basename "$f" .md
done > candidates.txt
while read -r id; do
  printf '%s %s\n' "$(printf '%s:%s' "$SEED" "$id" | sha256sum | cut -c1-16)" "$id"
done < candidates.txt | sort | head -30 | cut -d' ' -f2 > sample.txt
```

In astromech this gives 52 candidates, every one with fix commits under
step 1 below, and a sample of the 30 in the [table](#the-30-traces), listed
in draw order.

## Tracing One Bug Fix

Set `id` to the bug item. Each step names its command; the judgment calls
are in step 4.

1. **Find the fix commits.** They are the non-merge commits reachable from
   the manifest commit whose subject names the item and that change a file
   outside `.atelier/`:

   ```sh
   git -C "$R" log --no-merges --reverse --format='%h %s' "$C" --grep="$id" -- . ':(exclude).atelier' | grep -F "$id"
   ```

   A commit that only mentions the item in its body, such as one that filed
   it, is not a fix. If no subject names the item, or the ones that do change
   only test files or docs (as step 2 defines them), find the commit that first records it as done
   (`git log --reverse "$C" -- .atelier/issues/$id.md`, then `git show
   <commit>:.atelier/issues/$id.md` until `status: "done"`) and list its
   first-parent line (`git log --first-parent --format='%h %s' <commit> -25`).
   Reading down, the fix is the non-merge commits that change a file outside
   `.atelier/`, stopping at the first subject that names another item and
   not this one. "Names this one" includes the short form of its ID (`b0tn`
   for `astromech-b0tn`), so a follow-up filed "from the b0tn review" does
   not stop the walk. "Another item" means an ID with a record in
   `.atelier/issues/`; finding, evidence and review IDs look the same but do
   not count. Stop also at commits from before the item was created. Drop
   any commit that, read, is other work bundled on the branch.

2. **Blame the lines the fix removed or changed.** Skip these files, judged
   by path alone: `.atelier/`; Markdown and anything under `doc/` or
   `docs/`; lockfiles; and test files, meaning anything under a directory
   named `test`, `tests`, `fixtures`, `__tests__`, `e2e` or `journeys`, a
   file whose name starts with `test_`, or one with `.test.`, `.spec.` or
   `_test.` in its name. Tests written inside a source file, such as a Rust
   `mod tests`, are blamed with that file. For each hunk `@@ -a,b` of
   `git diff -U0 <fix>^ <fix> -- <file>`, run
   `git blame -w -L a,+b <fix>^ -- <file>`. A hunk that only adds lines
   (`b` is 0) has no faulty line; blame its two neighbours (`-L a,+2`) and
   treat the result as a lead, not an answer. If the fix changes only test
   files, blame those.

3. **Follow the faulty line back.** Blame names the last commit to touch a
   line, which is often a later edit. When the blamed commit did not write
   the faulty logic, find the one that did with
   `git log -S'<the faulty expression>' --format='%h %ad %s' <fix>^`, over
   the whole tree rather than the fixed file, and read the oldest hit. If
   that commit moved the logic from another file (its diff deletes an
   earlier form of the same check), search again for the deleted form, and
   repeat until a commit writes it new. The fix commit's own message often
   says what was wrong; read it first.

4. **Name the cause.** Read the fix and the candidate cause and pick the
   commit whose change made the program wrong. Two conventions keep this
   repeatable:
   - When a defect appears only because a later change met older code,
     the later change is the cause: it should have accounted for the code
     already there.
   - When several commits contributed, the cause is the one that wrote the
     faulty logic. Name the others in the notes; they do not count.

   Lines a fix changed only in tests are dropped at this step, unless the
   defect is in the test itself, such as a test that fails intermittently on
   the base. That counts like any other defect: it reached the base and
   failed every branch's CI (row 24).

5. **Name the cause's item.** If the cause commit's subject names an item ID
   (full or short form, for an item created before the commit), that is the
   item. Otherwise follow the commits after it on its own line, each the next
   commit with it as first parent, until a subject names an item:

   ```sh
   git -C "$R" rev-list --parents "$C" > parents.txt
   awk -v x="$(git -C "$R" rev-parse <commit>)" '$2==x{print $1; exit}' parents.txt
   ```

   Where several branches start from the same commit this walk can pick the
   wrong one, so confirm by reading the commit. A cause dated before the
   repository adopted Atelier has no item: name the commit and its pull
   request number if its subject has one. A cause after adoption that no
   item owns is "no item".

6. **Decide when it was found.** Compare the bug's `created_at` with the
   cause item's `closed_at`. `atelier merge` marks an item done, so
   `closed_at` is its merge time; in astromech's early history items were
   closed on their branch just before main moved to it, which is close
   enough. A bug filed at or before that time was known before the cause
   merged, usually from its review.

   *Amended 2026-11-10 by the full count* ([defects.md](defects.md#what-changed-from-the-method)):
   compare the bug's `created_at` with when the cause commit reached the
   base, read from history, and use `closed_at` only where history does not
   show it. The 30 rows and the Result below use step 6 as first written;
   under the amended rule row 25 is before merge.

Every trace ends in one of these outcomes:

| Outcome | Meaning | Counted in the scorecard row as |
| --- | --- | --- |
| After merge | Cause is an item; bug filed after it merged | A defect after merge, against that item |
| Before merge | Cause is an item; bug filed at or before its merge | Not a defect after merge; caught in review or by the author |
| Before adoption | Cause predates the repository's Atelier adoption | Reported, but against no item |
| No item | Cause after adoption, owned by no item | Reported against "no item" |
| Not traceable | No single cause, with the reason | Reported with its reason |

"Traced", for the threshold, means one of the first four: the merge that
introduced the defect was found.

## The 30 Traces

Astromech at `ae4d18ba`, adopted 2026-10-15. "Fix" lists the commits step 1
finds that change a file step 2 blames; row 11 also has two docs-only
commits. "Blamed" gives the commits that step 2 found, with lines blamed;
"how" says whether blame alone named
the cause, or step 3 or a reading of the fix was needed. Areas are
paraphrased. Pull requests are astromech's, merged before adoption.

| # | Bug item | Area | Fix | Blamed | Cause | Item or merge | Outcome | How |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | astromech-naa3 | tray menu | 88fda883 | b050dcc0 (4), bc1390cf (1) | bc1390cf | PR #16 | Before adoption | blame |
| 2 | astromech-e0zz | voice latency | e9984b57 | 93687e5d (1) | 93687e5d | PR #1 | Before adoption | blame |
| 3 | astromech-kbcm | keybinds page | 79612349 | e1f29831 (1) | e1f29831 | PR #4 | Before adoption | blame |
| 4 | astromech-f2k1 | config path | c4cce78b | f3eb3fae (14), 196415b3 (11), abdd8891 (9), 4df831fe (1) | abdd8891 | astromech-c9em | After merge | blame |
| 5 | astromech-nrhv | journal liveness | d2f40e46 | six commits, 1 to 2 comment lines each | e414024f | astromech-xvjs | After merge | step 3 |
| 6 | astromech-fyvh | link check | 7fb74f89 | 08d1a204 (1) | ab6ad12c | PR #3 | Before adoption | step 3 |
| 7 | astromech-qtiw | data cache | 5d25b771 | d7aab952 (5, test module), a2be550e (1) | 8f1e9012 | astromech-proz | After merge | read |
| 8 | astromech-qhi4 | engineer trips | f8c900b6 | 8976ee14 (34), d242981a and d5580467 (2,800, data catalog) | 8976ee14 | PR #14 | Before adoption | blame |
| 9 | astromech-thym | config load | fca4abe0 | ab6ad12c (4), 6cfe4df3 (2), bc1390cf (1), 93687e5d (1) | ab6ad12c | PR #3 | Before adoption | blame |
| 10 | astromech-zftk | journal liveness | b5404b19 | d2f40e46 (7), 12dfbfe7 (4), 746d65b4 (4), 04037f7d (3), three more | d2f40e46 | astromech-nrhv | Before merge | blame |
| 11 | astromech-1c66 | voice learning | 1d55799b, 66e61c28 | ed880d67 (24), 274db91e (13), d4d3633b (3), 93687e5d (1) | c1fc6565 | astromech-y296 | Before merge | read |
| 12 | astromech-k8yk | window start | b4e118b5 | ab6ad12c (5), cost-harness script lines (4), 0cffa0ad (1) | ab6ad12c | PR #3 | Before adoption | blame |
| 13 | astromech-zirr | config path | a1848909 | c4cce78b (8), abdd8891 (3), 196415b3 (1) | abdd8891 | astromech-c9em | After merge | read |
| 14 | astromech-6a7c | config path | f3eb3fae | abdd8891 (3), 196415b3 (2) | abdd8891 | astromech-c9em | Before merge | blame |
| 15 | astromech-st8o | upload guard | f895849d | b449243b (2, comments) | none | none | Not traceable: the fix changed comments and docs, no behaviour | none |
| 16 | astromech-r3s7 | journal liveness | a0ea8fb2 | 51ce0c0d (5), 210e4af3 (5), a5bff695 (3) | 210e4af3 | astromech-36g1 | After merge | blame |
| 17 | astromech-koa6 | settings text | 6448c00a | 5ec87289 (1) | 5ec87289 | astromech-09s3 | After merge | blame |
| 18 | astromech-36g1 | journal liveness | 210e4af3 | 33bdbfda (3), 88b3f919 (3), 04037f7d (1) | 33bdbfda | astromech-0d7l | Before merge | blame |
| 19 | astromech-k1fw | route follower | dc3cb646, 65ae8a16 | f9444f31 (7), dc3cb646 (3, the first fix), a66e0008 (2), 8e9e5572 (1) | a66e0008 | astromech-g0hm | Before merge | blame |
| 20 | astromech-0d7l | journal liveness | 33bdbfda, 88b3f919 | 33bdbfda (5, the first fix), 06e57df9 (3), 04037f7d (2) | 06e57df9 | astromech-xvjs | After merge | blame |
| 21 | astromech-84vh | dependencies | 81cf4fbb | lockfiles only | none | none | Not traceable: a security advisory against a dependency, no faulty line | none |
| 22 | astromech-sltm | bind dialog | 87791aa0 | 80f3f0fe (3), 4a43d080 (2), e1f29831 (2) | e1f29831 | PR #4 | Before adoption | blame |
| 23 | astromech-w064 | body values | 32367962 | 4a39a844 (5) | 4a39a844 | astromech-wu30 | After merge | blame |
| 24 | astromech-aiwv | flaky test | e5138f36 | 47cf61a0 (3) | 47cf61a0 | astromech-bh0c | After merge | blame |
| 25 | astromech-5t34 | history follower | ea70d324 | bcbf16aa (25), c2523420 (3), 23f2a9c4 (3) | bcbf16aa | astromech-3tlm | After merge | blame |
| 26 | astromech-g1ys | binds health | bf36a8f2 | ed880d67 (4) | ed880d67 | none (direct to main, 2026-10-11) | Before adoption | blame |
| 27 | astromech-txzq | materials refresh | 0012a201 | 1e5fd333 (6), b050dcc0 (1), 0c85341e (1) | 1e5fd333 | PR #8 | Before adoption | blame |
| 28 | astromech-yerd | spoken copy | 00b899f7, 6bf2d22e | 15 commits, from 12 items and earlier code | many | many | Not traceable: one fix for the same mistake in lines from many changes | none |
| 29 | astromech-5gxp | journal liveness | 2994dc84 | e414024f (7), 210e4af3 (1), 51ce0c0d (1) | e414024f | astromech-xvjs | After merge | blame |
| 30 | astromech-ki3j | journal following | 12dfbfe7 | 18 commits | 93687e5d | PR #1 | Before adoption | read |

Notes on the rows that needed more than blame:

- 5: the deleted lines were comments; `-S` on the comment's rule found the
  commit that wrote that rule, a review fix on astromech-xvjs's branch.
- 6: blame named a commit that only extended the URL check. `-S` on the
  check's expression, over the whole tree, finds 1ad98b21, which moved the
  check into this file from the window's source and deleted the earlier
  form there; `-S` on that earlier form finds PR #3 (ab6ad12c), which wrote
  it.
- 7: the unretried mode switch dates from astromech-h74l, but only became a
  race when astromech-proz made two processes share the file (first
  convention).
- 11: learning from a "yes" predates adoption; it became a defect when
  astromech-y296 (with astromech-8fey) made everyday near-misses ask. Both
  were unmerged when the bug was filed.
- 13: blame's majority is astromech-f2k1's partial fix; the stale note it
  repairs was written by astromech-c9em.
- 30: the fix's message names following the newest journal by modified
  time as the fault; `-S` on that selection found PR #1.
- 1, 16: blame named two commits; reading chose one.
- The walk in step 1 also listed bundled work for kbcm (a test setup and a
  page merge) and qhi4 (an unrelated freshness change); those were dropped.

## Result

| Outcome | Count |
| --- | --- |
| After merge | 11 |
| Before merge | 5 |
| Before adoption | 11 |
| No item | 0 |
| Not traceable | 3 |

27 of 30 were traced to the merge that introduced the defect, above the
threshold of 20, so the scorecard row is measured by tracing. The decision
rests on counting the 11 before-adoption rows: each names a merged pull
request or commit, which is the merge the threshold asks for, but no item.
Only 16 of the 30 trace to an item, and 11 of those are defects after merge.
If "traced" is read as "traced to an item", the sample falls short and the
fallback applies. The before-adoption share falls as old code is rewritten,
so the other repositories, and astromech later, will trace more to items.

Blame alone named the cause in 21 of the 27. The other six needed `-S` or a
reading of the fix message. An agent took 15 minutes of wall-clock time
from starting this item to committing the table, settling the rule as it
went: two or three commands a row when blame was enough, six to ten when it
was not.

The full count in astromech's window is 52 bug fixes. 5 more bug items were
still open at the manifest commit and 1 was superseded.

## What The Method Cannot See

- **Defects nobody has found.** A merge with a latent defect counts as clean
  until someone files it, so recent merges look better than old ones. Report
  each item's age at the manifest commit beside its count.
- **Defects not filed as bugs.** The rule reads `issue_type`. In astromech a
  title search for failure words finds 17 done tasks, several of which read
  as defect fixes; they are not counted.
- **Bugs not yet fixed.** An open bug item has no fix to blame. Report open
  bug items as a separate count.
- **Defects fixed inside other work**, with no item of their own.
- **Defects caught before merge** by tests, CI or review, unless filed as a
  bug item. Those belong to the scorecard's "where each defect was caught"
  row; this method sees only the ones that became items ("Before merge").
- **Causes outside the code**: dependency advisories, platform changes, a
  fix that only explains (rows 15 and 21).
- **Pre-adoption code.** Defects there are traced but belong to no item.
  The share falls as old code is rewritten; in astromech it was 11 of 30.
- **Attribution is a convention.** When a later change meets older code, or
  several commits contribute, the conventions in step 4 pick one cause. A
  second tracer could disagree on rows 7, 11 and 13.
