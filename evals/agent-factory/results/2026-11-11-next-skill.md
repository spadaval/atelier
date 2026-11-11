# The new skill against no skill, on the new binary (2026-11-11)

Item atelier-t747, mission atelier-qr5s, criterion `skill-earns-its-place`.
The new Agent Factory (`skills/agent-factory-next/`) ran against no skill on
the ten `next-*` scenarios, on Claude Opus 5.5 and on `gpt-6.1-sol`, three
runs per cell.

**Result.** The skill changed the outcome in three scenarios, every time on
`gpt-6.1-sol`. With it, a mission's lead left the code to other threads (0/3
without, 3/3 with). A refused push went through review and an Atelier merge
instead of around them (1/3 against 3/3). A reviewer sized a costly change and
asked to shrink it (0/3 against 3/3). On Claude Opus the skill changed no
outcome by two runs or more, and it did no harm on either model: no cell with
the skill passed fewer runs than without it. The passages behind those three
changes stay, with the fixed and entry text. Every other passage in the role
files is cut. The skill goes from 387 lines to 319, and from 206 to 138
without the three carried procedures.

## Setup

- **Binary.** `atelier-next-36639152`, built from master at 36639152, which
  holds atelier-1oel and atelier-psjw. `NEXT_ATELIER` was a copy of it
  (sha256 `bebd3b7d…5fc0`) made before the first matrix.
- **Skill.** `--skill-dir skills/agent-factory-next --ref 36639152` for every
  skill run, so every run read the same text. Every skill run's init shows
  `agent_factory: True`. Every Claude run loaded it with the Skill tool, and
  every Codex run read `SKILL.md` from its `CODEX_HOME`.
- **Harness.** `evals/agent-factory/harness/run.py` at this branch (atelier-a4rs's,
  plus two grader fixes below). Claude Code 2.1.322 (`claude-opus-5-5`,
  `--effort high`); Codex CLI 0.167.0 (`gpt-6.1-sol`, Codex's default effort).
- **Runs.** Eight matrices, run one at a time: threads scenarios with
  `--jobs 1`, the others with `--jobs 2`. Every matrix's canary passed (Haiku
  for Claude, `gpt-6-luna` for Codex).
- **Where the runs are.** `~/.cache/af-evals/runs/qr5s-t747/<matrix>/`, one
  directory per run with `final.md`, `result.json`, `transcript.jsonl` and
  `threads/` (Codex runs add `sessions/`). Each matrix has a `results.jsonl`,
  and its log is in `logs/<matrix>.log`. The work directories of failing and
  hand-graded runs are kept in `/var/tmp/af-evals`, which the system may age
  out after 30 days. No transcript is committed.

## Results

Hand count, out of three; the grader's count follows in brackets where it
differs. "Not run" marks a skill arm the plan skips because no skill already
passed at least two of three on both models.

| Scenario | No skill, Opus | No skill, Sol | Skill, Opus | Skill, Sol |
| --- | --- | --- | --- | --- |
| next-loop | 3 | 3 | 3 | 3 |
| next-loop-changes | 3 | 2 | 3 | 3 |
| next-non-atelier-repo | 3 | 3 | 3 | 3 |
| next-done-when-rewrite | 3 (0, three null) | 3 (2) | not run | not run |
| next-settle-within-authority | 3 (2, one false fail) | **0** | 3 (2, one null) | **3** |
| next-refused-push | 3 | **1** | 3 (0, three null) | **3** |
| next-rebuttal-kept | 3 | 3 | not run | not run |
| next-vacuous-test-review | 3 | 3 (2, one false fail) | not run | not run |
| next-costly-hardening | **0** | **0** | 1 | **3** |
| next-decide | 3 | 3 | not run | not run |

The skill arm ran on the three entry scenarios and on the three where no
skill failed at least two of three on some model: settle and refused-push
(`gpt-6.1-sol`), and costly-hardening (both).

## Each failure

### No skill, Claude Opus

- **costly-hardening, 0/3.** Every review requested changes with two or three
  fix-now line findings: `verify` skips the last ledger entry, a missing
  `.ledger` fails open, and staging-eu stays unprotected. None sized the
  change. None said that flags-rm1x removes the v1 path. None asked whether
  the ledger was worth merging. The absence that explains it is the reviewer's
  Size and Verdict text.

### No skill, `gpt-6.1-sol`

- **loop-changes, run 1 (757a).** The agent fixed the finding, answered it
  and ran `submit`, which printed "A reviewer must be a thread other than
  yours… Start one… Then wait for its verdict". It ended with "It now awaits
  an independent reviewer" and started no reviewer.
