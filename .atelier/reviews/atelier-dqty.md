---
schema: atelier.review
schema_version: 1
id: atelier-dqty
title: 'crates/atelier-app/src/criteria.rs:80,224: a receipt''s criteria are not tied to one mission. missions_for accepts a slug if ANY mission of the target defines it, and mission_proof counts every validates-linked receipt in scope by slug alone. With an item advancing two missions (supported: the refusal text says ''mission A or B''), a receipt recorded as S1..S8 for atelier-r6bt marks atelier-1w8c''s unrelated S1..S6 ''✓ proven'' and lists S7/S8 as orphaned in 1w8c''s view while ''check'' passes (reproduced in a scratch clone by linking atelier-fip8 to atelier-1w8c). Positional S<n> slugs are shared by every strategy, so collisions are the norm. Same leak via ''evidence attach'' to an item in another mission. Fix: refuse --criterion (record and check) when the target is in more than one mission, or store the mission id on the receipt and match on it; and only report orphans that check also reports.'
target: atelier-ho7o
identity: Independent reviewer (Claude subagent)
commit: 9d56d55fc73d6ac4a9dd5b30848f5f364f5eb0e2
created_at: 2026-10-17T18:21:22.341129Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/criteria.rs:80,224: a receipt's criteria are not tied to one mission. missions_for accepts a slug if ANY mission of the target defines it, and mission_proof counts every validates-linked receipt in scope by slug alone. With an item advancing two missions (supported: the refusal text says 'mission A or B'), a receipt recorded as S1..S8 for atelier-r6bt marks atelier-1w8c's unrelated S1..S6 '✓ proven' and lists S7/S8 as orphaned in 1w8c's view while 'check' passes (reproduced in a scratch clone by linking atelier-fip8 to atelier-1w8c). Positional S<n> slugs are shared by every strategy, so collisions are the norm. Same leak via 'evidence attach' to an item in another mission. Fix: refuse --criterion (record and check) when the target is in more than one mission, or store the mission id on the receipt and match on it; and only report orphans that check also reports.
