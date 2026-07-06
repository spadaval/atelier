---
created_at: "2026-07-06T18:22:48.951404330+00:00"
id: "atelier-e4lm"
evidence_type: "audit"
captured_at: "2026-07-06T18:22:48.951390785+00:00"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent PR 47 re-review at 3213218a: pass/approved. Forgejo instance OpenAPI publishes GET /repos/{owner}/{repo}/pulls/{index}/reviews and GET /repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments; PullReviewComment exposes position, resolver, path, and body, matching the implementation. Live target/debug/atelier review show --issue atelier-ye11 --comments succeeds and renders comments. Live target/debug/atelier review submit --approve returned provider review 35 with state APPROVED, proving exact official serialization. Focused provider/application/native-room/CLI suite passed 39/39. ADR 0011 now names review open --existing and removal of review link. cargo fmt, atelier check, stale-guidance search, and worktree diff checks passed; reviewer-owned atelier-9xvc trailing EOF blank was removed. Residual risk: review enumeration is intentionally bounded to the first 50 provider reviews; separate epic scenario validation remains required."
updated_at: "2026-07-06T18:22:55.345104270+00:00"
---

Independent PR 47 re-review at 3213218a: pass/approved. Forgejo instance OpenAPI publishes GET /repos/{owner}/{repo}/pulls/{index}/reviews and GET /repos/{owner}/{repo}/pulls/{index}/reviews/{id}/comments; PullReviewComment exposes position, resolver, path, and body, matching the implementation. Live target/debug/atelier review show --issue atelier-ye11 --comments succeeds and renders comments. Live target/debug/atelier review submit --approve returned provider review 35 with state APPROVED, proving exact official serialization. Focused provider/application/native-room/CLI suite passed 39/39. ADR 0011 now names review open --existing and removal of review link. cargo fmt, atelier check, stale-guidance search, and worktree diff checks passed; reviewer-owned atelier-9xvc trailing EOF blank was removed. Residual risk: review enumeration is intentionally bounded to the first 50 provider reviews; separate epic scenario validation remains required.
