---
created_at: "2026-06-18T00:55:08.984445134+00:00"
id: "atelier-xkp5"
evidence_type: "validation"
captured_at: "2026-06-18T00:55:05.217625212+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-app forgejo\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier lint atelier-mpah\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-mpah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-mpah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mocked Forgejo client request, sudo, state, and failure tests pass"
updated_at: "2026-06-18T00:55:12.976578572+00:00"
---

## Summary

Mocked Forgejo client request, sudo, state, and failure tests pass

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-app forgejo
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier lint atelier-mpah
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 715
Truncated: no

```text

running 7 tests
test forgejo::tests::finds_and_shows_pull_request_state ... ok
test forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship ... ok
test forgejo::tests::lists_review_comments_and_surfaces_api_failures ... ok
test forgejo::tests::opens_pull_with_role_sudo_header_and_payload ... ok
test project_config::tests::missing_forgejo_config_is_actionable ... ok
test project_config::tests::parses_valid_forgejo_config_and_sudo_mapping ... ok
test project_config::tests::invalid_forgejo_config_names_missing_role_and_token ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 36 filtered out; finished in 0.00s

Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 295
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.86s
```

