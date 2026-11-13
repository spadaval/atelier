# Atelier: product specification

This page says what Atelier is for and what it does. How it is built is in
the [design record](design.md), which also lists what was tried and removed.
Sumeet set the direction in conversation on 2026-11-09 after an assessment
of the project (`/var/tmp/atelier-assessment/reports/`). The tool was
rewritten to this page, and every repository on this host moved onto it on
2026-11-11.

On 2026-11-13 Sumeet set a second direction, the branch model of mission
atelier-to83: only a merge writes the base, and every change, code or
records, is made on a branch. This page describes that model before the
binary has it. The mission's items bring the binary to this page, and until
each has merged, the part of the page it covers is ahead of the code.

## Why

The assessment found that independent review catches real bugs (39 of 100
sampled fix-now findings in four product repositories) and that the machinery
around it cost more than it returned. 75% of commits touched only tracker
records. 38 of 50 sampled refusals were an agent tripping over the tool, not
the tool stopping a mistake. Sumeet asked "where are we?" 31 times in a month.
About 85% of the items finished since the October rebuild were Atelier working
on itself.

The cause was one gap. The tool had gates but no map: it did not know which
thread was asking or what stage the work was in, so an agent found its place
by running a command and reading the refusal.

## Who it is for

Sumeet, on two Linux hosts that both run T3 Code:

- **Home:** Claude, Codex and Grok agents; GitHub with Actions.
- **Work:** Codex agents only; GitHub without Actions or any other CI.

It is not designed for a second user. Backward compatibility with the old
tool's records and commands was not a goal: the cutover converted the records
once.

## Principles

1. **Purpose.** Agents produce correct, high-quality code with as little
   overhead as possible. Nothing else is sacred.
2. **The human at the boundaries.** Sumeet sets a mission's direction in
   conversation and reads its result. In between he is hands-off, never has
   to ask where things stand, and can stop a mission at any time.
3. **The tool owns order and flow.** The lifecycle is in the binary. Each
   command says who may run it, and `status` tells each thread what it holds
   and what comes next.
4. **Roles in tension.** A lead judges for the product as a whole, an
   implementer gets one change right, a reviewer comes fresh. No thread holds
   two of these on the same work.
5. **Independent review is not negotiable.** Rounds of review are not a cost
   to cut. Records and relays are.
6. **Trusted agents.** Show state before refusing. A refusal names the way
   out, and the way past: every rule can be skipped on purpose, with a
   reason.
7. **Records live with the code.** They are Markdown in the repository, on the
   branch that does the work, and each kind earns its place. The base holds
   what was agreed and what was finished, and only a merge writes it.
8. **Judgment in the skill, procedure in the tool.** Agent Factory says how to
   decide. Atelier says what to run.
9. **Hooks guard, commands act.** Agents use Git for what Git does: branches,
   commits, pushes. A rule about a Git action is checked by a Git hook, where
   the action happens. A command exists only to do what Git cannot: write a
   record, print orientation, or run the forge's steps. No command exists
   only to check.

## Words

Each word has one meaning in the CLI, its help, the docs and the skill.

| Word | Meaning |
| --- | --- |
| Mission | A batch of related work Sumeet hands to agents to run unattended. It has an outcome, limits on what agents may decide, and an end. A theme with no end is a label. |
| Plan | What a mission's two plan reviews approve: the Why, Outcome, Success criteria, Scope and Authority sections of `mission.md`, the documents beside it other than `notes.md` and its reviews, and, until the plan has merged, the mission's items. Nothing called a plan is stored. |
| Proposed | A mission whose record says `open` and is read from a branch that has not merged. It is shown, never stored. |
| Agreed | A mission whose record is on the base. Its plan merged after two approving plan reviews. |
| Branch | Where a thread's changes live until they merge. There are three kinds, named by convention: an item's ID, a mission's ID, and `backlog/<slug>` for record changes tied to neither. |
| Item | One change, on one branch named for the item, with one review boundary. An item that is never started and has no files to change is closed with a written result, and that record change merges like any other. |
| Review | One reviewer's examination of one commit: a summary, findings and a verdict, in one file. |
| Round | The reviews of one submission. Each `submit` opens a new round. |
| Finding | One problem in a review, with a recommendation: fix now, defer or no action. It is named by its review and its number, as in `2-m2bn.1`. |
| Verdict | A review's conclusion: approve or changes. |
| Ready | Someone with authority said this item should start now. Ready is about intent, and only an item has it. It says nothing about review. |
| Thread | The T3 Code thread that holds a role. Records store its ID. |
| Role | Lead, implementer, reviewer, decider or planner. A thread gets a role from the command it enters with; `start --mission` makes it a planner of the mission. |
| Gardener | Whoever keeps a repository easy to navigate, easy to change and easy to validate, and brings Sumeet the redesigns that need his taste. A run looks at one focus area, which Sumeet may name at the start, through agents' threads and the code itself. In the tool a gardener is the implementer of an item labelled `garden`: that item is its run, and the item's Report is the one page Sumeet reads. No command is reserved for it. |
| Status | A stored field on a mission or item, set only by the command that performs the act. |
| Done | The end state of merged work, of a closed item, and of a closed mission. |
| Record-only change | A branch every one of whose changed paths is under `.atelier/` other than `config.toml`, and which adds no mission and changes no plan. It merges once `atelier check` passes on the merged tree. |
| Check | `atelier check`, which the Git hooks and CI also run, or one of a project's configured merge checks. |
| Skip | Going past a rule on purpose, with `ATELIER_SKIP="<reason>"` set for one command. |

