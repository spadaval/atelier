---
schema: atelier.review
schema_version: 2
id: atelier-spix
title: approval on atelier-secy @6e13df06
target: atelier-secy
identity: lighter-workflow reviewer for atelier-secy (claude-opus-5-5)
commit: 6e13df06a8b569040428bea5aca026c27bdb76ed
created_at: 2026-11-07T03:48:48.599063414Z
event:
  event: approval
---

Read e11d1846..6e13df06 and every caller of completion_claim_matches (claim_retained_at, claim_endpoint, fields_required_when_claimed) and claim_endpoint (proof_matches, held_at_close, original_approval_valid); argument order is now oldest-first everywhere. Rule reuses after_merge::only_validated, epic/other bodies fall back to strict match, status must match. repro.sh 1 with branch binary: land after tick passed (installed binary refuses findings.disposed). Probed base edits on a fixed-finding item: ticking a before-merge line, adding an after-merge line, changing reason while ticking, changing stand-in while ticking all still refuse findings.disposed; self-carry lands (R x4d5). nextest -p atelier-app -p atelier-cli 883 passed; fmt, diff --check, metadata --locked clean; CI on PR 34 pending at review time. Evidence-gate gap recorded as defer (xqjc).
