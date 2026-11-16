---
schema: atelier.review
schema_version: 3
round: 1
commit: c52eca26ceab80d5a774d572fd898310bff69888
reviewers:
- thread:delegated-task:command%3Amcp%3A27fa4239-dd67-4ddb-959b-393a1d0e6b23%3Adelegate-task%3Ab3342ca1-e12d-4074-8a1e-c668082558ce
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-16T03:36:47.725717131Z
---

## Summary

Complexity: medium. The change adds host thread state, asynchronous tie reads
and navigation, with a small pure selector for choosing the page.

Value: high. Opening the current thread's work is a required outcome of the
embedded Desk mission and removes the manual item lookup.

Risk of merging it: medium. It changes the page and the thread-role line users
see in the embedded Desk; the effects are visible and reversible.

The value justifies the size. The message handler, selector and navigation
hook each have a clear job; a materially smaller change would lose required
behavior. Two fixes are needed before merging.

Reviewed `master...c52eca26ceab80d5a774d572fd898310bff69888` against every Done
when line and the Report, including the documented contract and the existing
`/api/ties` response and current-tie semantics. Created a detached worktree
from `origin/master`, then detached it at the submitted commit for testing.
The installed CLI required registering and recording the review in the item
branch checkout, which it printed explicitly.

Validation on the submitted commit: `pnpm test` passed all 57 tests;
`pnpm build` passed its TypeScript check and production build;
`DESK_E2E_PORT=4796 pnpm e2e --workers=2` passed all 104 desktop and phone
interface tests. The fixture host tests exercise the required roles, past
versus current ties, same-thread navigation preservation, thread switching
and an unlisted origin. Additional headless-browser probes against the built
app confirmed that a late response for a replaced thread is ignored and
reproduced Finding 1 with a 503 tie response. No application or test source
was changed for these probes. I did not test the external T3 extension or a
live Desk server; the interface suite and probes use the typed fixtures.

`git diff --check master...HEAD` failed on the item record. PR #173's CI run
43799184920 failed for the same reason before running the Desk job (Finding
2), so local passing tests do not establish passing CI.

## Finding 1: Hide the previous thread's role when the new lookup fails
Recommendation: fix now

In `desk/src/components/Shell.tsx:125`, `followed` holds only a `Place`, with
no identity, and is updated only after a successful read at line 136. A new
identity whose read fails returns at line 132 or reaches the catch without
invalidating the old value. The render at line 169 checks only the page
path, so the old thread's role remains attributed to the thread the host is
now showing.

Reproduced in a browser against the built app: send `host:thread` for
`tied.implementer`, which opens `/embed/r/atelier/item/atelier-uwxy` and
shows "This thread implements this item." Then send `tied.lead` and return
503 for its `/api/ties` request. The page correctly stays put under the
documented failure policy, but the implementer line stays too, although the
current host thread leads `atelier-z4cr`. It remains wrong indefinitely
because repeating the same identity does not retry the lookup.

Associate the chosen place with its thread identity and show the line only
for that identity, or otherwise clear the old line when the thread changes.
Keep the existing policy of leaving the page alone after a failed read. Add
a fixture-host regression for a successful selection followed by another
thread's failed lookup.

## Finding 2: Remove the trailing blank line that blocks CI
Recommendation: fix now

`.atelier/items/atelier-knx8/item.md:92` adds a blank line at EOF.
`git diff --check master...HEAD` exits 2 with "new blank line at EOF".
The `changes` job in CI run 43799184920 fails at `git diff --check` with that
same diagnostic, and its dependent Desk job is skipped. Remove the extra
blank line and obtain a passing CI run on the corrected branch.
