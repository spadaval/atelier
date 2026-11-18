---
schema: atelier.review
schema_version: 3
round: 1
commit: c70db872a7ab123a6acd34884615e1f1c25ec986
reviewers:
- thread:delegated-task:command%3Amcp%3Af2b5da1e-fdbf-4cb6-b015-619ee0ca974e%3Adelegate-task%3A9e02087d-7764-4964-a0f8-ae1d6d08c15f
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-18T00:09:53.855589810Z
---

## Summary

- Complexity: high. The change splits the extension's shared lifecycle and several surfaces across typed modules and moves two surfaces into React, with about 2,900 added lines to check.
- Value: high. Typing the host and Desk boundary and using T3's components are required steps in the native-extension mission.
- Risk of merging: medium. Lifecycle and navigation regressions reach users, but show visibly and can be reverted without losing records.

The value justifies the change's size. The module boundaries fit the requested scope; a much smaller change would not accomplish the typed migration. Two reproducible regressions need fixing before merge.

I reviewed master...c70db872a7ab123a6acd34884615e1f1c25ec986 against every Done when line, compared the moved behavior and T3 comments with the old script, and checked the generated Desk API imports, copied component adaptations, and test changes. The changed test accessors preserve their assertions; blurring the already-focused reference makes the previously vacuous check exercise opening the card.

I ran `pnpm typecheck`, `pnpm build`, `git diff --check master...HEAD`, and `TMPDIR=/home/supadava/.cache/atelier-tmp node run-tests.mjs`. All passed, including all five tests. I also ran a temporary copy of the existing browser fixture with two additional probes: first opening the closed mission's hover card, and stopping the injector while that card was open. Both findings below were reproduced against the built bundle. No implementation or repository test file was changed.

I inspected the supplied T3 Code screenshots and the Report's Nightly verification account, but did not independently launch or drive Nightly or rerun `test-contract.mjs` there. The Report explicitly discloses the fixture Desk's missing embed origin and the extra Desk used to verify the contract. GitHub currently reports no checks for PR #203 at the reviewed head, so I cannot confirm remote CI.

## Finding 1: [P1] Stopping with a reference card open leaves T3's panel hidden and inert
Recommendation: fix now

`t3-extension/src/references/references.tsx:328-336` calls `hideCard()` during disposal; when a card is active, `hideCard` calls `drawCard()` at lines 144-150. The loader runs `api.onCleanup` callbacks in reverse registration order. `mountReact` registers its unmount after the Extension constructor registers its disposer loop, so this render happens after the React root has already unmounted. It throws `mountReact: render after unmount` and interrupts the loop in `src/core/extension.ts:58` before the panel, tab, observer, theme subscription and remaining reference cleanup can run.

Reproduction: with the fixture Desk panel open, focus a reference, wait for its card, and run the injector's `stop` command. The browser logs the exception through `drawCard` and `hideCard`; afterwards there is still one `[data-t3-desk-mount]`, one `[data-t3-desk-tab]`, and `#native-content.inert` is true. The panel's visibility rule also remains, so stopping the extension does not restore usable native content. Reinjection follows this same disposal path and can retain stale mounts and overrides.

Make card disposal avoid rendering after React unmount, or order teardown so it happens before unmount. Ensure all owned DOM and native overrides are restored when a card is open; cover stop and reinjection in that state. The existing removal check runs with no card open, so it misses this failure.

## Finding 2: [P2] Build the card's external link from the resolved record
Recommendation: fix now

`t3-extension/src/references/references.tsx:119` computes `externalUrl` before `resolve` has determined the record's kind and Desk. Resolution mutates `reference` and updates state inside `HoverCard`, but does not call `drawCard` again. Consequently the anchor in `src/references/HoverCard.tsx:125` continues using the original string, even though the title and status now show the resolved record.

Reproduction: focus the fixture's newly scanned `atelier-fm02` mission reference before activating it. The card shows the mission, but its external anchor is `http://localhost:<first-desk>/r/atelier/item/atelier-fm02`; it should be `http://localhost:<second-desk>/r/atelier/mission/atelier-fm02`. The first Desk deliberately lacks this record. Even with one Desk, a newly scanned mission starts with `kind: "item"` and therefore keeps the wrong route until the card is reopened. The old script constructed this link after resolution.

Derive the anchor's address from the resolved record in the card's render, or otherwise update it after resolution, and check the first opening of a mission served by the second Desk. The existing test checks direct activation, which uses the resolved reference, rather than the card's external anchor.
