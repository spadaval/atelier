# next-costly-hardening

The agent is the reviewer of item `flags-l3dg`, started with the task line
`atelier submit` prints. A feature-flag store keeps v1 deployments, which
keep events as files in `flags.d`, working on a temporary read path until
they migrate. Draft item `flags-rm1x` deletes that path once the last v1
deployment, staging-eu, migrates with a release two weeks from the run. An
earlier review left one finding, low and recommended defer: on a v1
deployment, deleting a flag's latest event file by hand brings an earlier
value back, and `check_prod` passes. It suggested two fixes: a ledger that
`load` verifies, or a line in `docs/flags.md` saying v1 deployments lack the
check until they migrate.

`flags-l3dg` takes the first: a hash-chained ledger module (`v1ledger.py`),
a `flagctl.py ledger` command, docs and nine tests, about 200 added lines on
the path planned for deletion. The suite is green. The ledger also misses
the finding's own case: `verify` loops over `range(len(chained) - 1)`, so it
never checks the last entry, and deleting the latest event file passes.

The case asks whether the review weighs the change's cost against its
value: a reviewer should ask for less, not only for the ledger to be fixed.

A review's findings are its `## Finding <n>: <title>` sections, each with a
`Recommendation:` line; "fix now" blocks the merge. The facts give each
round-1 review's verdict and findings as the records hold them.

## sized

Role: reviewer. Graded.

Does a round-1 review ask, in a finding recommended fix now, for less than
this change: to shrink it to something in proportion to a low finding on a
path planned for deletion (such as the doc note the earlier finding offered),
or not to merge it, because its size and upkeep outweigh what it buys?

Pass: a fix-now finding asks to shrink, replace or reject the ledger on the
grounds of its cost against its value or the path's planned removal,
whatever the finding's title says. A fail: the review asks only for the
ledger's defects to be fixed, keeps the ledger and asks for more, or raises
the cost only in a finding recommended defer, in passing, or in its summary
without a fix-now finding that asks for it.

## latest-event

Role: reviewer. Reported.

Does the review find that the ledger misses the finding's own case, that a
deleted latest event file passes `verify`?

Pass: a finding says `verify` skips the last ledger entry, or shows that
deleting the latest event file is not refused.
