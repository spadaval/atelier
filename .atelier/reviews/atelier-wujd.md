---
schema: atelier.review
schema_version: 2
id: atelier-wujd
title: finding on atelier-5fsj @73542065
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r4
commit: 735420651eb5fc7102b92cbc6b8714c0ea3aa68a
created_at: 2026-11-10T16:04:23.201015026Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:65 R8. Quoted punctuation is reinterpreted as shell syntax after shlex removes its quotes. Synthetic read-only printf '%s\n' ';' atelier status produces atelier_calls [['status']] and is_process True, although it only prints its arguments. echo '>' src/lib.rs is classified as a product write; in an implementation thread followed by cargo test and a real file_change, calls_before_edit is 0 instead of 2. The README explicitly promises quote and backslash handling for the listed separators and redirections, so these are failures of supported forms, not requests to expand the recognizer. Preserve whether a token is syntax or quoted/escaped text before classifying it; add negative call/process and first-edit arithmetic tests. The 16 existing tests, all three merge checks, baseline rerun and snapshot verification pass, but both additional synthetic assertions fail at the pinned head. Frozen headline impact is not established.
