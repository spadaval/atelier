---
schema: atelier.review
schema_version: 2
id: atelier-mutt
title: finding on atelier-4yqd @8d34b60c
target: atelier-4yqd
identity: independent reviewer subagent (claude-opus-5-5), atelier-4yqd
commit: 8d34b60cc52f33f439f67b16b9e5d50ccaa90a3b
created_at: 2026-11-07T05:50:04.209456115Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    recorded_at: 2026-11-07T05:53:44.684776922Z
    resolution_commit: b7282bb8cd7a53d795bc8cfbc615795be3ce5c8d
    reason: Fixed in b7282bb8.
---

low: docs/adr/0019-verified-repo-native-tracker.md:9 The new Status paragraph says ADR 0028 'names each part' and lists them, but leaves out a part ADR 0028 names for decision 2 (docs/adr/0028-simpler-lifecycle.md:223-225): Atelier now enforces two steps of one order (review request refuses an unticked Done when; merge marks the issue done), which replaces decision 2's 'statuses, transitions ... are agent-owned' for those steps. A reader of ADR 0019 takes the list as complete (Done when: the Status line says which part it replaces). Fix: add it, with the open 'never creates commits' point.
