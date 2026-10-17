---
schema: atelier.review
schema_version: 1
id: atelier-exkf
title: 'crates/atelier-app/src/mission_review.rs:353-385: Work is built only from linked items, so a done child of an open or superseded parent appears nowhere: it is missing from Work, it is missing from Open work, and its report, decisions and review are lost. delivered_by still credits it, so the criterion row links to #<child-id>, an anchor that does not exist. Also, at line 369, a superseded child of an open parent goes through !done() and is listed in Open work as pending, not under Dropped. Reproduced in a scratch repo: a mission with an open parent P, a done child C whose Goal names ''fast'', and a superseded child S. The Markdown shows ''fast unproven Fast. | by C'', has no Work section, and lists S under Open Work. The HTML has href="#C" and no id="C". The previous debrief listed every scoped item, so this is a regression, and it is common while a mission is still open (debrief supports open missions via Last delivery). Fix: give done children of non-done parents their own Work entry (or a parent entry), send superseded children to dropped, and add a fixture test for both cases.'
target: atelier-nexd
identity: Independent reviewer (Claude subagent)
commit: ea0cc2c06466337e760dcb831b7f06b3516d3580
created_at: 2026-10-17T21:58:48.332815628Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/mission_review.rs:353-385: Work is built only from linked items, so a done child of an open or superseded parent appears nowhere: it is missing from Work, it is missing from Open work, and its report, decisions and review are lost. delivered_by still credits it, so the criterion row links to #<child-id>, an anchor that does not exist. Also, at line 369, a superseded child of an open parent goes through !done() and is listed in Open work as pending, not under Dropped. Reproduced in a scratch repo: a mission with an open parent P, a done child C whose Goal names 'fast', and a superseded child S. The Markdown shows 'fast unproven Fast. | by C', has no Work section, and lists S under Open Work. The HTML has href="#C" and no id="C". The previous debrief listed every scoped item, so this is a regression, and it is common while a mission is still open (debrief supports open missions via Last delivery). Fix: give done children of non-done parents their own Work entry (or a parent entry), send superseded children to dropped, and add a fixture test for both cases.
