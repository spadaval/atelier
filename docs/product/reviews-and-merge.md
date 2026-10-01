# Reviews and Merge

Reviews are immutable Markdown events under `.atelier/reviews/`. Each command
adds a new file with an explicit identity, target, full commit, and explanation.
Agent Factory owns review and finding assessment; Atelier checks the recorded
invariants. There is no external review backend or review room.

## Review commands

Record decisions against the branch owner, normally an epic or a standalone
issue. The commit defaults to current HEAD; `--commit` supplies a full ancestor
commit on the work branch.

```text
atelier review record <owner-id> --identity "Reviewer" "Checks and limits"
atelier review approve <owner-id> --identity "Reviewer" "Approval summary"
atelier review finding <owner-id> --identity "Reviewer" --recommendation fix_now "Problem"
atelier review trivial <owner-id> --identity "Author" "Why correctness is evident"
atelier review list <owner-id>
atelier review show <review-id>
```

`record` stores a review account; `approve` stores approval. Approval must come
from an identity other than the substantive Git authors. Identity is
self-declared and compared with author names and email addresses. `trivial`
is an explicit visible exemption with a reason, never an implicit fallback.
`review list` accepts `--json` and can omit the owner to list all events.

Every finding requires a disposition. Recommendations and decisions use
`fix_now`, `defer`, or `no_action`:

```text
atelier review disposition <finding-id> --identity "Reviewer" --decision fix_now --resolution-commit <full-commit> "Fix verified"
atelier review disposition <finding-id> --identity "Reviewer" --decision defer --follow-up <issue-id> "Follow-up accepted"
atelier review disposition <finding-id> --identity "Reviewer" --decision no_action "Reason accepted"
```

A FIX NOW disposition needs a resolution commit later than the finding's pin,
followed by fresh independent approval. DEFER requires an existing follow-up
issue. DEFER and NO ACTION need acceptance from a non-author identity. All
findings without accepted dispositions block merge. A later disposition
automatically links to the preceding disposition; commit the new
file rather than editing or deleting an old event. Merge checks review history
from the base, including edits subsequently reverted.

## Approval freshness

Code, policy, acceptance, relationship, metadata, or existing proof/review edits
make approval stale. Later commits may add new review/evidence files or change
in-scope lifecycle status, timestamps, and Close Reason without changing the
approved acceptance. Each commit is checked; changing and then restoring code
does not recover an old approval. New material edits require renewed review.

## Integration

Commit the implementation, proof, branch ownership, and done claim on the work
branch. The owner and its scoped children must be terminal with no open
transitive blockers. Evidence must satisfy the base policy's completion checks;
a newer failure blocks success. Review must have current independent approval
or an explicit trivial exemption and accepted finding dispositions.

```text
atelier merge <owner-or-child-id> --check
atelier merge <owner-or-child-id> --check --json
atelier merge <owner-or-child-id>
```

Run these from the recorded work branch. Source and any separate base checkout
must be clean. Canonical record and policy paths, bytes, and modes must match
committed HEAD, including ignored canonical files; Git index flags such as
assume-unchanged or skip-worktree cannot hide changed tracked files. Checks
cannot authorize a different branch, HEAD, base, or canonical snapshot while
they run. `--check` evaluates the gates without integrating.

Merge uses the base branch's committed policy and fast-forwards only. If the
base has diverged, reconcile with ordinary Git and renew stale proof or review.
Bootstrap a valid committed policy on the base before starting governed work;
a work branch cannot weaken its own merge policy. See
[workflow configuration](workflow-configuration.md).

Atelier creates no commit, writes no tracker state, and does not push. Git hooks
are disabled during gated integration and cleanup, with an explicit notice.
Put required read-only checks in the reviewed base policy's `merge_checks`.

Successful integration deletes the local work branch. If the base has a
separate checkout, the source checkout becomes detached and the command prints
its worktree cleanup instruction. Leave that checkout before removing it from
the base checkout. If cleanup fails after integration, inspect the named Git
state before retrying cleanup. Use [prune](retention-and-prune.md) for remaining
eligible artifacts.
