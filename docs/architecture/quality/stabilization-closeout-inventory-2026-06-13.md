# Stabilization Closeout Inventory - 2026-06-13

Mission: `atelier-man9`  
Validation issue: `atelier-ngat`

## Purpose

This inventory classifies the current stabilization state before final mission
closeout. It is a map for closeout auditors, not a repair patch: failed or
partial classifications name owner issues that must close or explicitly defer
the work before `atelier-man9` can close.

## Command Surface Inventory

Observed transcript: `target/debug/atelier --help`.

| Area | Visible surface | Classification | Owner |
| --- | --- | --- | --- |
| Setup | `init` | Pass. Single setup entrypoint. | Closed setup work. |
| Orientation | `prime`, `status`, `start`, `abandon` | Partial. Root help is compact, but work selection guidance is still under active issue ownership. | `atelier-i9ob`, `atelier-rgd1` |
| Issues | `issue`, `dep`, `search`, `link`, `graph`, `note` | Partial. Visible groups are coherent, but status/category alias behavior remains open. | `atelier-5a73`, `atelier-rgd1` |
| Missions/plans | `mission`, `plan` | Pass for visible grouping. Mission closeout still has open audit and proof gates. | `atelier-3iom`, `atelier-bk6n` |
| Evidence | `evidence record/show/attach/list` | Pass for normal help: predecessor `add` and `capture` are hidden from help. | Closed `atelier-u08r`, `atelier-of3h`; typed domain follow-up `atelier-ihz0` remains. |
| Worktrees | `worktree` | Pass as advanced worktree management. | Closed runtime/worktree items; complexity follow-ups remain. |
| State | `export`, `rebuild`, `import-beads` | Partial. State health is good, but predecessor import residue remains intentionally scoped to migration. | `atelier-10qm`, `atelier-d7lw` |
| Integrations | `integrations claude install` | Historical classification only. Superseded by `atelier-vau5`, which removes the integration command surface and bundled Claude hook/MCP assets from the target product. | `atelier-vau5` |
| Maintenance/diagnostics | `maintenance`, `diagnostics`, `lint`, `doctor` | Pass for explicit advanced/health surfaces. | `atelier-c9ej` for freshness checks. |

Observed transcript: `target/debug/atelier evidence --help`.

| Evidence command | Classification |
| --- | --- |
| `record` | Pass. Unified manual and command-transcript proof capture. |
| `show` | Pass. Reads typed evidence fields. |
| `attach` | Pass. Explicit proof linkage remains visible. |
| `list` | Pass. Inventory command. |
| `add`, `capture` | Pass for help: not visible. Residue remains in hidden implementation and tests only. |

## Canonical Record Inventory

| Record kind | Current classification | Evidence or owner |
| --- | --- | --- |
| Issue | Partial. Sectioned issue Markdown is readable, but issue body/description duplication remains open. | `atelier-nqp4` |
| Mission | Pass for readable sections and no escaped mission `data:` front matter. | Closed mission-record work; `rg '^data:' .atelier/missions -g '*.md'`. |
| Evidence | Pass for committed evidence records: typed front matter is rendered and `rg '^data:' .atelier/evidence -g '*.md'` returns no matches. | `atelier-of3h`, evidence `atelier-3b7u`. |
| Plan | Pass for typed command and RecordStore plumbing; plan projection summaries hydrate canonical Markdown for detail views. | `atelier-ihz0` follow-up refactor. |
| Activity sidecars | Partial. Canonical sidecars exist, but ownership and retention contract remain open. | `atelier-k3vs` |
| Runtime/cache | Pass after local repair: runtime database is `.atelier/runtime/state.db`; `.atelier/state.db` is ignored as rebuildable local residue. | `atelier-fyrm`; this audit also restores the ignore rule. |

## Module Boundary And Complexity Inventory

