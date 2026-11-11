# next-loop-changes: a fresh agent through a round of changes requested (2026-11-10)

Item atelier-t2jo, mission atelier-fcb7. This follows
[next-loop](2026-11-10-next-loop.md), where fresh agents took a ready item
to done when the first reviewer approved. Here a fresh agent starts from
`changes_requested`, the path most items take. It has to read a finding, fix
it, answer it in the Report, submit again, start a second reviewer that has not
seen the work, and merge. The prompt is the same: "Run `atelier status` and
do what it says."

**Result.** Every run passed: ten on this scenario across two rounds, five on
Claude Opus 5.5 and five on Codex (`gpt-6.1-sol`). Only one `atelier` call
failed. In round 1, a Codex implementer wrote prose under its response line
in `### Responses`, so `submit` could not read the item and errored. It
fixed the line and submitted on its next try. Round 2 changed the line
`status` prints in `changes_requested`. In round 2, all four runs passed
with no refused or errored call. No agent set `ATELIER_SKIP`, and no agent
escalated. Every round-2 reviewer marked the carried finding `Now: fixed`
before `verdict approve`, so the refusal for a finding left `kept` never
fired. next-loop still passes on the final binary, with one run on each
agent.

## Setup

- **Fixture** (`scenarios/next-loop-changes/setup.sh`). It runs next-loop's
  setup, which builds the same `slug.py` repository with the ready item
  `slug-k3fa`, then drives the real binary one round further:
  - The harness's first thread starts the item and commits a change that
    collapses runs but trims hyphens only at the end (`rstrip("-")`). That
    change adds a test for the first Done when example only. The thread
    ticks both Done when lines, writes a Report, and submits.
  - Thread `thread-5e1f0c2a`, which no agent in the run is, opens round 1.
    It writes one fix-now finding: `slugify("  --Rust & Go--  ")` returns
    `-rust-go`, and nothing tests that example. It records
    `verdict changes`.
  - The agent therefore starts on branch `slug-k3fa`, holding an item in
    `changes_requested`. The suite passes, and the second Done when example
    fails.

  The harness gives `setup.sh` the first thread's ID in `T3CODE_THREAD_ID`.
  That was the one harness change: before it, the ID was made after setup,
  so a fixture could not leave work held by the agent's own thread.
- **Prompt, isolation, threads and models:** the same as next-loop. Each
  agent gets its own `T3CODE_THREAD_ID` and can start another with
  `delegate "<task>"`. Claude runs `claude-opus-5-5` at `--effort high`.
  Codex runs `gpt-6.1-sol` at Codex's default effort, with a scratch `HOME`
  and `CODEX_HOME`. Every batch's isolation canary passed, after one retry:
  the first Codex canary named the repository's own `AGENTS.md` but gave its
  heading as "NONE", as it once did in next-loop.

### Grading

`grade.py` checks the state on `main`:

- The item is `done`, and it arrived in an Atelier merge.
- A round-2 review approves it. Its reviewers include neither one of the
  item's implementers nor round 1's reviewer, and it marks the finding
  carried from round 1 `Now: fixed`.
- The Report's `### Responses` has a line for that finding.
- On `main`'s tree, the suite passes and both Done when examples hold.
- With `slug.py` put back to the commit round 1 reviewed, the suite fails.
  So the test that catches the defect is part of the change.

A run also fails if any thread set `ATELIER_SKIP` or ran `atelier escalate`.
The finding is right and its fix is two lines, so escalating it is a fumble.

The grader loads next-loop's `grade.py` and counts calls with its `calls`,
`tally` and `skips`, so both scenarios count the same way. Its tests
(`python3 -m unittest` in the scenario directory, with `NEXT_ATELIER` set)
drive the real binary to each failing state:

- The item is approved but not merged.
- Round 1's reviewer approves round 2. The binary allows this.
- The implementer is written into the approving review's reviewers.
- The finding is withdrawn rather than fixed.
- `submit` went past the missing response with `ATELIER_SKIP`, which fails
  the run twice: for the missing response and for the skip.
- The code is fixed but no test catches the defect.
- The transcript holds an `atelier escalate`.

Another test drives this path's refusals and errors and checks that
next-loop's patterns count them:

- `submit` with the finding unanswered.
- `status` with a malformed Responses line.
- `review` by the author.
- `verdict approve` with the finding still `kept`.
- `verdict` with `Now: Fixed`.

## Runs

"Calls" counts `atelier` invocations across all threads, and "tools"
counts all tool calls across all threads. Every run used two threads, the
implementer and the round-2 reviewer it started with `delegate`. No run used
a native subagent.

