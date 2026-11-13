# Structure read, atelier, 2026-11-13

Crates (non-test `src/` lines): atelier-cli 5,807, atelier-app 5,393, atelier-desk 2,822, atelier-records 2,054, atelier-core 701. Dependencies match `docs/design.md`: cli and desk on app, records and core; app on records and core; records on core.

Where the acts live:

| Act | Crate and file |
| --- | --- |
| `ready` | atelier-app `plan.rs` |
| `decide` | atelier-app `decide.rs` |
| `new`, `set`, `drop`, `close` | atelier-cli `plan.rs` |
| `lead` | atelier-cli `views.rs` |
| `start`, `submit` (and the `status` view) | atelier-cli `commands.rs` |
| `escalate` | atelier-cli `questions.rs` |
| `review`, `verdict` | atelier-cli `review.rs` |
| `merge`, with its merge checks and pushes | atelier-cli `merge.rs` |

Two of thirteen acts are in atelier-app, the two the Desk performs. `docs/design.md`'s crate table said atelier-app owns "each lifecycle act" and "the merge checks"; this run corrected the table to the code.

Same names in two or three crates: `plan.rs` (app, cli), `review.rs` (core, app, cli), `git.rs` (app, cli), `acts.rs` (app, desk). Wording lives in three files: atelier-cli `output.rs` (1,471 lines, the largest non-test file), `view_output.rs` (420) and atelier-app `say.rs` (907).

The work planned next crosses the split. atelier-to83's items ob9u, 2vdq and xbog each scope both `crates/atelier-app` and `crates/atelier-cli`, and 2vdq names `plan.rs` in both. atelier-v958 has the Desk "stop" a mission by writing "the same record `atelier drop` writes", committing and publishing it; `drop` and the push are in atelier-cli, which the Desk does not link.

Six module headers cited seven old-tool files as `crates/...` paths that no longer exist (`atelier-app/src/git.rs`, `atelier-cli/src/forge.rs`, `merge.rs`, `setup.rs`, `atelier-records/src/at_commit.rs`, `ids.rs`); this run marks each as the old tool's, deleted in 819024c75.
