---
schema: atelier.item
schema_version: 3
id: atelier-plr9
title: 'Show a mission''s page in the thread: a one-file snapshot an agent renders with html_render'
status: draft
mission: null
labels:
- feature
priority: medium
blocked_by:
- atelier-v958
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T22:00:46.633461900Z
---

## Goal

Let an agent show a mission where it is being discussed, instead of saying "I've updated atelier-xxxx, go check the Desk". Sumeet asked for this on 2026-11-13: the Desk's tailnet address needs a link, and its pages are cut off from the thread. T3 Code's `html_render` tool shows a self-contained HTML file inline in a thread, on web and mobile, and keeps it as an attachment after the session unloads. So an agent that has committed a change to a mission runs one command, which writes that mission's Desk page as one file with its data inside, and shows it with `html_render`.

An MCP App card and T3's Browser panel were weighed and turned down in the same conversation: the first needs an MCP endpoint and config per provider and dies with its session, and the second streams from the server and lags off the LAN.

## Done when

- [ ] A trial comes first: a hand-made mission page shown with `html_render` has been judged by Sumeet on web and on his phone, and the Report says what a link inside the frame does on each and whether full screen is usable for a long page. If he turns it down, the item stops there and the Report says why.
- [ ] `pnpm build` writes a second, single-file page beside the app, with its script and styles inlined and no web fonts. It takes its colours and fonts from T3's theme variables, so it matches light and dark, and sets no page background.
- [ ] `atelier-desk page <id> --out <file>`, run in any checkout, writes that file for the mission or item `<id>` as the checkout's `HEAD` commit holds it. It embeds the responses the server would give for the mission, each of its documents and each of its items, as a map from API path to JSON, and the app reads that map where it would fetch. It finds the template in the configuration's `app` directory.
- [ ] The command prints the `html_render` call to make: the path, a title and a height.
- [ ] The page's first lines name the commit, the branch and the time it was made, and link the live Desk page. It offers no act and lists no live threads.
- [ ] Shown inline, the page is a brief that fits the height the command names: title, status, item counts and the plan reviews' verdicts. Full screen shows the whole mission page with its tabs and documents.
- [ ] A test writes a snapshot of a fixture mission and finds its mission data equal to the server's response for the same commit.
- [ ] The skill tells a planner or lead that proposes or changes a mission to show its page, and `docs/product/desk.md` says no agent reads the Desk, though one may show its page to the human. `docs/architecture/desk.md` describes the command and the build.

## Scope

In: `desk/`, `crates/atelier-desk`, the Desk's two documents, and the planner's and lead's files in `skills/agent-factory`.

Out: any change to T3 Code; an MCP endpoint on `atelier-desk`; acts from a snapshot; the `atelier` binary, which the Desk's app stays out of; a snapshot of the inbox, the repositories or the agents page.

## Context

Every read in the app goes through `fetchInto` in `desk/src/lib/api.ts`. `html_render` takes an absolute file path, of at most 25 MiB, and a height from 80 to 2,000 px; past that height a phone scrolls inside the frame. Its inline HTML is capped at 512,000 characters, less than the app's script, so the file path is the only way. The built app was 493 kB of script and 32 kB of styles on 2026-11-11. It waits on atelier-v958, which rewrites the mission page and reads plans from branches.
