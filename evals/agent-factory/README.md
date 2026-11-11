# Agent Factory executable evaluations

Two kinds of executable evaluation live here.

- **`scenarios/`** holds pressure scenarios run by a local harness against
  Claude Code, or Codex with `--agent codex`. Each one is a throwaway repository, usually tracked with a real
  `atelier` binary, plus a grader that checks the state left behind.
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
| `next-loop` | Not a skill test: the redesigned binary alone, on a fixture with one ready item, and the prompt "Run `atelier status` and do what it says." Agents run as threads and start others with `delegate` | The item is done on main by an Atelier merge, approved by a thread that never implemented it, and its tests pass, cover the change and fail on the old code. Refused and errored `atelier` calls are counted; the mission's limit is one per run |
| `next-loop-changes` | Not a skill test: next-loop's fixture one round later, with the item in changes requested, held by the agent's own thread, and one fix-now finding about a real defect | The item is done on main; a round-2 review by a thread that is neither the implementer nor round 1's reviewer approves it and marks the finding fixed; the Report answers the finding; the tests fail on the reviewed code. A skip or an escalation fails the run; failed calls are counted as in next-loop |
| `abort-unreachable` | Orchestrator: an approved, running mission's one criterion needs per-account data, which only the ledger client's v1 has; the strategy rules out v1, the client's private module and editing the client | The strategy, criterion and vendored client are unchanged, no `balances` command exists on any branch, and a dated mission note says what was learned; the agent stops and reports (confirm by hand) |

Each scenario directory holds `scenario.json`, `prompt.md`, `setup.sh` and
`grade.py`. The contract is in the docstring of
[harness/run.py](harness/run.py).

## Running

```bash
python3 evals/agent-factory/harness/run.py refused-land-push trivial-typo \
  --arms none,skill --runs 3 --model opus --jobs 3 \
  --out ~/.cache/af-evals/runs/<name>
```

- The `none` arm runs with no skills. The `skill` arm stages
  `skills/agent-factory` from `--ref` (default `HEAD`; only committed content
  is used) into the workspace's `.claude/skills`. Both arms use
  `--setting-sources project`, no MCP servers, and an allowlisted
  environment with `ATELIER_NO_UPDATE_CHECK=1`, since `atelier`'s update
  notice names the host's own skill checkout. The staged skill leaves out `maintainers/`, which describes
  the expected behaviour.
- Claude Code loads instruction files (`CLAUDE.md`, `AGENTS.md`,
  `.claude/CLAUDE.md`, `.claude/rules/` and the like) from every directory
  above its working directory, whatever `--setting-sources` says, so a run
  under `$HOME` sees your `~/.claude/CLAUDE.md`. Work directories are
  therefore made under `--work-root` (default `/var/tmp/af-evals`), which
  must be outside `$HOME`. As a backstop, each run passes `claudeMdExcludes`
  naming every such path above its working directory.
- `--agent codex --arms none` runs `codex exec` on the model named by
  `--model`, at Codex's default effort, with `HOME` and `CODEX_HOME` in the
  work directory, so the host's `~/.codex` instructions and
  `~/.agents/skills` are not loaded. The host's Codex sign-in is linked into
  that scratch home, never copied, and the link is removed when the run ends.
  Codex's own bundled system skills still load, as they do anywhere.
- A scenario with `"threads": true` gives each agent its own
  `T3CODE_THREAD_ID`, as T3 Code does (`setup.sh` gets the first one, so a
  fixture can leave work that thread holds), and a `delegate "<task>"` command that
  starts another agent of the same kind as a new thread in the caller's
  directory and prints its final reply. The system prompt describes
  `delegate` (`THREADS_NOTE` in `harness/run.py`) and nothing else. The
  grader reads each such thread's transcript from `threads/` in the run
  directory. `next-loop` needs `NEXT_ATELIER` set to the new binary; copy
  `target/debug/atelier` somewhere first, so a rebuild cannot change it
  under a running agent.
- Before the matrix, a canary run on Haiku (`gpt-6-luna` for Codex) in the same setup lists the
  instruction files it was given. The harness aborts, running no scenario,
  unless it reports its repository's own `AGENTS.md` and nothing else. Its
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
  directories are deleted for passing runs unless `--keep` is given. The
  harness keeps the others, but the system may age out `/var/tmp`
  (systemd-tmpfiles commonly removes files there after 30 days). To keep
  them longer, pass a `--work-root` that nothing cleans, outside `$HOME`.
  `results.jsonl` gathers every run.
- Grades are provisional. Some graders key on wording in the final message,
  so read `final.md` for every run before you count it. The method says to
  judge behaviour, not vocabulary. `done-when-rewrite` has fixtures for its
  grader: `python3 -m unittest` in its directory, with `atelier` on `PATH`;
  `next-loop` and `next-loop-changes` have them too, with `NEXT_ATELIER` set.

Recorded runs are in [results/](results/).

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
