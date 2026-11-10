---
schema: atelier.review
schema_version: 2
id: atelier-8fsp
title: finding on atelier-fcb7 @9394bb27
target: atelier-fcb7
identity: grok-4.6-amend3-r1
commit: 9394bb27f77fc5da005bab5e28496dbf8e6a0a27
created_at: 2026-11-10T04:53:38.674165262Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 6f5a88a3653711597773722d59daecc5a1c19c4a
    recorded_at: 2026-11-10T04:55:21.872293402Z
    reason: Fixed in 7ca68749 although recommended defer.
---

low atelier-mal9 and redesign-architecture.md:84: atelier-records is specified to read the three record kinds from a checkout or from a commit, which is one of the two dashboard primitives, and mal9's Done when is parse, write and validate files only. Consequence: step A can merge with no at-commit API, so the primitive exists only once atelier-oci2's site loader or atelier-8b0a is planned. Smallest fix: require records-at-commit on atelier-records when oci2 is planned; leave mal9 as file IO.
