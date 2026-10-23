---
schema: atelier.review
schema_version: 1
id: atelier-yif8
title: 'medium: crates/atelier-records/src/lib.rs:2154,2175,2196 Captured text is written unescaped, so a stream containing a line starting with ## or a triple-backtick fence re-parses wrongly: markdown_sections splits at an embedded ''## Stderr'', and first_fenced_block closes at the first inner fence, truncating the summary. Reproduced in a scratch repo: stdout printing ''## Stderr / Bytes: 5 / ``` '' made evidence show attribute the text to Stderr and report Stdout as ''not kept; passing output is summarized'' on a failing receipt. The new comment ''captured text cannot pose as a field'' holds only when the text has no fence. Pre-existing in kind, but failure tails now make such lines (test names, markdown in logs) likelier to be kept. Fix: fence with a longer backtick run than any in the text, and ignore headings inside fences in markdown_sections.'
target: atelier-dg3h
identity: Independent reviewer (Claude Sonnet subagent)
commit: 745baadda963713742a83eb26140afafb667a867
created_at: 2026-10-23T14:30:14.709306738Z
event:
  event: finding
  recommendation: defer
---

medium: crates/atelier-records/src/lib.rs:2154,2175,2196 Captured text is written unescaped, so a stream containing a line starting with ## or a triple-backtick fence re-parses wrongly: markdown_sections splits at an embedded '## Stderr', and first_fenced_block closes at the first inner fence, truncating the summary. Reproduced in a scratch repo: stdout printing '## Stderr / Bytes: 5 / ``` ' made evidence show attribute the text to Stderr and report Stdout as 'not kept; passing output is summarized' on a failing receipt. The new comment 'captured text cannot pose as a field' holds only when the text has no fence. Pre-existing in kind, but failure tails now make such lines (test names, markdown in logs) likelier to be kept. Fix: fence with a longer backtick run than any in the text, and ignore headings inside fences in markdown_sections.
