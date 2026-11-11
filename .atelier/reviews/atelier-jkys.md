---
schema: atelier.review
schema_version: 2
id: atelier-jkys
title: finding on atelier-u1ko @eba81732
target: atelier-u1ko
identity: GPT-6 Astra second-opinion reviewer
commit: eba817327d8e44e9ada8c7185545f20546dcda5e
created_at: 2026-11-11T08:40:52.686515493Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/cutover/rollback.sh:90 R1: Rollback can report success after a failed recovery command. undo_conversion is called in if t=$(undo_conversion ...) at line 174, and restore_clone in an OR-list at line 181, so Bash disables errexit throughout both functions despite set -e. A rejected or disconnected revert push at line 90 is followed by the pushed-the-reverts message, deletion of recovery clones and a successful return with an unpublished SHA. Fetch, merge, reset and checkout failures in restore_clone are likewise unchecked and can end with its success message; the fallback can then be installed while remotes or checkouts still have converted records. Confirmed Bash conditional-function behavior with a harmless false-then-echo example; code inspection shows the same call structure here. Explicitly propagate each failure, reconcile ambiguous rollback pushes from the remote, retain recovery state on failure, and validate restored refs and checkouts before success. Add trials for a rejected rollback push and a failed checkout or ref update; current trials inject cutover failures only.
