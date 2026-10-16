---
schema: atelier.review
schema_version: 1
id: atelier-6kcz
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:37-44 transcript figures still do not follow from the stated rule. Rule as written: Bash calls running atelier work/status/prime/issue show/issue list, piped to head/sed/grep/tail. Recount (all JSONL incl. subagents, current session 872b89d8 excluded, heredoc/python -c scripts excluded, pipe counted only when it follows the atelier command in the same pipeline): this repo 56 runs, 41 piped (~3/4); astromech (-astromech-atelier + -astromech dirs) 216 runs, 171 piped (~4/5). The ADR''s ''about a third'' here and ''about half'' there reproduce only if pipelines containing ''2>&1 |'' are dropped (18/56 here, 108/216 astromech), a condition the rule does not state; ''2>&1 | head'' is still piping. So the stated evidence understates piping here, and the implied contrast (earlier model pipes more) is an artifact of the unstated exclusion. Fix: either state ''most piped in both (about three quarters)'' or state the 2>&1 exclusion explicitly and drop the cross-repo contrast.'
target: atelier-bw0b
identity: Independent reviewer 3 (Claude subagent)
commit: 6c51447373552e58ab54acc8783d3a64c6a9991e
created_at: 2026-10-16T23:56:14.991595943Z
event:
  event: finding
  recommendation: fix_now
---

docs/adr/0022-status-orients-and-output-is-dense.md:37-44 transcript figures still do not follow from the stated rule. Rule as written: Bash calls running atelier work/status/prime/issue show/issue list, piped to head/sed/grep/tail. Recount (all JSONL incl. subagents, current session 872b89d8 excluded, heredoc/python -c scripts excluded, pipe counted only when it follows the atelier command in the same pipeline): this repo 56 runs, 41 piped (~3/4); astromech (-astromech-atelier + -astromech dirs) 216 runs, 171 piped (~4/5). The ADR's 'about a third' here and 'about half' there reproduce only if pipelines containing '2>&1 |' are dropped (18/56 here, 108/216 astromech), a condition the rule does not state; '2>&1 | head' is still piping. So the stated evidence understates piping here, and the implied contrast (earlier model pipes more) is an artifact of the unstated exclusion. Fix: either state 'most piped in both (about three quarters)' or state the 2>&1 exclusion explicitly and drop the cross-repo contrast.
