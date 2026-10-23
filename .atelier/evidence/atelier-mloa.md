---
created_at: "2026-10-23T16:13:11.482161839+00:00"
id: "atelier-mloa"
evidence_type: "test"
commit: "e249fae51f00de55ceeb0315f398eb748494516e"
result: "pass"
captured_at: "2026-10-23T16:13:07.972588126+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(an_uncommitted_approval_is_fixed_by_committing_it) | test(land_base_mismatch_prints_a_runnable_fix) | test(a_gate_on_a_record_that_differs_from_head_is_fixed_by_committing_it) | test(land_shows_no_pass_for_an_owner_whose_checkout_record_differs)'"
exit_status: "0"
agent_identity: "claude-implementer-0uaj"
target:
  kind: "issue"
  id: "atelier-0uaj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0uaj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Findings atelier-q5uh and atelier-hen4: an uncommitted approval is judged on HEAD with the records-commit fix and no pass in land and merge --check; the base-mismatch fix runs verbatim for other, a|b, it$s, a@b and q'q"
updated_at: "2026-10-23T16:13:11.482512283+00:00"
---

## Summary

Findings atelier-q5uh and atelier-hen4: an uncommitted approval is judged on HEAD with the records-commit fix and no pass in land and merge --check; the base-mismatch fix runs verbatim for other, a|b, it$s, a@b and q'q

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks -E 'test(an_uncommitted_approval_is_fixed_by_committing_it) | test(land_base_mismatch_prints_a_runnable_fix) | test(a_gate_on_a_record_that_differs_from_head_is_fixed_by_committing_it) | test(land_shows_no_pass_for_an_owner_whose_checkout_record_differs)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 883
SHA-256: fbfbcd7d3ab1b004d003fae7406c033c35771e21308d04130306f3db5bd0164c
Test summary: Summary [   1.265s] 4 tests run: 4 passed, 723 skipped
