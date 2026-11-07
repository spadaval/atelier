---
schema: atelier.review
schema_version: 2
id: atelier-87vf
title: finding on atelier-l5fu @ebc6e56f
target: atelier-l5fu
identity: Independent reviewer (Claude subagent)
commit: ebc6e56f46ff3ee8f0341a1a67a1a50bcd3fa804
created_at: 2026-11-07T21:35:06.677575462Z
event:
  event: finding
  recommendation: defer
---

low: spadaval/.agents d4db367 evals/agent-factory/README.md:62-64 says 'nothing deletes the others', meaning the work directories of failed or invalid runs. Under the new default /var/tmp/af-evals, that is not true on this host: /usr/lib/tmpfiles.d/tmp.conf has 'q /var/tmp 1777 root root 30d', and systemd-tmpfiles-clean.timer runs daily. A failed run's work directory, which holds the grader's inputs, can therefore disappear after about 30 days and leave the run directory's 'work' link dangling. Under the old ~/.cache layout those directories stayed. This item depended on that: it regraded four kept 11-07 work directories to prove the grader fix. Fix: say in the README that the OS may age out /var/tmp, and point to --work-root for longer retention. Optionally, copy a failed run's work directory, or its grader inputs, into --out.
