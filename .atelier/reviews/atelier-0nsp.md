---
schema: atelier.review
schema_version: 1
id: atelier-0nsp
title: 'crates/atelier-cli/tests/cli_integration/records_evidence.rs:1751,1797,1814-1815 — the merge resolution (d1adaaeb) left residue from 6xxb''s side of `assert_corrective_evidence_role_error`. The helper still takes `target` and `evidence` and is still documented "The recovery commands name the target and evidence the command was given", but its body is now master''s: it asserts only clap''s `invalid value ''validation'' for ''--role <ROLE>''` and `[possible values: validates]`, and that error carries no recovery commands. The callers still pass `target_id`, `"<evidence-id>"`, `reuse_id` and `&evidence_id`. The two parameters are never read; `#![allow(unused_variables)]` in tests/cli_integration.rs hides the warning. The result is a test that documents a claim it does not check, and the diff against master touches a test whose behaviour it does not change. Fix: take master''s version of the helper signature, its doc and both call sites, so records_evidence.rs differs from master only in the two evidence-list footer assertions.'
target: atelier-6xxb
identity: Independent reviewer (Claude subagent)
commit: 76f175de6e13491451a70d9accf7b545f2fc51a6
created_at: 2026-10-17T07:53:24.955212083Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/tests/cli_integration/records_evidence.rs:1751,1797,1814-1815 — the merge resolution (d1adaaeb) left residue from 6xxb's side of `assert_corrective_evidence_role_error`. The helper still takes `target` and `evidence` and is still documented "The recovery commands name the target and evidence the command was given", but its body is now master's: it asserts only clap's `invalid value 'validation' for '--role <ROLE>'` and `[possible values: validates]`, and that error carries no recovery commands. The callers still pass `target_id`, `"<evidence-id>"`, `reuse_id` and `&evidence_id`. The two parameters are never read; `#![allow(unused_variables)]` in tests/cli_integration.rs hides the warning. The result is a test that documents a claim it does not check, and the diff against master touches a test whose behaviour it does not change. Fix: take master's version of the helper signature, its doc and both call sites, so records_evidence.rs differs from master only in the two evidence-list footer assertions.
