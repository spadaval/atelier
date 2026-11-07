---
schema: atelier.review
schema_version: 2
id: atelier-jrya
title: approval on atelier-ilba @73613b72
target: atelier-ilba
identity: independent reviewer subagent (claude-opus-5-5), atelier-ilba
commit: 73613b7224fe1b07d00e1d7ee336d278298c8795
created_at: 2026-11-07T07:42:20.491618476Z
event:
  event: approval
---

Re-review of the atelier-oyh0 fix (git diff 2d736795 b9f2214c) plus the record-only commits 95ae0eaa and 73613b72. append_only now refuses any change to an approval or exemption (base or trivial) from its first commit, wherever it is. original_independent again requires an approval unchanged since it was introduced. land's uncommitted-records advice restores an edited approval. The docs (ADR 0028, PRODUCT_INTENT, CONTEXT, reviews-and-merge, sessions-and-hooks, architecture) state the rule as built. I reproduced the original re-pin with target/debug/atelier at 73613b72: land and merge --check now refuse it with review_append_only, and the new merge_gates test covers both the uncommitted and the committed edit. nextest at head: 996 passed, and 1002 passed with test-hooks. fmt, diff --check and metadata --locked pass. CI on 73613b72 was still pending. No fix_now remains; atelier-rrcf (no_action) notes that rewriting a finding into an approval is still possible, which is no wider than a self-declared review approve.
