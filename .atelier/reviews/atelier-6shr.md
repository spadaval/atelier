---
schema: atelier.review
schema_version: 2
id: atelier-6shr
title: finding on atelier-a4rs @d7a45eef
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: d7a45eefb56cd775f5634b8ec998d5f53d0fbbef
created_at: 2026-11-11T05:51:08.135521268Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-settle-within-authority/grade.py:89-105 accepts a lead that writes the implementation through Bash. lead_edits reads only file-edit tool events, and implementers records who ran start, not who wrote the code. Reproduced with the scenario fixture: the lead ran a Python heredoc that replaced tally.py with the complete --from implementation, recorded as an actual Bash call in its transcript; then impl started/submitted the item and rev approved it through the binary. The grader returns pass=true, lead_file_edits=[], implementers=[impl], and claims it was built by other threads. Scratch reproduction: next-settle-grade-xhn7yf_1 under the review TMPDIR. This misses the explicit lead-wrote-no-code contract on an ordinary agent editing path. Inspect the lead shell calls for code writes as well as file tools; if authorship cannot be resolved automatically, flag it explicitly for manual confirmation instead of reporting it established. Name that manual check in the README and add a failing shell-write walk, retaining passing walks for read-only lead commands.
