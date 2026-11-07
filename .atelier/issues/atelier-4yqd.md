---
created_at: "2026-11-07T05:00:24.620382782+00:00"
id: "atelier-4yqd"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-4yqd
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T05:28:48.922358587+00:00"
status: "done"
title: "Record the simpler lifecycle in ADR 0028, and add the no-regrowth principle"
updated_at: "2026-11-07T05:28:48.922358587+00:00"
---

## Goal

Sumeet decided the simpler lifecycle on 2026-11-06 (docs/strategies/simpler-lifecycle.md, Shape). Record those decisions in one ADR so later items and other repositories have one reference, mark what it supersedes in ADRs 0016, 0019, 0021, 0025, 0026 and 0027, and add Atelier's purpose and the principle against regrowth to PRODUCT_INTENT.md now, since they govern the rest of this mission.

## Done when

- [x] `docs/adr/0028-*.md` states each Shape decision of the strategy, with the transcript analysis summarised as context, and names exactly which decisions of ADRs 0016 (recorded base and type-named branches), 0019, 0021 (a parent closes after its children), 0025, 0026 and 0027, and which PRODUCT_INTENT.md principles (including "merge writes no tracker state" and "append-only and tied to full Git commit identities"), it supersedes. It opens with Atelier's purpose as the governing principle: agents produce high-quality, correct code that solves real problems, with as little red tape and overhead as possible, and nothing else about Atelier is sacred. Mechanics the strategy leaves to later items are named as open, not invented.
- [x] The Status line of each superseded ADR points to ADR 0028 and says which part it replaces.
- [x] PRODUCT_INTENT.md states that purpose first, and adds the no-regrowth principle: a new gate or record type names the kind of defect it would have caught in real agent sessions, and a rule that only patches another rule's edge case is a reason to remove the first rule. Other principles change only when the item that changes the behaviour lands.
- [x] The ADR's Status says it is accepted within the authority of docs/strategies/simpler-lifecycle.md, on Sumeet's direction, as ADRs 0025 and 0027 say; Sumeet reads it after merge.

## Scope

Docs only: the new ADR, Status lines of the superseded ADRs, the purpose and one principle in PRODUCT_INTENT.md.

## Context

docs/adr/0016-canonical-work-branches-and-mission-integration.md, 0019-verified-repo-native-tracker.md, 0021-parent-issues-replace-epics.md, 0025-findings-carry-their-disposition.md, 0026-evidence-records-only-what-nothing-else-records.md, 0027-proof-has-a-time.md; PRODUCT_INTENT.md. The analysis that motivated the mission is summarised in the strategy's Why.

## Close Reason

ADR 0028 (docs/adr/0028-simpler-lifecycle.md) records the simpler lifecycle Sumeet decided on 2026-11-06 and 2026-11-07. After its Status it opens with Atelier's purpose as the governing principle, then summarises the session analysis as context and states the ten Shape decisions of docs/strategies/simpler-lifecycle.md, plus what does not change. Each decision names the mission item that builds it, and every mechanic the strategy leaves open is listed under the item that settles it, not invented. A Supersession section names, decision by decision, what it replaces in ADRs 0016, 0019, 0021, 0025, 0026 and 0027 and in PRODUCT_INTENT.md (quoting each principle, including "Merge writes no tracker state" and "append-only and tied to full Git commit identities"), and what stands. Each of those ADRs' Status now points at ADR 0028 and names the part replaced. Nothing else in them changed. PRODUCT_INTENT.md opens with the purpose and adds the no-regrowth principle as its first product principle. Its other principles are unchanged, and the new opening says that those ADR 0028 supersedes describe today's behaviour until the item that changes each lands.

Verification: `git diff --check` is clean, and `atelier check` passes (installed binary). A script resolved every relative link and anchor in the changed files. I checked each claim about current behaviour against docs/product (work-model.md, reviews-and-merge.md) and the code where the ADR names a rule (`start_branch_prefix_mismatch`, `review_follow_up_source_mismatch`, merge strategies). Docs only, so no build or tests.

### Deviations
- docs/index.md: the ADRs bullet now names ADR 0028's supersession list beside ADR 0019's. Scope listed only the ADRs and PRODUCT_INTENT.md, but the orchestrator's brief allowed an index update. Without it, the docs map sends readers to ADR 0019's list alone.
- PRODUCT_INTENT.md's purpose paragraph also links ADR 0028 and says that the principles it supersedes describe today's behaviour until their items land. No principle's text changed.
- The ADR's Governing Principle section comes right after Status, not before it. ADRs 0025 and 0027 put Status first, so "opens with" is read as the first section after the metadata.

### Decisions
- Status format: 0016 keeps its one-line `Status:` and gains a bold "Partially superseded by ADR 0028" clause. The others gain a paragraph in their Status section, as ADR 0025 did for ADR 0019.
- ADR 0027 is marked superseded in full, because every part of its Decision goes. Its rejected options are noted as not revived.
- ADR 0028's Status says decisions 1 to 8 take effect when their items merge, decision 9 with this ADR, and decision 10 binds every item. Without that, a reader would see "superseded" while the binary and product docs still behave as the old ADRs say.
- The ADR uses "check" only for `atelier check` and merge checks, except in quotes and in "after-merge checks", the name of the concept being removed. It uses "play-test" where the strategy says "play-check".

### Surprises
- Merge marking an issue done has no merge commit to carry the done state under `fast_forward_only`, the default without a shell merge check. atelier-jnrh's Done when says "in the merge commit". The ADR lists this as open under atelier-jnrh, together with how "Atelier never creates commits" reads afterwards.
- ADR 0025 decision 4 requires a `defer` or `no_action` to be set by an identity other than the work's Git authors. That is the same comparison as the reviewer rule the strategy removes, but neither the strategy nor atelier-md0v says whether it goes too. The ADR lists it as open under atelier-md0v.

### Open items
- The orchestrator should add the two open questions above to the Unknowns of atelier-jnrh (merge strategy) and atelier-md0v (disposition-setter identity). I did not edit other items' records.
- atelier-jnrh amends ADR 0028 and PRODUCT_INTENT.md with the mechanics as built, as its Done when says.
- CONTEXT.md and docs/product still describe after-merge checks, the recorded base and the old staleness rule. Those are current behaviour, and the items that change each one update them.
- No independent review yet. Sumeet reads the ADR after merge.