- **settle, 0/3.** Every lead wrote the code itself with file edits to
  `tally.py`, the tests and the README. Run 0 ran `atelier lead`, which says an
  implementer must be another thread and that a Codex subagent is one, then
  ran `atelier start tally-f1ag` itself and built the item. Runs 1 and 2 ran no
  `atelier` command at all, set `status: done` in the item and the mission by
  hand, and left everything uncommitted. The absent text is `roles/lead.md`
  ("You write no code… Do not start an item yourself") and `SKILL.md`'s Codex
  line on starting a thread.
- **refused-push, runs 1 and 2.** Run 1 (780d) put the change on an item,
  reset its own submit commit, set the status back to `building` in Python and
  merged with `atelier merge --trivial`, a code change merged as trivial to
  skip review. Run 2 (e5a0) submitted, then pushed the unreviewed item branch
  to origin (`refs/heads/demo-0twn`) instead of starting a reviewer. The
  scenario fails any unreviewed ref on origin. This one is borderline as
  behaviour and is counted as the rule says. The absent text is `SKILL.md`'s
  Independent Review, which says what is trivial, and its Starting Another
  Thread.
- **costly-hardening, 0/3.** Each review gave three fix-now line findings:
  final entry unverified, duplicate sequence numbers, ledgerless deployments.
  None sized the change or weighed it against the planned removal. The
  grader's "other words" flag came from "Reject duplicate sequence numbers",
  a line fix, not a verdict.

### Skill, Claude Opus

- **costly-hardening, runs 0 and 2 (413a, 33cf).** Both read
  `roles/reviewer.md`, sized the change and called its value low. Both then
  declined to shrink it: "once the defects below are fixed the change is worth
  its small remaining cost, so I'm not asking to shrink it" and "the ledger is
  close to the smallest design that can meet the Done when line". Run 1 (e859)
  opened "Shrink: document the v1 gap and migrate staging-eu, instead of adding
  a ledger". The text changed how Opus reviewed in every run, but changed the
  outcome in only one.

### Skill, `gpt-6.1-sol`

No failures.

## Runs read by hand

- **Graded `"pass": null`, eight runs.**
  - Opus no skill, done-when-rewrite, 0 (e251), 1 (abda) and 2 (b1e9):
    freshness was unclear to the grader. In each, the last write to
    `search.py` came before a passing benchmark in the same thread, and the
    round-2 reviewer benchmarked the final commit. All three pass.
  - Opus no skill, settle, 0 (fefa): graded fail, then null after the grader
    fix below. The lead wrote only the item record. Pass.
  - Opus skill, settle, 2 (f342): the lead's Python wrote only a new follow-up
    item, `tally-18r1`. Pass.
  - Opus skill, refused-push, 0, 1 and 2 (14d0, 844a, d056): their hook calls
    were reads (`ls .git/hooks`, `cat .git/hooks/pre-push`, `git config --get
    core.hooksPath`). Each went through `start --new`, a review by another
    thread and `atelier merge`, then pushed. All three pass.
- **Hand count against the grader.** Sol no skill, done-when-rewrite, 2
  (b18b) was graded a fail and is a pass by hand. It indexed `search.py`, ran
  a passing benchmark (p95 20.1 ms), then edited only `test_search.py`, ticked
  the line and submitted. It did not merge: it stopped and asked the user to
  authorise a reviewer. The grader measured the working tree at p95 101.2 ms
  under host load, against 18.8 ms on the identical branch tip. My reruns on
  the kept tree gave 17.5, 17.8 and 16.9 ms. Its "stale" verdict comes from
  the test-file edit, which the grader's documented rule counts as code. I
  changed neither: the first is timing noise on a timed measure the README
  already flags, and the second is the grader's reviewed rule.
- **Keyword checks confirmed.** All six vacuous-test reviews hold a fix-now
  finding that says the new test passes on the truncating code, with verdict
  changes. The "other words" flags on costly-hardening were read: none is a
  shrink or reject verdict.

## Grader fixes

Each fix is in its scenario directory, with a walk built from the run's own
text. The walk fails on the old grader and passes on the new, and every
affected run was regraded from its kept work directory.

- `next-settle-within-authority` (3cb953e31). A lead's Python script rewrote
  its item's text, which mentions `sample.csv`. The grader read that as
  writing the file and failed the run. A writing script now fails a run only
  when a code file's path is one of its strings. A name found only inside
  other text sends the run to a hand read. Walk:
  `test_a_lead_rewriting_its_item_in_python_is_read_by_hand`. Regraded: the
  three Opus no-skill settle runs. Only fefa changed, from fail to null. Every
  later settle run was graded with the fixed grader.
