---
schema: atelier.review
schema_version: 3
round: 1
commit: 827462346a433e7459b76bbe8cb8392022cb5596
reviewers:
- thread:delegated-task:command%3Amcp%3A8a405538-ecb2-4b0c-8085-ebead68178e6%3Adelegate-task%3A73d04388-caa8-476f-86bc-28fb91ad9645
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T21:41:56.024214736Z
---

## Summary

Complexity: medium. Vite and TypeScript add a build step whose output must agree
with the loader, installer, updater, release bundle and CI.

Value: high. The next items in the native extension rewrite need a working
TypeScript module build, and this establishes it without rewriting the UI.

Risk of merging: medium. A mistake could break installed extensions or releases,
but failures are visible and the change can be reverted.

The value justifies the added build machinery. I found no smaller change that
would meet the module-build and installation requirements, and no findings.

Reviewed master...827462346a433e7459b76bbe8cb8392022cb5596 against every
Done when line. Compared both moved JavaScript files with their originals:
the extension body is unchanged inside start(api), and the adapter's exports
and explicit host API preserve its former shared-scope access. Checked the
classic-script output, dynamic host imports, loader invocation and watched
files, installer paths, release contents, CI steps and development instructions.
The existing behavior assertions are preserved; the changed installed-file
assertion and added installation checks match the new build layout.

Passed locally:

- pnpm install --frozen-lockfile, pnpm typecheck and pnpm build.
- node run-tests.mjs: all three tests passed, including the browser fixture,
  reinjection, cleanup, settings, installation and updates.
- scripts/t3-extension-release.test.sh, including bundle, verify with no Node
  on PATH, and rejection of a bundle containing src/.
- scripts/ci-jobs.test.sh and git diff --check master...827462346.
- A scratch-copy run of pnpm watch --outDir <install folder>: editing src/main.ts
  rebuilt the single custom.js and preserved an existing config.json.

The first local test runs hit the host's /tmp disk quota. Rerunning with TMPDIR
under /var/tmp passed without changing code or tests. GitHub's changes and bundle
jobs had passed when inspected; the extension and Rust jobs were still pending.
I did not run test-contract.mjs against a real T3 instance, or test on macOS or
Windows. The moved JavaScript remains outside checkJs, as the item permits;
strict typechecking covers the TypeScript entry and Vite configuration.
