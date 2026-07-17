---
created_at: "2026-07-17T00:13:40.755038967+00:00"
id: "atelier-hf6z"
evidence_type: "validation"
captured_at: "2026-07-17T00:13:17.054161855+00:00"
command: "bash -lc 'cargo fmt -- --check && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && git diff --check && target/debug/atelier check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-t876"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && git diff --check && target/debug/atelier check'"
updated_at: "2026-07-17T00:13:40.758068294+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && git diff --check && target/debug/atelier check'

## Command

```console
bash -lc 'cargo fmt -- --check && RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets && git diff --check && target/debug/atelier check'
```

Exit status: 0

## Stdout

Bytes: 13
Truncated: no

```text
Lint passed.
```

## Stderr

Bytes: 5303
Truncated: yes

```text
   Compiling proc-macro2 v1.0.104
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
    Checking cfg-if v1.0.4
   Compiling libc v0.2.178
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
    Checking stable_deref_trait v1.2.1
   Compiling shlex v1.3.0
   Compiling find-msvc-tools v0.1.6
    Checking once_cell v1.21.3
   Compiling autocfg v1.5.0
    Checking litemap v0.8.2
    Checking writeable v0.6.3
   Compiling version_check v0.9.5
    Checking smallvec v1.15.1
    Checking itoa v1.0.17
   Compiling icu_properties_data v2.1.2
   Compiling icu_normalizer_data v2.1.1
   Compiling serde_json v1.0.148
    Checking hashbrown v0.17.1
    Checking equivalent v1.0.2
    Checking zmij v1.0.0
    Checking memchr v2.7.6
    Checking typenum v1.20.1
    Checking iana-time-zone v0.1.64
    Checking utf8_iter v1.0.4
    Checking write16 v1.0.0
    Checking utf16_iter v1.0.5
   Compiling getrandom v0.3.4
   Compiling anyhow v1.0.100
    Checking bitflags v2.10.0
    Checking ryu v1.0.23
    Checking unsafe-libyaml v0.2.11
   Compiling vcpkg v0.2.15
   Compiling pkg-config v0.3.32
    Checking zeroize v1.9.0
    Checking foldhash v0.2.0
   Compiling cc v1.2.51
    Checking cpufeatures v0.2.17
   Compiling crc32fast v1.5.0
    Checking pin-project-lite v0.2.17
    Checking untrusted v0.9.0
    Checking log v0.4.29
    Checking tracing-core v0.1.36
   Compiling rustix v1.1.3
    Checking regex-syntax v0.8.8
    Checking fallible-iterator v0.3.0
    Checking percent-encoding v2.3.2
    Checking linux-raw-sys v0.11.0
   Compiling rustls v0.23.40
    Checking adler2 v2.0.1
    Checking hashbrown v0.16.1
    Checking rustls-pki-types v1.14.1
    Checking fallible-streaming-iterator v0.1.9
    Checking simd-adler32 v0.3.9
    Checking toml_write v0.1.2
    Checking fastrand v2.3.0
    Checking winnow v0.7.15
    Checking utf8parse v0.2.2
   Compiling zerocopy v0.8.31
    Checking subtle v2.6.1
    Checking is_terminal_polyfill v1.70.2
    Checking form_urlencoded v1.2.2
    Checking base64 v0.22.1
   Compiling generic-array v0.14.7
    Checking anstyle-query v1.1.5
    Checking anstyle-parse v0.2.7
    Checking colorchoice v1.0.4
    Checking anstyle v1.0.13
    Checking miniz_oxide v0.8.9
    Checking lazy_static v1.5.0
    Checking strsim v0.11.1
    Checking clap_lex v0.7.6
   Compiling heck v0.5.0
    Checking thread_local v1.1.9
   Compiling num-traits v0.2.19
    Checking bit-vec v0.8.0
    Checking fnv v1.0.7
    Checking tracing-log v0.2.0
    Checking sharded-slab v0.1.7
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-p4z2/crates/atelier-cli)
    Checking quick-error v1.2.3
    Checking nu-ansi-term v0.50.3
    Checking unarray v0.1.4
    Checking webpki-roots v1.0.8
    Checking dotenvy v0.15.7
    Checking arbitrary v1.4.2
    Checking anstream v0.6.21
    Checking indexmap v2.14.0
    Checking webpki-roots v0.26.11
    Checking bit-set v0.8.0
    Checking clap_builder v4.5.53
    Checking flate2 v1.1.9
    Checking hashlink v0.11.0
   Compiling syn v2.0.111
   Compiling ring v0.17.14
   Compiling libsqlite3-sys v0.36.0
    Checking fs2 v0.4.3
    Checking getrandom v0.2.17
    Checking wait-timeout v0.2.1
    Checking block-buffer v0.10.4
    Checking crypto-common v0.1.7
    Checking rand_core v0.9.3
    Checking digest v0.10.7
    Checking rand v0.9.4
    Checking rand_xorshift v0.4.0
    Checking sha2 v0.10.9
    Checking regex-automata v0.4.14
    Checking tempfile v3.24.0
    Checking ppv-lite86 v0.2.21
    Checking rusty-fork v0.3.1
    Checking rand_chacha v0.9.0
    Checking proptest v1.9.0
    Checking matchers v0.2.0
   Compiling synstructure v0.13.2
    Checking rustls-webpki v0.103.13
   Compiling zerofrom-derive v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling serde_derive v1.0.228
   Compiling zerovec-derive v0.11.2
   Compiling displaydoc v0.2.6
   Compiling tracing-attributes v0.1.31
   Compiling clap_derive v4.5.49
    Checking tracing v0.1.44
    Checking zerofrom v0.1.8
    Checking yoke v0.8.1
    Checking zerovec v0.11.5
    Checking zerotrie v0.2.3
    Checking cl
```

