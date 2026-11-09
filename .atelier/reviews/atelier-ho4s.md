---
schema: atelier.review
schema_version: 2
id: atelier-ho4s
title: approval on atelier-uma8 @00b22558
target: atelier-uma8
identity: gpt-6.1-sol-r1
commit: 00b225587418ad5121f44b292cc628469164bc67
created_at: 2026-11-09T21:16:16.438208772Z
event:
  event: approval
---

Verdict: merge. The inbox delivers the required host-wide human queue and links directly to the existing review pages without adding write actions or a new source of state.

Sizes: complexity medium (one app projection, fleet endpoint and React page); value high (a required mission outcome); risk medium (a wrong queue reaches the human, but is read-only, visible and reversible).

Contract: pass. Reviewed 5d6af8d3..00b22558 and its approved requirements. The inbox selects exactly status_view(index, Some(policy)).work.human, which status.rs counts as Needs a human. It groups entries by repository from the captured base commit; drafts require their workflow's initial status and an existing approval event, as the settled Unknown permits. Mission links union mission scope, typed relations and both blocker directions, deduplicate and exclude superseded/declined missions. Done missions still link their debrief. The fixture's hand-written expectations cover the decision, blocked_by loose end, superseded-only validation, approved draft and excluded unapproved draft. Home and boards link Inbox with the fleet count; home remains the repository list. The Report records the host command and counts.

Fitness: pass. atelier-app owns selection and mission membership; atelier-site caches the projection with base records and adds existing summaries/links; React renders the API's review decision. Generated types and fixture JSON match the implementation. No unnecessary new state, link role or write path.

Findings: none. No fix_now recommendation. Transient Git failure and injected-fault hardening remain outside this review under atelier-01wq and the human's explicit scope.

Proof run fresh at 00b225587418ad5121f44b292cc628469164bc67:
- git diff --check HEAD^ HEAD -- . ':!.atelier', cargo fmt -- --check, and cargo metadata --locked --format-version 1 > /dev/null: pass. Also checked whitespace across the entire review range.
- cargo nextest run -j 4 -p atelier-app -p atelier-site -p atelier-cli --test serve --lib: 246 passed, 1 ignored fixture-writer skipped. Includes the inbox rules, fixture comparison with both CLI repositories, current fixture JSON, generated types, and read-only routes.
- In crates/atelier-site/web: pnpm typecheck, pnpm test (38 passed), pnpm build: pass.
- cargo build -j 4 -p atelier-cli, then pnpm e2e --workers=2: 36 passed, including the fixture inbox click-through.
- Served the reviewed debug binary over ~/projects at 127.0.0.1:17351, then ATELIER_SERVE=http://127.0.0.1:17351 pnpm exec playwright test -c playwright.host.config.ts: 2 passed. Inbox: 5 repositories, 50 items, 5 draft missions, 61 pages, 0 failed. Missions: 42 missions, 84 pages, 0 failed.
- Additional Playwright check loaded /inbox, its five repository-board links and home: no console errors, page errors or alerts; home and every board showed an Inbox count.
All Cargo builds used four jobs, CARGO_TARGET_DIR=$HOME/.cache/uma8-review-target, and TMPDIR=$HOME/.cache/uma8-review-tmp. Full proof output was read. No release build or systemd service change.

Open questions: none.

Residual risk and tests not run: actual publication, installed-service restart and phone/Tailscale validation remain atelier-mtb3's work. The cross-repository CLI smoke script was not run; no CLI command implementation or record format changed. Host counts reflect the current base refs and may change with records.

Changes and review: independent review by gpt-6.1-sol-r1; only this approval record is written and committed on t3/ce3458cf. No code edits, branch changes, merge or push.
