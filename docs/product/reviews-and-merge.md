# Reviews and Merge

Reviews live as append-only Markdown under `.atelier/reviews/`. There is no
external review backend or native review room. Agent Factory decides who reviews
and how findings are assessed; Atelier guards the recorded invariants.

`atelier review` appends approvals, findings, dispositions, and explicit trivial
exemptions. Each decision identifies its actor, accountable work, and full
commit. An approval must come from an identity other than the change's authors.
Identity is self-declared; Git provides author and ancestry facts.

A finding requires a `FIX NOW`, `DEFER`, or `NO ACTION` disposition. Unresolved
FIX NOW findings block integration. Deferral records the follow-up; no action
records the reason. Corrections append new records instead of rewriting the
review history. A trivial exemption is explicit and visible, with a reason; it
is not an implicit fallback when approval is absent.

## Approval freshness

Approval is pinned to the reviewed commit. Later code, policy, acceptance,
relationship, metadata, or existing proof/review rewrites make it stale.
Post-approval commits may add review or evidence records and make allowed
in-scope status-only changes without staling approval. Evaluate each commit;
a sequence that changes and then restores code does not erase the intervening
change. New material edits require renewed review.

## Integration

Commit the done claim on the work branch, with implementation and proof.
`atelier merge <id>` resolves the branch owner and uses the base branch's
reviewed policy to check committed terminal scope, blockers, evidence, approval
or trivial exemption, and finding dispositions. Uncommitted record changes
cannot become a merge-time completion claim.

Merge fast-forwards only. If the base is no longer an ancestor of the work
branch, reconcile with ordinary Git and renew stale proof or approval. Atelier
creates no commit, writes no tracker state, and does not push. The base obtains
the done claim at the same point it obtains the implementation.

Read `atelier review --help` and `atelier merge --help` for exact operations and
flags. Use [retention and prune](retention-and-prune.md) to clean up work branches
when their records and Git state permit it.
