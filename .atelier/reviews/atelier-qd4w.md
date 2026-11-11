---
schema: atelier.review
schema_version: 2
id: atelier-qd4w
title: finding on atelier-u1ko @8b1b75e0
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: 8b1b75e02cfdd1a68e3a72c829ff9170f4fde8bc
created_at: 2026-11-11T05:04:39.892464398Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:68,72-79 Reset versus revert is decided from the journal's pushed flag, but no rule or trial covers a remote accepting a push before its success is recorded locally. The step-4 or step-5 push can update the remote and then report a transport failure; an interruption between push and journal update has the same state. Treating that entry as unpushed resets the local conversion while leaving converted records published, then reinstalls the old binary. The ten stops exercise acknowledged before/after states, not this uncertain outcome. Specify a small recovery rule before destructive rollback or retry: record the intended old/new refs and determine the actual remote tip; revert when the conversion reached it, reset only when publication is proved absent, and stop without resetting when that cannot be established. Add a local-bare trial whose push updates the ref but returns failure before the pushed marker is written, and verify the same restored-tree/local-remote invariant. This needs no general transaction framework.
