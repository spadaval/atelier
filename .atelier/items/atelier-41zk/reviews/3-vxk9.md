---
schema: atelier.review
schema_version: 3
round: 3
commit: 1f46ebbeaea7d80c31fb63268eb9f9ee2ba8c305
reviewers:
- thread:delegated-task:command%3Amcp%3A03454c8e-b27f-4061-997e-27a4558850ca%3Adelegate-task%3Ac93a4b93-2ad7-4d14-846c-a4fd2a5ccd1a
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-19T05:23:02.081047989Z
---

## Summary

Complexity: low. One README and four screenshots introduce no new runtime behavior.
Value: high. The requested introduction explains the tool, skill and Desk without requiring a reader to start with the specification.
Risk of merging it: low. Incorrect documentation is visible when followed and cheap to revert.

The requested breadth warrants the size. A smaller rewrite would omit requested capabilities or the concrete examples that explain them.

Read the complete README and the diff since round 2 against every Done when line. Compared its claims with the product specifications, Agent Factory, the extension and eval documentation, and the setup, identity, push-guard, config and merge implementations. The original install command and platform limitations are retained. Resolved all 32 relative link and image targets, inspected all six referenced screenshots in light and dark, and ran `git diff --check master...1f46ebbeaea7d80c31fb63268eb9f9ee2ba8c305` successfully. CI's test, changes and plan checks passed; the Desk, extension and eval jobs were skipped.

With installed `atelier 0.7.0`, independently followed both bootstrap variants in fresh repositories with existing code and local bare remotes. Each publishes the config and its selected harness's generated hooks before installing the Git hooks, leaves a clean checkout, and produces the documented `status` output. Independently replayed the two-round item loop with simulated alice, bob and carol identities: start, submit, review, changes requested, fix/response, submit, review, approve, log before merge, and merge. The displayed output matches apart from the naturally different IDs, hashes and timestamps. The merged base contains the change and both reviews, and `atelier check` passes.

Did not recapture the running Desk, repeat the author's GitHub rendering check, or exercise a live GitHub merge. The original scratch run's hashes and timestamps cannot be authenticated from the supplied artifacts. No new findings remain.

## Finding 1: Stage the Codex hooks when choosing the Codex setup alternative
Recommendation: fix now
Was: 2-9ovw.5
Response: fixed in 587c4b06a
Now: fixed

README.md:405-408 now names both `atelier setup codex` and `git add .atelier .codex` alongside the Claude commands. Following each variant against a local bare remote successfully publishes `.atelier/config.toml` and the appropriate session-hook file before `atelier setup` installs the base-branch push guard. The Codex alternative no longer stages a missing `.claude` directory or leaves its hooks unpublished.
