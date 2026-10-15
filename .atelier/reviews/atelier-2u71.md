---
schema: atelier.review
schema_version: 1
id: atelier-2u71
title: 'references/atelier.md:28-36: ''Author bodies with typed --body'' and the draft->ready flow (''fill in the proof route, then run the ready transition'') do not say how to change a body after creation. ''atelier issue update --body'' works only for missions; for an epic it fails with ''issue body edits use the canonical Markdown record''. Agents will hit that error before learning to edit .atelier/issues/<id>.md directly and rerun ''atelier check''. The error recovers this, so it is not blocking. Optional fix: add one line saying that non-mission bodies are edited in the record file.'
target: atelier-kr30
identity: Independent reviewer (Claude subagent)
commit: 9b61daecaa3d48e3f1657d488d81fb32a1c0fc3a
created_at: 2026-10-15T19:58:25.606094427Z
event:
  event: finding
  recommendation: no_action
---

references/atelier.md:28-36: 'Author bodies with typed --body' and the draft->ready flow ('fill in the proof route, then run the ready transition') do not say how to change a body after creation. 'atelier issue update --body' works only for missions; for an epic it fails with 'issue body edits use the canonical Markdown record'. Agents will hit that error before learning to edit .atelier/issues/<id>.md directly and rerun 'atelier check'. The error recovers this, so it is not blocking. Optional fix: add one line saying that non-mission bodies are edited in the record file.
