---
schema: atelier.review
schema_version: 3
round: 2
commit: 099b0ee4a20ccfd24e5280d9862b29c97d60e4d0
reviewers:
- thread:delegated-task:command%3Amcp%3A2c4752e5-6847-44cc-9070-11e693da27de%3Adelegate-task%3A81deba86-6011-428b-b64c-8f267d6d5f51
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T23:31:30.160536461Z
---

## Summary

Complexity: high. The change introduces a persistent process lifecycle, a downloaded Electron application, UI automation and an isolation check.

Value: high. Five mission items need a real T3 instance, and this removes their dependency on the Windows machine being available.

Risk of merging: high. Process cleanup and configuration discovery run as the same user as the live T3 server, so isolation mistakes could affect its data or credentials.

The value justifies the tool, but the submitted script must parse before it can provide that value.

I read the full change against master and the round 1 fixes, including the fixture refactor and README, at commit 099b0ee4a20ccfd24e5280d9862b29c97d60e4d0. `git diff --check master...099b0ee4a20ccfd24e5280d9862b29c97d60e4d0` passes. Both `node --check t3-extension/t3-instance.mjs` and `TMPDIR="$HOME/.cache/atelier-tmp" node t3-extension/t3-instance.mjs status` fail with the syntax error in Finding 3. The offending line is also present in the committed blob.

The parser error prevents runtime verification of start/reuse/stop, the free-model refusal, isolation, fixture ties, contract probes and screenshots at this head. I did not modify the implementation or launch a real instance. The README documents the pinned release, dependencies, sandbox exception and Windows release check. The previous findings' underlying changes are present on inspection, as detailed below, but their runtime regressions need rerunning after Finding 3 is fixed.

I also ran `TMPDIR="$HOME/.cache/atelier-tmp" node t3-extension/run-tests.mjs`. `test-fixture-desk.mjs` and `test-update.mjs` pass. `test-extension.mjs` fails waiting for its fixture page on port 19222; its browser log reports `bind() failed: Address already in use (98)`. That is the host port conflict described in the Report, not evidence of a regression in this change. No process was killed by name pattern. The PR's extension CI job was still running when inspected; I did not wait for it because the parse failure is independently reproducible.

## Finding 1: Isolate OpenCode's directory discovery as well as its environment
Recommendation: fix now
Was: 1-d4r6.1
Response: fixed in 538d83e04: the instance moved to /var/tmp outside the home folder, probes run in its home, and isolation reads OpenCode's log.
Now: fixed

The default root is outside the real home, `start` rejects a root inside it, and both OpenCode probes now set `cwd: home`. The runtime project is also under the new root. The isolation check now examines OpenCode's watched/working directories in its logs. These changes address the caller-directory and real-home discovery paths in the original reproduction. Runtime confirmation is blocked by Finding 3.

## Finding 2: Serialize starts before deleting the shared instance directory
Recommendation: fix now
Was: 1-d4r6.2
Response: fixed in 538d83e04: start takes start.lock before reading state or clearing the folder; a second start waits and reuses.
Now: fixed

`start` now acquires a complete hard-linked lock with its owner's PID and start time before calling `startLocked`, which reads or clears state. It holds the lock until initialization completes and makes another live starter wait. This addresses the original overlap between two starts on an initially unlocked root. Runtime confirmation is blocked by Finding 3.

## Finding 3: Fix the syntax error that prevents every instance command from running
Recommendation: fix now

`t3-extension/t3-instance.mjs:527` puts `human's home` inside a single-quoted JavaScript string without escaping the apostrophe. Both `node --check t3-extension/t3-instance.mjs` and the documented `status` command exit 1 with `SyntaxError: missing ) after argument list`. Node parses the whole module before dispatching commands, so `start`, `stop` and `isolation` are equally unusable. In particular, even an already-running instance cannot be stopped with the submitted script. Quote this string correctly and rerun the checks against the final submitted head; the Report's successful runtime results cannot describe this head.
