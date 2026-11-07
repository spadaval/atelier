# Evidence and Validation

Evidence is a durable receipt for a particular claim. Agent Factory decides when
proof needs a tracker record and when independent validation is necessary.
Atelier keeps receipts honest and commit-bound, and marks how fresh they are. No gate reads them: a receipt is a note for what nothing else
records ([ADR 0028](../adr/0028-simpler-lifecycle.md) decision 5).

Every receipt names its subject, producer, kind, summary, result, and full Git
commit. Results are `pass`, `fail`, `blocked`, or `not-applicable`. The last
three never count as a pass. Describe the method, artifacts, and limits
in the body; a broad test run does not establish every acceptance claim.

## What gets a receipt

A receipt records proof that leaves no other record:

- a manual, browser, device or deployed observation, such as a page at phone width,
  a run on a real build host, or a live service;
- a mission success criterion (see below);
- a measurement, such as a latency or a count;
- a failure, including a test that fails before its fix.

Suites, formatting and lint that CI or the policy's `merge_checks` run need
no receipt, and nothing in Atelier asks for one after a review fix or a base
merge. Run the test fresh after the last change and read its output all the
same; the item's Report names the targeted test that proves each claim and
its result. CI and the merge checks test the merged tree
where a repository has them; a repository without CI puts its suite in the
policy's `merge_checks` (a `shell.check`; see [workflow
configuration](workflow-configuration.md)), or nothing tests its merged tree
([ADR 0026](../adr/0026-evidence-records-only-what-nothing-else-records.md)).

## Work after the merge

Every line of an item's Done when holds, and is ticked, on the work branch
before review is requested. Work that changes no code and can only happen
after a merge or a release, such as pushing a tag, a play-test on a device or
an eval run, is not a Done when line: it is a **loose end**, an issue of type
`loose_end` ([ADR 0028](../adr/0028-simpler-lifecycle.md), decision 3).
Whatever part of it can run before merge, such as an emulator test for a
device or a test server for production, is still a Done when line of the item
it follows.

```text
atelier issue create "Push the v1 tag" --issue-type loose_end --body "## Goal ... ## Done when ..."
atelier issue link <loose-end> <item> --role blocked_by
```

