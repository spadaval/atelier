---
schema: atelier.review
schema_version: 2
id: atelier-xzaf
title: finding on atelier-qr5s @f2c0831b
target: atelier-qr5s
identity: gpt-6-astra-plan-r4
commit: f2c0831b8521c80df3d0b4f5eed070026e64a389
created_at: 2026-11-10T01:42:32.496148499Z
event:
  event: finding
  recommendation: defer
---

low atelier-mlwx: the claim that six commits past 0.4.0 require another fallback release is not supported by their code diff. git diff v0.4.0 8842f232 changes only review_request.rs wording in the binary; record reading, check, setup and serve are unchanged. A new release adds packaging and publication work that rollback has not been shown to need. At this step planning boundary, validate the existing 0.4.0 artifact on the old records and dashboard and reuse it if it passes, or name the specific capability requiring 0.4.1.
