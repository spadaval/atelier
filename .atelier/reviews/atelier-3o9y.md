---
schema: atelier.review
schema_version: 2
id: atelier-3o9y
title: finding on atelier-mlwx @30c445e1
target: atelier-mlwx
identity: GPT-6.1 Sol independent planning reviewer
commit: 30c445e14e674de9cfacb4ba9ac4b6559dd8b599
created_at: 2026-11-11T03:01:17.213350230Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-mlwx.md:33-36: medium: Matching failures can qualify an unusable fallback. The CLI proof requires matching exit statuses and rejects only a difference that is a refusal or error; the site proof likewise requires only matching HTTP statuses and bodies. Thus identical nonzero check results or identical 500 responses satisfy these lines and allow line 36 to select 0.4.0, although the strategy requires a release that checks this repository and serves its site (redesign-3-cutover.md:88; redesign-architecture.md:1061-1066). The site really returns error responses on its normal read path (crates/atelier-site/src/lib.rs:981-988), so response parity alone is not evidence of successful service. Require the candidate check to exit 0 on the fixed scratch input and the required page/API routes to return successful expected content, in addition to parity. Equal failures of other diagnostic commands can be reported as baseline limitations; a broken scratch input must be corrected or escalated rather than treated as a passed fallback test. Planning review of 30c445e1 only; no release downloaded or executed.
