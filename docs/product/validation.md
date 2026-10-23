# Evidence and Validation

Evidence is a durable receipt for a particular claim. Agent Factory decides when
proof needs a tracker record and when independent validation is necessary.
Atelier checks that receipts are honest, commit-bound, and fresh enough for the
boundary that consumes them.

Every receipt names its subject, producer, kind, summary, result, and full Git
commit. Results are `pass`, `fail`, `blocked`, or `not-applicable`. The last
three never count as a passing check. Describe the method, artifacts, and limits
in the body; a broad test run does not establish every acceptance claim.

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

Use `atelier evidence show <evidence-id>` and `atelier evidence list` to inspect
receipts; `list --result fail` filters for a particular explicit result. Use `attach` only when the existing proof also supports another item.
Correct a claim by adding a new receipt, not by rewriting a receipt that later
work may rely on.

## Freshness and conflicting results

A receipt must identify a real full commit, not a short revision, branch name,
or the word `HEAD`. A passing receipt older than the item's last relevant
change does not satisfy the check. Later implementation, policy, or acceptance changes require new proof. A newer failure blocks
completion even when an older passing receipt exists.

Record-only bookkeeping, including attachments, Notes, and status updates,
can follow the checked commit without changing its acceptance claim. Evidence
freshness compares the target's title, type, strategy, and typed body and rejects
intervening non-record changes. Correct receipts by appending new proof rather
than rewriting old observations. [Approval freshness](reviews-and-merge.md)
has the stricter rule for existing proof and review edits.

## Handoff

Report each acceptance claim with its result and receipt ID. Name skipped
checks and limits explicitly. A blocked check needs an explanation of the
blocking condition; a not-applicable result needs an explanation of why the
claim does not apply. Neither is a substitute for a passing requirement.