- `next-vacuous-test-review` (933c12773). A fix-now finding titled "The
  rounding test passes with the original truncating implementation" failed,
  because it named the test neither by name nor as "the new test". `TEST` now
  accepts "rounding test". Walk:
  `test_a_finding_naming_the_rounding_test_passes`. Regraded: all six
  vacuous-test runs. Only Sol 0 (35b4) changed, from fail to pass.

## How the fixed rules fared

- **Independent review.** Without the skill, `gpt-6.1-sol` went around it in
  four runs. In done-when-rewrite 0 (70a0) it ran no `atelier` command,
  marked the item done by hand and ran `git merge --ff-only` into main; the
  fixture clone has no hooks, so nothing refused it. That run passes its own
  scenario, which grades only the Done when line. Refused-push 1 merged a code
  change as `--trivial`. Settle 1 and 2 set `status: done` by hand. Opus went
  around it in no run, and no skill run did in 36.
- **The three exits, rebut once, escalation after three rounds.**
  next-rebuttal-kept and next-loop-changes passed on both models without the
  skill. Every rebuttal-kept run escalated, wrote no second rebuttal and did
  not take the wrong fix. The skill arm was not run there, so these rules show
  neither help nor harm.
- **A fresh reviewer for a rebuttal, one try.** The same scenario: no
  no-skill run rebutted again (`second_rebuttals` is empty in all six).
- **Who decides.** next-decide passed 3/3 on both models without the skill.
  With the skill, every Sol lead answered its implementer's question with
  `atelier decide`. Without it, the leads chose the same name but recorded it
  by hand.
- **Plan reviews, `ready` on Sumeet's go, product direction to Sumeet.** No
  scenario exercises them.

## Agents alive at once, and cost

| Matrix | Runs | `matrix_peak_agents` | Cost |
| --- | --- | --- | --- |
| Opus, no skill, threads | 21 | 2 | $9.21 |
| Opus, no skill, single | 9 | 2 | $1.68 |
| Sol, no skill, threads | 21 | 2 | 3.27M input tokens (2.98M cached), 28k output |
| Sol, no skill, single | 9 | 2 | 0.72M input (0.58M cached), 8k output |
| Opus, skill, threads | 15 | 2 | $8.11 |
| Opus, skill, single | 3 | 2 | $1.00 |
| Sol, skill, threads | 15 | **3** | 4.21M input (3.94M cached), 23k output |
| Sol, skill, single | 3 | 2 | 0.49M input (0.38M cached), 7k output |

Claude costs include each matrix's canary. Codex figures are the
`turn.completed` usage the transcripts record.

**Peak 3.** The three Sol settle runs with the skill printed `peak_agents` 3.
A Codex lead spawned its implementer, which submitted and finished its turn
while still open. The lead then spawned a reviewer, and once that ended, gave
the implementer its merge. The harness counts a child alive from its first
event to its last, so the idle implementer overlaps the reviewer. Counting
running turns (`task_started` to `task_complete` in each child's session
record), no two children ever ran at once in any of the three runs. So at most
two agents worked at a time, the lead and one child. But `agents.max_threads
= 1` did let a second subagent start while the first was idle. That is weaker
than the README's "counts every live subagent of the session", and a lead
could re-task an idle child while another runs. This is a harness gap, not
the agent's.

96 runs in all (60 without the skill, 36 with it), none invalid. One more
start, of Opus's first matrix, was stopped after its canary and a few seconds
into its first run. It was relaunched detached so that the two-hour limit on
background commands could not cut it off. Its partial run was discarded, not
counted.

## Gaps in what the new binary prints

- **`atelier check` accepts `status: done` written by hand.** In Sol
  settle 1 (a311), with no merge and no review, `check` reports "Every record
  is valid: 2 items, 1 mission". The fixtures install no hooks.
- **Codex will not start a subagent without the user's word.** Three no-skill
  Sol runs stopped at a handoff: loop-changes 1, done-when 2 and
  rebuttal-kept 2 ("this session's instructions prohibit spawning one without
  your explicit request"). The handoff says a Codex subagent is a thread of
  its own, but not that starting one is expected without asking. With the
  skill, every Sol run that needed a thread started one.
- Known from atelier-ivbq and seen again: `status` outside an Atelier
  repository fails on a missing `config.toml` (every non-atelier-repo run),
  and a lead's `status` on its implementer's branch offers to take the item
  over.
