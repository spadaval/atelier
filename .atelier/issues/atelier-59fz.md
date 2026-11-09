---
created_at: "2026-11-09T06:41:35.705830108+00:00"
id: "atelier-59fz"
issue_type: "task"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: t3/remove-cli-html-briefs
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6rts"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T20:37:44.243417527+00:00"
status: "done"
title: "Remove --html and .atelier/runtime/reviews, and send humans to the site's brief and debrief"
updated_at: "2026-11-09T20:37:44.243417527+00:00"
---

## Goal

Make the site the one place a human reads a brief or debrief: remove the CLI's `--html` output, its Rust HTML renderers and `.atelier/runtime/reviews/`, and point Agent Factory and the docs at the site's pages, so agents stop writing HTML files or publishing ad-hoc artifacts. The Markdown `atelier brief` and `atelier debrief` stay for agents.

## Done when

- [x] `atelier brief` and `atelier debrief` no longer accept `--html`, nothing in the CLI writes `.atelier/runtime/reviews/`, `crates/atelier-cli/src/review_page.rs`, `render_html` and `render_brief_html` are deleted with their tests, and their footers no longer suggest `--html`; when the site's address is configured they print the mission's page URL. CLI tests show `--html` refused on both commands with no `.atelier/runtime/reviews/` created, and the URL printed when configured.
- [x] `skills/agent-factory/procedures/plan.md` and `orchestrate.md` tell agents to send the human the mission's site page, and `PRODUCT_INTENT.md`, `docs/product/mission-reviews.md` and `docs/product/human-cli-output.md` describe the site instead of `--html`; `git grep -n -E -- "--html|runtime/reviews"` over `skills/`, `docs/` (historical strategies aside), `PRODUCT_INTENT.md`, `CONTEXT.md`, `README.md` and `crates/*/src/` finds only code that refuses the flag, and tests under `crates/*/tests/` may keep naming it to prove the refusal.
- [x] every open item whose Done when names the HTML debrief or brief (atelier-wg6p, atelier-les9, atelier-rdsa, atelier-okfy, atelier-ztuq, atelier-k7vp and any other the search finds) names the site's page instead, with its wording otherwise unchanged; the Report lists them.
- [x] `scripts/smoke-repos.sh` passes, and its Report names any repository whose own docs still call `--html`.

## Context

Can start only once loose end atelier-6rts has the site installed and published, so Sumeet never loses a way to read a brief or debrief. Sumeet's 2026-11-08 direction on atelier-yfqu: "the CLI's --html output and .atelier/runtime/reviews/ go away, while Markdown brief and debrief stay for agents."

## Unknowns

- How the CLI learns the site's address (an environment variable or host config); settle it at the start and record a decision note.

## Notes

- 2026-11-09 [decision] The CLI learns the site's address from one user-level file: the first line of site-url under $XDG_CONFIG_HOME/atelier, else ~/.config/atelier. brief and debrief then end with 'Send the human the page: <site>/<project_slug>/mission/<id>/brief' (or /debrief); with no file they name no page. Known gap: the CLI names the repository by its own project_slug, so a repository that shares its slug with another, which the site serves as <slug>-<n>, gets the other one's URL.
  Reason: Every atelier run on the host reads the file, from any repository or harness, with no environment to pass through T3 Code, Codex or hooks and no service restart. An ATELIER_SITE_URL variable would have to be set in the T3 Code service, the shell profiles and every harness separately.

## Report

`atelier brief` and `atelier debrief` print Markdown only: `--html` is refused by clap, `.atelier/runtime/reviews/` is never written, and `review_page.rs`, `render_html`, `render_brief_html` and their HTML helpers are gone (about 1,700 lines). When `~/.config/atelier/site-url` (or `$XDG_CONFIG_HOME/atelier/site-url`) holds the site's address, both footers end with `Send the human the page: <site>/<project_slug>/mission/<id>/brief` (or `/debrief`) instead of the old "Write the HTML page" step. `brief::brief_and_debrief_name_the_site_page_instead_of_writing_html` proves the refusal on both commands, that no runtime/reviews directory appears, no URL without the file, and the URL with it; the whole `atelier-cli` suite passes (884 tests, run with this host's real `site-url` present, so a configured host breaks no footer assertion). On this host both commands print `http://buildbot.tail47515a.ts.net:7341/atelier/mission/<id>/...`, which the live site answers with 200. The skill's `plan.md` and `orchestrate.md` send the human the footer's page; PRODUCT_INTENT.md, mission-reviews.md, human-cli-output.md, cli-surface.md, docs/product/index.md and ADR 0030's status lines describe the site; `docs/architecture/site.md` gains "Naming a page from the CLI". The Done when grep finds nothing outside `docs/strategies/`. `scripts/smoke-repos.sh` passed across all 5 repositories.

### Decisions

The site address lives in a user-level file, not an environment variable (decision note above). It is set on this host to `http://buildbot.tail47515a.ts.net:7341`.

Done when lines rewritten (HTML wording only): atelier-wg6p, atelier-les9, atelier-k7vp, atelier-okfy, atelier-ztuq ("both the Markdown debrief and the site's debrief page"), atelier-rdsa ("on the site's debrief page") and atelier-jdpf, which the search also found ("Markdown and the site's page").

### Deviations

HTML-only test assertions were deleted with the renderer. Where a test checked a view fact only through the HTML and the Markdown shows it too, the test now checks the Markdown (planned-work count, whole review summary, raw record text); tests of HTML structure alone (chips, cards, sidebar, escaping) went.

### Open items

- No other repository's docs or workflow call `--html`. Only atelier's own historical evidence receipts (atelier-1dpt, atelier-9mk4, atelier-9r9q) name it, as records of past runs.
- The CLI names a repository by its own `project_slug`; one whose slug repeats another's on the host, which the site serves as `<slug>-<n>`, gets the other's URL. None does today.
