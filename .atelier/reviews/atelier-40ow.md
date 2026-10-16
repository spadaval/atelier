---
schema: atelier.review
schema_version: 1
id: atelier-40ow
title: 'Reviewed ffabdab9 (allowed_edge exemption, test, docs). The scope ids come from the HEAD index for every edge. So an issue that is new but in scope at HEAD is never exempt. That covers children via the owner''s children list, mission advances relations declared in either record (get_typed_relations covers both directions), and issues that join scope later. Later edits to an exempted issue are regular-to-regular and fail. Deletes and renames fail. Only the new record file itself is exempt; code or other paths in the same commit still make approval stale. Index validation rejects an id/path mismatch and a second parent. Checked by hand in a scratch repo: a new out-of-scope issue keeps approval. A new issue whose children list adopts the owner leaves the owner without its branch ownership, so merge is refused. Executable, symlinked, subdirectory and non-.md files are all refused. The new test checks both directions of the claim. The docs are accurate.'
target: atelier-cafy
identity: Independent reviewer (Claude subagent)
commit: b1537aef4483d7a6867ef4924b3b16f603f550e5
created_at: 2026-10-16T18:27:43.697959529Z
event:
  event: approval
---

Reviewed ffabdab9 (allowed_edge exemption, test, docs). The scope ids come from the HEAD index for every edge. So an issue that is new but in scope at HEAD is never exempt. That covers children via the owner's children list, mission advances relations declared in either record (get_typed_relations covers both directions), and issues that join scope later. Later edits to an exempted issue are regular-to-regular and fail. Deletes and renames fail. Only the new record file itself is exempt; code or other paths in the same commit still make approval stale. Index validation rejects an id/path mismatch and a second parent. Checked by hand in a scratch repo: a new out-of-scope issue keeps approval. A new issue whose children list adopts the owner leaves the owner without its branch ownership, so merge is refused. Executable, symlinked, subdirectory and non-.md files are all refused. The new test checks both directions of the claim. The docs are accurate.
