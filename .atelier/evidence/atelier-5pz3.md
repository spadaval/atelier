---
created_at: "2026-11-05T02:19:43.787100625+00:00"
id: "atelier-5pz3"
evidence_type: "validation"
commit: "0a2ec1a757b32aca938657d1f2ee49afdc39a5ec"
result: "pass"
captured_at: "2026-11-05T02:19:43.786716831+00:00"
agent_identity: "Independent acceptance validator (Claude subagent)"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S1"
  result: "pass"
criteria_mission: "atelier-r6bt"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r6bt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S1 PASS on master 0a2ec1a7 (debug build of this commit). Fixture repo $HOME/.cache/atelier-r6bt-accept/fx1: 'issue note <id> --kind decision \"Cache in memory\" --reason ... --rejected Redis --rejected \"Disk cache\"' stores '[decision] <choice>' with indented Reason:/Rejected: lines; omitting --reason exits 1 ('A decision note needs its reason'); an empty choice exits 1; '--one-way' is now an unknown argument. Hand-edited notes: 'atelier check' refuses (exit 1, one finding each) a note with no Reason line, a misspelled 'Rejectd:', a lowercase 'reason:', an empty choice, and the tag '[decision one-way]'. 'work decisions <mission>' lists decided records first, then decision notes from the item and the mission (with because/rejected), then open, then superseded records. Limits: proof is CLI runs on a fixture, not the repo's integration tests (CI ran them); 'costly to reverse is a decision record' is guidance, checked only by the absence of the one-way flag/tag; candor is not checkable."
updated_at: "2026-11-05T02:19:43.788874286+00:00"
---

S1 PASS on master 0a2ec1a7 (debug build of this commit). Fixture repo $HOME/.cache/atelier-r6bt-accept/fx1: 'issue note <id> --kind decision "Cache in memory" --reason ... --rejected Redis --rejected "Disk cache"' stores '[decision] <choice>' with indented Reason:/Rejected: lines; omitting --reason exits 1 ('A decision note needs its reason'); an empty choice exits 1; '--one-way' is now an unknown argument. Hand-edited notes: 'atelier check' refuses (exit 1, one finding each) a note with no Reason line, a misspelled 'Rejectd:', a lowercase 'reason:', an empty choice, and the tag '[decision one-way]'. 'work decisions <mission>' lists decided records first, then decision notes from the item and the mission (with because/rejected), then open, then superseded records. Limits: proof is CLI runs on a fixture, not the repo's integration tests (CI ran them); 'costly to reverse is a decision record' is guidance, checked only by the absence of the one-way flag/tag; candor is not checkable.
