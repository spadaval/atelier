---
schema: atelier.item
schema_version: 3
id: atelier-xz5c
title: Make CLI output use check, close and done as CONTEXT.md defines them
status: dropped
mission: null
labels:
- size-m
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T12:14:42.350638924Z
---

## Goal

CONTEXT.md now gives check, review, land, done and close one meaning each (atelier-ml3r), and the CLI help and docs follow it, but command output still uses two of the words otherwise. Verdict and gate lines count gates as checks (`land: work in progress — 3 checks open (…)`, `merge: refused — 2 checks (…)`, `✓ 3 checks pass (…)`), the output contract in docs/product/human-cli-output.md calls terminal work `closed` (`closed ancestor <id> retires open descendant <id>`, `| closed <date>`), evidence freshness reads `held at close <commit>`, the `review_request_unfinished` refusal says asking for review means the work is done, though work is done only when it merges, and `issue transition`'s next steps label `atelier land` "Check handoff". atelier-ml3r kept output out of scope, since hooks and tests read these lines.

## Done when

- [ ] Verdict and gate lines name what they count without the word check, and the Stop hook, `--quiet` tokens and every test that pins them follow.
- [ ] Output calls terminal work done, not closed, except for a mission, which closes; the evidence freshness mark says what it means without close; the `review_request_unfinished` refusal says the work is implemented and every line holds.
- [ ] The review brief's approve template (crates/atelier-cli/src/commands/review_request.rs and light_path.rs) asks for "<what you verified>", not "<what you checked>" (finding atelier-pkfq).
- [ ] docs/product/human-cli-output.md, sessions-and-hooks.md and cli-surface.md quote the new lines, and the note in CONTEXT.md's Gate row goes.

## Context

CONTEXT.md (Check, Gate, Done, Close); docs/product/human-cli-output.md (verdict line, wording contract); crates/atelier-cli/src/commands/session.rs, merge.rs, merge_sync.rs and gate_report.rs print the verdicts.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-ml3r (Say the lifecycle once, plainly, in the skill and Atelier's own docs)

## Report

Dropped: `CONTEXT.md`, `land` and the gate lines it names were deleted in 0.5.
