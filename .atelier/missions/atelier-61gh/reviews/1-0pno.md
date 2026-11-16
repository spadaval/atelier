---
schema: atelier.review
schema_version: 3
round: 1
commit: 50ec3589efe3523a381e1445950f1720f64a3508
reviewers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3A97fb36e3-266a-47b7-9308-dfb178e73631
model: claude-opus-5-5
verdict: changes
created_at: 2026-11-16T02:21:49.172505893Z
---

## Summary

I read `mission.md` and the four items (atelier-vg1l, atelier-fnm5,
atelier-prx5, atelier-knx8) at 50ec358, and checked their claims against the
code they name: `desk/src/App.tsx`, `desk/src/components/Shell.tsx`,
`desk/src/components/Agents.tsx` (`ThreadTitle`), `desk/src/styles.css`,
`desk/e2e/fixtures.ts`, `crates/atelier-desk/src/read.rs` (the identity
map), `crates/atelier-desk/src/api.rs`, `docs/architecture/desk.md` and this
host's Desk configuration (`~/.config/atelier-desk/config.toml`). I checked
that `atelier-g961` is a mission and that this checkout's `origin` is
`https://github.com/spadaval/atelier`, so `one-read-lookups` can hold here.
The order of the items (vg1l and prx5 first, then fnm5, then knx8) is sound,
and each item's scope matches what it must change, except as Finding 1 says.

I could not check T3 Code's side: what origin its panel frames the Desk
from, what `repositoryKeyFromRemote` gives, or the extension's code. I built
and ran nothing.

## Finding 1: The thread read gives knx8 too little to choose the page, and knx8 cannot change it
Recommendation: fix now

atelier-knx8 picks "the started item first, then the open review, then the
mission". It needs, for each thing a thread holds, whether the item is
started and whether the thread's review round is open. atelier-prx5's
thread read comes "from the same map `/api/threads` uses", and that map
holds only `{repo, kind, id, role}` (`desk/src/api/Hold.ts`). It also names
every implementer an item ever had and every reviewer of every round, open
or closed (`crates/atelier-desk/src/read.rs:409-437`). So a thread that
reviewed a done item in round 1 still holds it as `reviewer`, with nothing
to tell it apart from an open review. knx8's scope is `desk/src`, the
generated types and the document, not `crates/atelier-desk`, so it cannot
add what it needs; it could only fetch every item page in turn.

Fix: have prx5's thread read give each hold what knx8 ranks by, such as the
record's `status`, whether the thread is its current holder
(`Entry::holder`, `Entry::reviewer`) and the review round's state, and say
in knx8 that it ranks by those fields. Or let knx8 into
`crates/atelier-desk`.

## Finding 2: The contract does not say which origins it posts to or accepts, and the Desk is framable by any page
Recommendation: fix now

atelier-fnm5 says the Desk "reads messages only from its parent window" and
tells its parent "its path and page title" on every navigation, and knx8
lets the parent choose the page by sending a thread identity. Neither item
says what `targetOrigin` the Desk posts with or which origins it accepts.
The Desk sends no `X-Frame-Options` or `frame-ancestors` (no match in
`crates/atelier-desk/src`), and on this host it listens on the tailnet
address as well as loopback (`listen` and `hosts` in
`~/.config/atelier-desk/config.toml`). So any web page open in a browser
that can reach it can frame `/?<embedded switch>`, and with `postMessage(...,
"*")` it would be told the repository, item and mission names and titles of
every page, and could steer the frame through any thread's records. The
mission's "any page that frames the Desk can use it" invites exactly that,
and the Authority section covers the `Host` check and CORS but not this.

Fix: say in the mission and in fnm5 that the Desk talks only to origins its
configuration names (for example a list of embedding origins, empty by
default), posts with that origin as `targetOrigin`, ignores messages from
any other origin, and stays out of embedded mode, or sends nothing, when
the parent's origin is not named. Add choosing those origins to what the
implementer asks about first. The implementer will need T3 Code's panel
origin to test with, so name where to find it.

## Finding 3: Thread links today are web links that open a new tab, not `t3code:` links, so asks-the-host tests the wrong thing
Recommendation: fix now

The mission (Why, and the `asks-the-host` criterion) and atelier-fnm5's Goal
say a thread link opens a `t3code:` link. In the code it does not.
`ThreadTitle` (`desk/src/components/Agents.tsx:26-33`) links only an
`http(s)` address, with `target="_blank"`, and the fixtures and the
architecture document give `link` as `https://app.t3.codes/threads/...`
(`desk/e2e/fixtures.ts:50`, `docs/architecture/desk.md` under
`GET /api/threads`); a `null` or non-web link is shown as plain text. A
test that the click "does not navigate the frame or open a `t3code:` link"
passes on the base's behaviour too, since the click opens a new tab, which
the frame's URL does not show.

Also, `ThreadTitle` is given only `{title, link}`, and the thread marks on
the mission and item pages (`ThreadMark`, `crates/atelier-desk/src/api.rs:402`)
carry no thread ID, so fnm5's "sends the parent one message with the
thread's identity" needs the identity passed down from the `Agent` or
`LiveThread` beside it. fnm5's Context points only at `LiveThread.link`.

Fix: describe the base's behaviour as it is, make the criterion and fnm5's
test check that no new page or popup opens and the frame's URL does not
change, and say in fnm5's Context where the identity comes from on each
page that shows a thread.

## Finding 4: The Agents pages are left out of the panel widths
Recommendation: no action

atelier-vg1l checks the inbox, repositories, mission, item, proposed, in
motion and disputed pages at 420 and 720 pixels, but not `/agents` or
`/r/{repo}/agents`. With the sidebar gone they may be reachable only by
URL, which would make leaving them out fine. The implementer should say so
in its Report, or include them.
