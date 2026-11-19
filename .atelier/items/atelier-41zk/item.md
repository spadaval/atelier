---
schema: atelier.item
schema_version: 3
id: atelier-41zk
title: 'Rewrite the README to show what Atelier does: the loop, the records, missions, the Desk and the T3 Code extension'
status: in_review
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- 8f64a072-9760-488f-a6e4-ed576ae6af38
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T05:08:07.309648548Z
---

## Goal

Sumeet, on 2026-11-19: "The readme file is deeply lacking, compared to all the
cool stuff this repo is capable of. Let's do a rewrite." The README said in
five sentences what Atelier is and linked the specification. A reader who
does not know the project could not tell from it what a run looks like, what
the review rules are, or that the Desk, the T3 Code extension and the evals
exist as more than a link.

Rewrite `README.md` so that reader sees what Atelier does and why, from the
page itself, and nothing on it is untrue of the tool as it stands on `master`.

## Done when

- [x] The README opens with what Atelier is for and its three parts, and shows one item's whole loop (start, submit, review, changes requested, approve, merge, log) in output the 0.7.0 binary printed in a scratch repository, with every cut marked.
- [x] It covers, each in a short section a reader can skip: `status`, the records and the branch model, independent review, missions, merge, hooks and `ATELIER_SKIP`, taking a role over, the gardener, the commands by role, the skill, the Desk, the T3 Code extension, the evals, and why the tool was rewritten.
- [x] It shows the Desk with screenshots of the running Desk on this repository's own records, in light and dark, and the T3 Code extension with a screenshot an item already holds.
- [x] The install text and the platform support it had are kept, and a section says how to track a repository: the config file and `atelier setup`, as tried in the scratch repository.
- [x] It says who the tool is built for and what a second user should not expect, as `docs/product.md` does.
- [x] Every relative link and image path in it names a file in the tree.

## Scope

In: `README.md`, `docs/images/`.

Out: `docs/`, `CONTRIBUTING.md`, `AGENTS.md` and the other READMEs.

## Report

The README now shows what Atelier does: one item's loop in the tool's own
output, a section for each thing the tool, the skill, the Desk, the extension
and the evals do, and why it was rewritten. It grew from 50 lines to about
430. Install and platform support are kept as they were.

**Where the words come from.** `docs/product.md` and `docs/product/desk.md`
as they stand on `master`, `skills/agent-factory`, `evals/agent-factory/README.md`,
`t3-extension/README.md`, `docs/design.md` (the config file and thread
identity) and `CHANGELOG.md`. The README states no rule those do not.

**The transcripts.** I ran the loop with the installed `atelier 0.7.0` in a
scratch repository with no forge: a hand-written `.atelier/config.toml`
(`schema_version`, `slug`, `base_branch`), `atelier setup`, `atelier setup
claude`, `start --new`, `submit`, a round-1 review with one fix-now finding
and `verdict changes`, a fix, a round-2 review by a third thread and `verdict
approve`, `merge` and `log`. The threads were named by setting
`T3CODE_THREAD_ID` to `alice`, `bob` and `carol`, which the README says. Its
output blocks are that run's output; `[...]` marks each cut, and two long
lines are cut in the middle.

**The screenshots.** `docs/images/desk-mission-*.png` and
`desk-in-motion-*.png` are the Desk service on this host (built from
c075093bb) at 1280 by 860, on this repository's records, taken with
Playwright's headless Chromium in each colour scheme. Their sidebar names the
host's four other repositories (astromech, demiurge, life-on-rails,
cosplay-studio); `desk/README.md` already names all four. The T3 Code
screenshot is `atelier-wvst`'s own artifact, linked where it lies, since a
done item's directory does not change. It shows fixture data, and the caption
says so.

**Left out on purpose.** The Desk's mission control and project home pages:
atelier-yy4j deletes them. Counts of missions, items and reviews, which would
go stale.

### Validation

- A script resolved every relative link target and every `src` and `srcset`
  in the README against the tree: none missing.
- `git diff --check` is clean.
- Not checked: how GitHub renders the page. The Mermaid state diagram and
  the `<picture>` elements were not rendered locally. The pull request shows
  both.
