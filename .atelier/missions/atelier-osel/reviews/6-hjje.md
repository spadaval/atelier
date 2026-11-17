---
schema: atelier.review
schema_version: 3
round: 6
commit: d5de3cc653f1f0396ebad55513d983285d08d345
reviewers:
- thread:delegated-task:command%3Amcp%3Ad12574ac-fe2d-4335-bf72-c30aa232c7f3%3Adelegate-task%3A0511fa34-9067-4d95-b20d-03d7a4faf984
model: gpt-6-astra
verdict: approve
created_at: 2026-11-17T21:11:25.989828683Z
---

## Summary

Approve the plan at d5de3cc653f1f0396ebad55513d983285d08d345. The
three carried findings are fixed in the plan; no new fix-now finding.

Complexity: high. The mission introduces a build and React mounting,
rewrites the host integration, moves three pages, and adds a Linux test
instance with provider and fixture setup.

Value: high. It improves the extension used to read Atelier and lets the
five implementation items check the real T3 app without waiting for the
Windows machine to be available each time.

Risk of merging it: medium. Implementation affects the daily extension,
but the work is split into items with preserved behavior checks, and the
Desk pages remain until the replacement release is installed. The test
instance now requires its own provider state.

The value warrants the work. Merely restyling the framed pages would not
satisfy the native-component and typed-code outcomes. The Linux command
has a specific purpose and retains a Windows check before release.

Read the mission and all nine items, compared the amendment with the
merged plan and the last-round changes, and checked the affected contracts
against test-contract.mjs, fixture-desk.mjs, deskReadUrl, the Desk's API
calls and page documentation. Read the relevant mockup markup and the
build spike's package and style-verification code on spike/extension-react.
Checked the dependency order, the seven added reads, the distinction
between native and remaining framed pages, and the removal gate.

`atelier check` validates all 197 items, 13 missions and config.toml. It
also reports existing empty Why sections on three unrelated missions.
No implementation tests, live T3 session or visual comparison were run:
this change is a plan. In particular, I did not independently reproduce
the fresh-home OpenCode experiment or the still-untried T3-side setup.
The plan explicitly leaves that setup and its proof to atelier-diru,
with an escalation path if the thread cannot be made.

## Finding 1: atelier-ut89 must not wait on atelier-diru before it starts
Recommendation: fix now
Was: 5-fgpi.1
Now: fixed

atelier-ut89 now depends only on atelier-6dm7 (item.md lines 10-11).
Its Context explicitly permits implementation before atelier-diru merges,
while the final seen-in-T3 criterion still requires that instance
(lines 39 and 49). This removes the start dependency without dropping
verification.

## Finding 2: atelier-diru's "every hook" line names a path the spike did not try, and omits setup test-contract.mjs already requires
Recommendation: fix now
Was: 5-fgpi.2
Now: fixed

atelier-diru now requires a message made inside T3 Code, the boring
sidebar, a project thread, and a new-thread draft of that project. The
start command prints both --thread and --draft for the contract check
(item.md lines 30-33). These match test-contract.mjs's thread-route,
folder-layout and draft-composer checks. Context names enabling OpenCode
and the remaining untried steps, distinguishes the CLI experiment from
a T3 thread, and requires asking the starting thread if this cannot work
(line 45). The plan no longer treats the experiment as proof that the
full setup has run.

## Finding 3: Linking the human's live OpenCode folders into the test instance breaks the isolation diru claims
Recommendation: fix now
Was: 5-fgpi.3
Now: fixed

atelier-diru now requires OpenCode to run in the instance's own home,
with no personal folder linked or copied in, no credential, and refusal
of a non-free model (item.md line 30). The mission and item context no
longer prescribe the links. This fixes the proposed shared-state path;
the implementation review still needs to verify those isolation
requirements in the launcher it builds.
