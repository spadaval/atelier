---
schema: atelier.review
schema_version: 1
id: atelier-1y1z
title: 'Test gap. The new pin clause in review_gate.rs:512-513 (!ancestor(pin, base_commit)) has no regression test proving that an approval pinned before the branch left the base is still refused after the base is merged in. I checked it by hand with the built binary: approve on main, move main, start, claim (record-only), move main again, merge main into the branch. The result is FAIL review.current, so the behaviour is correct. But no test fails if the clause is loosened, for example to accept any pin when the branch has record-only changes. Add a merge_gates case.'
target: atelier-n7g1
identity: Independent reviewer (Claude subagent)
commit: 1f9c88ded59b5f0bcd19473fb6e9e9c7753f45ca
created_at: 2026-10-16T18:31:51.221337944Z
event:
  event: finding
  recommendation: defer
---

Test gap. The new pin clause in review_gate.rs:512-513 (!ancestor(pin, base_commit)) has no regression test proving that an approval pinned before the branch left the base is still refused after the base is merged in. I checked it by hand with the built binary: approve on main, move main, start, claim (record-only), move main again, merge main into the branch. The result is FAIL review.current, so the behaviour is correct. But no test fails if the clause is loosened, for example to accept any pin when the branch has record-only changes. Add a merge_gates case.
