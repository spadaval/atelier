# Atelier: product specification

This page says what Atelier is for and what it does. How it is built is in
the [design record](design.md), which also lists what was tried and removed.
The human set the direction in conversation on 2026-11-09 after an assessment
of the project (`/var/tmp/atelier-assessment/reports/`). The tool was
rewritten to this page, and every repository on this host moved onto it on
2026-11-11.

On 2026-11-13 the human set a second direction, the branch model of mission
atelier-to83: only a merge writes the base, and every change, code or
records, is made on a branch. This page describes that model before the
binary has it. The mission's items bring the binary to this page, and until
each has merged, the part of the page it covers is ahead of the code.

## Why

The assessment found that independent review catches real bugs (39 of 100
sampled fix-now findings in four product repositories) and that the machinery
around it cost more than it returned. 75% of commits touched only tracker
records. 38 of 50 sampled refusals were an agent tripping over the tool, not
the tool stopping a mistake. The human asked "where are we?" 31 times in a month.
About 85% of the items finished since the October rebuild were Atelier working
on itself.

The cause was one gap. The tool had gates but no map: it did not know which
thread was asking or what stage the work was in, so an agent found its place
by running a command and reading the refusal.

## Who it is for

The human, on two Linux hosts that both run T3 Code:

- **Home:** Claude, Codex and Grok agents; GitHub with Actions.
- **Work:** Codex agents only; GitHub without Actions or any other CI.

It is not designed for a second user. Backward compatibility with the old
tool's records and commands was not a goal: the cutover converted the records
once.

## Principles

1. **Purpose.** Agents produce correct, high-quality code with as little
   overhead as possible. Nothing else is sacred.
2. **The human at the boundaries.** The human sets a mission's direction in
   conversation and reads its result. In between they are hands-off, never have
   to ask where things stand, and can stop a mission at any time.
3. **The tool owns order and flow.** The lifecycle is in the binary. Each
   command says who may run it, and `status` tells each thread what it holds
   and what comes next.
4. **Roles in tension.** A lead judges for the product as a whole, an
   implementer gets one change right, a reviewer comes fresh. By default no
   thread holds two of these on the same work; a lead that takes an item
   over says why.
5. **Independent review is the default.** Rounds of review are not a cost
   to cut. Records and relays are. A lead is responsible for finishing its
   mission and may merge past a review, with its reason on the record and
   never silently.
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
| Mission | A batch of related work the human hands to agents to run unattended. It has an outcome, limits on what agents may decide, and an end. A theme with no end is a label. |
| Plan | What a mission's two plan reviews approve: the Why, Outcome, Success criteria, Scope and Authority sections of `mission.md`, the documents beside it other than `notes.md`, `architecture.md`, `decisions.md` and its reviews, and, until the plan has merged, the mission's items. Nothing called a plan is stored. |
| Proposed | A mission whose record says `open` and is read from a branch that has not merged. It is shown, never stored. |
| Agreed | A mission whose record is on the base. Its plan merged after two approving plan reviews. |
| Branch | Where a thread's changes live until they merge. There are three kinds, named by convention: an item's ID, a mission's ID, and `backlog/<slug>` for record changes tied to neither. |
| Item | One change, on one branch named for the item, with one review boundary. An item that is never started and has no files to change is closed with a written result, and that record change merges like any other. |
| Review | One reviewer's examination of one commit: a summary, findings and a verdict, in one file. |
| Round | The reviews of one submission. Each `submit` opens a new round. |
| Finding | One problem in a review, with a recommendation: fix now, defer or no action. It is named by its review and its number, as in `2-m2bn.1`. |
| Verdict | A review's conclusion: approve or changes. |
| Open | Nobody has started this item and it is not ended. An item is open from the moment it is filed and needs no approval to exist or to start: the judgment is made when someone picks the work up. |
| Thread | The T3 Code thread that holds a role. Records store its ID. |
| Role | Lead, implementer, reviewer or planner. A thread gets a role from the command it enters with; `start --mission` makes it a planner of the mission. |
| Gardener | Whoever keeps a repository easy to navigate, easy to change and easy to validate, and brings the human the redesigns that need their taste. A run looks at one focus area, which the human may name at the start, through agents' threads and the code itself. In the tool a gardener is the implementer of an item labelled `garden`: that item is its run, and the item's Report is its one page. The run merges once review approves it. Its proposals then go to the human in one rolling `needs-human` item, `Garden proposals`, which keeps each until it is answered, and its bugs as `needs-human` items, both through `backlog/garden-proposals`; a planner applies the answers. No command is reserved for it. |
| Status | A stored field on a mission or item, set only by the command that performs the action. |
| Done | The end state of merged work, of a closed item, and of a closed mission. |
| Record-only change | A branch every one of whose changed paths is under `.atelier/` other than `config.toml`, and which adds no mission and changes no plan. It merges once `atelier check` passes on the merged tree. |
| Check | `atelier check`, which the Git hooks and CI also run, or one of a project's configured merge checks. |
| Skip | Going past a rule on purpose, with `ATELIER_SKIP="<reason>"` set for one command. |

