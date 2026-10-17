---
created_at: "2026-10-17T20:09:29.359762791+00:00"
id: "atelier-syrp"
evidence_type: "validation"
commit: "0edf25bb740431c17e96c6248e56d22c693eb968"
result: "pass"
captured_at: "2026-10-17T20:09:29.359674125+00:00"
agent_identity: "Independent validator (Claude subagent)"
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
title: "S1 PASS on master 0edf25bb (installed atelier 0.2.0+458d4874; later commit adds only an evidence record). Scratch repo ~/.cache/atelier-tmp/val-r6bt: 'issue note --kind decision' without --reason, with blank reason, or with empty choice is refused; --reason/--rejected/--one-way without --kind decision refused; reversible note with two --rejected and a --one-way note stored as [decision] / [decision one-way] with Reason/Rejected lines. Hand-edited notes: missing reason, empty reason, empty choice and tag [decision maybe] each make 'atelier check' fail with a named finding. 'atelier work decisions <mission>' lists one-way notes first across the mission scope. Suites: cargo nextest run 802/802 and --features test-hooks 806/806, 0 skipped (incl. decisions::check_refuses_malformed_decision_notes, decision_notes_record_choice_reason_rejected_and_one_way, decision_notes_refuse_missing_reasons_and_stray_flags). Observation: an unknown continuation line such as a misspelled 'Rejectd: x' passes check and is folded into the reason text."
updated_at: "2026-10-17T20:09:29.360814923+00:00"
---

S1 PASS on master 0edf25bb (installed atelier 0.2.0+458d4874; later commit adds only an evidence record). Scratch repo ~/.cache/atelier-tmp/val-r6bt: 'issue note --kind decision' without --reason, with blank reason, or with empty choice is refused; --reason/--rejected/--one-way without --kind decision refused; reversible note with two --rejected and a --one-way note stored as [decision] / [decision one-way] with Reason/Rejected lines. Hand-edited notes: missing reason, empty reason, empty choice and tag [decision maybe] each make 'atelier check' fail with a named finding. 'atelier work decisions <mission>' lists one-way notes first across the mission scope. Suites: cargo nextest run 802/802 and --features test-hooks 806/806, 0 skipped (incl. decisions::check_refuses_malformed_decision_notes, decision_notes_record_choice_reason_rejected_and_one_way, decision_notes_refuse_missing_reasons_and_stray_flags). Observation: an unknown continuation line such as a misspelled 'Rejectd: x' passes check and is folded into the reason text.
