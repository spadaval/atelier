---
created_at: "2026-11-06T23:48:31.143840217+00:00"
id: "atelier-cqv2"
evidence_type: "measurement"
commit: "f2c7fcb614bed1a7f6d1ca9a57bac055dfa8a01f"
result: "pass"
captured_at: "2026-11-06T23:48:31.143501857+00:00"
target:
  kind: "issue"
  id: "atelier-5fus"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-5fus"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Baseline for lifecycle-cost, measured at f2c7fcb6 with product code equal to 05f01d83 (the master commit the branch left from; git diff 05f01d83 HEAD touches only crates/atelier-cli/tests and the start record). Record-only commits on the work branch: 13 (start 1, receipt 1, close 1, findings 3, author dispositions 3, follow-up issue 1, reviewer acceptances 2, approval 1). Review files per finding: fix_now 2 (finding + author disposition), defer 3 and no_action 3 (finding + author disposition + reviewer acceptance). Obtained by crates/atelier-cli/tests/lifecycle_cost.rs (one_standard_lifecycle_costs_the_recorded_bookkeeping), which drives the strategy's scenario in an atelier init fixture, counts non-merge commits in fork..branch tip whose files are all under .atelier/, and counts files under .atelier/reviews containing each finding's ID; cargo nextest run -p atelier-cli -j 4 --test lifecycle_cost passed with these as its expected values, and fails (printing the record-only commit subjects) when either expected value is changed."
updated_at: "2026-11-06T23:48:31.144693347+00:00"
---

Baseline for lifecycle-cost, measured at f2c7fcb6 with product code equal to 05f01d83 (the master commit the branch left from; git diff 05f01d83 HEAD touches only crates/atelier-cli/tests and the start record). Record-only commits on the work branch: 13 (start 1, receipt 1, close 1, findings 3, author dispositions 3, follow-up issue 1, reviewer acceptances 2, approval 1). Review files per finding: fix_now 2 (finding + author disposition), defer 3 and no_action 3 (finding + author disposition + reviewer acceptance). Obtained by crates/atelier-cli/tests/lifecycle_cost.rs (one_standard_lifecycle_costs_the_recorded_bookkeeping), which drives the strategy's scenario in an atelier init fixture, counts non-merge commits in fork..branch tip whose files are all under .atelier/, and counts files under .atelier/reviews containing each finding's ID; cargo nextest run -p atelier-cli -j 4 --test lifecycle_cost passed with these as its expected values, and fails (printing the record-only commit subjects) when either expected value is changed.
