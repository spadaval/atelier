---
created_at: "2026-06-18T00:51:27.646964791+00:00"
id: "atelier-gi4b"
evidence_type: "validation"
captured_at: "2026-06-18T00:51:20.947092346+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-app project_config\ncargo test -p atelier-cli test_run_fresh_init\nrg -n \"\\[forgejo\\]|FORGEJO_ADMIN_TOKEN|sudo_users\" .atelier/config.toml docs/spec/storage/export/rebuild/canonical-layout.md\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-e7oj\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-e7oj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-e7oj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Forgejo config parser, sudo mapping guidance, and tracker checks pass"
updated_at: "2026-06-18T00:51:31.578424364+00:00"
---

## Summary

Forgejo config parser, sudo mapping guidance, and tracker checks pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-app project_config
cargo test -p atelier-cli test_run_fresh_init
rg -n "\[forgejo\]|FORGEJO_ADMIN_TOKEN|sudo_users" .atelier/config.toml docs/spec/storage/export/rebuild/canonical-layout.md
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-e7oj
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1323
Truncated: no

```text

running 3 tests
test project_config::tests::missing_forgejo_config_is_actionable ... ok
test project_config::tests::parses_valid_forgejo_config_and_sudo_mapping ... ok
test project_config::tests::invalid_forgejo_config_names_missing_role_and_token ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 36 filtered out; finished in 0.00s


running 1 test
test commands::init::tests::test_run_fresh_init ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 169 filtered out; finished in 0.11s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 354 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

docs/spec/storage/export/rebuild/canonical-layout.md:62:[forgejo]
docs/spec/storage/export/rebuild/canonical-layout.md:66:admin_token_env = "FORGEJO_ADMIN_TOKEN"
docs/spec/storage/export/rebuild/canonical-layout.md:68:[forgejo.sudo_users]
.atelier/config.toml:15:# [forgejo]
.atelier/config.toml:19:# admin_token_env = "FORGEJO_ADMIN_TOKEN"
.atelier/config.toml:21:# [forgejo.sudo_users]
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 902
Truncated: no

```text
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-d5750e597ef620d8)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.15s
     Running unittests src/lib.rs (target/debug/deps/atelier-ece93819391d9df2)
     Running unittests src/main.rs (target/debug/deps/atelier-c7f6cd92c6fe8d49)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-076346abfeeaa5b0)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-65c6eee9621a4c8f)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
```

