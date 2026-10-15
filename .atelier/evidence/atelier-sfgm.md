---
created_at: "2026-10-15T23:28:55.466054915+00:00"
id: "atelier-sfgm"
evidence_type: "test"
commit: "139bfaa1c024dfbd2671b03141a3ec125dd2ba9a"
result: "pass"
captured_at: "2026-10-15T23:28:28.444536326+00:00"
command: "cargo nextest run --workspace"
exit_status: "0"
agent_identity: "claude-opus-5-5"
target:
  kind: "issue"
  id: "atelier-ha6k"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ha6k"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full workspace nextest suite passes after merging master"
updated_at: "2026-10-15T23:28:55.466797128+00:00"
---

## Summary

Full workspace nextest suite passes after merging master

## Command

```console
cargo nextest run --workspace
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 78391
Truncated: yes

```text
   Compiling proc-macro2 v1.0.104
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.186
   Compiling typenum v1.20.1
   Compiling autocfg v1.5.0
   Compiling stable_deref_trait v1.2.1
   Compiling itoa v1.0.17
   Compiling serde_json v1.0.148
   Compiling zmij v1.0.0
   Compiling memchr v2.7.6
   Compiling iana-time-zone v0.1.64
   Compiling equivalent v1.0.2
   Compiling getrandom v0.3.4
   Compiling hashbrown v0.17.1
   Compiling once_cell v1.21.3
   Compiling anyhow v1.0.102
   Compiling version_check v0.9.5
   Compiling unsafe-libyaml v0.2.11
   Compiling litemap v0.8.2
   Compiling ryu v1.0.23
   Compiling num-traits v0.2.19
   Compiling writeable v0.6.3
   Compiling generic-array v0.14.7
   Compiling icu_normalizer_data v2.1.1
   Compiling icu_properties_data v2.1.2
   Compiling rustix v1.1.4
   Compiling smallvec v1.15.1
   Compiling indexmap v2.14.0
   Compiling bitflags v2.10.0
   Compiling const-oid v0.10.2
   Compiling syn v2.0.111
   Compiling hybrid-array v0.4.13
   Compiling linux-raw-sys v0.12.1
   Compiling cpufeatures v0.3.0
   Compiling utf8_iter v1.0.4
   Compiling write16 v1.0.0
   Compiling fastrand v2.3.0
   Compiling utf16_iter v1.0.5
   Compiling const-oid v0.9.6
   Compiling fs2 v0.4.3
   Compiling toml_write v0.1.2
   Compiling cpufeatures v0.2.17
   Compiling winnow v0.7.15
   Compiling tracing-core v0.1.36
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling utf8parse v0.2.2
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling regex-syntax v0.8.8
   Compiling anstyle-parse v0.2.7
   Compiling is_terminal_polyfill v1.70.2
   Compiling zerocopy v0.8.31
   Compiling anstyle-query v1.1.5
   Compiling digest v0.10.7
   Compiling colorchoice v1.0.4
   Compiling anstyle v1.0.13
   Compiling rand_core v0.9.3
   Compiling clap_lex v0.7.6
   Compiling sha1 v0.10.7
   Compiling anstream v0.6.21
   Compiling strsim v0.11.1
   Compiling lazy_static v1.5.0
   Compiling sha1-checked v0.10.0
   Compiling pin-project-lite v0.2.17
   Compiling heck v0.5.0
   Compiling log v0.4.29
   Compiling clap_builder v4.5.53
   Compiling tracing-log v0.2.0
   Compiling sharded-slab v0.1.7
   Compiling digest v0.11.3
   Compiling thread_local v1.1.9
   Compiling sha2 v0.11.0
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/ha6k/crates/atelier-cli)
   Compiling nu-ansi-term v0.50.3
   Compiling wait-timeout v0.2.1
   Compiling dotenvy v0.15.7
   Compiling quick-error v1.2.3
   Compiling bit-vec v0.8.0
   Compiling tempfile v3.27.0
   Compiling fnv v1.0.7
   Compiling bit-set v0.8.0
   Compiling rand_xorshift v0.4.0
   Compiling rusty-fork v0.3.1
   Compiling rand v0.9.4
   Compiling unarray v0.1.4
   Compiling arbitrary v1.4.2
   Compiling regex-automata v0.4.14
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.9.0
   Compiling proptest v1.9.0
   Compiling synstructure v0.13.2
   Compiling serde_derive v1.0.228
   Compiling zerofrom-derive v0.1.6
   Compiling yoke-derive v0.8.1
   Compiling zerovec-derive v0.11.2
   Compiling displaydoc v0.2.6
   Compiling tracing-attributes v0.1.31
   Compiling clap_derive v4.5.49
   Compiling matchers v0.2.0
   Compiling zerofrom v0.1.8
   Compiling yoke v0.8.1
   Compiling tracing v0.1.44
   Compiling zerovec v0.11.5
   Compiling zerotrie v0.2.3
   Compiling clap v4.5.53
   Compiling tinystr v0.8.2
   Compiling potential_utf v0.1.4
   Compiling icu_collections v2.1.1
   Compiling icu_locale_core v2.1.1
   Compiling icu_provider v2.1.1
   Compiling icu_normalizer v2.1.1
   Compiling icu_properties v2.1.2
   Compiling chrono v0.4.45
   Compiling serde_yaml v0.9.34+deprecated
   Compiling toml_datetime v0.6.11
   Compiling serde_spanned v0.6.9
   Compiling tracing-serde v0.2.0
   Compiling tracing-subscriber v0.3.23
   Compiling toml_edit v0.22.27
   Compiling atelier-core v0.2.0 (/home/supadava/projects/atelier-wt/ha6k/crates/atelier-core)
   Compiling atelier-workflow v0.2.0 (/home/sup
```

