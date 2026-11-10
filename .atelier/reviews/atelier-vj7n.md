---
schema: atelier.review
schema_version: 2
id: atelier-vj7n
title: finding on atelier-5fsj @13bd8595
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r3
commit: 13bd859528342422e94890d4a283c905b59a9b4c
created_at: 2026-11-10T15:44:37.589862469Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:175 R5. Python and patch targets are resolved against the final cd in the whole shell command, rather than the cd before the write. shell_writes processes every simple command first, then scans Python/patch text and calls add() with the final cwd. Synthetic implementation calls consisting of python3 -c "open('src/lib.rs', 'w').write('x')"; cd /tmp/check, then cargo test, then a product file_change report 2 calls before first edit instead of 0. Conversely a Python write under /tmp followed by cd /w/demo is falsely treated as a product edit. This regresses directory resolution in 31d3c6a1 and contradicts the README's preceding-cd rule. Resolve Python and patch targets in their executing command's directory, and add arithmetic tests for a write followed by a later cd in both directions before rerunning the baseline.

Review at pinned head 13bd859528342422e94890d4a283c905b59a9b4c: complexity medium (four readers plus command parsing); value high (required reproducible scorecard); risk high (silent measurement errors guide the redesign). Verdict: the outcome is worth merging after fixes, but approval is withheld. Contract and Fitness fail on R5-R7. Existing 16 unit tests pass. All three requested merge checks pass, as does whitespace checking over the whole range. Two fresh baseline runs give identical JSON and Markdown equal to the committed files; JSON SHA-256 2e8ea0fddc4e089b92645e2496e08a3d151b8a0e04e49517073307bc1c565777. Unknown-schema handling names the expected schema. Snapshot verification passes for 4197 files. Three additional synthetic arithmetic assertions reproduce R5-R7 and fail. The existing fresh-README rerun is recorded with who/when in the Report and atelier-gwrn; I read it, rather than starting another child. Code, snapshot and the four private repositories were not changed, no live stores were opened, and no private message text is included in this review. Residual limits include literal-path write detection, quiet record commits and heuristic thread ties. No unanswered requirement question.
