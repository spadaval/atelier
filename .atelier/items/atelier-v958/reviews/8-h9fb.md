---
schema: atelier.review
schema_version: 3
round: 8
commit: a9501e2ea05f53cdbed9fcf132ef9ed41f892782
reviewers:
- thread:delegated-task:command%3Amcp%3Ab13f1839-3e0a-4726-bee5-0d8939767c53%3Adelegate-task%3Ad88c3f35-d372-42e3-83fe-33b164bc03f2
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-14T03:05:59.602374400Z
---

## Summary

Complexity: high. This adds remote and forge polling, persistent news,
stop publication, cross-branch views and pages, and a shared-core/CLI
refactor.

Value: high. Seeing live proposals and retaining a published veto are
required once plans no longer wait for the human's go-ahead.

Risk of merging: high. Incorrect publication or branch state could tell
the human a plan is stopped while it can still proceed.

The value warrants the change. I found no substantially smaller change
that provides the required live views and publication behavior. The
carried fix-now finding is fixed; the relative notification-path problem
below is deferred because the installed host's configured helper uses
`~/`, which works, and the supported notification and stop paths pass.
There is no remaining fix-now finding.

I checked the Done when lines against the code and the product and API
documents. I read the fetch/forge state handling, branch-derived mission,
proposal, motion and dispute responses, isolated stop publication, news
persistence, thread joins, new pages and controls, and the extraction of
`drop_mission` and removal of the Desk's decide code. I also inspected the
complete changes since round 7.

Validation on a9501e2:

- `cargo nextest run -p atelier-desk -p atelier-app`: 82 passed.
  `cargo nextest run -p atelier-cli`: 207 passed, 1 skipped. These cover
  isolated writes, drop-record equivalence, branch states, forge failures,
  PR-list freshness and completeness, and CLI behavior after the shared
  drop-function move.
- `pnpm test`: 41 passed. `pnpm build` passed, including typechecking.
  `pnpm e2e --workers=2`: 50 passed across desktop and phone, including
  the new failed-close regression and the open mission's commands,
  pending and stopped views.
- Reran the previous review's two external Playwright reproductions
  against the new built app. Both now pass: after refresh to
  `stop_pending`, the failed-close diagnostic and recovery command remain
  visible, with a known PR and with no PR mark.
- `cargo fmt -- --check`, `git diff --check` and
  `desk/service/notify-mail.test.sh` passed. The mail test uses a stand-in.
- The installed configuration names the five live repositories and a
  300-second fetch timer. `/api/repos` reports build 77d270f, the reported
  installed code commit, with fetch timestamps and no repository errors.
  `desk-matches-cli` passed against that service: all 16 missions and five
  inboxes match the CLI built from this branch.
- `act-is-a-record` passed using the debug server and current built app:
  the Desk pushed one drop commit to its scratch bare remote, closed the
  stand-in PR, matched the CLI's message, paths and 36-line record, and
  showed Stopped on the reloaded page without browser errors.
- A separate temporary repository and server reproduced Finding 2 with a
  helper beside its configuration. It recorded the proposal but logged
  `cannot run the notify command ./notify-helper: No such file or directory`;
  the helper did not run. No repository source or test file was changed.
- PR #161's test and desk checks succeeded on a9501e2.

I did not stop a live plan, push a stop to GitHub, close a real PR, send
mail or reinstall the service. Publication and authentication under
systemd remain untested by writes; publication tests use scratch remotes
and stand-in forges. Notifications remain disabled on the installed host
as the product document specifies.

## Finding 1: Keep a failed stop publication's result visible after refreshing the page
Recommendation: fix now
Was: 7-8b05.2
Response: fixed in 77d270f
Now: fixed

`StopControl` now renders `ActOutcome` within its `stop_pending` branch.
The server's partial-success text, error and `gh pr close` command survive
`reloadAll()`, while the page retains its pending status and does not claim
that the plan was stopped. The new committed interface regression passes
on desktop and phone. Both external reproductions from round 7 pass too,
including the missing-PR-mark case. The pending note no longer claims that
a known pushed drop still needs pushing merely because the PR is unknown.

## Finding 2: Resolve relative notification helpers beside the config as documented
Recommendation: defer
Taken: atelier-b0lx by mcp:1bb2bc70-d102-4419-8ae0-631379f52f4b

`crates/atelier-desk/src/config.rs:127-133` resolves `notify.command[0]`
only when it starts with `~/`. A path such as `./notify-helper` stays
relative, and `news.rs` passes it to `Command::new` without changing the
working directory. It therefore resolves from the server's process
directory. `docs/architecture/desk.md`, under Running the server, explicitly
says relative paths, including the program of `notify.command`, resolve
from the configuration file's directory.

I reproduced this with a temporary configuration naming
`command = ["./notify-helper"]`, an executable helper beside that config,
and the server launched from the repository. A newly proposed mission
appeared in `news.json`, but the server logged ENOENT for the helper and
sent nothing. This can silently lose a configured notification beyond the
log, since that proposal has already been recorded as seen.

Resolve relative program paths containing a directory component against
the config directory while preserving PATH lookup for bare command names,
or narrow the documented contract to match the implementation. Add a
configuration regression for that distinction. Deferred because the
installed configuration's `~/.../notify-mail` path is resolved correctly;
this affects an alternate configuration and does not block the required
host behavior or the carried fix.
