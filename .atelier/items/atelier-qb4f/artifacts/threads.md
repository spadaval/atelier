# Threads read, atelier, 2026-11-13

Picked by garden.md step 3's order: longest in ready (atelier-s47q), building, in review and changes requested (atelier-wb05), approved (atelier-s47q, already picked), more than one round (atelier-f6dv). Three items, six threads; none more would have been picked. The gardener read atelier-wb05's two threads itself; a bulk reader extracted trips from the other four, and each trip used below was checked against its thread.

Trips, in the gardener's words:

- f6dv reviewer, round 2 (thread ID ending 5db8b0dc-2f1d-4c66-b073-0e0eae48b727), positions 9 and 10: searched `crates/atelier-cli/src/settle.rs`, then `submit.rs`, `status.rs` and `next.rs` in the same directory; none exists. It listed files with `rg --files` and found the code in `plan.rs` and `commands.rs`.
- wb05 reviewer, round 2 (ending 9410a0fe-2682-4c9b-8a63-9f0102ddefac), position 29: searched `crates/atelier-cli/src/commands/show.rs`, the old tool's layout; `show` is in `atelier-app/src/show.rs`, `atelier-cli/src/views.rs` and `view_output.rs`. It read `atelier show` output instead.
- wb05 reviewer, position 6, and s47q reviewer (ending e02619d2-c09e-4beb-b4f3-f8c6c984496b), position 6: `atelier review` refused in the reviewer's own worktree and named the implementer's checkout; each reran it there at once. Cheap, and the refusal said what to do.
- f6dv implementer (ending 932c2504-ac02-42e4-95a0-bbe6ecd5863b), positions 8 to 23: the item capped `garden.md`'s growth at 12 lines; the first edit was 3 over, and six more edits reflowed hard-wrapped paragraphs to reach the count.
- wb05 implementer (ending 5cec947d-b329-4fc8-a2a7-de5ceac8e5db), positions 27 to 61: the item capped `garden.md` and `readiness.md` at 298 lines together; the draft was 243 for `garden.md` alone, and about a dozen edits and recounts brought both to 296.
- wb05 implementer, position 6: read a mission file by a path relative to the wrong directory, then cd'd back. Once; left out.

Fix-now findings in the window, each counted once: atelier-f6dv 1-3oy0.1 (`skills/agent-factory/procedures/garden.md`, carried as 2-s2cm.1), atelier-wb05 1-net6.1 (`skills/agent-factory/procedures/readiness.md`, carried as 2-phww.1). No CI run on master failed in the window.

Calls before the first edit: f6dv implementer 5, s47q implementer 6, both reviewers 4 (bulk reader's count). Files read most before it: `skills/agent-factory/SKILL.md`, `roles/planner.md`, `procedures/groom.md`, `docs/product/desk.md` and `docs/architecture/desk.md`, twice each in the s47q implementer.
