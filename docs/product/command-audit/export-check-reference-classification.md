# `export --check` Reference Classification

This audit classifies remaining `export --check`, `atelier export`, and
export/rebuild references after routine Agent Factory proof guidance moved to
claim-specific evidence, `lint`, `doctor`, focused tests, transcripts, and
evidence records.

Search command:

```bash
rg -n 'export --check|atelier export|export/rebuild|lint/export|export check' \
  AGENTFACTORY.md docs/architecture/quality/validation.md \
  docs/product/command-audit docs/product/validation.md \
  /root/.agents/skills/agent-factory
```

## Allowed References

| Reference area | Classification | Reason |
| --- | --- | --- |
| `docs/product/command-audit/export.md` | Migration/debug | The file audits the export command itself and states that normal operators use `lint` and `doctor`. |
| `docs/product/command-audit/category-review.md` | Migration/debug | The category table explicitly keeps export/rebuild outside normal workflow and classifies `export --check` as hidden/advanced. |
| `docs/architecture/quality/validation.md` deterministic export/projection diagnostic row | Storage-rendering-specific | The validation router names `export --check` only for storage-rendering, migration, or debug claims. |
| `/root/.agents/skills/agent-factory/procedures/install.md` storage-rendering diagnostics list | Migration/debug | Install guidance keeps export commands out of normal tracker and health commands. |
| `/root/.agents/skills/agent-factory/standards/tracker.md` mission closeout caveat | Storage-rendering-specific | The tracker standard says to use `export --check` only when the mission changes deterministic export, projection freshness, or migration/debug surfaces. |

## Historical Or Deferred References

| Reference area | Classification | Reason |
| --- | --- | --- |
| `.atelier/issues/*.md` and `.atelier/issues/*.activity/*.md` | Historical | Closed or pre-existing tracker records preserve what the work claimed at the time. They should not be rewritten solely for search hygiene. |
| `docs/spec/storage/export/rebuild/*` | Storage-rendering-specific | These specs define the deterministic renderer and rebuild contract. |
| `docs/spec/agent-factory/tracker-replacement-mvp.md` | Historical | MVP parity text records the previous command contract and migration path. |
| Other open tracker issues not owned by `atelier-jezn` | Follow-up | They should be updated by their owning issue when the changed command surface affects the issue's actual proof. |

New Agent Factory and command-surface issue examples should not add routine
`export --check` proof by default. Use explicit export diagnostics only when an
issue changes deterministic rendering, projection freshness, migration, or
debug behavior.
