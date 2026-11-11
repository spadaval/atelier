---
schema: atelier.review
schema_version: 2
id: atelier-83nc
title: finding on atelier-z4cr @442c0e76
target: atelier-z4cr
identity: GPT-6.1 Sol independent reviewer
commit: 442c0e7686dc47a4b92e15e3edc918e27fec40be
created_at: 2026-11-11T03:26:27.133737828Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/strategies/desk.md:42-46: Make act-is-a-record deterministic. The criterion requires identical commit trees from two independent ready executions on clones of the same commit, but next/crates/atelier-cli/src/plan.rs:222-249 persists ready.at from chrono::Utc::now(); those record bytes and therefore trees differ even when both callers behave correctly. atelier-psjw changes committing and caller access, not that timestamp contract. Require one controlled approval timestamp for both test executions, or explicitly normalize ready.at while separately asserting it is valid, then compare every other tree entry, the commit message, the single-commit count and the ready page state. This is a proof-contract correction, not a request to change production timestamp semantics.
Verdict: merge after this acceptance-proof correction; the amendment serves the fixed direction to share core act execution.
Sizes: complexity medium for this amendment (core/CLI/Desk contract), value high (required human direction), risk medium (acts commit real records); the whole Desk mission remains high complexity.
Contract: fail only for the unqualified tree-equality criterion. Fitness: otherwise pass. The reviewed plan matches ready/save_with/commit_line, decide and its branch checkout rule, Repo::open and caller identity; acts.rs currently supplies policy and refusal checks rather than these executing use cases.
atelier-psjw is sound preliminary backlog work: typed core execution, CLI reuse, isolated commits, named caller and specification changes have falsifiable provisional Done when lines. Its crate boundary and committing policy are settled, and direct base-ref writes are explicitly excluded. Its owner/timing decision is route-only at the later boundary; atelier-1ie1 can map proposed interfaces without those functions existing. No new cutover dependency is recorded; Authority prohibits Desk next/ edits during u1ko or wd1i, and acts wait on psjw while read work proceeds. The other scoped plan records and the two documents on desk-acts-adr consistently require core calls rather than binary/output parsing.
Open questions: coordinate who lands psjw before the switch or after the host cutover, as already recorded; this does not block the first increment. Proof: static review of e4ae1d74, plan.md and the named next/ sources and scoped mission/items; git diff --check e4ae1d74^ e4ae1d74 passed. Residual risk: future Desk/act behavior is not implemented or executed by this planning review. No plan, source or product document was edited.
