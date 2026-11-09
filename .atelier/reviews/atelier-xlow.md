---
schema: atelier.review
schema_version: 2
id: atelier-xlow
title: review on atelier-yfqu @afdcfda0
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: afdcfda0c4ab99e6c2f020c0773a9a446803fe4b
created_at: 2026-11-09T01:08:37.249132983Z
event:
  event: review
---

Verdict: merge in scope and value, with planning approval withheld until the remaining contract failures are fixed. The human-authorized split removes the reason for the earlier shrink verdict; no further mission split is requested.
Sizes: complexity medium (one new read-only HTTP service and existing view/rendering integration); value high (the named project-orientation and one-click reading outcome); risk medium (incorrect navigation or stale state is visible and reversible, with a shared-host availability constraint).
Contract: fail. The revised inbox-target rule still cannot deliver the page the current human-read item actually asks for.
Fitness: fail. New whole-page caching omits mutable branch/review and time inputs.

Reviewed master afdcfda0c4ab99e6c2f020c0773a9a446803fe4b and the complete diff 14f5884a..afdcfda0, using a fresh detached worktree outside any repository. Read both strategies, all moved/new/changed item bodies, unchanged atelier-ahpf, decision atelier-7bs1, the ten earlier findings and review account, current product/architecture and view/hook guidance, classification and branch-snapshot code. Applied the requested Agent Factory review procedure with plan.md, references/atelier.md and sizing.md. Ran atelier status, atelier brief atelier-yfqu, atelier brief atelier-bnek, atelier check and git diff --check for the reviewed diff. Check passed with existing unrelated warnings about Done when sections without checkboxes; diff whitespace check passed.

Re-check of all ten findings:
- atelier-i8h1: addressed. Site/ADR/pages remain here; attribution, reviewer identity, nudge, T3 spike, live threads and metrics advance atelier-bnek. Integration blockers name the shared ADR and serve item. The dated plan note records Sumeet as authority for the split and criterion changes.
- atelier-79wo: addressed in atelier-7yy9. Attribution writes after command outcome; the store interface is write-only, guards cannot read it, and a test-hooks call counter checks the merge boundary in addition to unavailable-store behavior.
- atelier-gnh5: addressed in planning. Independent validation atelier-mtb3 owns the published inbox scenario after installation and repository pages; atelier-wo22 owns the cross-model scenario in atelier-bnek. The phone taste read is needs-human loose end atelier-pywh outside mission scope. Neither criterion depends on future human adoption.
- atelier-bihd: NOT fully addressed. Direct links and an explicit fixture are now required, but the new target selection treats any relates edge to any mission as a request to read that mission debrief. On current master the Needs a human item atelier-3km4 asks for atelier-vt8q in its title and points its feedback at atelier-r6bt in Done when, while its only mission relation is related atelier-gg6b, the superseded debrief-improvement mission. The mandated rule opens gg6b instead. A needs-human decision that advances a mission similarly needs its question/options, not that missions debrief. Multiple mission relations also have no target-selection rule. Locations: atelier-zpmb:33, fleet-dashboard.md:51-52, atelier-mtb3:36, and atelier-3km4:13-23,33. Keep the existing medium FIX NOW open. State an explicit reading-target contract distinct from generic mission membership/related links, make existing inbox records satisfy it before claiming the criterion, and test both a debrief read with an unrelated mission link and a decision linked to a mission. The validation instruction to add a related link only when one is absent does not fix the existing wrong link and can create multiple candidates. This is the remaining part of the original finding, not a second duplicate finding.
- atelier-b9wq: addressed as a planning fix in atelier-bnek. Item reads and explicit role associations are recorded; the attribution rule is now adjustable under Authority. Metrics remains backlog and must reconcile totals, attributed shares and unattributed shares on an implementer, fresh reviewer and reused orchestrator before building the page. This approves the evidence boundary, not the candidate window algorithm.
- atelier-5yz1: addressed. The pure HTML renderer is separated from the CLI file writer, and routes must work with the repository, Git and ignored runtime paths unwritable; status/ref assertions remain additional proof.
- atelier-roov: addressed. The ADR item now reconciles PRODUCT_INTENT, the current record-store architecture, ADR 0019 and other narrowed ADRs, with a concrete wider database/daemon/server claim audit. Historical strategies stay historical.
- atelier-ntq9: the original missing load/containment contract is addressed by the concurrency and response bound, peak-RSS/T3-latency measurement, cache and shipped resource-limited unit. Its new cache rule creates separate FIX NOW atelier-mhb4: master SHA alone cannot invalidate branch-review or time-dependent pages.
- atelier-8j0j: addressed. Unit/setup docs are reviewed before merge in the serve item; the loose end only installs, activates, measures restart and publishes, explicitly without a host reboot.
- atelier-726p: addressed. The reminder explicitly preserves the no-issue light path, stays nonblocking and has a frequency bound.

Shaping/readiness: the site now has one theme, can finish on this host through its own installation and validation work, and has a first docs item with observable checks. All scoped work fits the outcome; service installation and phone taste are correctly represented as loose ends. The two remaining site issues above must be settled before execution. Strategies remain draft and no human walkthrough/plan approval is inferred from permission to revise.

Open questions: no new product-direction decision requested. Choose an unambiguous reading-target representation and a cache invalidation contract in the fixes. In atelier-bnek the external T3 interface, usage ingestion, metric definitions and backfill remain evidence boundaries; review their shaped items before starting them.
Residual risk and tests not run: planning review only. No service, browser/phone, live T3 API, restart, load test or real token reconciliation was exercised. No source code changed; no Rust build/tests or smoke-repos run was needed for these review records. Nine earlier findings are addressed in this account, but their automatic resolution remains pending a later approval on atelier-yfqu after the outstanding fixes. Review-record publication requires a fresh successful atelier land.