A loose end is `blocked_by` the issue it waits on, so it shows in `work
blocked` until that merges and in `work ready` after, or in `work human` when
labelled `needs-human`. A release has no issue: file its tag on the release
branch with no blocker, and it reaches the base with the [light-path
merge](reviews-and-merge.md#a-change-with-no-issue-the-light-path) and is
ready there. It is never part of a mission: it is no issue's child
and advances no mission, a link `atelier check` reports and a write refuses,
so a mission can close once its code merges. It takes no branch, review or
merge: `atelier start` and `atelier merge` refuse it and name its `done`
transition, and nothing suggests starting it. Do it on the base, then mark it
done there with a one-line result, which `land` accepts:

```text
atelier issue transition <loose-end> done --reason "Pushed v1."
```

Record a receipt for a device or deployed observation, which leaves no other record.
A bad result files a linked bug (`atelier issue create --discovered-from
<loose-end>`) rather than reopening the merged item. A `validation`, an
independent assessment of a mission's criteria or of an increment's
user-facing outcome, is not a loose end, though it shares the loose end's
workflow: it advances its mission, and is done on the base the same way
before the mission closes.

Records written under [ADR 0027](../adr/0027-proof-has-a-time.md) may hold a
Done when line marked `After merge (<reason>): <text>. Stand-in: <...>`.
`review request` does not ask for such a line to be ticked; otherwise it is an
ordinary Done when line. `atelier migrate after-merge` (a dry run unless
`--apply`) turns each unticked one into a loose end `blocked_by` its item: the
text becomes the loose end's Done when, the reason goes in its Goal, and it
is labelled `needs-human` when the reason is the human's judgment or a device
or place no agent can reach, which the dry run shows. An item that is not done
keeps the line's stand-in as an ordinary Done when line, and loses a line that
gave none, unless that would leave its Done when empty: then the last such
line stays without its marker, as an ordinary Done when line, and the
migration names the item so it can be given a check it can meet before
review. A done item is left as it is. Lines carried to an existing issue
(`carried to <id>`) are skipped, and rerunning changes nothing. It refuses
when the policy declares no `loose_end` type.

## Recording proof

`atelier evidence record --target issue/<id> --kind <kind>` creates and attaches a new receipt. Manual proof
requires an explicit `--result` and pins current HEAD; include the observation
or artifact. Command capture runs the supplied command, records its exit status
and a summary of its output, pins HEAD, and derives `pass` or `fail` from the exit status.
A command that cannot start records `blocked`; a command that changes HEAD is
rejected without a receipt. Consult `atelier evidence record --help` for flags.

The receipt's `## Stdout` and `## Stderr` sections give each stream's
`Bytes:`, its `SHA-256:` and, when the stream printed one, its last test-count
line as `Test summary:` (nextest's `Summary [...]` or cargo test's
`test result:`). A passing capture stores nothing more, not the log. A failing
or blocked capture also keeps the stream's text in a fenced block: all of it up
to 4,096 bytes, otherwise the first 1,024 and the last 3,072 bytes joined by a
`… N bytes omitted …` line and marked `Truncated: yes`, so the failure a test
runner prints last stays in the record. Receipts recorded before this format
keep the first 4,096 bytes of every stream and have no digest; they still parse
and are not migrated.

```text
atelier evidence record --target issue/<id> --kind validation --result pass "Observed claim"
atelier evidence record --target issue/<id> --kind test -- <command>
```

Command capture refuses a caller-supplied result. Commit the state you intend
to prove before recording proof, so the receipt describes that committed state.

A receipt that judges mission success criteria names each one with its own
result, so one validation run can pass some criteria and fail others:

```text
atelier evidence record --target issue/<id> --kind validation --result fail \
  --criterion pipe-safe=pass --criterion quiet-tokens=fail "Validated the output"
atelier evidence record --target issue/<id> --kind test --criterion pipe-safe -- <command>
```

Command capture gives each bare `--criterion <slug>` the command's result. The
target must be the mission or an item in the scope of that one mission (an
item in several missions names no criteria; target the mission instead), and
each slug one the mission defines. The receipt stores that mission as
`criteria_mission`, so its proof stays with the mission if the item later
moves (see [mission authoring](mission-authoring.md)); `evidence
record` refuses others before it runs anything, and `check` reports a receipt
whose slug the mission no longer defines, so renaming a criterion never
silently orphans its proof. Issues carry no criterion bookkeeping.

`atelier work mission <id>` marks each criterion from the newest receipt
recorded against the mission that names it and whose commit is in HEAD's
history: `✓ proven` when it passes, `⚠ failing` when it
fails, and `unproven` with no such receipt or a blocked or not-applicable
one. Item-level freshness, which any later code commit breaks, is not
applied to mission criteria; the row instead says how many code commits
followed the deciding proof, and the `Criteria:` line counts proven criteria
whose proof came before later code commits.

Use `atelier evidence list <id>` to see an item's receipts, each marked
fresh, held at close, stale or superseded (a merged done claim judged at the
commit that made it done, other work at HEAD), ending with the
item's proof verdict; `--result fail` filters for one explicit result, and
`--limit`/`--all` set how many rows. `atelier evidence show <evidence-id>` is
brief, with `--full` for the stored record. `evidence record` confirms in
one line and ends with its verdict line; a recorded fail exits zero, and its
verdict says it is a note on the item that no gate reads. A `--target` is an issue ID, or
`issue/<id>` or `evidence/<id>`. Use `attach` only when the existing proof
also supports another item. See [Human CLI Output](human-cli-output.md#evidence-views).
Correct a claim by adding a new receipt, not by rewriting a receipt that later
work may rely on.

## Freshness and conflicting results

A receipt must identify a real full commit, not a short revision, branch name,
or the word `HEAD`. A passing receipt older than the item's last relevant
change is stale: later implementation, policy, or acceptance changes make it
stale, and `evidence list` marks it so. No gate asks for a fresh pass: a
policy naming the `evidence.attached` validator parses, and the name is
ignored.

A failure with no pass recorded after it is marked `⚠ no pass since` in
`evidence list` and noted in the branch views; it blocks nothing, neither
`merge`, `land` nor a done transition. Any pass recorded after it clears the
mark for good, whatever its command or pin, even once later commits make the
pass stale. Command capture refuses uncommitted code,
so a captured pass describes committed state; a manual pass is the
recorder's judgment.

Record-only bookkeeping, including attachments, Notes, and status updates,
can follow the proven commit without changing its acceptance claim. Evidence
freshness compares the target's title, type, strategy, and typed body and rejects
intervening non-record changes. Correct receipts by appending new proof rather
than rewriting old observations. Record edits, receipts included, never make
an approval stale (see [approval
freshness](reviews-and-merge.md#approval-freshness)); review records on the
base stay append-only by their own rule (`review_append_only`).

## Handoff

Report each acceptance claim with its result and the targeted test that
proves it, with the receipt ID where it has one. Name skipped tests and
limits explicitly. A blocked result needs an explanation of the blocking
condition; a not-applicable result needs an explanation of why the
claim does not apply. Neither is a substitute for a passing requirement.
