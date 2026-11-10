---
schema: atelier.review
schema_version: 2
id: atelier-ajst
title: finding on atelier-engh @b35525f1
target: atelier-engh
identity: gpt-6.1-sol
commit: b35525f1c74d993546622f48551e87d27e42561c
created_at: 2026-11-10T16:17:26.069627946Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/output.rs:216 Refusal::Unmet discards the actual failing conditions and prints entry.next, which can be a different act. Reproduced with an approved item held by t1, an unticked Done when line and an empty Report: submit is allowed from approved by the table, but exits 1 with only atelier submit refused; nothing changed and Then: atelier merge. Neither unmet condition is printed, so the implementer receives no explanation and is directed toward merge. This also hides NoBranch when start tries to take over an in_review item whose branch is absent, because next(entry) names review instead. Render the conditions carried by Refusal::Unmet for the attempted act and give its appropriate retry command, using the shared condition renderer; keep ordinary building-status parity with status. Add CLI coverage where the requested act differs from entry.next. The current parity unit test checks only the act selected by next and therefore misses this path.
