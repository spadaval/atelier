---
schema: atelier.review
schema_version: 2
id: atelier-v8i4
title: 'status.rs text(): policy diagnostics (Invalid working policy / Using last valid policy from <commit>) that prime_text put in the page now go only to stderr via main.rs, so status --hook claude additionalContext omits them; with a malformed working workflow.yaml the session silently runs on a fallback policy. Render base_policy::working diagnostics into the page and add a hook test.'
target: atelier-sdb0
identity: Independent reviewer (Claude subagent)
commit: cbb151fe4909ae92ee60a07ae384abe9dda975f2
created_at: 2026-10-17T02:41:19.914044160Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: bac5dcc787618df5219c29e488568efa6eed7e13
    recorded_at: 2026-10-17T02:46:54.564200966Z
    resolution_commit: eebfebe785f1a2c60d20cbb2bd4c010ef90da278
    reason: Policy diagnostics now render in a Policy Diagnostics section of the status page and hook context.
    migrated_from: atelier-6dci
---

status.rs text(): policy diagnostics (Invalid working policy / Using last valid policy from <commit>) that prime_text put in the page now go only to stderr via main.rs, so status --hook claude additionalContext omits them; with a malformed working workflow.yaml the session silently runs on a fallback policy. Render base_policy::working diagnostics into the page and add a hook test.
