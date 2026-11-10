---
schema: atelier.review
schema_version: 2
id: atelier-dnlm
title: finding on atelier-pz1y @19254e96
target: atelier-pz1y
identity: sol-pz1y-r2
commit: 19254e96331516cfca73c4aac5751c7b2a0f88da
created_at: 2026-11-10T22:36:59.409132694Z
event:
  event: finding
  recommendation: fix_now
---

Low: next/crates/atelier-cli/src/output.rs:710: R4. A pre-push refusal caused only by unread records gives no way out. Reproduced with a local bare remote whose base already has valid config, then a plain planning commit adding an item.md with no front matter (as can happen before hooks are installed or with pre-commit skipped). The real pre-push hook exits 1 and prints its refusal, the cannot-be-skipped line and the commit:path parse error, but no recovery instruction: push_refused adds instructions only for code or done findings. The Done when contract requires every refusal to name the way out. Explain that the offending unpublished commit must be repaired or removed, not merely followed by a fixing commit, since every new first-parent commit is rechecked; add a real-Git regression for this unread-only refusal.