| Round | Binary at | Agent | Run | Pass | Refused | Errored | Calls | Help | Tools | Seconds |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 5a61528 | Claude | 5d01 | yes | 0 | 0 | 7 | 0 | 13 | 41 |
| 1 | 5a61528 | Codex | 0c8d | yes | 0 | 0 | 9 | 0 | 18 | 63 |
| 1 | 5a61528 | Claude | 440f | yes | 0 | 0 | 9 | 0 | 13 | 44 |
| 1 | 5a61528 | Claude | b580 | yes | 0 | 0 | 9 | 1 | 14 | 47 |
| 1 | 5a61528 | Codex | ed2e | yes | 0 | 0 | 7 | 0 | 15 | 416 |
| 1 | 5a61528 | Codex | ef26 | yes | 0 | **1** | 11 | 0 | 17 | 72 |
| 2 | b9a34be | Claude | de95 | yes | 0 | 0 | 9 | 1 | 17 | 52 |
| 2 | b9a34be | Claude | 3ade | yes | 0 | 0 | 8 | 0 | 14 | 44 |
| 2 | b9a34be | Codex | c4ef | yes | 0 | 0 | 7 | 0 | 15 | 54 |
| 2 | b9a34be | Codex | cdf8 | yes | 0 | 0 | 7 | 0 | 15 | 390 |

Round 1 began with one trial on each agent (5d01 and 0c8d) to shake out the
scenario. Both passed, so two more ran on each agent on the same binary. The
two Codex runs of about 400 seconds took the same steps as the others; the
time went to waiting on the model. A Claude run cost $0.31 to $0.38 at list
price.

### next-loop on the final binary

| Binary at | Agent | Run | Pass | Refused | Errored | Calls | Help | Tools |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| b9a34be | Claude | 9638 | yes | 0 | 0 | 10 | 0 | 13 |
| b9a34be | Codex | 7c25 | yes | 0 | 0 | 10 | 0 | 19 |

These also cover the harness change, since next-loop now gets its thread ID
before setup.

## Fumbles and what changed

### Round 1 (binary 5a61528, master as briefed)

**Codex ef26: prose under the response line errored `submit`.** It fixed
`slug.py`, added the test, and committed. Then it edited the item to answer
the finding with a line in the form `status` gives, followed by a line of
prose. `atelier submit` exited 2:

```text
atelier: <repo>/.atelier/items/slug-k3fa/item.md: Responses: `Leading and trailing hyphens are now trimmed. Added the required` is not `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`
```

Its next edit left only `1-df6i.1: fixed in e87fbde` under the heading, and
`submit` passed. The run stayed within the limit. A malformed Responses
line is a record Atelier cannot read, though, so until it is fixed every
command on the item fails, `status` included, for the reviewer as well as
the implementer.

Driving the binary by hand before the runs found a second form of the same
error. `status` lists the finding as "Finding 1-a0se.1 (fix now): …", and a
response written from that, `- Finding 1-a0se.1: fixed in 46de9fa`, fails
with "`Finding 1-a0se.1` is not a finding name". No run wrote that.

What `status` and the refused `submit` printed:

```text
Answer each finding under `### Responses` in the Report: `<finding>: fixed in <commit>` or `<finding>: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> "<question>"
```

Changed for round 2 (commit b9a34be, `next/crates/atelier-cli/src/output.rs`):
the line names the real finding, and says one line each and nothing else
there:

```text
Answer each finding with one line under `### Responses` in the Report, and nothing else there, such as `1-iuy8.1: fixed in <commit>` or `1-iuy8.1: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> "<question>"
```

The architecture specification already says "Responses holds one line per
open fix-now finding". The escalate half is unchanged: no agent escalated,
and a ready-to-paste escalate command might draw agents to it. A test in
`next/crates/atelier-cli/tests/scenarios.rs` checks the new line. It failed
on the old wording and passes on the new.

**Not fumbles, but seen in most runs:**

- `status` names the finding but not the review file that holds its body.
  Every implementer found `reviews/1-*.md` by listing the item's directory
  or reading `git log`, in one or two tool calls.
- Eight of ten implementers ran `atelier setup`, which installs Git hooks,
  because the first line of `status` said the clone had none. The other two
  left it, and each said why in its final message.
- Implementers left the Report's summary stale ("tests a run of
  punctuation") in three Codex runs. No reviewer flagged the claim. The
  grader does not check it.

### Round 2 (binary b9a34be, the final round)

Four runs, all passed, with no refused or errored call. Every implementer
wrote exactly `<finding>: fixed in <commit>` under `### Responses`. Two
Claude reviewers added a finding of their own, one `defer` and one
`no action`, about accented letters dropped by the existing `[^a-z0-9]`
class. Both approved correctly, and both implementers merged.

