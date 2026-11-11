---
schema: atelier.review
schema_version: 2
id: atelier-wzek
title: finding on atelier-u1ko @e64f35c5
target: atelier-u1ko
identity: GPT-6 Astra second-opinion reviewer
commit: e64f35c5821086fb1dd9b10f277f54c7edefd6cb
created_at: 2026-11-11T10:18:53.032842477Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/rollback.sh:165 R1: Rollback does not restore everything the cutover moves aside. convert_atelier moves whole ignored paths under CONVERTED_PATHS into STATE/aside/atelier, but restore_clone enumerates only find -type f and skips every existing destination. An ignored symlink or empty directory is left in the backup after a stop even though rollback reports success; a recreated file can silently prevent restoration of its original. This violates the promised preservation of pre-existing ignored files. Restore the saved paths with their types and metadata, detect conflicting destinations instead of claiming success, and add a trial with an ignored symlink and directory plus content checks (the current status snapshot cannot prove their contents).
