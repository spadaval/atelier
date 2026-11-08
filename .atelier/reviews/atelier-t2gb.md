---
schema: atelier.review
schema_version: 2
id: atelier-t2gb
title: approval on atelier-pmib @873d0b86
target: atelier-pmib
identity: reviewer-pmib
commit: 873d0b86548491b69e2b4cb1e76db9b4ca3d76a1
created_at: 2026-11-08T06:05:07.647475006Z
event:
  event: approval
---

Checked ADR 0029 against every Done when line, the mission strategy docs/strategies/agent-factory-in-atelier.md, and ADRs 0020, 0022 and 0023. It records all six strategy outcomes that belong in an ADR. It supersedes 0023 in full and keeps the one-copy rule. It retires only Decision 3's sync rule and keeps the concepts/syntax boundary, which resolves atelier-uplq. It replaces Decision 1's plan-file fallback and lists all three required alternatives plus vendoring. The Status sections of 0020 and 0023 name 0029, and docs/index.md cites it. Claims checked: the skill in agents-skills names Atelier 63 times (more than fifty), and 0029 exists only on this branch. git diff --check, cargo fmt --check and cargo metadata --locked all pass. merge-ci-passed.sh exits 2 because HEAD is not a merge commit and no CI has run; that is expected before a PR.
