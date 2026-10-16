---
schema: atelier.review
schema_version: 1
id: atelier-3rr4
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:29-31: ''about 1,600 orientation commands'' does not match the transcripts. Parsing Bash tool_use inputs in ~/.claude/projects/-home-supadava-projects-atelier/*.jsonl (79 files) gives 110 orientation invocations (atelier work|status|prime|issue show|issue list) deduplicated by tool_use id, 150 without dedup; all Bash atelier invocations of any kind total 1,116. ~1,600-1,731 matches only a raw text grep that also counts prose, prompts, tool output and duplicated resumed-session lines. Pipe counts are roughly right (mine: head 38-62, grep 11-16, sed 9-12, tail 4-5). The wrong denominator makes piping look like ~6% of orientation commands when it is ~40%, understating the ADR''s own argument. Fix: state the method and use the tool_use count (about 110-150), or drop the denominator. Qualitative claims (Hierarchy sed loops, prime | grep -i human) verified.'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:53.511257932Z
event:
  event: finding
  recommendation: fix_now
---

docs/adr/0022-status-orients-and-output-is-dense.md:29-31: 'about 1,600 orientation commands' does not match the transcripts. Parsing Bash tool_use inputs in ~/.claude/projects/-home-supadava-projects-atelier/*.jsonl (79 files) gives 110 orientation invocations (atelier work|status|prime|issue show|issue list) deduplicated by tool_use id, 150 without dedup; all Bash atelier invocations of any kind total 1,116. ~1,600-1,731 matches only a raw text grep that also counts prose, prompts, tool output and duplicated resumed-session lines. Pipe counts are roughly right (mine: head 38-62, grep 11-16, sed 9-12, tail 4-5). The wrong denominator makes piping look like ~6% of orientation commands when it is ~40%, understating the ADR's own argument. Fix: state the method and use the tool_use count (about 110-150), or drop the denominator. Qualitative claims (Hierarchy sed loops, prime | grep -i human) verified.
