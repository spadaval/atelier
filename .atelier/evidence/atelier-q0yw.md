---
created_at: "2026-07-06T18:15:52.863320031+00:00"
id: "atelier-q0yw"
evidence_type: "test"
captured_at: "2026-07-06T18:15:24.440238995+00:00"
command: "cargo nextest run -p atelier-app -p atelier-cli forgejo::tests::provider_contract forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship atelier-app::pr::tests atelier-app::review_room::tests provider_request_review_pushes_source_before_opening_pr review_surface_derives_open_context_and_uses_submit_and_show"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ye11"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ye11"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent review fixes pass provider-contract, provider CLI, application PR, native-room, and collapsed-surface regression coverage."
updated_at: "2026-07-06T18:15:58.553843794+00:00"
---

## Summary

Independent review fixes pass provider-contract, provider CLI, application PR, native-room, and collapsed-surface regression coverage.

## Command

```console
cargo nextest run -p atelier-app -p atelier-cli forgejo::tests::provider_contract forgejo::tests::comments_and_reviews_with_distinct_sudo_authorship atelier-app::pr::tests atelier-app::review_room::tests provider_request_review_pushes_source_before_opening_pr review_surface_derives_open_context_and_uses_submit_and_show
```
Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 6100
Truncated: yes

```text
   Compiling proc-macro2 v1.0.104
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.178
   Compiling serde_core v1.0.228
   Compiling stable_deref_trait v1.2.1
   Compiling serde v1.0.228
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.6
   Compiling once_cell v1.21.3
   Compiling autocfg v1.5.0
   Compiling litemap v0.8.2
   Compiling version_check v0.9.5
   Compiling writeable v0.6.3
   Compiling smallvec v1.15.1
   Compiling itoa v1.0.17
   Compiling icu_normalizer_data v2.1.1
   Compiling icu_properties_data v2.1.2
   Compiling hashbrown v0.17.1
   Compiling equivalent v1.0.2
   Compiling serde_json v1.0.148
   Compiling getrandom v0.3.4
   Compiling bitflags v2.10.0
   Compiling zmij v1.0.0
   Compiling typenum v1.20.1
   Compiling zeroize v1.9.0
   Compiling memchr v2.7.6
   Compiling vcpkg v0.2.15
   Compiling pkg-config v0.3.32
   Compiling iana-time-zone v0.1.64
   Compiling anyhow v1.0.100
   Compiling foldhash v0.2.0
   Compiling crc32fast v1.5.0
   Compiling ryu v1.0.23
   Compiling untrusted v0.9.0
   Compiling cc v1.2.51
   Compiling tracing-core v0.1.36
   Compiling rustls-pki-types v1.14.1
   Compiling unsafe-libyaml v0.2.11
   Compiling log v0.4.29
   Compiling hashbrown v0.16.1
   Compiling rustls v0.23.40
   Compiling simd-adler32 v0.3.9
   Compiling percent-encoding v2.3.2
   Compiling utf8_iter v1.0.4
   Compiling pin-project-lite v0.2.17
   Compiling adler2 v2.0.1
   Compiling winnow v0.7.15
   Compiling fallible-streaming-iterator v0.1.9
   Compiling toml_write v0.1.2
   Compiling fallible-iterator v0.3.0
   Compiling cpufeatures v0.2.17
   Compiling subtle v2.6.1
   Compiling rustix v1.1.3
   Compiling miniz_oxide v0.8.9
   Compiling generic-array v0.14.7
   Compiling linux-raw-sys v0.11.0
   Compiling form_urlencoded v1.2.2
   Compiling base64 v0.22.1
   Compiling utf8parse v0.2.2
   Compiling regex-syntax v0.8.8
   Compiling zerocopy v0.8.31
   Compiling colorchoice v1.0.4
   Compiling is_terminal_polyfill v1.70.2
   Compiling num-traits v0.2.19
   Compiling anstyle v1.0.13
   Compiling anstyle-parse v0.2.7
   Compiling fastrand v2.3.0
   Compiling anstyle-query v1.1.5
   Compiling heck v0.5.0
   Compiling strsim v0.11.1
   Compiling lazy_static v1.5.0
   Compiling clap_lex v0.7.6
   Compiling thread_local v1.1.9
   Compiling webpki-roots v1.0.8
   Compiling tracing-log v0.2.0
   Compiling anstream v0.6.21
   Compiling sharded-slab v0.1.7
   Compiling webpki-roots v0.26.11
   Compiling nu-ansi-term v0.50.3
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-ye11/crates/atelier-cli)
   Compiling dotenvy v0.15.7
   Compiling fnv v1.0.7
   Compiling bit-vec v0.8.0
   Compiling indexmap v2.14.0
   Compiling quick-error v1.2.3
   Compiling unarray v0.1.4
   Compiling flate2 v1.1.9
   Compiling arbitrary v1.4.2
   Compiling hashlink v0.11.0
   Compiling clap_builder v4.5.53
   Compiling bit-set v0.8.0
   Compiling syn v2.0.111
   Compiling ring v0.17.14
   Compiling libsqlite3-sys v0.36.0
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling fs2 v0.4.3
   Compiling wait-timeout v0.2.1
   Compiling digest v0.10.7
   Compiling sha2 v0.10.9
   Compiling rand_core v0.9.3
   Compiling rand v0.9.4
   Compiling rand_xorshift v0.4.0
   Compiling regex-automata v0.4.14
   Compiling tempfile v3.24.0
   Compiling ppv-lite86 v0.2.21
   Compiling rusty-fork v0.3.1
   Compiling rand_chacha v0.9.0
   Compiling proptest v1.9.0
   Compiling matchers v0.2.0
   Compiling synstructure v0.13.2
   Compiling zerofrom-derive v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling serde_derive v1.0.228
   Compiling zerovec-derive v0.11.2
   Compiling displaydoc v0.2.6
   Compiling tracing-attributes v0.1.31
   Compiling clap_derive v4.5.49
   Compiling rustls-webpki v0.103.13
   Compiling zerofrom v0.1.8
   Compiling tracing v0.1.44
   Compiling yoke v0.8.1
   Compiling zerovec v0.11.5
   Compiling zerotrie v0.2.3
   Compiling tinystr v0.8.2
   Compiling potential_utf v0.1.4
   Compiling
```
