---
schema: atelier.item
schema_version: 3
id: atelier-9z58
title: Say a repository could not be read, where the native pages now show it as empty
status: approved
mission: atelier-osel
labels: []
priority: medium
blocked_by:
- atelier-wvst
implementers:
- thread:delegated-task:command%3Amcp%3A7aabecbe-fbf4-40c7-b97b-a453084bded8%3Adelegate-task%3A735d65ac-63bf-41cb-b025-4838112805f2
ready:
  by: lead of atelier-osel, from deferred finding atelier-9l8h/1-wi75.3
  at: 2026-11-18T02:11:47.307276428Z
trivial: null
caused_by: null
created_at: 2026-11-18T02:11:39.193307375Z
---

## Goal

Taken from deferred finding atelier-9l8h/1-wi75.3, in .atelier/items/atelier-9l8h/reviews/1-wi75.md:

> Finding 3: Show repository failures carried inside successful inbox and motion responses
> Recommendation: defer
>
> `t3-extension/src/thread/ProjectGlance.tsx:25-27,42-59` reads a repository's rows but checks only the request-level error. It never checks `moving.error` or `waiting.error`. The backend intentionally returns HTTP 200 with an error and empty arrays for a failed repository (`crates/atelier-desk/src/server.rs:660-664,721-725`). `ThisThread` also uses the inbox for its waiting/empty decision without surfacing those repository errors.
>
> Reproduced by having the `nothing` fixture return HTTP 200 for `/api/motion` and `/api/inbox`, with the selected repository's error set to `fixture repository unreadable` and its result arrays empty, as the backend does. The page showed zero notices, no Waiting on you, `Nothing is started` and `Nothing is ready to start`. These are unknown results presented as empty results.
>
> This is inherited from the Desk's existing `ProjectGlance`, so it need not expand this item's migration. Track a follow-up to expose per-repository errors in both presentations and avoid empty-result claims for the unreadable parts, with a successful-HTTP/failed-repository fixture.

## Done when

- [x] When the Desk answers `/api/motion` or `/api/inbox` with HTTP 200 and an error for a repository, This thread and the project home name that repository and its error, and do not say nothing is started, nothing is ready or nothing waits on the human for it.
- [x] A fixture check for each page fails without the change: the response is HTTP 200 with the selected repository's error set and its arrays empty, as `crates/atelier-desk/src/server.rs` answers for a repository it could not read.
- [x] Any other read the native pages make that carries a per-repository error the same way is handled the same, and the Report lists which reads were looked at.

## Scope

In: the native pages under `t3-extension/src` (`thread/ProjectGlance.tsx`,
`thread/ThisThread.tsx`, and the project home atelier-wvst adds) and their
fixture checks.

Out: the Desk's own `ProjectGlance` and pages, which atelier-yy4j deletes
where T3 no longer needs them; if a Desk page that stays has the same fault,
say so in the Report. The Desk's responses do not change.

Blocked by atelier-wvst so that the project home exists and the two do not
edit the same files at once.

## Report

A repository the Desk could not read is now named on the native pages instead of being drawn as
empty. The Desk answers `/api/motion`, `/api/inbox` and `/api/sidebar` with HTTP 200, that
repository's `error` set and its lists empty. Before this change the pages checked only whether
the request failed.

- This thread (`t3-extension/src/thread/ThisThread.tsx`): each repository the inbox answers with
  an error becomes a note, "Could not read the repository atelier, for what waits on you (GET
  /api/inbox): <error>". It counts as a failed read, the same as an inbox request that fails, so the
  page does not say the thread holds nothing. I made it consistent with the existing rule ("until
  every read answered, and if one failed, the page does not say the thread holds nothing"). The cost:
  while any repository on the Desk is unreadable, a thread that holds nothing shows the note and no
  project glance. The other choice was to name the error and still show the glance, since the inbox
  only feeds "Waiting on you". I did not take it, because it would treat a failed repository
  differently from a failed request.
- The project at a glance (`t3-extension/src/thread/ProjectGlance.tsx`): an error for the selected
  repository in `/api/motion` or `/api/inbox` becomes a note, "The Desk could not read what is in
  motion in atelier (GET /api/motion): <error>". The In motion and Next up blocks are left out, so
  it never says "Nothing is started" or "Nothing is ready to start" for a repository it could not
  read. Waiting on you shows only when something waits, so it never claimed "nothing waits".
- The sidebar's missions (`t3-extension/src/sidebar/missions.ts`): a repository `/api/sidebar`
  could not read is named in the missions' error note. Before, it showed no missions with no word why.
- Settings, Desks (`t3-extension/src/settings/Customizations.tsx`): a repository `/api/repos` could
  not read is listed as `name (could not be read: <error>)` and marks the Desk's line red. Before,
  it showed `name (identity)` as if it were well.

Fixture checks, each confirmed to fail with its source change reverted:
- `test-thread.mjs`: `?unreadable=<path>` in `check/thread.tsx` answers that path the way the Desk
  answers for an unreadable repository. The `nothing` scenario with `/api/inbox` unreadable names
  the repository and does not say "holds nothing". With `/api/motion` unreadable, the glance names
  it and draws neither In motion nor Next up.
- `test-extension.mjs`: the existing unreadable-repository case (option renamed `inboxError` to
  `unreadable`) now also adds the repository to `/api/sidebar` and checks the missions' note
  names it, then clears.

Every read the extension makes was looked at for a per-repository error:
- Carry one: `/api/inbox` (This thread and the glance, fixed here; the sidebar's Inbox count already
  named it), `/api/motion` (the glance, fixed), `/api/sidebar` (the sidebar's missions, fixed),
  `/api/repos` (repositories and the settings page, fixed in settings; the repository list keeps
  the repository, so mapping is unaffected).
- Carry none (the read is for one repository and fails with an HTTP status):
  `/api/repos/<repo>/missions`, `/api/repos/<repo>/missions/<id>`, `/api/repos/<repo>/items/<id>`,
  `/api/roadmap`, `/api/history`, `/api/launch`, `/api/ties`, `/api/thread` and `/api/records/<id>`.
  So the project home (`src/home/ProjectHome.tsx`), which reads only missions, a mission, the
  roadmap and the history, needed no change. The extension does not read `/api/proposed` or
  `/api/disputed`.

The Desk's own pages: `desk/src/components/Glance.tsx` (`ProjectGlance`) has the same fault. It
checks only the request-level error of `/api/motion` and `/api/inbox`. Only the Desk's This thread
page uses it, and atelier-yy4j deletes that page, so I left it. The Desk's Inbox, Motion, Proposed,
Disputed and Repos pages already show each repository's error.

Ran: `pnpm typecheck`, and `node run-tests.mjs` (all 7 tests pass) against a Desk built from this
checkout.
