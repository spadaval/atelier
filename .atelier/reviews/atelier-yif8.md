---
schema: atelier.review
schema_version: 2
id: atelier-yif8
title: 'medium: crates/atelier-records/src/lib.rs:2154,2175,2196 Captured text is written unescaped, so a stream containing a line starting with ## or a triple-backtick fence re-parses wrongly: markdown_sections splits at an embedded ''## Stderr'', and first_fenced_block closes at the first inner fence, truncating the summary. Reproduced in a scratch repo: stdout printing ''## Stderr / Bytes: 5 / ``` '' made evidence show attribute the text to Stderr and report Stdout as ''not kept; passing output is summarized'' on a failing receipt. The new comment ''captured text cannot pose as a field'' holds only when the text has no fence. Pre-existing in kind, but failure tails now make such lines (test names, markdown in logs) likelier to be kept. Fix: fence with a longer backtick run than any in the text, and ignore headings inside fences in markdown_sections.'
target: atelier-dg3h
identity: Independent reviewer (Claude Sonnet subagent)
commit: 745baadda963713742a83eb26140afafb667a867
created_at: 2026-10-23T14:30:14.709306738Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 0598e60fd907755f650a830ef27af7199cce377b
    recorded_at: 2026-10-23T14:35:35.221295475Z
    resolution_commit: 4cdaf39f9ed01ecdc89fabff4e53a320cc03a372
    reason: Keeping the failure tail makes fence-shaped lines in output likely to survive; a misparsed tail defeats S3. Fixed with variable-length fences and fence-aware section splitting.
    migrated_from: atelier-h1ah
---

medium: crates/atelier-records/src/lib.rs:2154,2175,2196 Captured text is written unescaped, so a stream containing a line starting with ## or a triple-backtick fence re-parses wrongly: markdown_sections splits at an embedded '## Stderr', and first_fenced_block closes at the first inner fence, truncating the summary. Reproduced in a scratch repo: stdout printing '## Stderr / Bytes: 5 / ``` ' made evidence show attribute the text to Stderr and report Stdout as 'not kept; passing output is summarized' on a failing receipt. The new comment 'captured text cannot pose as a field' holds only when the text has no fence. Pre-existing in kind, but failure tails now make such lines (test names, markdown in logs) likelier to be kept. Fix: fence with a longer backtick run than any in the text, and ignore headings inside fences in markdown_sections.
