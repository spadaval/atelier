---
schema: atelier.review
schema_version: 3
round: 3
commit: 0a23ddd827247f339b5504c17c81a31937c53cf4
reviewers:
- thread:delegated-task:command%3Amcp%3A2c4752e5-6847-44cc-9070-11e693da27de%3Adelegate-task%3A65ebff2c-e490-4bcf-9084-dc86ea40a17c
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T23:37:59.114869723Z
---

## Summary

Complexity: high. This adds a persistent process lifecycle, a downloaded Electron application, UI automation and an isolation check.

Value: high. Five mission items need a real T3 instance, and this removes their dependency on the Windows machine being available.

Risk of merging: high. Configuration discovery and process cleanup run as the same user as the live T3 server, so isolation mistakes could affect its data or credentials.

The value justifies the tool. I found no remaining fix-now issue in the submitted change.

I read the full diff against master at 0a23ddd827247f339b5504c17c81a31937c53cf4, the changes since round 2, the fixture refactor and the README. `node --check t3-extension/t3-instance.mjs` and `git diff --check master...HEAD` pass. All test commands used `TMPDIR="$HOME/.cache/atelier-tmp"`.

Two overlapping `start` commands exited successfully with one instance: the second waited on the lock, then printed the first instance's thread and draft. The free model answered `ready`. With the printed values, `test-contract.mjs` passed all 35 probes. The fixture's `/api/ties?thread=<id>` returned the instance's thread as implementer of `atelier-fi01` and lead of `atelier-fm01`; `/api/sidebar` showed the running mission, and `/api/records/atelier-fm02` returned the done mission. `shot` saved both PNGs, which I opened and checked for the light and dark themes. `stop` ended all 13 instance processes by PID.

`isolation` then passed: it checked 15 instance processes, stopped them, found no instance IDs in the 19 changed/added and one removed host entries, found no OpenCode watched/working folder inside the real home, and reported `No authenticated integrations`. An earlier isolation invocation ended with SIGTERM after stopping its processes, before its final scan result; its cause was not established. I reran the unchanged command in a terminal session and obtained the complete successful result above.

`start --model opencode/claude-fable-5-2` refused the named model and listed the credential-less models. A root inside the real home was refused before starting anything. With neither display server on PATH, start exited with one line naming Xvnc, Xvfb and the packages to install. `node t3-extension/run-tests.mjs` passed all three tests, including `test-extension.mjs`; the noted port conflict did not occur in this run. The PR's extension CI job also passed at the submitted head.

I used the cached pinned AppImage rather than downloading it again, so I checked the fetch/extract path by inspection only. I did not check Windows; the README explicitly retains that check before release. I changed no implementation file and used only the documented instance subcommands for its lifecycle.

## Finding 1: Fix the syntax error that prevents every instance command from running
Recommendation: fix now
Was: 2-4ito.3
Response: fixed in cc9d69645: the string is double-quoted; node --check passes and every command was rerun at that head.
Now: fixed

Line 527 now uses a double-quoted string. The module parses, `status` runs, and the successful start, contract, screenshot, stop and isolation runs above confirm that the commands are usable at the submitted head.
