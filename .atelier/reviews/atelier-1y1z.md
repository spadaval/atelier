---
schema: atelier.review
schema_version: 2
id: atelier-1y1z
title: 'Test gap. The new pin clause in review_gate.rs:512-513 (!ancestor(pin, base_commit)) has no regression test proving that an approval pinned before the branch left the base is still refused after the base is merged in. I checked it by hand with the built binary: approve on main, move main, start, claim (record-only), move main again, merge main into the branch. The result is FAIL review.current, so the behaviour is correct. But no test fails if the clause is loosened, for example to accept any pin when the branch has record-only changes. Add a merge_gates case.'
target: atelier-n7g1
identity: Independent reviewer (Claude subagent)
commit: 1f9c88ded59b5f0bcd19473fb6e9e9c7753f45ca
created_at: 2026-10-16T18:31:51.221337944Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 1b916f400b7c37ed7bb110695d534e4236a85ba7
    recorded_at: 2026-10-16T18:33:32.993260029Z
    follow_up: atelier-7hh4
    reason: Deferred to atelier-7hh4 as recommended; not exploitable through atelier merge.
    migrated_from: atelier-jt22
---

Test gap. The new pin clause in review_gate.rs:512-513 (!ancestor(pin, base_commit)) has no regression test proving that an approval pinned before the branch left the base is still refused after the base is merged in. I checked it by hand with the built binary: approve on main, move main, start, claim (record-only), move main again, merge main into the branch. The result is FAIL review.current, so the behaviour is correct. But no test fails if the clause is loosened, for example to accept any pin when the branch has record-only changes. Add a merge_gates case.
