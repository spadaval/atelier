---
schema: atelier.review
schema_version: 1
id: atelier-7neu
title: 'Misattribution for historical claims: crates/atelier-cli/src/commands/session.rs:485-499 adds ''Uncommitted changes make passing proof stale: <paths>'' whenever any failed node reason contains ''does not prove the current checkout'' and the checkout is dirty, but crates/atelier-app/src/review_gate.rs:703-706 emits that reason for historical claims too (newest_failure with fresh=false), where proof_matches ignores the working tree entirely. Scenario: a retained done item has fail, then pass at C1, then a code commit C2, then the close at C3 (the C1..C3 edges include code, so the pass is invalid historically); land run with any unrelated dirty file names that file as the cause, and committing it does not clear the gate. The gate reason also says ''current checkout'' in historical mode, where the claim endpoint is what is being proved. The CLI matching a substring of an app-layer prose reason is brittle as well: rewording review_gate.rs silently drops the note. Suggested fix: have NewestFailure carry whether the pass is stale because the checkout is dirty (fresh && !code_clean), use a structured flag or check id rather than a substring, and word the historical case separately.'
target: atelier-749l
identity: Independent reviewer (Claude subagent)
commit: 0860b910f2663ccc7aa445ac13ce8abf6a722165
created_at: 2026-10-16T19:11:53.228250058Z
event:
  event: finding
  recommendation: defer
---

Misattribution for historical claims: crates/atelier-cli/src/commands/session.rs:485-499 adds 'Uncommitted changes make passing proof stale: <paths>' whenever any failed node reason contains 'does not prove the current checkout' and the checkout is dirty, but crates/atelier-app/src/review_gate.rs:703-706 emits that reason for historical claims too (newest_failure with fresh=false), where proof_matches ignores the working tree entirely. Scenario: a retained done item has fail, then pass at C1, then a code commit C2, then the close at C3 (the C1..C3 edges include code, so the pass is invalid historically); land run with any unrelated dirty file names that file as the cause, and committing it does not clear the gate. The gate reason also says 'current checkout' in historical mode, where the claim endpoint is what is being proved. The CLI matching a substring of an app-layer prose reason is brittle as well: rewording review_gate.rs silently drops the note. Suggested fix: have NewestFailure carry whether the pass is stale because the checkout is dirty (fresh && !code_clean), use a structured flag or check id rather than a substring, and word the historical case separately.
