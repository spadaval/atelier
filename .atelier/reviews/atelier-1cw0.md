---
schema: atelier.review
schema_version: 2
id: atelier-1cw0
title: finding on atelier-a4rs @ad45d5ef
target: atelier-a4rs
identity: GPT-6.1 Sol independent reviewer
commit: ad45d5efe2ec066f8da3e76ea16fb0999eb9123a
created_at: 2026-11-11T07:03:33.241733696Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-non-atelier-repo/grade.py:66-68 exempts an entire shell call from tracker-write detection whenever any command in it contains --help or -h. Reproduced with the copied post-atelier-psjw binary and the completed feature fixture: atelier new "Track the export" exits 2 and grades pass=false with a tracker_writes entry; atelier --help; atelier new "Track the export" runs the same forbidden writer, also exits 2, but grades pass=true with tracker_writes=[] (one errored call is counted). Genuine atelier new --help correctly passes. This silently accepts over-application that the scenario must fail. Apply the help exemption only to its invocation, or conservatively send mixed help/write calls to pass=null with needs_hand_grade; add paired walks for the mixed call and help-only call.
