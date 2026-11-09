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

Build what atelier-ah82 documents: a GitHub forge provider that shells out to `gh`, and a typed `ci.passed` merge check that passes when the trusted CI workflow succeeded on a pull request run for a commit whose code matches the branch head. Switch this repository's policy from `scripts/merge-ci-passed.sh` to it, so once this merges a base that gained only docs, strategies or records since the CI run no longer needs a sync and a new run. Keep the script: this branch merges under master's committed policy, which still runs it, and atelier-7o4r deletes it once master's policy uses `ci.passed`.

## Done when

- [ ] Integration tests with a fake `gh`: `ci.passed` passes when CI succeeded on the branch head, or on an earlier branch commit with the same code; refuses with the run's state and link when it is pending or failed, and when no run exists; and refuses naming `gh auth login` when `gh` is missing or signed out.
- [ ] A test where the base gained a docs-only commit after the branch's CI run: `ci.passed` passes without a new run, and the merge's local `shell.check`s still run on the merged tree.
- [ ] A test where the base gained a code commit after the run: `ci.passed` still passes, and the merge output says CI on the base after the push covers the combination, as atelier-ah82's docs say.
- [ ] A policy with no forge parses and merges exactly as before (existing merge tests pass unchanged), and `atelier check` refuses a policy whose forge or CI workflow key is malformed, naming the key.
- [ ] `.atelier/workflow.yaml` uses `ci.passed` in place of the script's `shell.check`, and the script stays. An integration test shows the switch: with a base policy that runs a CI script check and a branch whose policy switches to `ci.passed`, the branch's merge runs the base's check, and the next branch's merge onto the new base runs `ci.passed`.

## Scope

The provider, `ci.passed`, policy parsing and this repository's policy. Not pushing branches or the base, pull requests, or `--wait` (atelier-fad5 and the next item).

## Context

The script's logic (code paths, matching an earlier commit's code, the record-only shortcut) is in `scripts/merge-ci-passed.sh`; keep its record-only behaviour. Merge checks run from the base's committed policy, so this branch merges under the script's check, which stays. The installed binary cannot read the branch's `ci.passed` policy and refuses the merge with `workflow_config_invalid_validator` (reproduced in planning review finding atelier-8wdb), so run this item's merge with the reviewed branch build: `cargo install --path crates/atelier-cli --locked` from the approved head, or `target/debug/atelier merge <id>` built from it, then `atelier land` with that binary. The base policy and the script are unchanged by doing so.

## Unknowns

- Whether `gh run list --commit` or `gh api` gives stable enough output; the strategy's Risks section says what to do if not.
