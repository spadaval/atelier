---
schema: atelier.review
schema_version: 3
round: 1
commit: 0046cbd5204e1d8aeb688ae8e54d3f8a099b19d3
reviewers:
- thread:delegated-task:command%3Amcp%3A433ad4d2-079c-428d-b9f5-c9d5b8b96fbf%3Adelegate-task%3Ae1f17d47-44ff-4b4f-a08f-8263f857e9a7
model: gemini-4-neon
verdict: changes
created_at: 2026-11-27T00:59:57.731843104Z
---

## Summary

Checked the full diff `master...0046cbd5204e1d8aeb688ae8e54d3f8a099b19d3` across `pi-extension/index.ts`, `pi-extension/log.ts`, `pi-extension/README.md`, `pi-extension/test.mjs`, and `.atelier/items/atelier-u88z/item.md`, plus PR #289's CI run. Verified every added Pi event handler (`session_start`, `tool_execution_start`, `tool_execution_end`, `turn_end`, `session_compact`, `session_compact_failed`, `message_end`, `agent_settled`) and context method (`ctx.sessionManager.getSessionFile`, `ctx.getContextUsage`, `ctx.hasPendingMessages`) against Pi 1.6.1's `docs/extensions.md`, `dist/core/extensions/types.d.ts`, `dist/core/extensions/runner.js`, `dist/core/agent-session.js`, and `dist/core/nested-tool-calls.js`. Confirmed all added handlers return `undefined` (including actionable/transform events `turn_end` and `message_end`), send nothing to the model, do not mutate tool calls or messages, and log tool names, IDs, timings, and byte sizes without storing tool arguments or results.

Ran `TMPDIR=$HOME/.cache/pi-ext-test node --test --test-timeout=30000 test.mjs` (all 13 tests pass), `git diff --check master...0046cbd5204e1d8aeb688ae8e54d3f8a099b19d3` (failed on `item.md`), and an end-to-end verification against Pi 1.6.1's `createAgentSession` using a local stub provider (no paid model call) covering `session_start`, nested `executeTool("bash")` calls (`parent` ID and no secret leakage), `turn` context usage, `diagnostics`, `settled`, `compact_failed`, and an unwritable `ATELIER_PI_LOG_DIR` with zero extension errors. Could not check a live provider-triggered auto-compaction (`session_compact` with `reason: "threshold"` or `"overflow"`) against a real model.

## Finding 1: Trailing blank line in item.md fails `git diff --check` and breaks CI
Recommendation: fix now
`.atelier/items/atelier-u88z/item.md:43` ends with an extra blank line. Both `git diff --check master...0046cbd5204e1d8aeb688ae8e54d3f8a099b19d3` (exit code 2) and PR #289's `changes` CI job fail on `.atelier/items/atelier-u88z/item.md:43: new blank line at EOF.`, which blocks `atelier merge` and contradicts the Report's claim that `git diff --check` passes. Remove the trailing blank line.

## Finding 2: Unserializable `message.diagnostics` drops the entire diagnostics row
Recommendation: fix now
In `pi-extension/index.ts:130-132`, `bytes` comes from `size(message.diagnostics)`, which returns `-1` when `JSON.stringify` throws (`pi-extension/log.ts:32-38`). Because `-1 <= 20_000` evaluates to `true`, the unserializable `message.diagnostics` value is passed into `log()` and causes `write()`'s `JSON.stringify` to throw and drop the entire `diagnostics` row instead of recording `{ count, bytes: -1, diagnostics: null }`. Guard the inclusion with `bytes >= 0 && bytes <= 20_000`.

## Finding 3: Earlier tests in `pi-extension/test.mjs` pollute `~/.local/state/atelier-pi/`
Recommendation: fix now
The pre-existing tests in `pi-extension/test.mjs:166-237` emit `session_start`, `turn_end`, and `session_compact` without setting `ATELIER_PI_LOG_DIR`, so every run of `node --test test.mjs` appends synthetic test events into `~/.local/state/atelier-pi/ephemeral-<pid>.jsonl`—the real state directory used to analyze comparison runs (verified by inspecting `~/.local/state/atelier-pi/` after running the test suite). Also, `test.mjs:297` deletes `ATELIER_PI_LOG_DIR` outside a `finally` block rather than restoring it. Point `ATELIER_PI_LOG_DIR` at a temporary directory for the test file (or in `load()`) and restore it cleanly in `finally` around the unwritable-directory check.

## Finding 4: `agent_settled` reads `ctx` accessors outside `try/catch` after awaiting `git`
Recommendation: defer
In `pi-extension/index.ts:137-148`, `agent_settled` awaits `run("git", ["branch", "--show-current"], ...)` (up to 5s) and then evaluates `ctx.cwd`, `ctx.hasPendingMessages?.()`, and `ctx.getContextUsage?.()` before entering `log()`'s `try/catch`. In Pi's `ExtensionRunner` (`runner.js:644-695`), those accessors call `runner.assertActive()`, so if the session is disposed or replaced while `git` is still running, the handler throws into `ExtensionRunner.emitError`. Pi isolates handler errors so this does not disrupt the run, but wrapping the `agent_settled` logging body in `try/catch` would keep logging silent across teardown too.
