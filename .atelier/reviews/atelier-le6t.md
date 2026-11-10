---
schema: atelier.review
schema_version: 2
id: atelier-le6t
title: finding on atelier-zrss @6de2edc7
target: atelier-zrss
identity: gpt-6.1-sol
commit: 6de2edc7137fae91cf09c80402389a0e0e34e537
created_at: 2026-11-10T21:38:26.562267242Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-zrss.md:35,38,49: Round-1 finding atelier-0tx8 is only partly resolved. The expanded rule definition now covers planning guards, configured checks and CI, but Context still falsely says every command refusal already comes from acts.rs. plan.rs:304-311 refuses parseable record validation directly, and merge.rs:106-111 checks reported CI separately. For example, set on a parseable item missing its Goal heading reaches save validation, not an act-table condition or an unreadable record. The plan still does not explicitly distinguish that skippable validation rule from unparseable input, and its positive tests omit both parseable validation and reported CI failure, which the original finding requested. A worker following the stated integration premise can leave those paths unskippable while passing every listed positive test. Smallest fix: correct the Context premise, explicitly classify parseable validation as a rule, and add positive bypass scenarios for a parseable item missing a required heading and a successful forge query reporting failed CI; keep parse failures and adapter-operation failures unskippable. No new command, option, dependency or rules framework is requested. Recommendation: FIX NOW. Planning verdict: merge after this remaining contract correction. Sizes: complexity medium, value high, risk low for the planning records.