| Area | Classification | Owner |
| --- | --- | --- |
| `src/record_store.rs` | Fail/assigned. It still owns parsing, rendering, validation, migration helpers, and tests for several record kinds. | `atelier-2ehd` |
| Generic payload plumbing | Pass for current typed record paths; first-class records use typed structures rather than generic internal payload shuttles. | `atelier-ihz0` follow-up refactor. |
| Main command routing and oversized handlers | Fail/assigned. Root dispatch and some handlers remain broad. | `atelier-d7lw` |
| Workflow policy parser/validator | Fail/assigned. Parser and validation hotspots were triaged and need split work. | `atelier-wj05` |
| Runtime database schema/projection migration | Fail/assigned. Runtime path is correct, but schema/migration hotspots remain. | `atelier-ggls` |
| Integration test fixtures | Fail/assigned. RecordStore-aware integration fixture duplication remains. | `atelier-kpm8` |
| Issue workflow orientation in Agent Factory commands | Fail/assigned. Orientation logic still needs extraction. | `atelier-4u5h` |
| Dead inherited paths | Fail/assigned. Dead-code inventory and removal remain open. | `atelier-10qm` |

## Residue Searches

Command used:

```bash
rg -n "work start|work status|workflow validate|evidence add|evidence capture|beads|chainlink|Claude|state\\.db|data:" AGENTS.md CONTEXT.md PRODUCT_INTENT.md docs src tests .atelier -g '!runtime/**' -g '!cache/**' -g '!*.db'
```

Classification:

| Residue | Classification | Owner or rationale |
| --- | --- | --- |
| `workflow validate` | Mostly historical/diagnostic/test residue. Normal help no longer exposes it as closeout guidance. | Closed `atelier-im60`; freshness guard owner `atelier-c9ej`. |
| `work start` / `work status` | Mixed historical issue activity, product-doc removal tables, and tests. Normal help uses root `start`; `work` root is not visible. | `atelier-rgd1`, `atelier-10qm` for cleanup inventory. |
| `evidence add` / `evidence capture` | Product docs classify removal; hidden implementation strings remain. Normal evidence help exposes `record`. | `atelier-10qm` or explicit hidden-command removal follow-up if required. |
| `data:` | Canonical mission/evidence records are clean; tests and docs retain legacy fixtures/examples. Plan/generic record plumbing remains. | `atelier-ihz0`; `docs/product/work-model.md` stale example should be handled there or by docs freshness work. |
| `state.db` | `.atelier/runtime/state.db` is the runtime path. Root `.atelier/state.db` is local rebuildable residue and is ignored by this audit patch. Some docs still mention legacy root state in compatibility/history contexts. | `atelier-fyrm` closed; stale docs can be swept by `atelier-c9ej` if not intentionally historical. |
| `beads` / `chainlink` | `import-beads` is the explicit predecessor import bridge; `PRODUCT_INTENT.md` and context mention repository provenance. | `atelier-10qm` for inherited residue cleanup where not product intent. |
| `Claude` | Historical residue from the previous optional integration surface. Superseded by `atelier-vau5`, which removes Claude hook/MCP assets and keeps only explicitly historical mentions. | `atelier-vau5` |

## Current Open Owner Map

Observed transcript: `target/debug/atelier work queue --ready` after this audit
started.

| Epic | Remaining ready owner issues |
| --- | --- |
| `atelier-lpnr` | `atelier-nqp4` |
| `atelier-p2m2` | `atelier-i9ob`, `atelier-rgd1`; `atelier-5a73` is claimed/in progress by concurrent work |
| `atelier-yqg9` | `atelier-10qm`, `atelier-2ehd`, `atelier-ihz0`, `atelier-4u5h`, `atelier-d7lw`, `atelier-ggls`, `atelier-k3vs`, `atelier-kpm8`, `atelier-wj05` |
| `atelier-cve1` | `atelier-c9ej`, `atelier-gzel`, `atelier-4ykl` |
| `atelier-foy0` | `atelier-3iom`; `atelier-bk6n` remains blocked by this audit until `atelier-ngat` closes |

## Health Transcript

Commands run after removing local root `state.db` residue and rebuilding:

```bash
target/debug/atelier rebuild
target/debug/atelier lint
target/debug/atelier export --check
target/debug/atelier doctor
```

Observed results:

| Command | Result |
| --- | --- |
| `rebuild` | Pass. Rebuilt `/root/atelier/.atelier/runtime/state.db` from `/root/atelier/.atelier`. |
| `lint` | Pass. |
| `export --check` | Pass. Canonical export is current. |
| `doctor` | Pass. Config, ignored runtime paths, rebuild readiness, projection freshness, runtime tables, and legacy health all report ok. |

## Closeout Guidance

`atelier-ngat` can close when this artifact is committed and attached as proof.
It does not make `atelier-man9` closeable by itself. Mission closeout remains
blocked until owner issues above either close with evidence or record explicit
deferrals that the mission audit accepts.
