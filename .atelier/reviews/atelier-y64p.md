---
schema: atelier.review
schema_version: 2
id: atelier-y64p
title: finding on atelier-u1ko @eba81732
target: atelier-u1ko
identity: GPT-6 Astra second-opinion reviewer
commit: eba817327d8e44e9ada8c7185545f20546dcda5e
created_at: 2026-11-11T08:41:06.343985459Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/cutover/cutover.sh:76 R2: Preflight fails open when the old tracker cannot report active work. Each worktree query discards stderr and uses || true, and line 137 masks failure of the complete old_in_progress pipeline with another || true. Thus an unreadable or invalid tracker record, or a failing status command, can produce no IDs and satisfy the no-active-work check. This is especially unsafe for a feature worktree: conversion verifies the base, not that worktree, and can proceed to replace the host binary and shared hooks without establishing it is quiet. The contract requires no in-progress item as reported by the installed tool, not merely no output from a possibly failed command. Capture and check each tracker exit status separately, fail preflight with the repository/worktree and diagnostic on any error, and distinguish grep finding no IDs from an upstream failure. Add a trial whose old tracker fails in a feature worktree and another whose status fails; both must refuse before conversion.
