---
created_at: "2026-06-12T04:58:38.294509848+00:00"
id: "atelier-tcmr"
data: "{\"constraints\":[\"Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.\",\"Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.\",\"Every repair item must name observable behavior and evidence before it can close.\"],\"evidence\":[],\"milestones\":[],\"plans\":[],\"risks\":[\"Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.\",\"Large rework can sprawl unless grouped under one mission with explicit blockers and validation.\"],\"validation\":[\"Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.\",\"Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.\",\"Mission closeout requires a contract audit mapping every mission and epic Outcome line to linked work and attached evidence.\",\"Mission closeout requires an independent adversarial validation pass by a validation agent that did not implement the slices being validated.\",\"Mission closeout requires positive and negative command transcripts for each major repaired surface, including old-command absence or replacement behavior.\",\"Mission closeout requires focused tests, stale/ignored-test inventory, docs/help/Agent Factory guidance parity checks, export/lint/doctor checks, and attached evidence records for each major repair area.\"],\"work\":[]}"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-40ou"
    role: "advances"
  - kind: "issue"
    id: "atelier-efpk"
    role: "advances"
  - kind: "issue"
    id: "atelier-nzy1"
    role: "advances"
  - kind: "issue"
    id: "atelier-wpyb"
    role: "advances"
  - kind: "issue"
    id: "atelier-ymfl"
    role: "advances"
  - kind: "issue"
    id: "atelier-zue4"
    role: "advances"
  - kind: "issue"
    id: "atelier-v9id"
    role: "validates"
  relates: []
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Repair CLI workflow rework and validation gaps"
updated_at: "2026-06-12T05:11:48.642892539+00:00"
---
