---
schema: atelier.item
schema_version: 3
id: atelier-5cqz
title: Let framed Desk pages connect in the T3 test instance, and give its done mission items
status: ready
mission: atelier-osel
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: lead of atelier-osel, from the Reports of atelier-toee and atelier-ut89
  at: 2026-11-18T00:25:19.832037677Z
trivial: null
caused_by: null
created_at: 2026-11-18T00:25:03.508714447Z
---

## Goal

Let the T3 test instance show the pages this mission's remaining items must
be seen in. Three gaps in its fixture Desk made the screenshots of
atelier-ut89 and atelier-toee sparse or needed a second Desk by hand:

- The Desk allows no embedding origin, and T3 Code Nightly's page origin is
  `t3code://app`. `GET /api/embed` answers `{"origins":[]}`, so no framed
  Desk page in the instance ever gets `desk:ready`: not the panel, the inbox
  or an item's page. The mission's frames-kept criterion cannot be seen there.
- The Desk's configuration has no `[t3]` table, so the running mission page
  says T3's threads cannot be read.
- The fixture's done mission has no items and no Result, so the finished
  mission page is nearly empty.

## Done when

- [ ] After `node t3-extension/t3-instance.mjs start`, an item's page and the inbox open framed in the instance and finish connecting, with no second Desk and no hand edit.
- [ ] The running mission page in the instance lists the instance's thread among its threads, and does not say T3's threads cannot be read.
- [ ] The finished mission page in the instance shows a Result, at least two lanes and at least two merged items with different numbers of review rounds.
- [ ] The isolation check still passes: the fixture Desk reads the instance's T3 and nothing of the human's.
- [ ] `test-contract.mjs` still passes with every hook against the instance.
- [ ] The Report holds a screenshot from the instance of a framed item page that has connected, of the running mission page and of the finished one.

## Scope

In: `t3-extension/fixture-desk.mjs`, `t3-extension/t3-instance.mjs` and the
fixture's records. The first gap is one line in the instance's `desk.toml`,
`embed_origins = ["t3code://app"]`; atelier-toee's Report says how it was
found. atelier-ut89's Report describes the other two.

Out: the extension's own sources under `t3-extension/src`, which
atelier-40dc is rewriting at the same time, and the Desk's code. If the
`[t3]` table cannot point the fixture Desk at the instance's T3 without a
change to the Desk, say so in the Report and leave that line unticked for
the lead.
