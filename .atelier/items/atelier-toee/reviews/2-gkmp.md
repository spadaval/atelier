---
schema: atelier.review
schema_version: 3
round: 2
commit: b59037eedf012d2cc98b3edf0985bb24eac3caec
reviewers:
- thread:delegated-task:command%3Amcp%3Af2b5da1e-fdbf-4cb6-b015-619ee0ca974e%3Adelegate-task%3Aeeb60849-4fbc-4ee3-bc70-25a50405c278
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-18T00:17:43.786730263Z
---

## Summary

Complexity: high. This moves shared lifecycle state and several surfaces from one script into typed modules, while changing the card and settings to React.

Value: high. Typed host and Desk boundaries and separately owned surfaces are required by the mission and cover paths used throughout the extension.

Risk of merging it: medium. A regression can disrupt T3's panel or settings for extension users, but it is visible and the extension can be stopped or reverted.

The value warrants the change. The module boundaries follow the existing surfaces; a smaller extraction would leave the item's typed boundaries and React surfaces unfinished. No new findings.

I reviewed master...b59037eedf012d2cc98b3edf0985bb24eac3caec and the changes since round 1 against every Done when line. I compared the host adapter, shared state, repository selection, panel/tab and reference behavior with the old script, checked the React card/settings and copied component plumbing, and checked that Desk reads import the generated API types. The T3 behavior comments and the Report's explanation of the test changes agree with the code.

Independently ran `pnpm typecheck`, `pnpm build`, `TMPDIR=/home/supadava/.cache/atelier-tmp node run-tests.mjs`, and `git diff --check master...b59037eed`: all passed. All five browser/update suites passed, including the first-open mission card's external address on the second Desk and stopping with a card open. The existing extension assertions retain their meaning; the control access changes follow the React components.

I inspected the committed Nightly light card/panel and dark settings screenshots. I did not rerun the real Nightly instance or its contract test, so the Report remains the evidence for that manual acceptance check. `gh pr checks 203` reported no checks at review time; this approval does not establish a passing CI run.

## Finding 1: [P1] Stopping with a reference card open leaves T3's panel hidden and inert
Recommendation: fix now
Was: 1-0ypk.1
Response: fixed in 295604369. Disposal drops the open card without drawing it after its root unmounted, and `Extension` runs each part's cleanup on its own, so one failure no longer skips the rest. `test-extension.mjs` now stops the extension with a card open and checks that everything goes; that check fails on the previous code ("Full extension removal timed out").
Now: fixed

The references disposer clears timers and drops `active` without calling `hideCard` or rendering. The Extension disposer also isolates each surface's teardown. The full extension removal test passed with a card open and confirmed that the native content is no longer inert.

## Finding 2: [P2] Build the card's external link from the resolved record
Recommendation: fix now
Was: 1-0ypk.2
Response: fixed in 295604369. The card reads the record's address when it draws, after resolution has set its Desk and kind. `test-extension.mjs` opens the card of `atelier-fm02`, a mission only the second Desk serves, before anything resolved it, and checks its Open externally link is `<second Desk>/r/atelier/mission/atelier-fm02`; that check fails on the previous code.
Now: fixed

`HoverCard` now calls the address function during render. Resolution updates the reference's Desk and kind before setting the record state, so the render that first shows its title also builds the correct address. The new assertion passed for an unresolved mission served only by the second Desk.
