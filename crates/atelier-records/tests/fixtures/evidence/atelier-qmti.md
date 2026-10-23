---
created_at: "2026-10-23T14:59:36.029250791+00:00"
id: "atelier-qmti"
evidence_type: "test"
commit: "c0b9a62fbf9ad8139d45113ccd816d55dc6665a7"
result: "pass"
captured_at: "2026-10-23T14:59:33.935485894+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'"
exit_status: "0"
agent_identity: "atelier merge --sync"
target:
  kind: "issue"
  id: "atelier-h1nc"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h1nc"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "## Summary\n\natelier-ooay: Stop-hook reason and repeated-stop message are a sentence, then land's refusal lines, ending with the land verdict; a precondition refusal's reason ends with its verdict\n\n## Command\n\n```console\ncargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nSHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n\n## Stderr\n\nBytes: 691\nSHA-256: 840f3f82fb59f3b8478cda16c578a4c2fcead82f7827b6510ba1d3e5e6285fec\nTest summary: Summary [   0.325s] 2 tests run: 2 passed, 896 skipped (rerun after merging master)"
updated_at: "2026-10-23T14:59:36.029536384+00:00"
---

## Summary

## Summary

atelier-ooay: Stop-hook reason and repeated-stop message are a sentence, then land's refusal lines, ending with the land verdict; a precondition refusal's reason ends with its verdict

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 691
SHA-256: 840f3f82fb59f3b8478cda16c578a4c2fcead82f7827b6510ba1d3e5e6285fec
Test summary: Summary [   0.325s] 2 tests run: 2 passed, 896 skipped (rerun after merging master)

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check) | test(documented_hook_events_use_nested_cwd_and_stop_loop_protection)'
```

Exit status: 0

## Stdout

Bytes: 0
SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Stderr

Bytes: 691
SHA-256: 22337bf7cdcf321efad28345aa12473e36eb87fdafdea037a8fa409e77b0e897
Test summary: Summary [   0.320s] 2 tests run: 2 passed, 910 skipped
