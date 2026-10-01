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
or artifact. Command capture runs the supplied command, records its transcript
and exit status, pins HEAD, and derives `pass` or `fail` from the exit status.
A command that cannot start records `blocked`; a command that changes HEAD is
rejected without a receipt. Consult `atelier evidence record --help` for flags.

```text
atelier evidence record --target issue/<id> --kind validation --result pass "Observed claim"
atelier evidence record --target issue/<id> --kind test -- <command>
```

Command capture refuses a caller-supplied result. Commit the state you intend
to check before recording proof, so the receipt describes that committed state.

Use `atelier evidence show <evidence-id>` and `atelier evidence list` to inspect
receipts. Use `attach` only when the existing proof also supports another item.
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
