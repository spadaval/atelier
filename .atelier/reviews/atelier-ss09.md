---
schema: atelier.review
schema_version: 2
id: atelier-ss09
title: finding on atelier-5fsj @f35e6e42
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r1
commit: f35e6e4206d454b2355d188a33aa4e7689f5c005
created_at: 2026-11-10T14:57:30.564426444Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:15 R2. ATELIER_CALL matches the word atelier anywhere after a non-word character rather than at an executed command position. Synthetic echo of a quoted atelier status is counted as both an atelier call and process, and grep of a quoted atelier merge --help is counted as an atelier help lookup although neither executes atelier. The frozen baseline has three literal echo/search commands in the atelier window included in confusion, one also included as help; this was checked without exporting their content. Conversely, atelier followed by an unknown subcommand is excluded by SUBCOMMANDS even when it produces a real clap error. Recognize executable command positions, including shell wrappers, instead of searching quoted argument and heredoc text, and do not require a known subcommand to recognize an attempted invocation. Add regressions for literal mentions, wrappers, compound commands and invalid subcommands, then regenerate the measures. Both thread counts and transcript token attribution use this recognizer.
