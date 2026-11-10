---
created_at: "2026-11-10T16:22:33.761553231+00:00"
id: "atelier-x1hy"
evidence_type: "test"
commit: "e0e65ac147676bc4fbb143c682d0e561e7b6c1a7"
result: "pass"
captured_at: "2026-11-10T16:22:33.761475556+00:00"
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
title: "Mutation check after round 4: 36 one-line changes to measure definitions in a scratch copy of scripts/scorecard each made the unittest suite fail on an assertion (adds quotes ignored, double-quoted text split, single-quoted text read as shell, substitutions read out of order or not at all, wrapper option arguments kept, Python read as text and comments read as commands to round 3's set); the unchanged copy passed. Each round 4 regression (R8 quoted ; and >, R9 cd before and inside a substitution, R10 Python string and comment, R11 env -u and stdbuf -o, a comment holding ;) failed against the round 3 commands.py and passes now."
updated_at: "2026-11-10T16:22:33.901475021+00:00"
---

Mutation check after round 4: 36 one-line changes to measure definitions in a scratch copy of scripts/scorecard each made the unittest suite fail on an assertion (adds quotes ignored, double-quoted text split, single-quoted text read as shell, substitutions read out of order or not at all, wrapper option arguments kept, Python read as text and comments read as commands to round 3's set); the unchanged copy passed. Each round 4 regression (R8 quoted ; and >, R9 cd before and inside a substitution, R10 Python string and comment, R11 env -u and stdbuf -o, a comment holding ;) failed against the round 3 commands.py and passes now.