"Approve" means only a reviewer's verdict. An item has no go-ahead, and
neither has a mission: a mission's direction is given when it starts, in a
conversation with the human or a proposal they agreed, and its Why says which.

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
| `<item-id>` | One item's change: its code and its records, and the new items its implementer, its reviewers or any other thread file on it | `start <id>` on an open item, or `start --new` |
| `<mission-id>` | A new mission's plan, its reviews, its items until the plan merges, and other new items filed on it | `start --mission "<title>"` |
| `backlog/<slug>` | Record changes tied to no started item and no proposed plan: filing by a thread that has no branch, changing an item, dropping one, started or not, leading, closing or stopping an open mission, a lead's notes, and an amendment of an agreed mission's plan with its plan reviews | `start --backlog <slug>` |

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
long ago the remote was fetched. Commands fetch two things and no more: the
base, by `start <id>` and `submit` to judge an item by it as it is now and
by `merge` to read back the forge's merge; and, by `start`, the branch
whose claim it lost.

A branch ends when it merges. `merge` then deletes it on the forge's remote,
only while it is still at the merged commit, and here, and leaves the
checkout on a detached HEAD at that commit. The next change starts with
`start`: the planning commands write nothing on a detached HEAD, while
`review` and `verdict` work in a reviewer's detached checkout. A commit made
on a detached HEAD that no branch, remote-tracking branch or tag holds is
shown by `status`, and `start` refuses to switch away from it.

## Lifecycle

### Item

| Status | Held by | Set by | Leaves when |
| --- | --- | --- | --- |
| `open` | Nobody | `new` | A thread runs `start`, once the record is on the base and its blockers are done; or `close` or `drop` runs |
| `building` | Implementer | `start` | Done when lines are ticked, the Report is written, and `submit` runs |
| `in_review` | Reviewer | `submit` | The reviewer runs `verdict` |
| `changes_requested` | Implementer | `verdict changes` | Each open fix-now finding is fixed, rebutted or answered not built, and `submit` runs, from round 3 on after the lead rules to fix |
| `approved` | Implementer | `verdict approve` | The merge checks pass on the head, and `merge` runs; or the approval went stale, and `submit` runs again |
| `done` | Nobody | The merge of its branch, or `close` | Never |
| `dropped` | Nobody | `drop` | Never |

