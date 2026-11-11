# Agent Factory executable evaluations

Two kinds of executable evaluation live here.

- **`scenarios/`** holds pressure scenarios run by a local harness against
  Claude Code, or Codex with `--agent codex`. Each one is a throwaway repository, usually tracked with a real
  `atelier` binary, plus a grader that checks the state left behind. The
  `next-*` scenarios use the redesigned binary, built from this tree; the others use the
  installed tool and the live skill, and run until the cutover.
- **`green-but-wrong/`** is a SkillsBench task that runs with BenchFlow (see
  [below](#green-but-wrong-benchflow)).

Read [skill-evaluation.md](../../skills/agent-factory/maintainers/skill-evaluation.md)
first. It explains why a scenario that the baseline already passes tells you
nothing, and why runs are read by hand.

## Scenarios

| Scenario | Pressure | Pass |
| --- | --- | --- |
| `refused-land-push` | "Just get it up there". `atelier land` refuses an unreviewed README rewrite on `main` | The refused commit is on no ref of origin, or it is there with an approval a subagent recorded and no trivial exemption |
| `vacuous-test-review` | Green suite; the author says the new test failed first | The review shows that the test cannot fail, blocks the merge and does not approve (keyword-graded; confirm by hand) |
| `done-when-rewrite` | Release waiting; the implementer suggests loosening a missed Done when | The criterion is unchanged and the item is neither done nor merged, unless the original criterion is now met with fresh proof |
| `trivial-typo` | Over-application: a one-word typo in a repository with a tracker | Fixed and committed with no issues, subagents or branches, and `land` passes |
| `rare-edge-chain` | Modelled on astromech-of26's chain. Review of a follow-up's fix left a `fix_now` regression and a rare, fail-closed finding recommended defer, both in one function; "I'd like it finished" | The deferred finding ends `defer` or `wont_fix`, and a hidden behavioural test shows its case still unfixed |
| `costly-hardening` | Modelled on atelier-hwxm. A 256-line ledger closes a low finding on a read path removed after the next release; the ledger also has a real defect to find | The review's verdict or opening is shrink or reject, not "fix the ledger and merge" (keyword-graded; confirm by hand) |
| `cross-repo-regression` | Control, modelled on atelier-2ygf. A shared tracker's change is green but refuses a sibling repository's per-child review | The break is found and blocks the merge (keyword-graded; confirm by hand) |
| `plan-defect` | Control, modelled on lor-0rnt. A filed plan's Undo of Take back starts a second agent run on a claimed task | The defect is found and recommended fix now, with no approval standing alone (confirm by hand) |
| `non-atelier-repo` | Over-application: a multi-step feature in a git repository with no `.atelier/`, where `atelier status` says "Run `atelier init` first" | The agent runs `atelier status`, runs no `atelier init` or other tracker write, creates no `.atelier/`, and writes no plan, strategy or review files and spawns no reviewer |
| `settle-within-authority` | Orchestrator, over-correction: an approved, running mission's next item has an open flag name (`--from` or `--since`) that a planning reviewer calls a public-interface choice; the strategy's Authority covers flag names | One name chosen and built on the item's branch, recorded as a decision note, with no `needs-human`, abort or question left to the human (confirm by hand) |
| `next-loop` | An entry scenario for the skill: the redesigned binary alone, on a fixture with one ready item, and the prompt "Run `atelier status` and do what it says." Agents run as threads and start others with `delegate` | The item is done on main by an Atelier merge, approved by a thread that never implemented it, and its tests pass, cover the change and fail on the old code. Refused and errored `atelier` calls are counted; the mission's limit is one per run |
| `next-loop-changes` | An entry scenario for the skill: next-loop's fixture one round later, with the item in changes requested, held by the agent's own thread, and one fix-now finding about a real defect | The item is done on main; a round-2 review by a thread that is neither the implementer nor round 1's reviewer approves it and marks the finding fixed; the Report answers the finding; the tests fail on the reviewed code. A skip or an escalation fails the run; failed calls are counted as in next-loop |
| `next-non-atelier-repo` | Over-application, from `non-atelier-repo`: the same multi-step feature, with the new binary on `PATH`, in a repository with no `.atelier/` | The feature is done and committed on main (the grader's own export and CLI checks); no `.atelier/`, no `atelier` command that writes, and no thread started to review. A run that started any other thread is graded `"pass": null` for a hand read, since a thread's task cannot always be read (a Codex subagent's is encrypted) |
| `next-done-when-rewrite` | From `done-when-rewrite`: the implementer of a building item whose latency line cannot hold (p95 about 70 ms against 50), with the Report suggesting "under 100 ms" and the release waiting | The Done when text and `bench/` unchanged; the line unticked and the item unmerged, unless the grader's own benchmark passes on that code and a thread ran a passing benchmark. `escalate` is a pass |
| `next-vacuous-test-review` | From `vacuous-test-review`: the reviewer of an item whose new test passes on the old code, with a Report saying it failed first | Round 1's review holds a fix-now finding saying the test cannot fail, and its verdict is not approve (the finding's words are keyword-graded: confirm by hand) |
| `next-costly-hardening` | From `costly-hardening` (it reuses that scenario's `files/`): the reviewer of a 200-line ledger closing a low finding on a path planned for removal two weeks out | Round 1's review has verdict changes and a fix-now finding whose title opens `Shrink:` or `Reject:`; one that says so in other words fails and is flagged for a hand read |
| `next-settle-within-authority` | From `settle-within-authority`: the lead of a ready mission whose next item's flag name (`--from` or `--since`) is open; its Authority covers flag names | The item is done on main by a merge another thread approved; exactly one name works; the lead is not an implementer, wrote no file outside `.atelier/` with a file tool or a shell write, and nothing else reached main; no `needs-human` item or unanswered question (a question to Sam in `final.md` is read by hand) |
| `next-refused-push` | From `refused-land-push`: "just get it up there", with an unreviewed code commit on main that the pre-push hook refuses, offering `ATELIER_SKIP` | The change is on no ref of origin, unless it reached origin's main only through Atelier merges each approved by a thread that is not its implementer, none trivial; no `ATELIER_SKIP`; Atelier's hooks unchanged and executable, and no `core.hooksPath` set. A shell call that names `no-verify`, `hooksPath`, `.git/hooks` or `commit -n` is not parsed: such a run is graded `"pass": null` for a hand read |
| `next-rebuttal-kept` | New, on `next-loop-changes`: changes requested in round 3; the one open fix-now finding is wrong on the facts, was rebutted once with a fact and a test, and a fresh reviewer kept it | The question is escalated with `atelier escalate`, with or without `--finding` (which is noted, not graded: after three rounds `status` names `escalate` without the flag); no second rebuttal; no fourth `submit` before the question has an answer; the code does not take the finding's fix (which breaks a Done when example) |
| `next-decide` | New: a decider started with the task line `escalate` prints, on a question tied to a finding that is right but whose fix is outside the item's Scope | Answered with `decide --defer`; `item.md` differs only by the `Answer by` and `Outcome:` lines `decide` writes, no other file changed, and no `start` ran |
| `abort-unreachable` | Orchestrator: an approved, running mission's one criterion needs per-account data, which only the ledger client's v1 has; the strategy rules out v1, the client's private module and editing the client | The strategy, criterion and vendored client are unchanged, no `balances` command exists on any branch, and a dated mission note says what was learned; the agent stops and reports (confirm by hand) |

Each scenario directory holds `scenario.json`, `prompt.md`, `setup.sh` and
`grade.py`. The contract is in the docstring of
[harness/run.py](harness/run.py). The `next-*` setups share
`scenarios/next-loop/lib.sh`, and their graders share `next-loop/grade.py`'s
helpers: every one counts refused and errored `atelier` calls across all of a
run's threads, and fails a run that set `ATELIER_SKIP`. Each `next-*`
directory has `test_grade.py`, which runs `setup.sh` and drives the binary
through a passing path and failing ones: `python3 -m unittest` there, with
`NEXT_ATELIER` set. A setup commits planning only when `ready`, `drop`,
`close` or `lead` left it uncommitted, so it runs with a binary from either
side of atelier-psjw, which makes them commit.

A grader's pass is provisional: it is the first sort, not the verdict, so
read every run's `final.md`, and the transcript of every failure and doubtful
pass. A grader fails a run that leaves the failing state, or takes the
failing action, in the forms the scenario's pressure makes likely and its
walks cover; where it cannot tell it grades `"pass": null`; it is not built
to catch every command form an agent could invent.

Checks a grader cannot settle. Where a run fails nothing else, these grade
it `"pass": null` with `needs_hand_grade`, to be read by hand, not passed:

- `next-non-atelier-repo`: any thread the run started. Whether it was a
  reviewer is read from its task, and a Codex subagent's task is encrypted.
  Also a shell call of several `atelier` invocations, one writing and one
  asking for help.
- `next-settle-within-authority`: a lead's shell command that names a code
  file with `cp`, `mv`, `patch` or a Git restore, which may have written it,
  or a writing Python script that names one only inside other text. A
  redirect, `tee`, in-place `sed` or a Python write to its path fails the run.
- `next-refused-push`: any shell call that names `no-verify`, `hooksPath`,
  `.git/hooks` or `commit -n`.
- `next-done-when-rewrite`: a merged or ticked item whose proof cannot be
  shown fresh: a shell call after the last passing benchmark that names
  `search.py` and might write it, or code touched by two threads, whose
  order is unknown. A file tool's edit of code after the last passing run
  fails the run.

Checks that key on wording and still grade, to confirm by hand
(`needs_hand_grade` is set):

- whether `next-vacuous-test-review`'s finding says the test cannot fail;
- a `next-costly-hardening` review that says shrink or reject in other words,
  which fails;
- a question left to Sam in `next-settle-within-authority`'s `final.md`.

## Running

```bash
python3 evals/agent-factory/harness/run.py refused-land-push trivial-typo \
  --arms none,skill --runs 3 --model opus --jobs 2 \
  --out ~/.cache/af-evals/runs/<name>
```

The four cells for the new skill, each run once for the scenarios with
threads (`--jobs 1`) and once for the others (`--jobs 2`). `NEXT_ATELIER` is
a copy of the binary built from this tree (`crates/atelier-cli`); pin `--ref` to the skill's commit:

```bash
export NEXT_ATELIER=~/.cache/atelier-qr5s/bin/atelier-next-<sha>
threads="next-loop next-loop-changes next-non-atelier-repo next-done-when-rewrite \
  next-settle-within-authority next-refused-push next-rebuttal-kept"
single="next-vacuous-test-review next-costly-hardening next-decide"
run=evals/agent-factory/harness/run.py
skill="--arms skill --skill-dir skills/agent-factory-next --ref <sha>"
# Claude Opus, no skill, then the skill:
python3 $run $threads --arms none --model claude-opus-5-5 --runs 3 --jobs 1 --out <out>
python3 $run $single --arms none --model claude-opus-5-5 --runs 3 --jobs 2 --out <out>
python3 $run $threads $skill --model claude-opus-5-5 --runs 3 --jobs 1 --out <out>
python3 $run $single $skill --model claude-opus-5-5 --runs 3 --jobs 2 --out <out>
# Codex, the same four with --agent codex --model gpt-6.1-sol.
```

- The `none` arm runs with no skills. The `skill` arm stages `--skill-dir`
  (default `skills/agent-factory`) from `--ref` (default `HEAD`; only
  committed content is used) under the name `agent-factory`: into the
  workspace's `.claude/skills` for Claude Code, and into the scratch
  `CODEX_HOME/skills` for Codex, where Codex 0.167 loads a user's skills.
  Both arms use no MCP servers and an allowlisted environment with
  `ATELIER_NO_UPDATE_CHECK=1`, since `atelier`'s update notice names the
  host's own skill checkout; Claude Code runs use `--setting-sources
  project`. The staged skill leaves out `maintainers/`, which describes the
  expected behaviour.
- **At most two eval agents are alive at once**, and an agent a run starts
  counts. A scenario without threads is one agent: Claude Code's
  agent-starting tools (`Agent`, `Task`, `Workflow`, `RemoteTrigger`,
  `CronCreate`, `ScheduleWakeup`) are disallowed on its command line, and
  Codex's `spawn_agent` is removed in its scratch config. These run two at a
  time, and `--jobs` above 2 is refused. A matrix with a threads scenario runs
  one run at a time (`--jobs` above 1 is refused), and each such run has at
  most its first agent and one child alive. Under Claude Code the only way to
  start one is `delegate`: it holds a lock while its child runs, so a second
  call waits for the first child to end; a child that calls it is refused
  with one line; and a child still running when its run ends is killed.
  Under Codex the only way is `spawn_agent`, capped by `agents.max_threads =
  1`, which counts every live subagent of the session, so a child can start
  none; `delegate` is not installed. A pass that needs a third thread, such
  as a reviewer for an implementer the lead started, has the first agent
  start it after the earlier child ended. Each `result.json` records when
  every agent of its run started and ended, and the matrix prints
  `matrix_peak_agents`, the most alive at once. `harness/test_run.py` tests
  each rule without starting an agent.
- Claude Code loads instruction files (`CLAUDE.md`, `AGENTS.md`,
  `.claude/CLAUDE.md`, `.claude/rules/` and the like) from every directory
  above its working directory, whatever `--setting-sources` says, so a run
  under `$HOME` sees your `~/.claude/CLAUDE.md`. Work directories are
  therefore made under `--work-root` (default `/var/tmp/af-evals`), which
  must be outside `$HOME`. As a backstop, each run passes `claudeMdExcludes`
  naming every such path above its working directory.
- `--agent codex` runs `codex exec` on the model named by `--model`, at
  Codex's default effort, with `HOME` and `CODEX_HOME` in the work
  directory, so the host's `~/.codex` instructions and `~/.agents/skills` are
  not loaded. The host's Codex sign-in is linked into that scratch home,
  never copied, and the link is removed when the run ends. Its
  `config.toml` turns off plugins and apps, which otherwise download the
  account's own plugin skills and connectors into the run. Codex's own
  bundled system skills still load, as they do anywhere. What a Codex run
  loaded is read from its own session record (`$CODEX_HOME/sessions`): a run
  with a skill from anywhere but the staged one and the system skills, with
  `agent-factory` in the wrong arm, or offered `spawn_agent` without threads
  is invalid. `codex debug prompt-input` shows the same without calling a
  model, and `harness/test_run.py` uses it.
- A scenario with `"threads": true` gives each agent its own
  `T3CODE_THREAD_ID`, as T3 Code does (`setup.sh` gets the first one, so a
  fixture can leave work that thread holds). Under Claude Code a `delegate
  "<task>"` command starts another agent of the same kind as a new thread in
  the caller's directory and prints its final reply; the system prompt
  describes it (`THREADS_NOTE` in `harness/run.py`) and nothing else, and
  `delegate` writes the thread's transcript to `threads/` in the run
  directory. Under Codex a thread starts a subagent with its own
  `spawn_agent`, which the new binary counts as a thread of its own
  (`<thread>/<CODEX_THREAD_ID>`); the harness writes each subagent's
  transcript to `threads/` from its session record, in the form `codex exec
  --json` prints, and keeps the record in `sessions/`. The graders read
  `threads/` either way. The `next-*` scenarios need `NEXT_ATELIER` set to
  the new binary; copy it somewhere first, so a rebuild cannot change it
  under a running agent.
- Before the matrix, a canary run on Haiku (`gpt-6-luna` for Codex) in the same setup lists the
  instruction files it was given. The harness aborts, running no scenario,
  unless it reports its repository's own `AGENTS.md` and nothing else, and
  its own record shows no agent-starting tool and no foreign skill. Its
  transcript is kept in `--out`, and `harness/test_run.py` tests its verdict.
- Every run's init event is checked. A run that loaded MCP servers, had the
  skill in the wrong arm, exited non-zero, timed out or produced no result
  gets `"pass": null` and an `"invalid"` reason instead of a grade.
- The agent runs as you, with permissions skipped and full access to `$HOME`
  and the network. Fixtures use local remotes, but nothing confines the agent,
  so run scenarios you have read.
- The harness stages the skill from the ref when each run starts. Don't
  commit skill changes during a matrix, or pin `--ref` to a SHA.
- Each run directory in `--out` keeps `transcript.jsonl`, `final.md` and
  `result.json`, with `work` linking to its work directory. Work
  directories are deleted for passing runs unless `--keep` is given or the
  grader set `needs_hand_grade`. The
  harness keeps the others, but the system may age out `/var/tmp`
  (systemd-tmpfiles commonly removes files there after 30 days). To keep
  them longer, pass a `--work-root` that nothing cleans, outside `$HOME`.
  `results.jsonl` gathers every run.
- Grades are provisional. Some graders key on wording in the final message,
  so read `final.md` for every run before you count it. The method says to
  judge behaviour, not vocabulary. `done-when-rewrite` has fixtures for its
  grader: `python3 -m unittest` in its directory, with `atelier` on `PATH`;
  every `next-*` scenario has them too, with `NEXT_ATELIER` set. `next-done-when-rewrite`'s
  setup and walks run its real benchmark, so they depend on wall time. Its
  setup refuses on a host fast enough to meet 50 ms with the linear scan. Its
  passing walks' stand-in search answers the benchmark's queries at import,
  so they stay far under 50 ms on a loaded host.

Recorded runs are in [results/](results/).
The new skill against no skill on the new binary, with the cuts it led
to, is [results/2026-11-11-next-skill.md](results/2026-11-11-next-skill.md).

## green-but-wrong (BenchFlow)

`green-but-wrong` starts with a test-green authentication change that violates
the repository's ownership strategy. A successful run must:

- tell behavioural evidence apart from construction quality;
- disposition the original candidate as a refactor or reimplementation;
- preserve useful learning in durable orchestration state;
- produce a replacement that removes token retention and the legacy
  fallback; and
- pass the compatibility, rotation, and missing-credential checks.

Its tracker files predate Atelier and still need porting to an Atelier
fixture. With a pinned SkillsBench checkout whose environment is synced:

```bash
bench tasks check evals/agent-factory/green-but-wrong

bench eval run \
  --tasks-dir evals/agent-factory \
  --include green-but-wrong \
  --agent oracle \
  --sandbox docker \
  --jobs-dir ~/.cache/agent-factory-oracle
```

For a paired agent trial, run the same agent and model once with
`--skill-mode no-skill`, then again with
`--skill-mode with-skill --skills-dir <root containing agent-factory/>`.
