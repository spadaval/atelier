---
schema: atelier.review
schema_version: 1
id: atelier-3jh7
title: 'crates/atelier-cli/src/commands/mission_review.rs:646-656: the record-mention label is the first six words of the record''s summary, inserted as escaped text and never passed back through mention linking, so a summary that opens with an ID puts a bare ID in the visible page text. Scenario: a close reason says ''Review atelier-xxxx settled it'' where atelier-xxxx''s summary is ''Pre-existing; filed atelier-dpvz.'' or ''Deferred to atelier-imbo as recommended.'' (both real summary shapes in .atelier/reviews); the page reads ''“Pre-existing; filed atelier-dpvz.”'', against Done when''s ''no bare IDs in the HTML text''. The atelier-r6bt page has no such case today. Fix: when building the label, replace ID tokens with the issue''s title (view.title_of) or drop them, and add a fixture whose mentioned finding''s summary names an issue.'
target: atelier-nexd
identity: Independent reviewer 3 (Claude subagent)
commit: 1f8465d99fbeaa96241d5b0f1031bf6259a43aa9
created_at: 2026-10-17T22:12:50.556173001Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/mission_review.rs:646-656: the record-mention label is the first six words of the record's summary, inserted as escaped text and never passed back through mention linking, so a summary that opens with an ID puts a bare ID in the visible page text. Scenario: a close reason says 'Review atelier-xxxx settled it' where atelier-xxxx's summary is 'Pre-existing; filed atelier-dpvz.' or 'Deferred to atelier-imbo as recommended.' (both real summary shapes in .atelier/reviews); the page reads '“Pre-existing; filed atelier-dpvz.”', against Done when's 'no bare IDs in the HTML text'. The atelier-r6bt page has no such case today. Fix: when building the label, replace ID tokens with the issue's title (view.title_of) or drop them, and add a fixture whose mentioned finding's summary names an issue.
