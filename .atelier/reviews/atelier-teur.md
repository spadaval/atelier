---
schema: atelier.review
schema_version: 1
id: atelier-teur
title: finding on atelier-w5hx @77a9a8e9
target: atelier-w5hx
identity: independent planning reviewer subagent B (claude-opus-5-5)
commit: 77a9a8e97e2aaae53d62ef065706ff2056fe84f6
created_at: 2026-10-30T06:56:13.907311356Z
event:
  event: finding
  recommendation: fix_now
---

R4: (a) There are four one-way notes, not six: fip8 has two (Notes lines 57 and 61), ho7o one, sfsq one (`atelier work decisions atelier-r6bt` lists four). The six grep hits include fip8's Design-section example (line 48) and the tag quoted inside note 57's choice text; rewriting those would falsify what was decided. Say four notes, change the tag only. (b) Done when is silent on the brief decision view that status and issue show print (human_output.rs decision_brief_lines: 'Decisions: N current, K one-way:' plus rows; r6bt S6 requires it). With one-way gone the worker must invent what replaces those rows (current decision records? count only?). State it. (c) The debrief HTML's one-way sidebar index (atelier-8bil decision) and the .oneway CSS class in crates/atelier-cli/src/review_page.rs:240 are outside Scope and not matched by the grep; add review_page.rs and main.rs (help text) to Scope and 'oneway' to the grep. (d) Open atelier-y8xw ('Cap the one-way decisions in the brief decision view') becomes obsolete; supersede it or fold it in. (e) r6bt's proven S4 and S6 name one-way decisions; say how their wording changes when the strategy is revised.
