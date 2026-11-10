---
schema: atelier.review
schema_version: 2
id: atelier-x588
title: approval on atelier-jah8 @92d9893f
target: atelier-jah8
identity: gpt-6-astra-jah8-close
commit: 92d9893f6a180600e40d97e9774314ee6a845146
created_at: 2026-11-10T17:43:35.607824150Z
event:
  event: approval
---

Verdict: merge. Sizes: complexity medium (snapshot, readers and classification handoffs), value high (reproducible baseline needed to judge the redesign), risk high (misleading measurements could silently steer later decisions). Contract: pass. Fitness: pass. This is a planning re-review of the four named findings, based on the current strategy and all six items read with issue show --full, especially Goal, Done when, Scope and Context; completion reports and earlier approvals were not treated as proof. atelier-xkwm is fixed: strategy Shape and Design plus scx2 require a retained private snapshot, one cutoff, repository SHAs and per-file hashes; 5fsj requires snapshot arguments, manifest-pinned Git, byte-identical fresh-agent reproduction and eligible rework cohorts. The strategy excludes lines younger than two weeks, and README reruns the snapshot without publishing raw messages. atelier-kxxx is fixed: scx2 requires one SQLite online backup and forbids other live queries; strategy and 5fsj require every measurement reader to use the copy, with tests against live-store defaults. The manifest cutoff replaces a mutable end date, and README has no live measurement default. atelier-9jcj is fixed: 3kr7 names samples/<measure>.json, .labels.json and .second.json, assigns the implementer to start a child from another lab with only rubric.md and the unlabelled sample, requires the first 30 entries and agreement reporting. atelier-fagb is fixed: strategy binds every item including the spike to manifest commits; z5bp explicitly bounds both its sample and history reads; c71h now requires classified.md final per-repository review-yield rates and depends on 3kr7, with no provisional-rate permission. 6sti assembles the resulting measures without new measurement. Corroboration read only: committed manifest, README, classified.md, defect-method.md, defects.md and final report; defects.md uses 8/20, 6/20, 8/20, 6/20 and 18/20 from classified.md, and the report carries those results. Findings: none within this re-review. Open questions: none for the four fixes. Residual risk: the snapshot is on one disk and classification has acknowledged uncertainty. Private stores were not opened; snapshot integrity, runtime server load and numerical reproducibility were not independently rerun. Approval judges the plan, not a new implementation certification.
