---
created_at: "2026-11-09T02:02:16.894255456+00:00"
id: "atelier-7tul"
issue_type: "feature"
labels:
- "forge"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1gb3"
    type: "advances"
  - kind: "issue"
    id: "atelier-ah82"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Add the GitHub forge and a ci.passed merge check, so a docs-only base change no longer forces a CI rerun"
updated_at: "2026-11-09T02:02:16.894255456+00:00"
---

## Goal

Build what atelier-ah82 documents: a GitHub forge provider that shells out to `gh`, and a typed `ci.passed` merge check that passes when the trusted CI workflow succeeded on a commit whose code matches the branch head. Switch this repository's policy from `scripts/merge-ci-passed.sh` to it and delete the script, so a base that gained only docs, strategies or records since the CI run no longer needs a sync and a new run.

## Done when

- [ ] Integration tests with a fake `gh`: `ci.passed` passes when CI succeeded on the branch head, or on an earlier branch commit with the same code; refuses with the run's state and link when it is pending or failed, and when no run exists; and refuses naming `gh auth login` when `gh` is missing or signed out.
- [ ] A test where the base gained a docs-only commit after the branch's CI run: `ci.passed` passes without a new run, and the merge's local `shell.check`s still run on the merged tree.
- [ ] A test where the base gained a code commit after the run: `ci.passed` still passes, and the merge output says CI on the base after the push covers the combination, as atelier-ah82's docs say.
- [ ] A policy with no forge parses and merges exactly as before (existing merge tests pass unchanged), and `atelier check` refuses a policy whose forge or CI workflow key is malformed, naming the key.
- [ ] `.atelier/workflow.yaml` uses `ci.passed` and `scripts/merge-ci-passed.sh` is deleted; this item merges through the new check, with the Report linking the run it trusted.

## Scope

The provider, `ci.passed`, policy parsing and this repository's policy. Not pushing branches or the base, pull requests, or `--wait` (atelier-fad5 and the next item).

## Context

The script's logic (code paths, matching an earlier commit's code, the record-only shortcut) is in `scripts/merge-ci-passed.sh`; keep its record-only behaviour. Merge checks run from the base's committed policy, so the installed binary must read the new keys before this merges: install the branch build first (`cargo install --path crates/atelier-cli --locked`, as `AGENTS.md` says).

## Unknowns

- Whether `gh run list --commit` or `gh api` gives stable enough output; the strategy's Risks section says what to do if not.
