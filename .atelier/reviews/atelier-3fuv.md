---
schema: atelier.review
schema_version: 2
id: atelier-3fuv
title: finding on atelier-z4cr @8627b943
target: atelier-z4cr
identity: GPT-6.1 Sol (xhigh), independent Desk review round 3
commit: 8627b9437cf9519dd2e6d92343fcc5ae2506964a
created_at: 2026-11-11T02:42:33.417524301Z
event:
  event: finding
  recommendation: fix_now
---

R1 (low) .atelier/issues/atelier-z4cr.md:23 and docs/strategies/desk.md:54-57: The mission Outcome still requires every act to be a commit made by an atelier command, and Sizing still justifies the risk with execution in a scratch worktree. Both contradict the corrected Design at desk.md:110-138 and atelier-h3hl.md:27-32. In next/crates/atelier-cli/src/plan.rs:218-289,658-764 and output.rs:705-735, ready writes without committing and asks its caller to commit; questions.rs:91-149 leaves decide uncommitted for the implementer's submit. A conforming implementation therefore fails the mission's stated Outcome, and acts actually touch existing base/item checkouts rather than the isolation claimed in Sizing. Recommendation: FIX NOW, a small correction of contradictory requirements: align the mission Outcome and risk explanation with the settled Design (the Desk commits ready's printed paths; submit later commits decide; existing checkouts). This is the remaining summary-consistency part of atelier-xaqd, not a request for another strategy or for fully designing a later increment.

Verdict: merge after this correction; the mission serves the human's missing planning and results interface, but approval is withheld.
Sizes: complexity high (a server, app, core views and a write contract); value high (the fixed product direction needs this interface); risk medium (visible, reversible repository writes and reads across five repositories).
Contract: fail only for R1's stale Outcome and risk description. Fitness: otherwise pass at plan.md's evidence-boundary standard.

Review round 3 independently checked atelier-z4cr, all six mission items, atelier-qxmr, the amended cutover strategy and oci2/qr5s/wd1i/u1ko/h3hl/aab1 records, plus ADR 0032 and product/desk.md at branch head cffbca9c. Also inspected the cutover strategy amendments and qr5s readiness that reached master at 0a92afec and 8627b943 during this review. The real need, chosen crate/server/app dependencies, read/write boundaries, and scoped first increment meet plan.md. atelier-1ie1 is ready: a docs-first field/view mapping and converted-clone proof, with missing views explicitly sized before code. Later work remains backlog at evidence boundaries; API details, missing views and the real T3 launch belong there. No costly-to-reverse choice remains assigned to an implementer. The Desk closes on converted clones; only the external loose end qxmr waits for wd1i, and the cutover has no Desk dependency. Checked source contracts in next/plan.rs (off_base, ready, save_with), output.rs (commit_line, planned), questions.rs (decide), app/view.rs (all three public functions and Located::record), identity.rs, CommitReader, the converter, and existing planning/decisions tests by reading. Searched the reviewed plan/docs: the new tracker has no site, serve, brief page or debrief page; mentions of today's site and the fallback are historical or explicit transition requirements.

Earlier findings: xaqd is fixed in the executor design, h3hl and the branch's durable documents, but not fully fixed because the mission Outcome still contradicts it (R1). tq1m, ja82, rfpw, 7kw4 and osjk are addressed. kmnc is addressed at the plan boundary by explicit pending-answer presentation and h3hl proof; the detailed before/after-submit cases remain that increment's responsibility. 5c32 and xg5q are addressed by aab1's Desk-specific title/Goal and link investigation, with the read probe distinguished from the unexercised launch.

Open questions and residual risk: no implementation or converted-clone/browser/service validation was run in this planning review. T3 launch and phone links remain the recorded later proof boundaries; the aab1 probe was read, not rerun. No code or plan edits were made.
