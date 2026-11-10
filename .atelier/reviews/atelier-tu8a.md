---
schema: atelier.review
schema_version: 2
id: atelier-tu8a
title: finding on atelier-5fsj @13bd8595
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r3
commit: 13bd859528342422e94890d4a283c905b59a9b4c
created_at: 2026-11-10T15:44:48.192403548Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:177 R6. Python-write detection scans raw text from every shell command, even when no Python code is executed. For example the read-only command echo "open('src/lib.rs', 'w')" produces shell_writes ['src/lib.rs']; an implementation thread with that command, cargo test and then a source file_change reports first edit at 0 instead of 2. A thread that only prints this example is incorrectly included as an implementer that edited product code. The same issue affects quoted search patterns and Python code being written as data. An aggregate-only scan of the frozen inputs finds two read-only commands classified as Python writes (one in atelier, one in life-on-rails); neither changes an implementer first-edit result in the current baseline, but the supported command classifier is incorrect. Restrict Python write/binding recognition to code passed to an executed Python interpreter (including -c and its heredoc), preserving the scope of that code, and add negative arithmetic tests for echo/grep examples alongside real Python writes. The synthetic assertion fails at the pinned head while all 16 existing tests pass. No private command or message content is quoted here.
