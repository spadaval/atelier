---
schema: atelier.item
schema_version: 3
id: atelier-n4g7
title: Move the T3 Code extension into this repository with its history
status: building
mission: atelier-m8ye
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A932557e7-853d-4352-845a-e9951a7775d5%3Adelegate-task%3A8cf46553-0a49-4b52-9758-d74d257bf425
ready:
  by: 'sumeet, in conversation on 2026-11-16: do this, and the rest of the mission'
  at: 2026-11-16T19:26:24.749789035Z
trivial: null
caused_by: null
created_at: 2026-11-16T19:25:26.767907122Z
---

## Goal

Put the extension's code where the Desk it depends on is changed, so one item
and one review cover both sides of their contract. It lives in
`github.com/spadaval/t3-code-atelier`, and every change to it so far also
changed the Desk here.

## Done when

- [x] The extension's files are under `t3-extension/`, brought in with their history from `spadaval/t3-code-atelier` at `a242410` (branch `main`), so `git log --follow t3-extension/custom.js` reaches its first commit. None of that repository's tags comes with it: fetch with `--no-tags`, since its `v0.1.0` matches the release workflow's tag pattern.
- [x] That repository is private and this one is public. Before anything is pushed, the history is rewritten in a scratch clone so that no imported commit holds a file under `artifacts/` (15 screenshots there show Sumeet's other projects and thread titles), and the rewritten history is scanned, every commit and not only the tip, for secrets, tokens, private addresses and host names, and personal details. The Report lists what the scan looked for and found. Anything found besides the authors' Git identities stops the push until the thread that started the item answers.
- [x] At the last imported commit, before any edit of this item's, the tree under `t3-extension/` equals `a242410`'s tree byte for byte (that commit holds no `artifacts/`), and the Report gives the command a reviewer runs to see it. Every change this item makes to those files, line endings and trailing whitespace included, is in commits after it. `git diff --check` passes on the merged tree.
- [x] `scripts/ci-jobs.sh` does not run the `rust` or `desk` job for a change under `t3-extension/` alone, and `scripts/ci-jobs.test.sh` covers it.
- [x] `node --check` passes on each script, and `node t3-extension/test-extension.mjs` passes on this host with `BROWSER=/usr/bin/google-chrome` and `DESK=http://127.0.0.1:3780`, run from its new place.
- [x] `AGENTS.md` names `t3-extension/` and the command that tests it, the extension's own `AGENTS.md` is folded into its `README.md` or kept as a file the root one links, and its README no longer speaks of a separate repository or of committing to `main`.
- [x] `docs/design.md` records the decision in its Decisions table: the extension is in this repository and outside the `atelier` release, as the Desk is (ADR 0032), and `atelier` still has no Node step in its build or release.

## Scope

In: `t3-extension/`, `scripts/ci-jobs.sh` and its test, `AGENTS.md`, `README.md`, `docs/design.md`.

Out: a CI job for the extension (atelier-7tg6); its release (atelier-451s); how it installs and updates (atelier-wlyn); any change to what the extension does.

## Context

A clone of the extension is at `/tmp/claude-1000/-home-supadava-projects-atelier/efd339ea-b573-40a9-99be-465d565b2949/scratchpad/repo` if it still exists; otherwise clone `git@github.com:spadaval/t3-code-atelier.git`. It was last edited on Windows, and its installed copies there differed from the commit in line endings. `needs_rust` in `scripts/ci-jobs.sh` ends in `return 0`, so a new top-level folder runs the Rust job until it is named there.

## Report

The extension is under `t3-extension/` with its 36 commits, and `git log
--follow t3-extension/custom.js` reaches `620c4bd2b Add initial Atelier
extension prototype for T3 Code`.

**Import.** In a scratch clone of `main` made with `--no-tags --single-branch`
(HEAD `a242410`), `git filter-repo --path artifacts/ --invert-paths
--to-subdirectory-filter t3-extension` dropped `artifacts/` from every commit
and moved the rest under `t3-extension/`. No commit became empty. A second
`filter-repo` pass scrubbed what the scan found (below). The branch fetched the
result with `--no-tags` and merged it with `--allow-unrelated-histories`
(`031bea8bb`); `git tag -l 'v0.1*'` is empty. No imported commit holds a path
under `artifacts/`:
`git log 031bea8bb^2 --name-only --format= | grep -c '^t3-extension/artifacts/'`
prints 0.

**Byte-for-byte check.** The last imported commit is `26fe290`, the rewritten
`a242410`. A reviewer runs:

    git rev-parse 26fe290:t3-extension
    gh api repos/spadaval/t3-code-atelier/git/commits/a24241057c2b2138553fcf777813ef865e9fdd9c --jq .tree.sha

Both print `28de536c55eff9d45d54ede6fbfc6d98bb434116`, so every file is
identical. The imported files have no CRLF line endings and no trailing
whitespace, so none needed fixing; `git diff --check` passes. This item's only
edit to them is in `2501060af`.

**Scan.** Every commit of the rewritten history was scanned twice: `git log -p`
of all 36 commits with their messages, and `git grep` over the tree of every
commit, which covers the two merge commits that `log -p` shows no patch for. It
looked for token and key formats (GitHub, OpenAI, Anthropic, Slack and AWS
prefixes, private key blocks), the words key, secret, token, password, bearer
and authorization, long base64-like strings, IPv4 addresses including private
ranges, email addresses, URLs and host names including Tailscale and
local-network suffixes, home and user paths on Linux, macOS and Windows, thread
IDs and UUIDs, and names of people, machines and projects. It also listed every
repository name, slug, `/r/<repo>` path, `github.com/<owner>/<repo>` string
and `<prefix>-<4 characters>` ID in any commit.

The first scan found no secret, token, key or password (the token hits are
code that forwards T3's own token, and prose about it); only `127.0.0.1` as an
address; only Claude's co-author address and `git@github.com` as emails; and
GitHub repository names under `spadaval`. Fixture threads and IDs are
invented. Besides the authors' Git identities, it found four things, which held
the push until the thread that started the item answered:

1. The host name `buildbot` (`http://buildbot:3780` and `:3781`), in the
   README, `docs/VERIFICATION.md`, `config.example.json` and comments in
   `custom.js`. **Accepted:** `master` already names it in five files, and
   removing it would break the byte-for-byte match.
2. The local user name `supadava`, in the macOS bundle ID
   `local.supadava.t3-custom`. **Accepted** for the same reasons: `master` has
   it in 21 files.
3. The name of another of Sumeet's repositories, as a repository, slug and
   `/r/` path, and two of its IDs, used as the test's repository in older
   versions of the README, `config.example.json` and `test-extension.mjs`, and
   once in a commit message. Absent at the tip. **Scrubbed** with
   `--replace-text` and `--replace-message`: each ID became `example-<id>`
   and the name became `example`, in every commit's files and messages.
4. A sentence about which computers Sumeet uses, in the message of the commit
   now titled "Run the loader and installer on Windows as well as macOS".
   **Scrubbed** with `--replace-message` to "The extension is also used on a
   Windows PC."

The scan rerun over every rewritten commit, files and messages, finds no trace
of 3 or 4, and nothing besides the authors' Git identities and items 1 and 2.

**Checks.** `node --check` passes on `custom.js`, `host-adapter.js`,
`devtools.mjs`, `t3-custom.mjs`, `test-contract.mjs` and `test-extension.mjs`.
`BROWSER=/usr/bin/google-chrome DESK=http://127.0.0.1:3780 node
t3-extension/test-extension.mjs`, run from the repository root, passes all 26
checks; so did `node test-extension.mjs` from inside `t3-extension/` before
any edit. The test found its files from the working directory, so from the root
it would not; it now finds them from its own folder, which changes only the
test. `scripts/ci-jobs.test.sh` passes, with two new cases: a change under
`t3-extension/` alone runs neither job, and one that also touches
`desk/src/api/` still runs both.

**Docs.** The extension's `AGENTS.md` stays as a file, linked from the root
`AGENTS.md` and from the extension's README. The root `AGENTS.md` names
`t3-extension/` and its test command, and says CI skips it. The extension's
README installs from this repository and links the host contract in
`docs/architecture/desk.md`. `README.md` names the folder, and
`docs/design.md` has the decision. `docs/VERIFICATION.md` is left as written:
it is a record of past runs.
