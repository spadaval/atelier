---
created_at: "2026-06-18T00:38:10.762214780+00:00"
id: "atelier-48tj"
evidence_type: "validation"
captured_at: "2026-06-18T00:38:06.415437056+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-records issue_record_round_trips_typed_fields\ncargo test -p atelier-sqlite issue_fields_round_trip_through_projection_table\ncargo test -p atelier-app rebuild_round_trips_canonical_issue_fields\ncargo test -p atelier-app rebuild_rejects_issue_fields_that_violate_workflow_schema\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-rgmg\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-rgmg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rgmg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Typed issue field parse, projection, validation, and tracker checks pass"
updated_at: "2026-06-18T00:38:14.756322363+00:00"
---

## Summary

Typed issue field parse, projection, validation, and tracker checks pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-records issue_record_round_trips_typed_fields
cargo test -p atelier-sqlite issue_fields_round_trip_through_projection_table
cargo test -p atelier-app rebuild_round_trips_canonical_issue_fields
cargo test -p atelier-app rebuild_rejects_issue_fields_that_violate_workflow_schema
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-rgmg
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 813
Truncated: no

```text

running 1 test
test tests::issue_record_round_trips_typed_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.00s


running 1 test
test issues::tests::issue_fields_round_trip_through_projection_table ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 68 filtered out; finished in 0.07s


running 1 test
test rebuild::tests::rebuild_round_trips_canonical_issue_fields ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.18s


running 1 test
test rebuild::tests::rebuild_rejects_issue_fields_that_violate_workflow_schema ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.08s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 770
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_records-7077b966ddcbf0d6)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/atelier_sqlite-1bd72f9e4bfd282b)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
```