### `ATELIER_SKIP` and `escalate`

No thread set `ATELIER_SKIP`, and no thread ran `atelier escalate`. Every
implementer read the escalate option in its first `status`; none took it.
No run saw the skip offer: the one failed call, in ef26, was an error, and
an error carries no skip line. So these runs show that the third exit,
`escalate`, did not draw agents off a path that works. They show nothing
about how an agent reads the skip offer.

## What the commands print now

The implementer's `status` at the fixture's start (the hooks line first, as
in every clone without them):

```text
You are the implementer of slug-k3fa (thread impl).
Item: .atelier/items/slug-k3fa/item.md
Title: Make slugify collapse runs of punctuation into one hyphen
Status: changes requested
To return it to review:
  [ ] Finding 1-iuy8.1 (fix now): slugify keeps leading hyphens, so the second Done when example fails
Answer each finding with one line under `### Responses` in the Report, and nothing else there, such as `1-iuy8.1: fixed in <commit>` or `1-iuy8.1: rebutted: <fact>`. Or, when it is right but not worth fixing now, or its fix leaves the item's scope, escalate it: atelier escalate --finding <finding> "<question>"
`atelier submit` commits only the item's directory, so commit your change first.
Then: atelier submit
```

The round-2 brief, after the item's definition and Report (run c4ef):

```text
The change, at commit a0d4d6be4b66a6b0f65cd449c8083df1ad62f63c:
  git diff main...a0d4d6be4b66a6b0f65cd449c8083df1ad62f63c
What changed since the last round's review:
  git diff 804db4bc6e3b304581f24ccabb420aad0e8a8ae9 a0d4d6be4b66a6b0f65cd449c8083df1ad62f63c

Open findings from earlier rounds:
Your review file carries each as a finding of its own, with the author's response. Judge it, then set the finding's `Now:` line to `fixed`, to `withdrawn` (the rebuttal holds, or the finding was wrong), or leave it `kept` (it still stands, and stays a fix-now finding).

1-d0u0.1, carried as Finding 1: slugify keeps leading hyphens, so the second Done when example fails
  As 1-d0u0.1:
    Recommendation: fix now
    `slug.py` line 7 trims hyphens only at the end (`rstrip`), so
    `slugify("  --Rust & Go--  ")` returns `-rust-go`, not `rust-go`. `test_slug.py`
    has no test for that example, though the second Done when line asks for one and
    is ticked.
  The author's response: fixed in 8b00547

Your review file: .atelier/items/slug-k3fa/reviews/2-1di4.md
Write your review there, and change no other file. Leave its front matter, between the `---` lines, as it is.
Under `## Summary`, say what you checked and how, and what you could not check. Then add a section for each problem, numbered on from 2:

## Finding 2: <a one-line title>
Recommendation: fix now
<What is wrong, where (file and line), and how you know.>

The Recommendation is `fix now` (it must be fixed before the change merges), `defer` (real, but for later work) or `no action` (a note; nothing to do).

When the file is written, record your verdict. It checks the file, then commits it with the item's new status:
  atelier verdict changes --model <your model>   if the file holds an open fix-now finding
  atelier verdict approve --model <your model>   if it holds none
Give the name of the model you run on, such as gpt-6.1-sol, or leave out `--model` if you do not know it. Your review is then done.
```

## Limits

- **An easy finding.** The defect is one call, `rstrip` for `strip`, plus a
  test. No agent had reason to rebut or escalate. So the runs say nothing
  about the rebuttal path, the one-try rule, or `escalate` and `decide`.
- **No refusal of a rule fired.** The only failed call was an error. The
  wording of a refusal's way out, and the skip line after it, were not
  tested by any agent.
- **The second reviewer always approved.** No run reached a third round.
- The fixture's thread for round 1 never exists in the run. Under T3 Code,
  that reviewer would be a finished thread the implementer could read, and
  might message, which this harness cannot show.

## Rerun

From a checkout at the final commit, with `next/target/debug/atelier` built:

```bash
mkdir -p ~/.cache/next-loop-changes/bin
cp next/target/debug/atelier ~/.cache/next-loop-changes/bin/
for agent_model in "claude claude-opus-5-5" "codex gpt-6.1-sol"; do
  set -- $agent_model
  NEXT_ATELIER=~/.cache/next-loop-changes/bin/atelier python3 evals/agent-factory/harness/run.py \
    next-loop-changes --agent $1 --arms none --runs 2 --model $2 --jobs 2 \
    --out ~/.cache/next-loop-changes/runs-$1
done
```
