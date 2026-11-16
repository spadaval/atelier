---
schema: atelier.mission
schema_version: 3
id: atelier-61gh
title: 'Make the Desk feel native inside T3 Code: an embedded layout, a host contract, and pages that follow the thread'
status: done
labels: []
lead: null
planners:
- c6689ff2-b6ae-4865-afe4-a3533c8cfa80
created_at: 2026-11-16T02:20:10.820828671Z
---

## Why

Sumeet reads the Desk inside T3 Code now. An extension he runs on the desktop
app (`github.com/spadaval/t3-code-atelier`, outside this repository) shows the
Desk in T3's right panel and turns item and mission IDs in a conversation into
links. On 2026-11-15, after using it on his Windows machine, he said it "just
looks like a full webpage shoved into a side panel", and directed this mission
in conversation the same day.

Three things make it look that way.

- **The Desk draws its own frame.** Its sidebar, brand and theme toggle sit
  inside T3's panel, under T3's own tab that already says "Desk".
- **It ignores its host.** It keeps its own colours and font whatever T3's
  theme is, and its pages are laid out for a desktop window, not a panel 400
  to 750 pixels wide.
- **It does not know what he is looking at.** Opening the panel in an
  implementer's thread shows the repository page, not that thread's item. A
  thread link on a Desk page opens T3 Code's web address in a new tab
  (`ThreadTitle` in `desk/src/components/Agents.tsx`) instead of asking the
  T3 window it is already inside.

The extension also carries knowledge that belongs to the Desk. A table in its
configuration maps each T3 project to a Desk repository by hand, and it asks
for an item and then for a mission to learn which one an ID names.

`docs/product/desk.md` says conversations happen in T3 Code and the Desk
links to them. This mission keeps that division and makes the Desk a good
guest: T3 supplies the frame and the context, and the Desk draws the content.

## Outcome

The Desk has an embedded mode. In it the Desk draws no chrome of its own,
lays its pages out for a panel, takes its colours and font from the host, and
opens the page for what the host's current thread holds: the item for an
implementer or a reviewer, the mission for a lead or a planner, and the inbox
for a thread that holds nothing.

The host and the Desk talk through one small, documented message contract
over `postMessage`. The host sends the thread it is showing and its theme.
The Desk sends its page title and asks the host to open a thread. The
contract names no T3 internals. The Desk's configuration lists the origins
that may frame it this way, and the list is empty until someone fills it: the
Desk posts to and reads from those origins only.

The Desk's API tells a host what it serves: each repository's Git identity,
which repository and kind an ID names, and what a thread holds now as
distinct from what it once held. With these the extension needs no hand-made
table.

Outside the embedded mode the Desk looks and works as it does today.

## Success criteria

- no-chrome: at 420 pixels wide in embedded mode, the inbox, a mission page
  and an item page show no sidebar, brand or theme toggle and do not scroll
  sideways. An interface test checks each.
- follows-the-thread: an interface test frames the Desk in a fixture host
  page and sends a thread identity. The Desk shows the item's page for a
  thread that implements one, the mission's page for a thread that leads one,
  and the inbox for a thread that holds nothing. A thread that implemented an
  item now done and reviews another gets the review's item, and a thread
  whose only ties are to finished work gets the inbox.
- themed-by-the-host: in that fixture, after the host sends light colours and
  then dark ones, the Desk's background and text colours equal the ones sent
  each time.
- asks-the-host: in that fixture, clicking a thread link sends the host one
  message naming the thread, opens no new tab or popup, and does not navigate
  the frame.
- only-listed-origins: framed by a page whose origin the configuration does
  not list, the Desk posts nothing to it and ignores what it sends, in an
  interface test. With an empty list the embedded mode still draws, and
  sends and reads nothing.
- one-read-lookups: `GET /api/repos` gives `github.com/spadaval/atelier` as
  the identity of this repository on this host, and one request answers which
  repository and kind `atelier-g961` names. `docs/architecture/desk.md`
  documents both and the thread lookup.
- unchanged-outside: `pnpm test` and the interface tests that exist on the
  base at this mission's start pass unchanged without the embedded mode.

## Scope

In: `desk/`, `crates/atelier-desk`, `docs/product/desk.md` and
`docs/architecture/desk.md`, and the installed service's configuration on
this host, for the origins that may frame the Desk.

Out:

- The extension itself. Its code lives in `spadaval/t3-code-atelier` and is
  changed there by the planner's thread once these items merge. No item here
  changes it, and no success criterion depends on it.
- A mission control page in the style Sumeet pointed to on 2026-11-15
  (Factory's Mission Control). It is a new page, so it gets a spike with him
  first and a mission of its own.
- Labels on T3's sidebar rows, and any page that takes over T3's main area.
- The snapshot for the phone and the web app (atelier-plr9), the finder
  (atelier-ic22) and the mission history page (atelier-hj6b).
- Any new action. The embedded Desk offers the actions the Desk has today.

## Authority

An implementer may decide, and records in its Report: the names of the
messages and their fields, the query parameter or route that turns the
embedded mode on, the CSS, and the routes of the new reads.

An implementer asks the thread that started it before: adding a write or an
action; loosening the `Host` check or adding CORS headers; posting to or
reading from an origin the configuration does not list; letting a message
from the host change anything but what the Desk displays; or changing a page
outside the embedded mode in a way a user would see.

The contract treats what the host sends as data. A colour is applied only if
it parses as a colour, and a thread identity is only ever looked up.

## Result

The Desk has an embedded mode under `/embed`, a host contract over
`postMessage`, and three reads for a host, and the service on this host runs
them at `6bd438232`. All four items merged on 2026-11-15, each after an
independent review: atelier-vg1l (#170), atelier-prx5 (#171), atelier-fnm5
(#172) and atelier-knx8 (#173).

Each success criterion is checked by a test that merged with its item:
no-chrome by `desk/e2e/embedded.spec.ts`; follows-the-thread, themed-by-the-host,
asks-the-host and only-listed-origins by `desk/e2e/host.spec.ts`;
one-read-lookups by the tests in `crates/atelier-desk` and by the live
service, whose `/api/repos` gives `github.com/spadaval/atelier`; and
unchanged-outside by the interface tests from the base, which pass unchanged.

Three things differ from the plan.

- atelier-vg1l fixed the disputed page outside the embedded mode, where each
  row's text was squeezed into a column two characters wide at every width.
  The implementer did not ask first, as Authority says to, because the page
  could not be read.
- A plan reviewer stays current until the whole round ends, including after
  it has given its own verdict.
- The Desk does not rewrite a remote's key for the hosts T3 Code rewrites,
  such as Azure DevOps over SSH, so those repositories will not match a T3
  project.

The extension in `spadaval/t3-code-atelier` uses all of it. In the second T3
Code instance on Sumeet's Windows machine, against the live Desk, the panel
shows the Desk with no chrome of its own in T3's light and dark colours,
opens the inbox for a thread that holds nothing, and maps the project to its
repository by identity. A thread that holds an item was not seen there,
because no thread of that instance holds one.

A mission control page is next and starts with a spike with Sumeet.