"Approve" means only a reviewer's verdict. "Ready" is said of an item only.
A mission has no go-ahead: its direction is given when it starts, in a
conversation with Sumeet or a proposal he agreed, and its Why says which.

## Branches

Only a merge writes the base. The base is the agreed backlog and the history
of finished work. A branch is work in motion, with its own records, held by
one thread. Nothing is committed on the base directly, so Atelier works on a
repository whose base is protected, where the forge refuses every push to it
but its own merge of a pull request.

Every change a thread makes, to code or to records, lives on a branch that
`start` makes. There are three kinds:

| Branch | Holds | Made by |
| --- | --- | --- |
| `<item-id>` | One item's change: its code and its records | `start <id>` on a ready item, or `start --new` |
| `<mission-id>` | A new mission's plan, its reviews, and its items until the plan merges | `start --mission "<title>"` |
| `backlog/<slug>` | Record changes tied to no started item and no proposed plan: filing a bug, readying or dropping an item, leading, closing or stopping an open mission, a lead's notes, and an amendment of an agreed mission's plan with its plan reviews | `start --backlog <slug>` |

`config.toml` may set one branch prefix, such as `x/`, for a host whose
forge has a naming rule. It applies to all three kinds: `x/<item-id>`,
`x/<mission-id>` and `x/backlog/<slug>`. Without one, branches are named as
above.

A branch merges on what it holds, whatever its kind:

| A branch that | Needs before it merges |
| --- | --- |
| Changes code: any path outside `.atelier/`, or `config.toml` | An independent review that approves its head, as now |
| Adds a mission, or changes a mission's plan | Two approving plan reviews of the plan its head holds, from threads that are not its planners |
| Changes only records, and no plan | `atelier check` passing on the merged tree |

`merge` applies every row a branch's contents meet, and `atelier check` on
the merged tree is always one of them. A code review is an item's, so only
an item's branch may change code; code on any other branch is refused,
naming `start` for an item. A plan's reviews may be on any branch: a new
mission's on its own branch, and an amendment of an agreed mission's plan on
a backlog branch, where the thread that submits it becomes one of the
mission's planners and two other threads review it. An item's branch may
not change a plan, since its reviewer judges the change against the plan;
it is refused, naming the backlog branch an amendment goes on.

Three commands act on Git: `start` makes and claims a branch, `submit`
publishes it, and `merge` merges it. Every other command that changes a
record writes the file and stops. The agent commits and pushes with Git, and
the hooks validate what it commits.

A branch is claimed by being made. Where a remote is configured, `start`
pushes the new branch before it says it has started, so two clones that
start one item at once cannot both win: one is refused and told which thread
holds it.

`atelier list`, `atelier show` and the Desk read the base and every branch
the convention names, local and fetched, so work in motion is seen from any
clone that has fetched. They say which ref each record was read from and how
long ago the remote was fetched. No command fetches, except to read back
what it just did: `start` the branch whose claim it lost, and `merge` the
base the forge merged into.

## Lifecycle

### Item

| Status | Held by | Set by | Leaves when |
| --- | --- | --- | --- |
| `draft` | Nobody | `new` | Someone runs `ready` on it |
| `ready` | Nobody | `ready` | A thread runs `start` |
| `building` | Implementer | `start` | Done when lines are ticked, the Report is written, and `submit` runs |
| `in_review` | Reviewer | `submit` | The reviewer runs `verdict` |
| `changes_requested` | Implementer | `verdict changes` | Each open fix-now finding is fixed, rebutted or decided, and `submit` runs |
| `approved` | Implementer | `verdict approve` | The merge checks pass on the head, and `merge` runs; or the approval went stale, and `submit` runs again |
| `done` | Nobody | The merge of its branch, or `close` | Never |
| `dropped` | Nobody | `drop` | Never |

