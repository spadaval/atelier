---
created_at: "2026-11-10T15:59:35.644218292+00:00"
id: "atelier-55cn"
evidence_type: "test"
commit: "3839a17f5ed017f4d5933575af0210c8745902d8"
result: "pass"
captured_at: "2026-11-10T15:59:35.644139566+00:00"
target:
  kind: "issue"
  id: "atelier-5fsj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5fsj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Mutation check after round 3: 28 one-line changes to measure definitions in a scratch copy of scripts/scorecard each made the unittest suite fail on an assertion (adds single-quoted substitution run, substitutions not entered, Python read from any command and heredoc not attached to round 2's 24); the unchanged copy passed. The round 3 regression tests (R5 later cd, R6 Python in echo/grep/heredoc text, R7 single-quoted and escaped substitution) failed against the round 2 commands.py and pass now."
updated_at: "2026-11-10T15:59:35.786944920+00:00"
---

Mutation check after round 3: 28 one-line changes to measure definitions in a scratch copy of scripts/scorecard each made the unittest suite fail on an assertion (adds single-quoted substitution run, substitutions not entered, Python read from any command and heredoc not attached to round 2's 24); the unchanged copy passed. The round 3 regression tests (R5 later cd, R6 Python in echo/grep/heredoc text, R7 single-quoted and escaped substitution) failed against the round 2 commands.py and pass now.
