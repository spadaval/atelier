---
schema: atelier.review
schema_version: 3
round: 1
commit: 02f6495ac31e75ad37c111a3385faf2fa60b279a
reviewers:
- thread:delegated-task:command%3Amcp%3A433ad4d2-079c-428d-b9f5-c9d5b8b96fbf%3Adelegate-task%3Aee5cb3b0-c98a-4981-a08d-cabe4769d088
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-26T23:03:18.995200847Z
---

## Summary

Complexity: medium. This adds a Pi extension that connects lifecycle hooks, Atelier's prose output, an external guard and tmux.

Value: high. The requested comparison needs a Pi thread to carry Atelier's role and goal across a long session.

Risk of merging it: medium. It changes model continuation and command execution, but the experiment is isolated and can be reverted.

The size is justified by the comparison; no smaller replacement is needed. The three findings below must be fixed first.

I reviewed all six changed files at 02f6495ac31e75ad37c111a3385faf2fa60b279a against every Done when line. `TMPDIR=/home/supadava/.cache/pi-ext-test node --test test.mjs` passes all 12 tests. I checked the installed Pi 1.6.1 extension documentation and runtime source, including nested tool hooks, handler blocking, message delivery and compaction. I also read T3's `packages/provider-pi/src/server/mcpExtensionSource.ts`: the extension does not replace its active tool list, and the handoff suffix matcher covers its public and alias names.

Using Pi's real SDK with a local deterministic response stream, I verified startup injection, the turn-25 reminder, and manual compaction through Pi's built-in summary path. The summary stays intact and the records follow it. That SDK run also demonstrated the unwanted continuation in Finding 2. A scratch repository using the real `atelier status` command demonstrated Finding 1. A real tmux start/read/stop passed, as did additional commands exiting with status 7. PR #288's `changes` job fails for Finding 3; the remaining substantive CI jobs were skipped after it failed.

I did not run another paid Sol turn or launch a T3 child to independently repeat the Report's live handoff checks. The stand-in tests and Pi's documented nested-call contract support the direct and codemode guard behavior; the Report's live smoke-test claims were not independently repeated.

## Finding 1: Read the lead's mission when it has no started items
Recommendation: fix now

`pi-extension/lib.ts:78` only recognizes mission listings containing `(… led by <thread>)`, but `atelier status` prints those under `Missions in progress` only while a mission has a started item. On a backlog branch before the first item starts, or after the last one finishes, the canonical status line is `You lead mission demo-m: Lead test`; the parser returns null, and `groundTruth` returns an empty string. I reproduced this with an open mission led by `lead1` in a scratch repository and the real CLI. The lead then gets neither its outcome and success criteria at startup nor the periodic or post-compaction reminders, contrary to the first and fourth Done when lines. Parse the explicit `You lead mission` line (which also avoids requiring a T3-specific identity variable), and cover this real status shape.

## Finding 2: Append reminders without starting another model response
Recommendation: fix now

`pi-extension/index.ts:62` sends reminders with `{ deliverAs: "steer" }` and leaves `triggerTurn` unspecified. Pi queues that message as new work during `turn_end`, even when the assistant has just completed a final answer with no tool calls. In a real Pi SDK session, 25 prompts with one final response each produced 26 model requests: the reminder forced a second response to prompt 25. With the documented `ATELIER_PI_REMIND_EVERY=1` setting, a single prompt repeatedly produced another reminder and another request; I stopped the local response stub after its fourth request. This changes when the agent stops and can keep it working after completion, which also contaminates the harness comparison. Use Pi's append-only delivery (`triggerTurn: false`) or boundary entries without requesting continuation, and verify both the default cadence and interval 1 against the real runtime.

## Finding 3: Remove the trailing blank line that fails CI
Recommendation: fix now

`.atelier/items/atelier-40e4/item.md:54` adds a blank line at EOF. `git diff --check master...02f6495ac31e75ad37c111a3385faf2fa60b279a` exits 2 for it, and PR #288's `changes` job fails with the same message before the dependent checks run. Remove that blank line so the branch can pass its merge gate.