An item is done once its branch has merged. Where the forge makes the merge
commit, nothing writes `done` into the record: a started item's record
reaches the base only through its merge, so the base's copy of it reads as
done whatever started status it holds ([design record,
Merge](design.md#merge)). `close` writes `done` on an item that was never
started.

Items keep `draft` and `ready`. A draft item is agreed to exist; a ready one
is agreed to start now. An idea for a mission is a draft item on the base,
so the base stays the one list of what might be done.

A question the implementer escalates is not a status. It is an unanswered
entry in the item's Decisions section. `submit` and `merge` wait for the
answer, and `status` shows it.

### Mission

| Stored status | Set by | Leaves when |
| --- | --- | --- |
| `open` | `start --mission` | Every item is done or dropped, the Result is written, and `close` runs; or `drop` runs |
| `done` | `close` | Never |
| `dropped` | `drop` | Never |

A mission has no draft status and no go-ahead. What it is depends on where
its record is read from:

| Shown as | Means |
| --- | --- |
| Proposed | Its record says `open` and is on its branch, which has not merged. Its plan is being written or reviewed. |
| Agreed | Its record is on the base: its plan merged after two approving plan reviews. An agreed mission with a started or done item is shown as "in progress". |
| Dropped | Its record says `dropped`, on the base or on its branch. A mission dropped on its branch is dropped, not proposed, and cannot merge. |
| Done | Its record on the base says `done`. |

Its items merge to the base one at a time, so the base shows a mission in
progress as it is: agreed, with some items done. A mission can be led and
closed once it is agreed. `start` asks of an item's mission only that it is
not dropped. Items are made ready one by one, or several in one `ready`
command, so later work can wait as drafts until the work before it has shown
what it shows.

`lead` is attribution, not a lock: it names the thread leading the mission
so that `status`, `show` and the Desk can say who it is. It reaches the base
by a record-only change, and a second lead set on another branch loses at
merge as a conflict.

### Three facts that stay separate

| Fact | Meaning | Recorded as |
| --- | --- | --- |
| On the base | Agreed, or finished | The merge that brought it |
| Reviewed | Independent agents examined this text at this commit | A review file |
| Ready | This item should start now | The item's status, with who said so |

A mission's plan is reviewed before it reaches the base, and its two
approving plan reviews are what let it merge. Nobody says go after them.

### When a thread dies

Threads stall and die, and work must not be stranded with them. Any command
that binds a role can take the role over, and says which thread held it:

- `start <id>` on an item that is already started switches to its branch and
  makes the caller its implementer, in whatever status the item is in.
- `review <id>` on a round whose review file has no verdict and belongs to
  another thread makes the caller its reviewer.
- `lead <mission>` makes the caller its lead.
- `submit` on a mission's branch by a thread that is not one of its planners
  adds it to them, and it takes the plan over.

A thread that has ever been an item's implementer can never review that
item or decide a question on it, a thread that has ever been a review's
reviewer can never decide that review's findings, and a planner of a mission
can never review its plan. Taking a role over does not erase who held it
before.

Dropping is final, and counts once it is published. A started item is
dropped on its own branch: once that commit is pushed, the item's `submit`
and `merge` are refused, and its implementer's `status` says it was dropped
and why. The base's copy follows by a record-only change. A proposed plan is
dropped on its branch, which is then pushed and its pull request closed. An
item that is not started, and an agreed mission, are dropped on a backlog
branch, and the drop counts once that merges. Until a drop is published,
`status` and `show` say so.

## Commands

Eighteen commands are built, plus `setup`. The
tool has no site: the human's pages are the Desk's
([the Desk](product/desk.md)).

Three commands act on Git: `start`, `submit` and `merge`. Every other
command that changes a record (`new`, `set`, `ready`, `drop`, `close`,
`lead`, `review`, `verdict`, `escalate` and `decide`) only writes the file,
in the checkout it runs in, prints the paths it wrote and the Git command
that commits them, and stops. It makes no commit, no branch and no push. Run
in the checkout that has the base out, it writes nothing and names the
`start` that makes a branch for the change.

### Anyone

| Command | Behaviour |
| --- | --- |
| `status` | Who this thread is, what it holds, what each thing needs, and the next command. On the base with nothing held: the ready items `start` would accept now, one line counting the other ready items by what stops them, missions in progress and what waits on Sumeet, which is not listed as ready. For a mission's lead, its mission and what each item needs. |
| `show <id>` | An item in full, with a summary of each review: its round, reviewer, model, the commit it reviewed and its verdict, and each finding's name, recommendation, state and title. A finding's text is in its review file. For a mission, the whole mission on one page: whether it is proposed, agreed, done or dropped, its outcome and criteria, its items grouped by status with who holds each and what blocks each, its open questions, and its reviews by round. Either says which ref the record was read from, and, for a fetched ref, how long ago the remote was fetched. |
| `list` | With no flags, all open work on the base, grouped by mission and then by label, and apart from it what is proposed: missions on unmerged branches and record changes waiting to merge, each with its branch. Narrowed by `--ready`, `--blocked`, `--needs-human`, `--proposed`, `--mission <id>`, `--label <l>`, `--mine` and `--deferred`; `--done` shows finished missions and items instead. |
| `log` | The history of acts, read from Git, one line each and oldest first: when, the commit, which item or mission, the act, the thread where the record holds one, what changed, and any rule its commit skipped. For everything, one item, or one mission with its plan reviews and its items, and only acts `--since` a time. It adds nothing up. |
| `check` | Validates every record. The Git hooks and CI run it. |

A lead or a planner reads where things stand with `show`, `list` and
`status`. Which of these replaced each of the old tool's nine `work` views is
in the design record's [Tried and removed](design.md#tried-and-removed).

### Planner

A planner works on a mission's branch or a backlog branch, never on the
base.

| Command | Behaviour |
| --- | --- |
| `start --mission "<title>"` | Makes branch `<mission-id>` from the base, writes the new mission's record there with status `open` and the caller as its planner, commits it and pushes the branch where a remote is configured. |
| `start --backlog <slug>` | Makes branch `backlog/<slug>` from the base and pushes it where a remote is configured. It commits nothing, since nothing is claimed. |
| `new "<title>"` | Writes a draft item and prints its path. `--in <mission>` sets membership. `--from <finding>` makes the item from an unsettled deferred finding of a done item, and marks the finding taken. |
| `set <id> ...` | Changes front-matter fields: title, priority, labels, mission, blocked-by, and, on a bug, `--caused-by <item>` for the item that introduced it. |
| `ready <id>... --by <name>` | Marks each named item ready and records who said so. A mission is refused: it needs no go-ahead. |
| `drop <id> --reason` | Ends work that will not be done. Given an unsettled deferred finding of a done item, it marks the finding dropped. |
| `close <id>` | Ends an agreed mission whose items are all done or dropped and whose Result is written. With a result text, ends a draft or ready item that has no files to change. |
| `submit` and `merge` on a mission's or a backlog branch | As for an item: `submit` pushes the branch and opens its pull request, and `merge` merges it once it has what it needs. |

Agents edit the Markdown body of a record directly. They never hand-edit front
matter; `set` and the status commands write it.

### Implementer

| Command | Behaviour |
| --- | --- |
| `start <id>` | On a ready item: creates branch `<id>` from the base, sets `building`, binds this thread as implementer, and pushes the branch where a remote is configured, which is the claim. On a started item: switches to its branch and takes it over. `start --new "<title>"` creates an item on a new branch already `building`, for work Sumeet asked for directly. In the checkout that has the base out, `start` makes the branch without switching and prints the Git command that adds a worktree for it. |
| `submit` | Checks every Done when line is ticked, the Report is written, and each open fix-now finding has a response; sets `in_review` and commits the item's directory; opens a new round; prints the reviewer's handoff line. Where a forge is configured it then pushes the branch and opens its pull request, so CI runs while the review does; a push or pull request that fails is a warning, and `merge` does it later. |
| `escalate "<question>"` | Adds an unanswered entry to Decisions and prints the decider's handoff line. `--finding <finding>` ties the question to a finding. |
| `merge` | Runs the checks below and merges the branch. With a forge, the forge merges the pull request, as a merge commit; without one, `merge` makes the merge commit and moves the base locally. If CI has not finished it waits for it, up to 9 minutes, then says "not yet" and is run again later. Its only flag is `--trivial "<why>"`, which skips review for a change whose correctness is evident from the diff and records the claim. |

### Reviewer

| Command | Behaviour |
| --- | --- |
| `review <id>` | On an item in review: opens this round's review file if it has none and binds this thread as its reviewer, then prints the brief. Run again by the same thread, it only prints the brief. On a mission: opens a plan review for the commit the checkout is at, refused to a planner of the mission. |
| `verdict approve` or `verdict changes` | Validates the review file and writes the verdict into it. On an item it also sets the status. On a mission it changes no status. `approve` is refused while the file holds an open fix-now finding. The reviewer commits the file with Git; a plan reviewer also pushes it to the branch it reviewed: the mission's branch for a new plan, the backlog branch for an amendment. |

### Decider and lead

| Command | Behaviour |
| --- | --- |
| `decide <id> "<answer>"` | Records the answer to the open question, with this thread's ID. On a question tied to a finding, `--defer` or `--no-action` settles that finding without a fix. No past or present implementer of the item, and no reviewer of the review that raised the finding, can decide it. |
| `lead <mission>` | Writes this thread as lead of an agreed mission, and prints where each item stands and which need a thread. The lead commits it on a backlog branch, and it reaches the base as a record-only change. |

### What `status` prints

```text
You are the implementer of atelier-06jc (thread <thread>).
Item: .atelier/items/atelier-06jc/item.md
Title: Put open findings in full in the review brief
Status: changes requested in round 2
To return it to review:
  [ ] Finding 2-m2bn.1 (fix now): the brief omits Scope when the item has no parent
Answer each finding with one line under `### Responses` in the Report, ...
Then: atelier submit
```

It lists only what is not met: a finding already answered is not shown. A
Done when line that wraps over several lines of the record is shown whole; a
wrapped line that begins `#123` is text, not a heading. It
gives the round of an item in review or with changes requested, and once
three rounds have ended without an approval it says, on one line, that the
Review section requires escalation now, naming `atelier escalate
--finding <finding>` with an open fix-now finding no question is about yet.
Once a question has been asked about each open finding, the line is gone. On
an approved item it names, before `Then: atelier merge`, the merge checks the
base's `config.toml` lists, which `merge` runs on the merged tree. It does not
say when they last passed, since nothing records that.

On an item's branch, a thread that holds nothing on the item, a reviewer
whose review is done among them, is told each of its unanswered questions,
and `Then: atelier decide <id> "<answer>"` when it may decide them, before
the offer to take the item over. The lead of the
item's mission is told it leads the mission and is offered no take-over. In
a Git checkout with no `.atelier/`, `status` says this is not an Atelier
repository and exits 2.

A thread that runs another role's command gets one line back and no change:
"You are the reviewer of atelier-06jc. Reviewers run `review` and `verdict`."

### Handoff lines

Atelier never starts an agent. Whoever holds the status starts the next thread
with the harness's own tool, using a line the tool prints:

- `submit` prints: "A reviewer must be a thread other than yours, and
  starting it is your job: it is part of the task you were given, even as
  another thread's child or subagent, and nobody else will start it. So
  start one yourself now, without waiting to be asked. Under Codex a subagent
  you start is one; under Claude Code a subagent runs as your thread and is
  refused, so use your harness's tool for a new agent thread. Give it fresh
  context, not a fork of yours, and give it this task: Run atelier review
  atelier-06jc and follow what it prints. Then wait for its verdict, and run
  `atelier status`." A Claude Code subagent carries its parent's identity, so
  it cannot review its parent's item; a Codex subagent has one of its own
  ([design record, Threads and
  roles](design.md#threads-and-roles)). It says starting the
  thread is the reader's own job and part of its task because, told less,
  threads under both harnesses stopped: Codex threads waited for their user's
  word before starting a subagent (atelier-t747), and a Claude Code thread
  that T3 Code had started as a child, told to start no agent unless its
  task said so, stopped for its lead (the first real item under the new
  tool, mission atelier-qr5s). The line and its task have no backticks,
  which an agent pasting the task into a double-quoted shell argument would
  run as a command.
- `escalate` prints the same for a decider, with `show` and `decide`.
- `lead` prints one, with a `start` task for each item that can start now,
  and the lead's `status` repeats it.

## Records

```text
.atelier/
  config.toml
  missions/<id>/
    mission.md
    design.md             optional
    notes.md              optional; the lead's notes, not part of the plan
    <other documents>     spikes, comparisons, mockups
    reviews/1-k3fa.md     plan reviews
  items/<id>/
    item.md
    reviews/1-x7qd.md 2-m2bn.md
    artifacts/            validation screenshots and transcripts
```

| File | Front matter | Sections | Written by |
| --- | --- | --- | --- |
| `mission.md` | status (`open`, `done` or `dropped`), labels, lead thread, planner threads | Why, Outcome, Success criteria (optional), Scope, Authority, Result | Sumeet's direction, recorded by the planner, whose Why says whose direction it is; the lead writes Result |
| `design.md` and other documents | none required | Shape, Design, Sizing, Derisking, Risks and unknowns | Agents |
| `notes.md` | none | Free text | The lead |
| `item.md` | status, mission, labels, priority, blocked-by, caused-by, implementer threads, ready, trivial | Goal, Done when, Scope, Report (with Validation and Responses), Decisions | The planner writes the definition; the implementer writes the Report |
| `reviews/<round>-<suffix>.md` | round, commit reviewed, reviewer threads and model, verdict | Summary and limits, then one heading per finding with its recommendation | The reviewer |

Rules:

- Only missions and items have IDs. A review is named by its round and a
  short random suffix, so two reviewers working blind on the same round never
  write the same file. A finding is named for its review and its number, as
  in `2-m2bn.1`, so two blind reviews never share a finding name either.
- An item's branch is named for its ID, and a mission's for its. No field
  records either.
- Membership is stored once, as `mission:` on the item.
- A mission's plan changes only at Sumeet's direction, and a change to it
  needs two approving plan reviews again before it merges. The other
  documents in a mission's directory are the agents', and they may change
  them during the run inside the limits Authority sets; a change to one of
  them is a change to the plan, except to `notes.md`. The Result, the lead,
  the notes and the status are records, not plan.
- When a mission was created is read from the commit that created it, which
  `log` already reads; a mission's record stores no time.
- A small mission may put a short Design section in `mission.md` and have no
  other document.
- A question for Sumeet is an item labelled `needs-human`. He answers, and an
  agent closes it with the answer.

### Where records change

Every change reaches the base one way, `atelier merge` of a branch, and no
record is committed on the base.

- A new mission, its documents and the items it starts with are written on
  the mission's branch, with its plan reviews, and reach the base when the
  plan merges.
- Every other record change that belongs to no started item is written on a
  backlog branch: filing an item, `set`, `ready`, dropping an item that is
  not started, `lead`, `close`, a lead's notes, an item added to an agreed
  mission, and dropping an agreed mission. Each merges as a record-only
  change. An amendment of an agreed mission's plan is written on a backlog
  branch too, and merges after two approving plan reviews of it.
- Once an item is started, its directory changes only on its branch, and those
  changes reach the base in the merge. A drop of a started item is written
  there too, and the base's copy follows by a record-only change.
- Once an item is done, its directory changes only when one of its deferred
  findings is settled. That is written where the command is run: on a
  backlog branch, or on a gardener's branch, where it is reviewed with the
  run and reaches the base in its merge.
- The commands that write these only write files. The agent commits them
  and pushes the branch with Git, and `submit` refuses while a record the
  branch changes is not committed.
- Two branches that change one record conflict when the second merges, and
  `atelier check` on the merged tree catches two that are each valid and
  contradict each other.

## Review

- **One file per review.** The reviewer writes it in the implementer's
  checkout, which is where T3 starts a delegated child by default.
- **What the reviewer is given.** The brief `review <id>` prints: the item's
  definition, the diff, the Report with its validation, earlier rounds' open
  findings with the author's responses and any decisions, and the branch's
  pull request where a forge is configured, where CI's result can be read.
  It carries no author reasoning beyond the Report.
- **Recommendations.** Fix now, defer, or no action.
- **A fix-now finding has three exits.** The implementer fixes it. Or it
  rebuts it with a fact or a test in the Report's Responses, and the next
  reviewer, who must be a fresh one, withdraws the finding or keeps it; a
  rebuttal gets one try. Or it escalates, when the finding is right but not
  worth fixing now or the fix leaves the item's scope.
- **The implementer cannot dismiss a finding.** A reviewer withdraws one. A
  decider settles one as deferred or as no action, and the next review shows
  it as decided and cannot reopen it.
- **Escalation is required** after three rounds without an approve verdict,
  after a rejected rebuttal the implementer still will not fix, and when a fix
  would leave the item's scope.
- **The skill applies three of these rules:** a rebuttal's one try, the fresh
  reviewer for a rebuttal, and escalation after three rounds. The tool
  refuses none of them; `status` shows the round, the one-try rule and the
  three-round rule.
- **Who decides.** The mission's lead, when the item's mission has one. A
  fresh agent given the item, the finding and both positions when it does
  not. Anything that touches product direction goes to Sumeet as a
  `needs-human` item.
- **Deferred findings stay in their review file.** Merge creates no
  follow-up records. `list --deferred` shows the ones not yet settled, across
  done items, and the gardener settles each: `new --from` takes it into an
  item, and `drop` drops it with a reason. Either is written into the
  finding, with the thread that did it, so the list shrinks. A finding
  already settled cannot be settled again.
- **Plan reviews.** A mission's plan gets two blind reviews from different
  labs where two are available, and `merge` refuses the mission's branch
  until two reviews of the current round, from threads that are not its
  planners, approve the plan its head holds. Each reviewer works in its own
  checkout at the commit `submit` pushed, commits its review file and pushes
  it to the branch it reviewed, the mission's branch or, for an amendment,
  the backlog branch. Blind means a reviewer forms its view without
  the other's; one that goes looking, with `show` or a fetch, can read a
  verdict already pushed, and that is accepted, since the tool's checks are
  nudges for trusted agents.

## Proof and validation

- **Checks.** A project lists its merge checks in `config.toml`. At home that
  includes CI on the pull request. At work it is local commands. `merge` runs
  or reads them. Nobody records a receipt.
- **Manual validation.** The Report's Validation section says what was driven
  and what was observed, with artifacts beside the item. A validation claim
  with no artifact is a finding. Each project's verify skill says what to
  capture.
- **Evidence records do not exist.**

## Merge

`merge` merges the branch its checkout is on, of any of the three kinds,
once the branch has what its contents need (see [Branches](#branches)). A
mission's or a backlog branch needs its two plan approvals or its passing
`atelier check`, and a mission dropped on its branch is refused. On an
item's branch, `merge` runs only on an approved item, or with `--trivial`.
It checks:

1. The latest round has an approve verdict that still covers the head: since
   the commit it reviewed, nothing changed outside the item's own directory,
   and the item's Goal, Done when and Scope did not change.
2. No question in Decisions is unanswered.
3. The configured merge checks pass on the merged tree. Where the forge names
   a CI workflow, CI passed on the head, or on an earlier commit of the
   branch from which the head differs only inside the item's directory, such
   as the commit `submit` pushed before `verdict approve` committed. The
   latest finished run among those decides, except that when it failed and
   a newer one is still running, the newer one is waited for.

4. The item, and its mission, are not dropped, as read from the base and
   from the item's branch.

With a forge, the forge makes the merge commit: once its checks pass,
`merge` asks the forge to merge the pull request at the head it checked, as
a merge commit, and never pushes the base. Where the forge will not merge
yet, because a review or a check its branch protection requires is missing,
`merge` stops with "not yet" (exit status 3) and says which; it does not
wait for a person, and run again later it merges. Without a forge, `merge`
makes the merge commit and moves the base locally.

With a forge, `submit` has already pushed the branch and opened its pull
request, so CI ran during the review. `merge` pushes the branch whenever the
forge's copy is not the head, since the forge merges exactly the head of the
pull request and that must hold the approving verdict the reviewer committed
in the implementer's checkout. That push starts a CI run on the new head;
`merge`'s own check still counts a run on an earlier commit from which the
head differs only inside the item's directory, but a base whose protection
requires CI on the head makes the forge wait for the new run, and `merge`
then stops with "not yet" until it passes. It opens the pull request if
there is none. Then
it waits for CI, for up to 9 minutes: it merges as soon as a run that counts
passes, and refuses as soon as one fails with no newer run still going. The
wait holds no lock and writes nothing, so it can be interrupted. At the bound it stops with "not yet", not
a refusal, and the same command run again picks up. `merge` does the waiting
because the thread that runs it is almost always a delegated child, which
the harness neither lets watch a pull request nor wakes reliably. A forge
with no CI workflow still gets the branch pushed and a pull request opened,
and `merge` waits for nothing.

A merge that runs the checks says, before the line that reports the merge, how many of the base's merge checks ran on the
merged tree and that they passed, or that `config.toml` names none. A
failing check passed with `ATELIER_SKIP` gets its skip line instead, and
nothing says the checks passed. A run that finds the branch already merged
says so, and says nothing of checks.

An item that is approved and not merged shows that way, with since when, in
`status` and `show`, so a wait that was lost is visible. The
time is that of the commit that recorded the approving verdict. It comes
from the records; nothing looks up CI to show it.

A base that moved on needs no catch-up unless the merge conflicts or a check
fails on the merged tree. Catching up is `git merge` of the base, and the
approval then needs a fresh look, since the branch changed outside the item.

## Hooks and the bypass

`atelier setup` installs three Git hooks in a clone. Each runs `atelier
check` and says what it found in the output of the Git command it guards.

| Hook | Guards | Rule |
| --- | --- | --- |
| pre-commit | `git commit` | Every staged record is valid, a changed status is one the lifecycle allows from the status before, and no commit on the base changes a record |
| pre-push | `git push` of the base | The base receives only merges. With a forge, every push that moves the base is refused, since the forge makes its merges. Without one, each new commit on the base's own line must be an Atelier merge; the commits a merge brought in are not looked at. |
| post-checkout | `git switch` | Nothing; it prints `status` for the branch just switched to |

Every rule a hook or a command applies can be skipped on purpose:

```text
ATELIER_SKIP="the reviewer is down and Sumeet said to merge" atelier merge
```

- The reason is required; an empty one is refused.
- The skip is printed. Where Atelier makes the commit, as `merge` does, the
  commit message gets a line naming the rule and the reason. A skip in a
  hook shows in the transcript, as the variable on the Git command.
- Every refusal names the proper way out first and the skip second.
- A record Atelier cannot parse cannot be skipped past. That is not a rule;
  the tool has nothing to read.

Git's own `--no-verify` still exists. It skips every hook, a project's own
included, and leaves no trace, so agents are told to use `ATELIER_SKIP`.
`status` says when a clone has no hooks installed.

## Configuration

`.atelier/config.toml` holds the project slug, the base branch, the
optional branch prefix, the merge checks and the forge. `merge` reads it
from the base
branch, so a work branch cannot weaken its own checks, and a change to it
counts as code: it is reviewed and merged like any other. There is no
workflow file and no per-project lifecycle. A named option is added when a
second real project needs one.

## Scorecard

Atelier is judged on these. Outcomes are the constraint, costs are what it
drives down, and exchange rates decide what to cut.

| Group | Measure | Before |
| --- | --- | --- |
| Outcome | Defects found after merge, traced to the item that caused them | 15 of 277 merged items in atelier (5.4 per 100), a floor; 18 of 361 in astromech; the other three filed almost no bugs |
| Outcome | Sumeet's interventions per mission, by kind: status question, unsticking a stall, correcting direction | 0 to 17 per started mission, median 2.5; 39 of 60 correcting direction |
| Cost | Process share: an item's tool calls and tokens not spent reading, editing or testing product code | atelier 28.0% of shell commands and 21.6% of token cost; 7.6% to 20.3% of commands elsewhere |
| Cost | Confusion: refused or errored calls, help lookups, commands run by the wrong role | atelier 25.2 per 100 `atelier` calls, about half help lookups; 16.39 to 24.27 elsewhere; wrong role not measured |
| Cost | Tokens per merged item | Median $3.06 in atelier, $3.22 to $13.81 elsewhere, at list price, orchestrator threads left out |
| Exchange | Gate precision: refusals that stopped a real mistake | 11 of 100 |
| Exchange | Review yield: fix-now findings that are product defects | 46 of 100 (45 with one label corrected) |
| Exchange | Skips: how often each rule is passed with `ATELIER_SKIP` | No such mechanism |
| Codebase health | Tool calls before an agent's first edit on an item | Median 22 in atelier, 10 to 21 elsewhere |
| Codebase health | Rework: lines deleted within two weeks of being written | atelier 10.8%; 3.4% to 7.2% elsewhere |
| Codebase health | Where each defect was caught: the author's tests, CI, review, after merge | atelier: review about 191, CI 6, after merge 15; the author's tests not observable |
| Codebase health | Non-test lines of Rust | 48,989 |

The Before column is the November 2026 baseline. Each number's method,
coverage and limits are in
[scorecard-baseline-2026-11.md](architecture/quality/scorecard-baseline-2026-11.md).

The codebase health rows are trends for one repository against its own past.
Agents are not scored on them, and none is a limit.

The tool computes none of this. Its part is facts: `log` says
what happened to each item, `list` says how things stand, and the records hold the
findings and each bug's `caused-by`. Rework and line counts come from Git.
Tool calls, tokens and interventions come from T3's events and the
transcripts, by scripts on the host that has them. Whether a finding was a
product defect is a judgment. Which numbers to take, which threads to read
behind them, and what to make of both, is the gardener's procedure in the
skill.

## Out of scope

- Atelier starting, stopping or steering agents.
- A session database, and live thread status on a page. Both are the Desk's.
- Per-project lifecycles.
- A command that computes the scorecard, and a scorecard page. The page is
  the Desk's.
- A view of what changed between the merge of a mission's plan and its
  result. It is a derived view and the Desk's to build later. The tool
  gives it the data: records readable at any commit, and, from `log`, every
  act with the commit it happened in.
- Coordinating clones beyond the claim. Two clones of one remote cannot
  both start one item, since `start` pushes the branch it makes, and each
  sees the other's work once it has fetched. Nothing else is coordinated
  across clones, and no command fetches.
- An integration branch for a mission, a database or a lock service for
  records.
- Anything for a second user: onboarding docs, installers beyond what exists,
  other forges.
