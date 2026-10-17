---
schema: atelier.review
schema_version: 1
id: atelier-ry08
title: crates/atelier-app/src/use_cases.rs:30-35 — a malformed --target ('evidence record --target bad ...') still exits 1 with '--target must use kind/id syntax', while the edited Errors rule says an invalid argument value is a usage error that exits 2 wherever it is checked. Arguably a syntax error rather than a value outside a set, so the doc's scope is ambiguous; either word it as a usage error (exit 2) or narrow the doc to set-membership values. Pre-existing behavior, not worsened.
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:31.800602933Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/use_cases.rs:30-35 — a malformed --target ('evidence record --target bad ...') still exits 1 with '--target must use kind/id syntax', while the edited Errors rule says an invalid argument value is a usage error that exits 2 wherever it is checked. Arguably a syntax error rather than a value outside a set, so the doc's scope is ambiguous; either word it as a usage error (exit 2) or narrow the doc to set-membership values. Pre-existing behavior, not worsened.
