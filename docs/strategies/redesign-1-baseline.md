---
status: draft
last_changed: "2026-11-09: revised after planning review round 1: frozen inputs, two-week rework, second rater"
---

# Why

The redesign is judged on a scorecard
([product specification](redesign-product.md#scorecard)). Its most important
row, defects found after merge, has never been measured, and every other row
can be improved by deleting review. Without a number from before the change,
nobody can tell whether the redesign made correctness worse.

The data is also disappearing. Claude transcripts survive for 13% of threads
and Codex keeps a few days. The inputs have to be frozen now, even if the
report takes longer.

# Outcome

One report gives each scorecard measure for the five Atelier repositories on
this host, from 2026-10-14 to the moment the inputs were frozen, with the
method, the limits and a proposed target for each cost measure. The scripts
that produced it are in the repository, and a fresh agent can rerun them on
the frozen inputs and get the same numbers.

# Success Criteria

- rerunnable: a fresh agent, given only `scripts/scorecard/README.md`, reruns
  the counted measures on the frozen inputs and gets the report's numbers.

# Sizing

- Complexity: medium. New scripts read three stores, and three measures need
  hand classification.
- Value: high. The outcome rows are the constraint on every later mission.
- Risk: low. Everything is read-only, and nothing an agent runs changes.

It is worth its cost because the alternative, judging the redesign by how it
feels, is how the project reached 75% record-only commits twice.

# Shape

- **Freeze first.** The first item copies every input once, and every
  script reads the copy. Nothing reads T3's live database after that, so a
  rerun matches and the server Sumeet's phone depends on is not loaded. The
  cutover waits for the freeze, not for the report.
- The scripts are Python with the standard library only, in
  `scripts/scorecard/`. Mission 4 reruns them after the cutover, so they are
  kept and reviewed like code.
- The research scripts in `/var/tmp/atelier-assessment/` are starting
  material. They are not committed as they are.
- Counted measures need no judgment. Classified measures use a written
  rubric, a seeded sample, one rater, and a second rater from another lab on
  30 of the sample, with their agreement reported. The implementer starts
  the second rater as a child given only the rubric and the unlabelled
  sample file.
- When precision and coverage conflict, precision wins: a smaller sample
  judged carefully beats a larger one guessed.

# Design

A snapshot, four readers and one report.

The snapshot lives outside the repository, in
`~/.local/share/atelier-baseline/2026-11/`, because it holds private
messages. It is a copy of T3's database made with SQLite's online backup, the
Claude and Codex transcript directories of the five repositories, and a
manifest. The manifest is committed: the cutoff time, each repository's base
commit at the cutoff, and each copied file's size and hash. The window is
2026-10-14 to the cutoff.

| Part | Reads | Gives |
| --- | --- | --- |
| Thread reader | The snapshot's copy of T3's database | Commands per thread, each thread's role and repository, Sumeet's typed messages |
| Transcript reader | The snapshot's transcripts | Tokens per thread |
| Record reader | `.atelier/` at each repository's manifest commit, and its Git history | Reviews, findings, merges, items |
| Git reader | Each repository's history up to its manifest commit | Rework, record-only commits, line counts |
| Report | The readers' JSON | `docs/architecture/quality/scorecard-baseline-2026-11.md` |

How each measure is taken:

| Measure | Method |
| --- | --- |
| Process share | For threads tied to a merged item: the share of shell commands that run `atelier`, read or write under `.atelier/`, or commit only there. Tokens where transcripts survive. |
| Confusion | `atelier` calls that printed a refusal or an error, and `--help` lookups, per 100 calls |
| Tokens per merged item | Thread tokens at list price, summed over the threads tied to the item |
| Interventions per mission | Sumeet's typed messages in a mission's threads between its start and close, classified as status question, unsticking, or correcting direction |
| Gate precision | A seeded sample of 100 refusals, classified: stopped a real mistake, or friction |
| Review yield | A seeded sample of 100 fix-now findings, classified with the assessment's seven categories |
| Defects after merge, and where defects were caught | Chosen by the first item's spike |
| Calls before first edit | Commands in an implementer thread before its first write outside `.atelier/` |
| Rework | Lines deleted within two weeks of being written, for lines written at least two weeks before the cutoff |
| Line count | `scripts/count_non_test_rust.py`, and the equivalent count for each product repository's main language |

The thread reader ties a thread to an item by the branch it worked on and the
item IDs in its title and first message, as the assessment did.

# Derisking

The riskiest assumption is that a defect found after merge can be traced to
the item that caused it. The first item tests it: take 30 bug fixes merged in
astromech since 2026-10-15 and trace each to the merge that introduced the
defect, by `git blame` on the fixed lines and a read of both changes. If
fewer than 20 can be traced, the measure falls back to bug items filed per
merged item, by repository and week, which is coarser and always available.

# Scope

- In: atelier, astromech, demiurge, life-on-rails and cosplay-studio on this
  host; the snapshot; the measures above; the scripts; the report.
- Out: the work host; building any of this into the `atelier` binary; a page
  on the site; changing anything the measures describe.
- Target environment: this host, Linux, Python 3.

# Authority

The orchestrator may choose sample sizes at or above those stated, the
fallback defect measure, the scripts' structure, and how to tie threads to
items where the stated method fails. It may not change a measure's definition
in the product specification, write to T3's database, commit anything from
the snapshot except its manifest, or change any repository other than
atelier.

# Risks and Unknowns

- **Token data is partial.** Token measures cover only threads whose
  transcripts survive. The report states the coverage beside each number.
- **T3's database format is internal.** The scripts name the schema they were
  written against and fail clearly on another.
- **The snapshot is on one disk.** It is about 6 GB and not backed up. If it
  is lost, the counted measures cannot be rerun, and the report says so.
- **Classification is judgment.** The rubric, the seed and the second rater's
  agreement are in the report so a later run can be compared fairly.
