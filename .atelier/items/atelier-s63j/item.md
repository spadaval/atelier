---
schema: atelier.item
schema_version: 3
id: atelier-s63j
title: Strip the old tracker's process text from replay fixtures of any era
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3A4ba032a9-6dcf-46e1-8710-fe3d178c213b
ready:
  by: 'the lead of atelier-kw8u: both change what the replayed reviews measure, so they come before the matrix'
  at: 2026-11-19T02:52:02.027431283Z
trivial: null
caused_by: null
created_at: 2026-11-19T02:50:32.559838815Z
---

## Goal

Taken from deferred finding atelier-z8rg/1-yrne.1, in .atelier/items/atelier-z8rg/reviews/1-yrne.md:

> Finding 1: Remove retained historical process instructions in a shared-setup follow-up
> Recommendation: defer
>
> `evals/agent-factory/README.md:123-128` correctly discloses that hierarchy-rules and priority-contract retain old process guidance. I verified their base trees: hierarchy-rules' AGENTS.md explicitly requires the removed `.agents/skills/agent-factory/SKILL.md` and obsolete tracker commands, while priority-contract also loads AGENTFACTORY.md with its own workflow instructions. `replay.strip_process` only removes three differently named headings, so these instructions survive in the fixtures. The retained priority-contract result records a failed `atelier issue show atelier-zivs` against today's tracker, consistent with the Report.
>
> Equal exposure in both arms does not remove the competing workflow instructions or the extra process guidance in the none arm. Extend the shared sanitization in a separate item and protect it with fixture assertions before treating these two cases as clean measurements of the current skill's contribution. This is deferred because changing the shared setup is explicitly outside this item's scope, the limitation is disclosed, and the recorded run still completes its independent review.

## Done when

- [x] A replay fixture holds none of the old tracker's process text, whatever headings its base's AGENTS.md uses, and no root-level guidance file of that tracker, such as `AGENTFACTORY.md`.
- [x] `replay_tests.py` asserts this for every replay case, and `hierarchy-rules` and `priority-contract` pass it.
- [x] The evals README no longer needs its note that those two cases show the old process.

## Scope

## Report

Replay fixtures no longer carry the old tracker's process, in any era. The
old strip removed three named AGENTS.md sections, which missed June 2026's
`Start Here` and `Common Commands` and the `## Missions` section four
October and November cases kept (`after-merge-checks`, `brief-transitions`,
`merge-sync`, `quiet-tokens`, naming `atelier work missions`). It also kept
`CONTRIBUTING.md` and `.github/PULL_REQUEST_TEMPLATE.md` in every fixture,
both pure old process ("Find or create the relevant Atelier issue", "Evidence
records"), and `priority-contract`'s `AGENTFACTORY.md`.

- `harness/replay.py`: `product_map` replaces `strip_process`. It ignores
  headings and keeps an AGENTS.md list item, paragraph or code line only if
  it names a file the fixture holds or a `cargo`/`git` command, and names no
  `atelier` command, tracker file (`.atelier`, `.agents`, `.claude`,
  `.codex`), Agent Factory skill or evidence. A heading stays only over what
  is left. `DROPPED` adds `AGENTFACTORY.md`, `CONTRIBUTING.md` and the pull
  request template, and `dropped()` matches a path in it.
- `harness/replay_tests.py`: a new test for every replay case asserts none of
  those files exists and no line of AGENTS.md or CLAUDE.md names the old
  tracker's commands, skill, files or evidence, and that AGENTS.md still
  holds `cargo nextest run` when the base's did. It fails on the old code for
  `hierarchy-rules` (the whole `Start Here`) and `after-merge-checks`
  (`Missions`, and `CONTRIBUTING.md` when that alone is put back).
- `harness/test_run.py`: `ProductMapTest` checks `product_map` on made-up
  headings, including a `#` comment inside a code block.
- `README.md`: the fixture description says what is left out, the note on
  the two June cases is gone, and the results paragraph says which recorded
  replays ran on fixtures that still held old process.

Evidence: `evals/agent-factory/test.sh` with the installed atelier 0.7.0
(the tree's version) passes, all harness tests and every scenario. I read
the rewritten AGENTS.md for each of the five base eras (June, July, two
October, November).

What the rule costs and leaves:

- AGENTS.md prose naming no file or command goes even when it is product
  guidance, such as June's "Do not add compatibility aliases" bullet. The
  docs it points at stay in the tree.
- Two October cases keep a `## Current Mission` heading over its two product
  pointers, the rebuild strategy and ADR 0019. Its tracker record line goes.
- The tree's own README.md and docs/ still describe that era's commands.
  They are the product under review, not guidance to the reviewer, so the
  fixture keeps them.
- The rule is a marker list. Process text in a future era that names none of
  the markers but does name a file or a cargo command would survive, and the
  per-case test checks the same kind of markers, so add any new marker to
  both.
- The results in `results/` were recorded on the old fixtures. They are not
  rerun here.

