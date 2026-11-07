---
created_at: "2026-11-07T15:34:14.270890079+00:00"
id: "atelier-pznw"
evidence_type: "validation"
commit: "72d05b0303f8a1f547c9e994ad6438ee44902f0c"
result: "fail"
captured_at: "2026-11-07T15:34:14.270846848+00:00"
agent_identity: "independent validator (claude-opus-5-5), atelier-2bho"
target:
  kind: "issue"
  id: "atelier-mmpv"
  role: "validates"
criteria:
- slug: "legible"
  result: "fail"
criteria_mission: "atelier-mmpv"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mmpv"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "legible partly holds. A fresh subagent read only the agent-factory skill (spadaval/.agents main 9f60b8e: SKILL.md and every file it links), astromech origin/main e6ae38df AGENTS.md and .atelier/workflow.yaml, and the installed CLI's help (0.3.1+135c1f9e). It stated all three answers correctly against the strategy's Shape. Lifecycle: start, implement, tick every Done when and write the Report, review request (refuses until both hold), fix_now fixes re-reviewed, approve, merge marks the work done, no close step. Stale: only code, meaning anything outside .atelier/issues, reviews and evidence, not counting a clean base merge, with policy and config counting as code. Loose end: code-free work that can only follow a merge or release, unless it is an independent validation; it is blocked_by the item, never a child, and done on the base with a one-line result; a bad result files a bug. It did find contradictions, and I confirmed each one. (1) Where to file an in-flight item's loose end: references/atelier.md:144-147 says \"on the work branch while the item is in flight\", but astromech AGENTS.md:36-39 says planning changes, filing included, \"are committed straight to `main`\". (2) close: atelier.md:27-31 says \"Nothing else closes\", but the policy requires `close_reason` on done, decide and supersede (workflow.yaml:67,97,110,118,141), and `issue transition --help` documents `--reason <CLOSE_REASON>` for them. (3) done: atelier.md:24-26 makes done a status with superseded as \"the other terminal status\", but workflow.yaml:32-35,41-43 puts superseded in category done and in done_statuses, and `doctor --help` says \"Judge every done item\". (4) tracker-records.md:216-217, decide.md:38 and repository-shape.md:67 say the decision record is the ADR, but `issue update --help` has `--adr` to \"Set the repository-relative ADR a decision record produced\". Stale: tracker-records.md:237-238 warns against `issue note --one-way`, a flag the CLI lacks. Ambiguity, not counted: atelier.md:65-67 says a rewritten Done when line \"gets a fresh approval\", next to :88-92 \"Record edits never make an approval stale\" (the same pairing is at `atelier --help` 44-46 and AGENTS.md:21-22). No source mentions After merge lines, a work close step or a validation branch. Limits: one cold reader; (2)-(4) are terminology, and only (1) bears on the three questions."
updated_at: "2026-11-07T15:34:14.388725129+00:00"
---

legible partly holds. A fresh subagent read only the agent-factory skill (spadaval/.agents main 9f60b8e: SKILL.md and every file it links), astromech origin/main e6ae38df AGENTS.md and .atelier/workflow.yaml, and the installed CLI's help (0.3.1+135c1f9e). It stated all three answers correctly against the strategy's Shape. Lifecycle: start, implement, tick every Done when and write the Report, review request (refuses until both hold), fix_now fixes re-reviewed, approve, merge marks the work done, no close step. Stale: only code, meaning anything outside .atelier/issues, reviews and evidence, not counting a clean base merge, with policy and config counting as code. Loose end: code-free work that can only follow a merge or release, unless it is an independent validation; it is blocked_by the item, never a child, and done on the base with a one-line result; a bad result files a bug. It did find contradictions, and I confirmed each one. (1) Where to file an in-flight item's loose end: references/atelier.md:144-147 says "on the work branch while the item is in flight", but astromech AGENTS.md:36-39 says planning changes, filing included, "are committed straight to `main`". (2) close: atelier.md:27-31 says "Nothing else closes", but the policy requires `close_reason` on done, decide and supersede (workflow.yaml:67,97,110,118,141), and `issue transition --help` documents `--reason <CLOSE_REASON>` for them. (3) done: atelier.md:24-26 makes done a status with superseded as "the other terminal status", but workflow.yaml:32-35,41-43 puts superseded in category done and in done_statuses, and `doctor --help` says "Judge every done item". (4) tracker-records.md:216-217, decide.md:38 and repository-shape.md:67 say the decision record is the ADR, but `issue update --help` has `--adr` to "Set the repository-relative ADR a decision record produced". Stale: tracker-records.md:237-238 warns against `issue note --one-way`, a flag the CLI lacks. Ambiguity, not counted: atelier.md:65-67 says a rewritten Done when line "gets a fresh approval", next to :88-92 "Record edits never make an approval stale" (the same pairing is at `atelier --help` 44-46 and AGENTS.md:21-22). No source mentions After merge lines, a work close step or a validation branch. Limits: one cold reader; (2)-(4) are terminology, and only (1) bears on the three questions.
