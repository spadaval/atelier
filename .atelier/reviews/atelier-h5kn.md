---
schema: atelier.review
schema_version: 2
id: atelier-h5kn
title: finding on atelier-5fsj @13bd8595
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r3
commit: 13bd859528342422e94890d4a283c905b59a9b4c
created_at: 2026-11-10T15:44:59.755977766Z
event:
  event: finding
  recommendation: fix_now
---

low: scripts/scorecard/commands.py:70 R7. simple_commands loses quote context through shlex and recursively executes every apparent $(...) substring, including ones that were single-quoted. Thus the synthetic read-only echo '$(atelier status)' yields atelier_calls [['status']], is_process True and one Atelier call in thread_stats, although the shell runs only echo. A literal $(atelier merge --help) similarly invents a help lookup. This retains the quoted-mention counting error R2 required removing and contradicts the README's statement that quoted mentions do not count. Preserve enough quote/escape context to distinguish an executed substitution from literal text, with negative single-quote/escaped-dollar cases and a positive double-quote substitution test. The synthetic arithmetic assertion expects zero Atelier calls and gets one at the pinned head. A narrow scan found no standalone single-quoted substitution-only echo/printf example in the baseline; baseline impact is unproven. This is a supported parser path, so fix before accepting the scripts for reruns. Proof shared with R5: the existing suite, merge checks and baseline reproduction pass, but the additional synthetic regression assertion fails.