An item is done once its branch has merged. Where the forge makes the merge
commit, nothing writes `done` into the record: a started item's record
reaches the base only through its merge, so the base's copy of it reads as
done whatever started status it holds ([design record,
Merge](design.md#merge)). `close` writes `done` on an item that was never
started.

An unstarted item is `open`, with no approval between filing and starting.
Work begins because the human, a lead or a plan said so, and nothing starts
an open item unasked. Later work is held back by a blocker. An idea for a
mission is an open item on the base, so the base stays the one list of what
might be done. A record from before 2026-11-22 says `draft` or `ready` for
`open`; both read as `open`, and `show` and `log` still say who made one
ready.

A question is not a status, and no record holds one. The implementer asks
the thread that started it, or an advisor, and writes into the Report
anything the answer changed ([Review](#review)).

### Mission

| Stored status | Set by | Leaves when |
| --- | --- | --- |
| `open` | `start --mission` | Every item is done or dropped, the Result is written, and `close` runs; or `drop` runs |
| `done` | `close` | Never |
| `dropped` | `drop` | Never |

A mission has no status of its own for proposed and no go-ahead. What it is depends on where
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
not dropped, so its items need nothing after the plan merges. Work that
must wait until earlier work has shown what it shows names that work in
`blocked_by`.

`lead` is attribution, not a lock: it names the thread leading the mission
so that `status`, `show` and the Desk can say who it is. It reaches the base
by a record-only change, and a second lead set on another branch loses at
merge as a conflict.

### Two facts that stay separate

| Fact | Meaning | Recorded as |
| --- | --- | --- |
| On the base | Agreed, or finished | The merge that brought it |
| Reviewed | Independent agents examined this text at this commit | A review file |

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
item, and a planner of a mission can never review its plan. Taking a role over does not erase who held it
before.

Dropping is final, and counts once it is published. An item, started or
not, and an agreed mission are dropped on a backlog branch, and the drop
counts once that merges: the item's `submit`, `merge` and take-over are then
refused, its implementer's `status` says it was dropped and why, and that
`merge` closes the item's pull request. `drop` names the other items that
only the dropped item's branch filed, committed or still in its checkout,
with the commands that carry each over, so none is lost unseen. A started item
may also be dropped on its own branch, which stops it once that commit is
pushed; the base's copy then follows by a record-only change. A proposed
plan is dropped on its branch, which is then pushed and its pull request
closed. Until a drop is published, `status` and `show` say so.

## Commands

Fifteen commands are built, plus `setup`. The
tool has no site: the human's pages are the Desk's
([the Desk](product/desk.md)).

Three commands act on Git: `start`, `submit` and `merge`. Every other
command that changes a record (`new`, `set`, `drop`, `close`,
`lead`, `review` and `verdict`) only writes the file,
in the checkout it runs in, prints the paths it wrote and the Git command
that commits them, and stops. It makes no commit, no branch and no push. Run
in the checkout that has the base out, or, but for `review` and `verdict`,
on a detached HEAD, it writes nothing and names the `start` that makes a
branch for the change.

### Anyone

| Command | Behaviour |
| --- | --- |
| `status` | Who this thread is, what it holds, what each thing needs, and the next command, with what waits on this thread before anything that does not and the backlog last. Away from an item's branch: where the plan a records branch changes stands in review; what it holds, with the way to the first, in the base's checkout the checkout that has the item's branch or the `git worktree add` that makes one, elsewhere `start <id>`; what the checkout has not committed or pushed, and a detached HEAD's commits that no branch holds. For a mission's lead, next, what waits on it: each finding not built, each item at the round cap, and the handoff for each item that can start now. Then each mission it leads: one line of review pace, with rounds per item, the time a round took and what rounds after the first found, then one line for each item not done, with its status, its blocker or how many, and no holder's thread ID or Done when lines, which `show <item>` gives and `status` names. Last, the backlog, without the items of the missions it leads: of the open items `start` would accept now, the 10 most urgent by priority and then ID, or 3 when the thread holds an item or leads a mission, and how many more `list --startable` lists, with none named as the next command, since the thread's task says which to start; one line counting the other open items by what stops them; the items other threads hold; the missions in progress it does not lead; what waits on the human, which is not listed as able to start; and, from 20 merged local branches or 10 detached worktrees, one line that counts both and names the Git commands that list them; it deletes nothing. |
| `show <id>` | An item in full, with a summary of each review: its round, reviewer, model, the commit it reviewed and its verdict, and each finding's name, recommendation, state and title. A finding's text is in its review file. For a mission, the whole mission on one page: whether it is proposed, agreed, done or dropped, its outcome and criteria, its items grouped by status with who holds each and what blocks each, what is disputed in it as `list --disputed` gives it, and its reviews by round. Either says which ref the record was read from, and, for a fetched ref, how long ago the remote was fetched. |
| `list` | With no flags, all open work on the base, grouped by mission and then by label, and apart from it what is proposed: missions on unmerged branches and record changes waiting to merge, each with its branch. Narrowed by `--startable` (open and can start now), `--blocked`, `--needs-human`, `--proposed`, `--mission <id>`, `--label <l>`, `--mine` and `--deferred`; `--done` shows finished missions and items instead. `--disputed` lists instead, across open and done items, each open fix-now finding answered `not built`, which waits on the lead, each finding whose Response is `rebutted` with what the next round made of it (kept, withdrawn, fixed, kept at defer or no action, or not yet reviewed), and each plan-review round whose reviewers gave different verdicts; `--mission <id>` narrows it. The listing refuses nothing; `submit` refuses another plan round after two split rounds in a row (see Review). |
| `log` | The history of actions, read from Git, one line each and oldest first: when, the commit, which item or mission, the action, the thread where the record holds one, what changed, and any rule its commit skipped. For everything, one item, or one mission with its plan reviews and its items, and only actions `--since` a time. It adds nothing up. |
| `check` | Validates every record. The Git hooks and CI run it. |
| `new "<title>"` | Writes an open item and prints its path, on any branch `start` made: a started item's, whether the thread implements it, reviews it or neither, a mission's or a backlog branch. The item reaches the base with that branch's merge, is shown as proposed until then, and makes no approval stale. It joins no mission unless `--in <mission>` names one, and nobody holds it. On the branch of an item that is done or dropped, which will not merge, it is refused. A thread with no branch makes one with `start --backlog`. `--from <finding>` makes the item from an unsettled deferred finding of a done item, and marks the finding taken; that changes the finding's item, so it runs on a backlog branch or by an item's implementer on its branch. |

A lead or a planner reads where things stand with `show`, `list` and
`status`. Which of these replaced each of the old tool's nine `work` views is
in the design record's [Tried and removed](design.md#tried-and-removed).

### Planner

A planner works on a mission's branch or a backlog branch, never on the
base, and creates items there with `new`.

| Command | Behaviour |
| --- | --- |
| `start --mission "<title>"` | Makes branch `<mission-id>` from the base, writes the new mission's record there with status `open` and the caller as its planner, commits it and pushes the branch where a remote is configured. |
| `start --backlog <slug>` | Makes branch `backlog/<slug>` from the base and pushes it where a remote is configured. It commits nothing, since nothing is claimed. |
| `set <id> ...` | Changes front-matter fields: title, priority, labels, mission, blocked-by, and, on a bug, `--caused-by <item>` for the item that introduced it. |
| `drop <id> --reason` | Ends work that will not be done: on a backlog branch, for a started item too, or on a started item's own branch. Given an unsettled deferred finding of a done item, it marks the finding dropped. |
| `close <id>` | Ends an agreed mission whose items are all done or dropped and whose Result is written. With a result text, ends an open item that has no files to change. |
| `submit` and `merge` on a mission's or a backlog branch | As for an item: `submit` pushes the branch and opens its pull request, and `merge` merges it once it has what it needs, then deletes it. A backlog merge that drops a started item also closes that item's pull request. |

Agents edit the Markdown body of a record directly. They never hand-edit front
matter; `set` and the status commands write it.

### Implementer

| Command | Behaviour |
| --- | --- |
| `start <id>` | On an open item on the base whose blockers are done: creates branch `<id>` from the base, sets `building`, binds this thread as implementer, and pushes the branch where a remote is configured, which is the claim. On a started item: switches to its branch and takes it over. For either, where a remote is configured, it first fetches the base and refuses an item done or dropped there, and after its push looks again. `start --new "<title>"` creates an item on a new branch already `building`, for work the human asked for directly. In the checkout that has the base out, `start` makes the branch without switching and prints the Git command that adds a worktree for it. |
| `submit` | Where a remote is configured, fetches the base and refuses an item done or dropped there. Checks every Done when line is ticked, the Report is written, and each open fix-now finding has a response; sets `in_review` and commits the item's directory; opens a new round; prints the reviewer's handoff line. On a branch that proposes or changes a plan, it opens a round of plan review, and refuses to once the change's last two rounds split. Where a forge is configured it then pushes the branch and opens its pull request, so CI runs while the review does; a push or pull request that fails is a warning, and `merge` does it later. |
| `merge` | Runs the checks below and merges the branch. With a forge, the forge merges the pull request, as a merge commit; without one, `merge` makes the merge commit and moves the base locally. If CI has not finished it waits for it, up to 9 minutes, then says "not yet" and is run again later. Once merged, it deletes the branch, on the forge's remote and here. Its only flag is `--trivial "<why>"`, which skips review for a change whose correctness is evident from the diff and records the claim. |

### Reviewer

| Command | Behaviour |
| --- | --- |
| `review <id>` | On an item in review: opens this round's review file if it has none and binds this thread as its reviewer, then prints the brief. Run again by the same thread, it only prints the brief. On a mission: opens a plan review for the commit the checkout is at, refused to a planner of the mission. |
| `verdict approve` or `verdict changes` | Validates the review file and writes the verdict into it. On an item it also sets the status. On a mission it changes no status. `approve` is refused while the file holds an open fix-now finding. The reviewer commits the file with Git; a plan reviewer also pushes it to the branch it reviewed: the mission's branch for a new plan, the backlog branch for an amendment. |

### Lead

| Command | Behaviour |
| --- | --- |
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
gives the round of an item in review or with changes requested. A line
under `### Responses` in none of the three forms is listed with the forms,
whatever the item waits for, and is never read as an answer. A finding
whose rebuttal a reviewer kept says the rebuttal has had its one try. A
finding answered `not built` stops the merge until the lead rules, and
`status` says how the lead rules. When round 3 or later asked for changes
it says the 3-round cap is reached and gives the lead's rulings in place of
`Then: atelier submit`, and the lead's `status` gives them too. On a branch
that proposes or changes a plan it gives where that plan's review stands,
and after two split rounds in a row the human's ruling as the way out. On
an approved item it names, before `Then: atelier merge`, the merge checks the
base's `config.toml` lists, which `merge` runs on the merged tree. It does not
say when they last passed, since nothing records that.

Away from an item's branch, a mission's lead reads first what waits on it,
then its missions one line an item, and the backlog last:

```text
You are thread <lead>, on the base branch main.
Nothing in the missions you lead waits on you now.
Each item's holder, blockers and Done when lines: atelier show <item>.
You lead mission hub-v2-bg14: Ask the human early: rubrics, check rows, builds to look at, and a decision in the browser
  Review pace: 22 rounds for 11 items, 2.0 each; a reviewer took 15m a round at the median, 31m at most; from one verdict to the next took 55m at the median, 1h 22m at most; later rounds found 6 new findings, 0 to fix now.
  hub-v2-aohy (building): Take one cell of the sorting hall to the human's decision on the running Hub
  And 11 done.
The backlog, which waits on nobody in particular:
Can start now:
  hub-v2-cram (medium): Bound what a model can make the 3D viewer decode
```

A reviewer's time runs from its review's opening to the commit that wrote its
verdict. For each round after the first, the time from one verdict to the
next runs from the latest verdict of an earlier round to this round's, so it
holds the implementer's fixing and the wait as well as the review: what a
later round cost the mission. A finding is new when it carries no earlier
one. A lead's
`status` of a twelve-item mission stays under 2 KB.

On an item's branch, a thread that holds nothing on the item is offered the
take-over, and a reviewer whose review is done is told to stop. The lead of
the item's mission is told it leads the mission and is offered no take-over. In
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
  context, not a fork of yours, and give it this task: In <path>, run
  atelier review atelier-06jc and follow what it prints. Then wait for its
  verdict, and run `atelier status`." `<path>` is the checkout on the item's
  branch, where `review` runs: a new thread often starts in the main
  checkout. A Claude Code subagent carries its parent's identity, so
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
- `lead` prints one, with a `start` task for each item that can start now,
  and the lead's `status` repeats it.

## Records

```text
.atelier/
  config.toml
  missions/<id>/
    mission.md
    design.md             optional
    architecture.md       optional; living technical map, not part of the plan
    decisions.md          optional; living DACI tradeoffs and decisions, not part of the plan
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
| `mission.md` | status (`open`, `done` or `dropped`), labels, lead thread, planner threads | Why, Outcome, Success criteria (optional), Scope, Authority, Result | The human's direction, recorded by the planner, whose Why says whose direction it is; the lead writes Result |
| `design.md` and other documents | none required | Shape, Design, Sizing, Derisking, Risks and unknowns | Agents |
| `notes.md`, `architecture.md`, `decisions.md` | none | Free text (notes, living contracts, DACI decisions) | Agents / the lead |
| `item.md` | status, mission, labels, priority, blocked-by, caused-by, implementer threads, trivial, and in an old record who made it ready | Goal, Done when, Scope, Report (with Validation and Responses) | The planner writes the definition; the implementer writes the Report |
| `reviews/<round>-<suffix>.md` | round, commit reviewed, reviewer threads and model, verdict | Summary and limits, then one heading per finding with its recommendation | The reviewer |

Rules:

- Only missions and items have IDs. A review is named by its round and a
  short random suffix, so two reviewers working blind on the same round never
  write the same file. A finding is named for its review and its number, as
  in `2-m2bn.1`, so two blind reviews never share a finding name either.
- An item's branch is named for its ID, and a mission's for its. No field
  records either.
- Membership is stored once, as `mission:` on the item.
- A mission's plan changes only at the human's direction, and a change to it
  needs two approving plan reviews again before it merges. The other
  documents in a mission's directory are the agents', and they may change
  them during the run inside the limits Authority sets; a change to one of
  them is a change to the plan, except to `notes.md`, `architecture.md` and
  `decisions.md`. The Result, the lead, the living documents and the status
  are records, not plan.
- When a mission was created is read from the commit that created it, which
  `log` already reads; a mission's record stores no time.
- A small mission may put a short Design section in `mission.md` and have no
  other document.
- Where a mission builds something whose goal is not clear until it is seen,
  it is derisked by a spike: a rough build of the whole thing, made with
  the human before the plan is written, on a `spike/<slug>` branch that never
  merges. Derisking says what the spike showed and whether the items rework
  its code or write fresh with it as reference. Its code reaches the base
  only through those items and their reviews. Any other mission's Derisking
  names its riskiest assumption and how that is settled.
- A question for the human is an item labelled `needs-human`. They answer, and an
  agent closes it with the answer.

### Where records change

Every change reaches the base one way, `atelier merge` of a branch, and no
record is committed on the base.

- A new mission, its documents and the items it starts with are written on
  the mission's branch, with its plan reviews, and reach the base when the
  plan merges.
- A new item is filed on the branch its thread is on, of any of the three
  kinds, and reaches the base with that branch's merge. Only a thread with
  no branch needs a backlog branch for it, as does a plan reviewer, who
  files once its verdict is published. Dropping a started item names each
  item filed on its branch, so that those still wanted are carried over.
- Every other record change that belongs to no started item is written on a
  backlog branch, as is a drop of any item: `set`,
  `drop`, `lead`, `close`, a lead's notes, and dropping an agreed
  mission. Each merges as a record-only
  change. An amendment of an agreed mission's plan is written on a backlog
  branch too, and merges after two approving plan reviews of it.
- Once an item is started, its directory changes only on its branch, and those
  changes reach the base in the merge. The exception is a drop, which a
  backlog branch may write to the base's copy: a `dropped` record on the
  base wins over the item's branch.
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
  findings with the author's responses, and the branch's
  pull request where a forge is configured, where CI's result can be read.
  It carries no author reasoning beyond the Report. It says the one thing
  the reviewer may write there beside its review: a new item, for a problem
  outside the item under review. A plan's brief says to publish the verdict
  first and then file such a problem on a backlog branch. From round 2 on it puts
  first what changed since the last round. Where the base was merged in
  between, that diff names only the paths the branch changes. Where the last
  round's commit is no longer in the branch's history, there is no such
  diff, and the reviewer is asked to review the whole change.
- **Three rounds, then the lead.** When round 3 or later asks for changes,
  the implementer takes it to the mission's lead, or with no mission to the
  thread that started it, before submitting again. The lead merges with
  `ATELIER_SKIP="<ruling>"`, adjusts its Scope or Done when, drops it, or
  rules to fix it, and then `submit` opens the next round. `status` says so;
  `submit` does not enforce it.
- **Recommendations.** Fix now, defer, or no action.
- **A fix-now finding blocks the merge, and has three exits.** The
  implementer fixes it. Where it is wrong, the implementer rebuts it with a
  fact or a test in the Report's Responses, and the next reviewer, who must
  be a fresh one, judges the rebuttal: it withdraws the finding, keeps it,
  or, when the finding is right but not for this item, keeps it at defer or
  no action with a line that says why. A rebuttal gets one try. Where it is
  real and not worth building, the implementer answers it `not built:
  <reason>`, which takes it straight to the mission's lead: the lead's
  `status` shows it, the next reviewer leaves it alone, and `merge` holds
  the item until the lead rules. With no mission, the thread that started
  the implementer rules. A finding at defer or no action blocks nothing,
  and the implementer may leave it with a basic reason.
- **The implementer does not ignore a fix-now finding.** A reviewer settles
  one without a fix, by withdrawing it or lowering it, and so does the
  mission's lead, who rules on a kept finding or one not built, either way,
  and records the ruling. The implementer passes no rule on its own work.
- **A question is advice.** An implementer that cannot settle something asks
  the thread that started it, or an advisor, as a working model asks a
  stronger one at a decision point and carries on. Nothing records the
  question or waits on it; the Report says what the answer changed. Anything
  that touches product direction goes to the human as a `needs-human` item.
- **A disagreement is found, not enforced.** The records already hold one: a
  `rebutted` Response with the next round's judgment of it, and a plan-review
  round whose reviewers differ. `list --disputed` prints them, `show
  <mission>` and the lead's `status` give a lead its mission's, and the
  gardener reads them first. Nothing blocks on them. A lead settles one either
  way: it merges past the finding with its ruling as the skip's reason, has
  the implementer fix it, or moves the disputed work out of the item's
  Scope into an item of its own.
- **The skill applies three of these rules:** a rebuttal's one try, the
  fresh reviewer for a rebuttal and the 3-round cap. The tool refuses none;
  `status` shows the round, the one-try rule and the cap.
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
- **A later plan round checks what changed.** From round 2 on, the brief
  gives first the diff of the plan's own paths since the last round, or says
  the plan is unchanged. On an amendment that diff has the items the
  amendment edited, an edit it took back included, and not what the base
  merged. The reviewer judges that and the carried findings, and blocks on
  an unchanged part only for a serious defect that was missed, else
  `defer`. Where the last round has no one commit in this history, the
  reviewer gets the whole plan.
- **Two split rounds, then the human.** A round splits when one reviewer
  approves and another asks for changes. After two in a row on one plan
  change, `submit` refuses another round, and `status` and `merge` say to
  bring the open findings to the human. On their ruling the planner merges,
  or amends and submits, with `ATELIER_SKIP="<ruling>"`, or drops the
  change. An amendment counts only its own rounds, so it starts at none.

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
   the commit it reviewed, nothing changed outside the item's own directory
   but to file new items, and the item's Goal, Done when and Scope did not
   change. Filing adds the record of an open item that neither that commit
   nor the base holds; any other change to another item's record is stale.
   An item filed and not committed holds the merge until it is committed.
2. The configured merge checks pass on the merged tree. Where the forge names
   a CI workflow, CI passed on the head, or on an earlier commit of the
   branch from which the head differs only inside the item's directory, such
   as the commit `submit` pushed before `verdict approve` committed. An
   item filed after that push is outside it, so CI runs again for it. The
   latest finished run among those decides, except that when it failed and
   a newer one is still running, the newer one is waited for.

3. The item, and its mission, are not dropped, as read from the base and
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
failing check refuses the merge, naming the check and how it failed, and
`ATELIER_SKIP` passes it only when its reason names the check, its command
and then a colon, as in
`ATELIER_SKIP="npm test: <why it may fail>"`: a reason given for another
rule never carries a merge past a failed check. One passed that way gets
its skip line instead, nothing says the checks passed, the last line says
which check failed and that the merge went ahead, and the merge commit
lists it under the checks that failed, not those that passed. A run that finds the branch already merged
says so, and says nothing of checks.

Once the base this clone has fetched holds the merged commit, `merge`
deletes the branch. On the forge's remote it does so only while the branch
is still at that commit, so a later push is kept, and a refusal is a
warning. Here it leaves the checkout detached at that commit; where a skip
left changes uncommitted, the branch stays checked out, and `merge` says so.
When the fetch after the merge fails it deletes neither, and `merge` run
again after a fetch does.

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

When pre-push refuses commits made on the base, it prints one command line
that makes a branch holding them and puts the base back to the remote's
commit, keeping uncommitted changes. Where those changes touch the same
files, Git refuses the move and loses nothing, and the hook says how to
save them and finish. Where a new branch would not hold the commits, as
when the base has diverged from the remote's, it prints no such command and
says what to do instead.

Every rule a hook or a command applies can be skipped on purpose:

```text
ATELIER_SKIP="the reviewer is down and the human said to merge" atelier merge
```

- The reason is required; an empty one is refused.
- The skip is printed. Where Atelier makes the commit, as `merge` does, the
  commit message gets a line naming the rule and the reason; where a command
  only writes files, the commit line it prints carries that line. A skip in a
  hook shows in the transcript, as the variable on the Git command.
- Every refusal names the proper way out first and the skip second.
- A record Atelier cannot parse cannot be skipped past. That is not a rule;
  the tool has nothing to read.
- A malformed line under `### Responses` leaves the record readable.
  `check`, `submit` and `verdict` refuse it as a rule, which can be skipped.
  `merge` cannot skip it: what merges must be valid, and the line may have
  meant `not built`.

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
| Outcome | The human's interventions per mission, by kind: status question, unsticking a stall, correcting direction | 0 to 17 per started mission, median 2.5; 39 of 60 correcting direction |
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
  action with the commit it happened in.
- Coordinating clones beyond the claim. Two clones of one remote cannot
  both start one item, since `start` pushes the branch it makes, and each
  sees the other's work once it has fetched. Nothing else is coordinated
  across clones, and no command fetches more than the base and a lost
  claim's branch.
- An integration branch for a mission, a database or a lock service for
  records.
- Anything for a second user: onboarding docs, installers beyond what exists,
  other forges.
