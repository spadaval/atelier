---
created_at: "2026-11-09T14:30:40.055404283+00:00"
id: "atelier-8p7u"
issue_type: "decision"
labels:
- "fleet-dashboard"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-c5vk"
    type: "supersedes"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T14:30:47.335589590+00:00"
status: "done"
title: "Build the site's UI as a Vite, TypeScript and React app over a read-only JSON API, embedded in the atelier binary"
updated_at: "2026-11-09T14:30:47.335589590+00:00"
---

## Context

`docs/strategies/fleet-dashboard.md` Design; decision atelier-7bs1; ADR 0030 (atelier-tu8c). Supersedes decision atelier-c5vk, which chose server-rendered HTML from the existing `review_page.rs` helpers mainly to avoid rewriting about 1,500 lines of brief and debrief renderers.

## Question

How is the site's user interface built, tested and shipped? Decision atelier-c5vk settled the crate (`crates/atelier-site` in the `atelier` binary) and the server (`axum`, `tokio`, `tower`); Sumeet reopened the HTML choice on 2026-11-09 because server-rendered pages are slow to iterate on and hard to test as a UI.

## Options

- Keep atelier-c5vk: hand-written HTML strings from Rust. No new toolchain and the renderers move rather than get rewritten, but every visual change is a Rust rebuild, pages can only be tested as strings, and interaction (filters, live status from atelier-bnek) means hand-written JavaScript.
- Rust templates (`maud`, `askama`) with htmx. Typed templates and some interaction without Node, but still a Rust rebuild per change, no component tests, and a rewrite of the renderers anyway.
- A Vite, TypeScript and React app served by `atelier serve` over a read-only JSON API. Instant reloads while developing, component tests against fixture data, a browser test of real pages, and room for interaction. Costs a Node toolchain in development, CI and release, a JSON contract between Rust and TypeScript, and a rewrite of the brief and debrief pages as components.

## Decision

The third option. `crates/atelier-site` keeps `axum`, `tokio` and `tower`, and serves two things: a read-only JSON API under `/api/`, and a single-page app built from `crates/atelier-site/web/` with Vite, TypeScript and React, using pnpm, React Router and `react-markdown`.

- **One view model.** The API returns the same view models the CLI's Markdown output is built from (`atelier-app`'s brief, debrief and status views), serialized with `serde`; it holds no logic of its own. TypeScript types are generated from those Rust types with `ts-rs` and checked in, and a Rust test fails when they are stale.
- **Shipping.** The built app is embedded in the binary (`rust-embed`), so `atelier serve` stays one binary with no files beside it. A `cargo install` or release build never produces a binary without the app: it builds it or fails naming the missing step. Rust tests and debug builds do not need Node.
- **Markdown and escaping.** The API sends Markdown as text. The app renders it with `react-markdown` with raw HTML skipped, and one URL policy for links and images: relative paths, fragments, `http`, `https` and `mailto` are kept, and any other scheme renders as plain text. React escapes every other string.
- **Tests.** Rust tests check each API route against fixture repositories and write its responses as checked-in fixture JSON; Vitest and Testing Library test components against that JSON, so the frontend's fixtures are real API output; one Playwright suite loads every route of `atelier serve` on a fixture repository, at a 390 px viewport. CI gains a job that runs the typecheck, Vitest, the build and Playwright.
- **Agents keep Markdown.** The JSON API exists for the app. Agents read the CLI's Markdown (ADR 0022), and nothing tells them to call the API.

The Rust `--html` renderers are not ported or moved; atelier-59fz deletes them with the flag. Decided by Sumeet on 2026-11-09 in conversation, on Claude's recommendation; his approval of the mission's brief confirms the details above.

## Report

Vite, TypeScript and React app over a read-only JSON API built from the shared view models, embedded in the atelier binary; supersedes atelier-c5vk's server-rendered HTML. Decided by Sumeet on 2026-11-09; his approval of the atelier-yfqu brief confirms the details.
