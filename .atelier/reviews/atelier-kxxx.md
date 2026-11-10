---
schema: atelier.review
schema_version: 2
id: atelier-kxxx
title: finding on atelier-jah8 @6f837e9f
target: atelier-jah8
identity: grok-4.6-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T01:06:14.559770273Z
event:
  event: finding
  recommendation: fix_now
---

high atelier-5fsj: scripts query the live 4 GB ~/.t3/userdata/statev2.sqlite on the host that runs the T3 server, with no snapshot and a window that ends the day measuring starts, so a rerun cannot match counted.json and a table scan can stall the phone server. Smallest fix: copy the DB once, pin window start 2026-10-14 and window end to that copy's mtime, and point every reader at the copy; drop the live path from the README.
