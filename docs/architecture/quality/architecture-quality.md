# Architecture Quality Vocabulary

**Abstraction:** protect real invariants: valid records, graph integrity,
dependency closure, traceable active work, freshness, review, and safe cleanup.
Each abstraction should serve a current need.

**Coupling:** connect CLI, application queries, workflow checks, and record
persistence through typed values. Do not parse human command output as state.
Git owns integration facts; Markdown owns durable tracker facts.

**Cohesion:** keep parsing/rendering in the records crate, graph queries in the
application layer, and CLI arguments and external-command adapters at the
edge. A read-only invariant check should not branch, commit, or merge.

**Determinism:** make rendering, iteration order, identifiers, timestamps, and
bundle preview ordering explicit where they affect reproducibility or diffs.

**Operability:** errors identify the record, gate, policy, or path that failed
and give a concrete recovery step. Commands expose focused current context.

**Proof:** claim-specific evidence names its method, explicit result, full
commit, artifacts, and limits. Fresh independent review establishes approval;
a broad green test suite or a receipt's existence does not replace it.

**Preservation:** preserve current documented behavior through focused tests.
Remove retired subsystems and compatibility shims when the governing migration
or ADR permits it. Historical behavior does not define the current contract.
