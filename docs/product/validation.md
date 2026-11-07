# Evidence and Validation

Evidence is a durable receipt for a particular claim. Agent Factory decides when
proof needs a tracker record and when independent validation is necessary.
Atelier checks that receipts are honest, commit-bound, and fresh enough for the
boundary that consumes them.

Every receipt names its subject, producer, kind, summary, result, and full Git
commit. Results are `pass`, `fail`, `blocked`, or `not-applicable`. The last
three never count as a passing check. Describe the method, artifacts, and limits
in the body; a broad test run does not establish every acceptance claim.

## What gets a receipt

A receipt records a check that leaves no other record:

- a manual, browser, device or deployed check, such as a page at phone width,
  a run on a real build host, or a live service;
- a mission success criterion (see below);
- a measurement, such as a latency or a count;
- a failure, including a test that fails before its fix.

Suites, formatting and lint that CI or the policy's `merge_checks` run need
no receipt, and nothing in Atelier asks for one after a review fix or a base
merge. Run the check fresh after the last change and read its output all the
same; the item's Close Reason names the targeted test or check that proves
each claim and its result. CI and the merge checks test the merged tree
where a repository has them; a repository without CI puts its suite in the
policy's `merge_checks` (a `shell.check`; see [workflow
configuration](workflow-configuration.md)), or nothing tests its merged tree
([ADR 0026](../adr/0026-evidence-records-only-what-nothing-else-records.md)).

## Before and after merge

Each line of an item's Done when is checked before merge, on the work branch,
unless it is marked to run after merge. Post-merge validation is only what
cannot be checked before the merge: a check that needs the deployed or merged
system, a device or place no agent can reach, the human's judgment, or time
and real use. Its nearest stand-in still runs before merge (an emulator for a
device, a test server for production), and the after-merge check covers only
the gap between the stand-in and the real thing
([ADR 0027](../adr/0027-proof-has-a-time.md)):

```text
- [ ] After merge (device): the launcher opens on the phone. Stand-in: the emulator test passes.
```

Mark the split when the work is planned. Moving a check to after merge once
the item has started is a Done when edit that `merge --check` and the review
brief show (see [reviews and merge](reviews-and-merge.md#a-done-when-edited-after-the-start)).
The item closes on its work branch and merges as usual, and its Close
Reason's Open items name each open after-merge check with its reason and the
stand-in that ran. While a check is open, `status`, `work all`, and `work
mission` and `work parent` list it under Awaiting Validation, the debrief
says so under the item, and closing its mission warns and names it.

Validate on the base, as a planning change that needs no reviewer identity:
run the check, record its receipt (a device or deployed check leaves no other
record), tick the box (`- [x] After merge ...`) and commit the record. If it
fails, file a follow-up bug with `atelier issue create --discovered-from
<item>` and add `carried to <bug>` to the line, rather than reopening the
merged item; the bug's Done when holds the check from then on. `carried to`
counts only when it names an existing issue. Validate every check that can
be validated now; carry only a failed check, to its follow-up bug, or one
that needs time and real use beyond the mission, and name each carried check
in the mission's close report. Nothing refuses an open after-merge check.

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
to check before recording proof, so the receipt describes that committed state.

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
fresh, held at close, stale or superseded as the evidence gates judge it
(a merged done claim at its close, other work at HEAD), ending with the
item's proof verdict; `--result fail` filters for one explicit result, and
`--limit`/`--all` set how many rows. `atelier evidence show <evidence-id>` is
brief, with `--full` for the stored record. `evidence record` confirms in
one line and ends with its verdict line; a recorded fail exits zero, and its
verdict names the item it blocks. A `--target` is an issue ID, or
`issue/<id>` or `evidence/<id>`. Use `attach` only when the existing proof
also supports another item. See [Human CLI Output](human-cli-output.md#evidence-views).
Correct a claim by adding a new receipt, not by rewriting a receipt that later
work may rely on.

## Freshness and conflicting results

A receipt must identify a real full commit, not a short revision, branch name,
or the word `HEAD`. A passing receipt older than the item's last relevant
change is stale: later implementation, policy, or acceptance changes make it
stale, and `evidence list` marks it so. A stale pass does not satisfy an
`evidence.attached` gate a policy configures, but no other gate asks for a
fresh one.

A failure blocks completion (close, `land` and `merge` refuse with
`evidence.no_newer_failure`) until a pass is recorded after it. Any pass
recorded after it clears it for good, whatever its command or pin, even once
later commits make the pass stale. Command capture refuses uncommitted code,
so a captured pass describes committed state; a manual pass is the
recorder's judgment.

Record-only bookkeeping, including attachments, Notes, and status updates,
can follow the checked commit without changing its acceptance claim. Evidence
freshness compares the target's title, type, strategy, and typed body and rejects
intervening non-record changes. Correct receipts by appending new proof rather
than rewriting old observations. Record edits, receipts included, never make
an approval stale (see [approval
freshness](reviews-and-merge.md#approval-freshness)); review records on the
base stay append-only by their own check.

## Handoff

Report each acceptance claim with its result and the targeted test or check
that proves it, with the receipt ID where it has one. Name skipped
checks and limits explicitly. A blocked check needs an explanation of the
blocking condition; a not-applicable result needs an explanation of why the
claim does not apply. Neither is a substitute for a passing requirement.
