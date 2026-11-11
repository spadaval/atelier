---
schema: atelier.item
schema_version: 3
id: atelier-ivbq
title: 'Fix what the new skill and the eval fixtures showed status gets wrong: six small gaps in what a thread is told'
status: ready
mission: atelier-qr5s
labels:
- redesign
priority: medium
blocked_by: []
implementers: []
ready:
  by: sumeet
  at: 2026-11-11T15:43:38.001833021Z
trivial: null
caused_by: null
created_at: 2026-11-11T06:17:49.741506717Z
---

## Goal

Close the gaps in what the new binary tells a thread that writing the skill, the eval fixtures and the eval runs each ran into. None is a new rule: each is `status` or a handoff saying less, or something other, than the specifications' own commands allow, and each sent a fresh agent the wrong way at least once.

## Done when

- [ ] In a Git checkout with no `.atelier/`, `status` says this is not an Atelier repository and exits as it does today for that case, where it now says only that it cannot read the repository's Atelier config. A checkout whose config exists and does not parse still gets the parse error.
- [ ] A thread that holds nothing on an item with an unanswered question is told of the question and of `atelier decide`, before any offer to take the item over. A thread that leads the item's mission is not offered the take-over as its next step on that item's branch.
- [ ] After three rounds without an approval, the line that names `atelier escalate` includes `--finding <name>` when a kept finding is open, and is not shown once a question about it has been asked.
- [ ] `status` shows the whole of a Done when line that wraps over several lines of the record, not its first line.
- [ ] The handoff line a Codex thread reads tells it to start the other thread itself, without waiting to be asked. The line still has no backticks, and is still true under Claude Code.
- [ ] Each line above has a test that fails before the change, and `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` pass, each with `-j 4` where it builds. The product page's `status` example and Handoff lines passage, and the design record's Threads and roles, match what the binary now prints.

## Scope

`crates/atelier-app`, `crates/atelier-cli`, their tests, the passages of the two specification pages named above (wherever atelier-6w8d has put them by then), and this item's record. No new command, flag, record field or rule. Not the eval harness (atelier-dytk).

Left out on purpose, each for whoever plans it: thread identity inside Codex's session-start hook, which needs `status` to read the hook's input and is more than wording; `atelier log` printing a `<parent>/<child>` identity whole, which is for the item that builds `log` (atelier-8b0a); and two things that would be new rules and are Sumeet's to decide: a lead can `start` an item of its own mission, and `check` accepts a status set to `done` by hand because `close` makes the same change.

## Context

Mission atelier-qr5s, step H ("fix what first use breaks"). Where each was seen: the Reports of atelier-orb2, atelier-1oel, atelier-a4rs and atelier-t747, which Git keeps (`git log --all --oneline -- .atelier/issues/atelier-a4rs.md`), and the results file `evals/agent-factory/results/2026-11-11-next-skill.md`.

This host runs a service its owner depends on: one cargo command at a time, with `-j 4`; set `TMPDIR` to a directory under `$HOME` outside any repository, since `/tmp` is a small quota'd tmpfs.

Expected size: about 120 lines of Rust outside tests.
