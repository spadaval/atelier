---
schema: atelier.review
schema_version: 1
id: atelier-lu12
title: '[low] procedures/orchestrate.md, same paragraph: ''Run them only when the merge conflicts, a generated file needs regenerating, or the integration step refuses because the base changed records about the item'' reads as exhaustive but omits a merge check failing on the merged tree for any other reason (a semantic conflict caught by tests). That case also needs the catch-up merge, a fix, fresh proof and review of the result. A generated file needing regeneration is in practice only observable as such a failure (for example cargo metadata --locked), so phrasing the trigger as ''the merge conflicts, a merge check fails on the merged tree (such as a stale lockfile), or the integration step refuses because the base changed records about the item'' covers both and avoids an ''only'' list the Guidance Form section warns against.'
target: atelier-zqee
identity: Independent reviewer (Claude subagent)
commit: dcb2107de131a531bb41298e5edd137d643dcbc5
created_at: 2026-10-17T07:19:15.129331469Z
event:
  event: finding
  recommendation: fix_now
---

[low] procedures/orchestrate.md, same paragraph: 'Run them only when the merge conflicts, a generated file needs regenerating, or the integration step refuses because the base changed records about the item' reads as exhaustive but omits a merge check failing on the merged tree for any other reason (a semantic conflict caught by tests). That case also needs the catch-up merge, a fix, fresh proof and review of the result. A generated file needing regeneration is in practice only observable as such a failure (for example cargo metadata --locked), so phrasing the trigger as 'the merge conflicts, a merge check fails on the merged tree (such as a stale lockfile), or the integration step refuses because the base changed records about the item' covers both and avoids an 'only' list the Guidance Form section warns against.
