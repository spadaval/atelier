---
schema: atelier.review
schema_version: 1
id: atelier-q0g2
title: 'crates/atelier-cli/src/commands/issue.rs:113 and :930, crates/atelier-app/src/lint.rs:141: size labels are checked only by ''check'', bundles and ''update --body''. ''issue create'' and ''issue update -l'' accept invalid sizes. Repro in a scratch repo: ''issue create X -l size:M'' and ''issue create Y -l size:s -l size:l'' both exit 0, and ''issue update <id> -l size:huge'' exits 0. ''check'' then fails on all of them. Case makes this worse: ''issue show'' prints ''Size:     M'' for the invalid label size:M because the display uppercases the value, so the label looks valid. Next, ''issue update <id> --body ...'' on that record fails with ''Invalid --body for <id>: Size label size:huge is not one of ...'', which blames the body for a label problem. Where the lint.none_blocking gate is configured, one bad label on any record blocks that gate across the whole repository. Fix: run the same size check when labels are added in create and update -l (or lowercase size:M to size:m when it is added). Show Size only for a valid value. Add create and update cases to test_size_labels_are_validated_shown_and_replaced.'
target: atelier-b7hb
identity: Independent reviewer (Claude subagent)
commit: 316cc70ef3a1117026263abb3c68de001aba148f
created_at: 2026-10-16T01:36:43.136314923Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/issue.rs:113 and :930, crates/atelier-app/src/lint.rs:141: size labels are checked only by 'check', bundles and 'update --body'. 'issue create' and 'issue update -l' accept invalid sizes. Repro in a scratch repo: 'issue create X -l size:M' and 'issue create Y -l size:s -l size:l' both exit 0, and 'issue update <id> -l size:huge' exits 0. 'check' then fails on all of them. Case makes this worse: 'issue show' prints 'Size:     M' for the invalid label size:M because the display uppercases the value, so the label looks valid. Next, 'issue update <id> --body ...' on that record fails with 'Invalid --body for <id>: Size label size:huge is not one of ...', which blames the body for a label problem. Where the lint.none_blocking gate is configured, one bad label on any record blocks that gate across the whole repository. Fix: run the same size check when labels are added in create and update -l (or lowercase size:M to size:m when it is added). Show Size only for a valid value. Add create and update cases to test_size_labels_are_validated_shown_and_replaced.
