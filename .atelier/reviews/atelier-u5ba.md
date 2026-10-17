---
schema: atelier.review
schema_version: 1
id: atelier-u5ba
title: 'Checked merge d1adaaeb (master with atelier-7w1g into bug/atelier-6xxb) using git show --cc and git diff master...HEAD. evidence.rs: validate_evidence_relation_role and its call sites are gone, taken from master. 6xxb kept only the evidence-list footer naming the newest record, and nothing 6xxb still needs is lost: clap now rejects bad --role values and lists `validates`, so that error has no placeholder recovery hint left. The remaining role-related placeholder is relate.rs:70 `atelier evidence attach <evidence-id> issue {source}`, which is justified because no evidence ID is known there. Master brought no new `<...-id>` placeholders, single-quoted commands or conflict markers. The docs merged cleanly: Help was added beside the Next Footer rule. Evidence atelier-z865 (nextest) passes at d1adaaeb, and the test build has no warnings. One fix_now finding (atelier-0nsp): the merge left dead parameters and a false doc comment on assert_corrective_evidence_role_error in records_evidence.rs. Not approving until that is resolved. Housekeeping: I briefly created an empty untracked tests/cli_integration/main.rs with touch and removed it; git status is clean.'
target: atelier-6xxb
identity: Independent reviewer (Claude subagent)
commit: 76f175de6e13491451a70d9accf7b545f2fc51a6
created_at: 2026-10-17T07:53:31.319274386Z
event:
  event: review
---

Checked merge d1adaaeb (master with atelier-7w1g into bug/atelier-6xxb) using git show --cc and git diff master...HEAD. evidence.rs: validate_evidence_relation_role and its call sites are gone, taken from master. 6xxb kept only the evidence-list footer naming the newest record, and nothing 6xxb still needs is lost: clap now rejects bad --role values and lists `validates`, so that error has no placeholder recovery hint left. The remaining role-related placeholder is relate.rs:70 `atelier evidence attach <evidence-id> issue {source}`, which is justified because no evidence ID is known there. Master brought no new `<...-id>` placeholders, single-quoted commands or conflict markers. The docs merged cleanly: Help was added beside the Next Footer rule. Evidence atelier-z865 (nextest) passes at d1adaaeb, and the test build has no warnings. One fix_now finding (atelier-0nsp): the merge left dead parameters and a false doc comment on assert_corrective_evidence_role_error in records_evidence.rs. Not approving until that is resolved. Housekeeping: I briefly created an empty untracked tests/cli_integration/main.rs with touch and removed it; git status is clean.
