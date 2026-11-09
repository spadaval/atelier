---
created_at: "2026-11-09T06:41:09.791217420+00:00"
id: "atelier-c5vk"
issue_type: "decision"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T06:41:17.067334663+00:00"
status: "done"
title: "Build the site as crate atelier-site in the atelier binary, on axum, with the existing HTML helpers"
updated_at: "2026-11-09T06:52:38.191978430+00:00"
---

## Context

`docs/strategies/fleet-dashboard.md` Design; decision atelier-7bs1; ADR 0030 (atelier-tu8c).

## Question

Where does the site's code live, how does it ship, and which HTTP, HTML and Markdown libraries does it use? Sumeet's 2026-11-08 direction on atelier-yfqu left this open, with a proposal from Claude: a new crate `crates/atelier-site` shipped in the `atelier` binary as `atelier serve`, in Rust with axum and maud.

## Options

- A separate `atelier-site` binary or repository. Keeps HTTP dependencies out of the CLI, but a second install can disagree with the CLI about the record format, and one installed binary already serves every repository on this host.
- A crate in this workspace shipped in the `atelier` binary, on `tiny_http`, a small synchronous server (about five dependencies) that fits a codebase which shells out to Git synchronously. Its last release was in 2022, and the concurrency cap, timeouts and 503 handling would be hand-written.
- A crate in this workspace shipped in the `atelier` binary, on `axum` with `tokio` and `tower`. Maintained and familiar to agents; `tower` layers give the concurrency cap and timeout; costs about sixty crates of compile time and an async runtime whose handlers all call blocking code through `spawn_blocking`.
- For HTML: `maud` templates, which escape at compile time but mean rewriting the existing hand-written brief and debrief renderers (about 1,500 lines) or keeping two styles; or the existing helpers (`review_page.rs`) for every page, escaping by one `escape` function, checked by a test that renders hostile record text on every route.

## Decision

A new library crate `crates/atelier-site`, called by `atelier serve` in `atelier-cli`, so the site ships in the one `atelier` binary and always reads records the way the CLI does. HTTP on `axum`, `tokio` and `tower`: the cap and timeout come from maintained layers rather than our code, and later work (live thread status from atelier-bnek, write actions under a later decision) fits it; the compile-time cost is accepted. HTML from the existing helpers moved into the site crate, not `maud`, so there is one style and no rewrite of the brief and debrief; a test renders `<script>` in record text on every route and finds it escaped. Whole Markdown documents render with `pulldown-cmark`, raw HTML escaped. A record loader that reads a commit's tree in memory lives in `atelier-app`. Decided by the planner of atelier-yfqu on 2026-11-09 under the plan procedure; Sumeet's approval of the mission's brief confirms it, and the strategy's Authority reserves changing it to him.

## Notes

- 2026-11-09 [decision] The site applies one shared URL policy to rendered Markdown: relative paths, fragments, http, https and mailto are kept, and any other scheme (javascript:, data:, vbscript: and the rest) is rendered as plain text. The library choice stands; the strategy's Design and atelier-zpmb carry the rule and its fixture.
  Reason: planning review finding atelier-wjok: escaping raw HTML leaves link and image destinations with any scheme

## Report

atelier-site crate in the atelier binary, on axum/tokio/tower, existing HTML helpers rather than maud, pulldown-cmark for Markdown, and a commit-tree record loader in atelier-app. Decided by the planner on 2026-11-09; Sumeet's brief approval of atelier-yfqu confirms it.
