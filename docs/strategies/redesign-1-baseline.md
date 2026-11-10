---
status: draft
last_changed: "2026-11-09: first draft"
---

# Why

The redesign is judged on a scorecard
([product specification](redesign-product.md#scorecard)). Its most important
row, defects found after merge, has never been measured, and every other row
can be improved by deleting review. Without a number from before the change,
nobody can tell whether the redesign made correctness worse.

The data is also disappearing. Claude transcripts survive for 13% of threads
and Codex keeps a few days. The baseline has to be taken now.

# Outcome

One report gives each scorecard measure for the five Atelier repositories on
this host, from 2026-10-14 to the day measuring starts, with the method, the
limits and a proposed target for each cost measure. The scripts that produced
it are in the repository, and a fresh agent can rerun them.

# Success Criteria

- rerunnable: a fresh agent, given only `scripts/scorecard/README.md`, reruns
  the counted measures on the same window and gets the report's numbers.

# Sizing

- Complexity: medium. New scripts read three stores, and two measures need
  hand classification.
- Value: high. The outcome rows are the constraint on every later mission.
- Risk: low. Everything is read-only, and nothing an agent runs changes.

It is worth its cost because the alternative, judging the redesign by how it
feels, is how the project reached 75% record-only commits twice.

# Shape

- The scripts are Python with the standard library only, in
  `scripts/scorecard/`. Mission 4 reruns them after the cutover, so they are
  kept and reviewed like code.
- The research scripts in `/var/tmp/atelier-assessment/` are starting
  material. They are not committed as they are.
- Counted measures need no judgment. Classified measures use a written
  rubric, a seeded sample, one rater, and a second rater from another lab on
  30 of the sample, with their agreement reported.
- When precision and coverage conflict, precision wins: a smaller sample
  judged carefully beats a larger one guessed.

# Design

Four readers and one report.

| Part | Reads | Gives |
| --- | --- | --- |
| Thread reader | T3's database, read-only, every query bounded | Commands per thread, each thread's role and repository, Sumeet's typed messages |
| Transcript reader | Claude and Codex transcripts where they survive | Tokens per thread |
| Record reader | `.atelier/` at each repository's base, and its Git history | Reviews, findings, merges, items |
| Git reader | Each repository's history | Rework, record-only commits, line counts |
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
| Rework | Lines deleted within four weeks of being written, by `git blame` over the window |
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
  host; the measures above; the scripts; the report.
- Out: the work host; building any of this into the `atelier` binary; a page
  on the site; changing anything the measures describe.
- Target environment: this host, Linux, Python 3.

# Authority

The orchestrator may choose sample sizes at or above those stated, the
fallback defect measure, the scripts' structure, and how to tie threads to
items where the stated method fails. It may not change a measure's definition
in the product specification, write to T3's database, or change any
repository other than atelier.

# Risks and Unknowns

- **Token data is partial.** Token measures cover only threads whose
  transcripts survive. The report states the coverage beside each number.
- **T3's database format is internal.** The scripts name the schema they were
  written against and fail clearly on another.
- **Classification is judgment.** The rubric, the seed and the second rater's
  agreement are in the report so a later run can be compared fairly.
