---
schema: atelier.review
schema_version: 3
round: 2
commit: 38a7662a52c7b2f40ae9b77053a4915cd7213fad
reviewers:
- thread:delegated-task:command%3Amcp%3Ae3a4defb-881b-44dc-a35c-6c026d858852%3Adelegate-task%3Aae28eb6d-9447-46be-80e7-e79ed2e9effd
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T20:47:59.377861179Z
---

## Summary

Complexity: medium. Release lookup, downloads, the Python installer and persistent status form one update path shared by the command, injector and page.

Value: high. Installing and updating without a checkout is a required mission outcome.

Risk of merging it: medium. Updates replace files on users' devices; failures are visible and an installation can be replaced from a bundle.

The value warrants the scope. I found no substantially smaller change that delivers the required installation and update controls.

I read the full change, the round-1 review and both Responses, and inspected the changes since c7d40ff35. I reviewed and ran the code in a detached worktree at origin/atelier-wlyn, b022596ac. The review command required registration in the implementer's branch checkout and records 38a7662a, which differs only in the Report's CI links. No code was changed during this review.

`git diff --check master...38a7662a` passes. CI run https://github.com/spadaval/atelier/actions/runs/43953385705 concluded success at b022596ac. I read its extension job log: `node t3-extension/run-tests.mjs` passed test-extension.mjs and test-update.mjs, including the interrupted-download regression. The bundle verification run https://github.com/spadaval/atelier/actions/runs/43953385635 also passed.

Locally, test-update.mjs passed unmodified. The first run of the full test command failed before opening the browser because the existing Desk fixture parsed an incomplete URL (Finding 3). With a temporary wrapper outside the repository that buffers the existing Desk binary's output until newline, the same command passed both test files in 42s and 3s. It used the atelier-spike-t3 checkout's existing atelier-desk binary and desk/dist. Coverage includes installing from a bundle into an empty home, version recording, config preservation, release selection, --check, checksum refusal, startup checks, autoUpdate off and on, status and Customizations rendering.

I also ran separate local HTTP-server probes interrupting release metadata and checksum bodies after HTTP 200 headers. Both command errors named the operation and URL, kept VERSION at 0.1.0, and included the response-read failure. With autoUpdate on, the injector reported the interrupted checksum in status.update.error, kept lastError null and remained on 0.1.0. The existing zip-interruption regression exercises the third body reader. Both carried fix-now findings are fixed; there are no new merge blockers. Round 1's deferred pagination limitation remains unchanged.

I inspected the daily interval rather than waiting a day. I did not run on Windows or macOS, use a real T3 Code application or published release, or rebuild the Desk artifacts.

## Finding 1: Remove the trailing blank line so CI can run the extension tests
Recommendation: fix now
Was: 1-4i4u.1
Response: fixed in 99b178aa7. The item no longer ends with a blank line, and `git diff --check origin/master...HEAD` is clean. Round 2's CI run, at `b022596ac`: https://github.com/spadaval/atelier/actions/runs/43953385705, with the release workflow's at https://github.com/spadaval/atelier/actions/runs/43953385635. Both were running when this was written.
Now: fixed

The whitespace check passes locally and in CI. Run 43953385705 concluded success, and its extension job log shows both test files ran and passed. The linked run now meets the criterion that was missing in round 1.

## Finding 2: Keep the download operation and URL when reading the response body fails
Recommendation: fix now
Was: 1-4i4u.2
Response: fixed in b723a8d77. `get` now reads the body (`json`, `text` or `arrayBuffer`) inside the same wrapper as the request. A failure while the body arrives now reads, for example, `Downloading t3-extension-0.2.0.zip: GET <url>: reading the response: terminated`. `test-update.mjs` has a new case: the fixture sends 200 with the full `Content-Length`, writes 100 bytes and drops the connection. The case asserts the message and that `VERSION` and `custom.js` are unchanged. With the wrapper removed, it fails with `actual: 'terminated'`.
Now: fixed

`t3-extension/t3-custom.mjs:86-90` consumes all three response-body types inside the operation-and-URL wrapper. The added zip-interruption regression passes locally and in CI. Separate interrupted-metadata and interrupted-checksum probes confirm those paths too; the injector exposes the named failure through status without a fatal lastError or a changed release.

## Finding 3: Wait for the complete Desk log line before accepting its URL
Recommendation: defer

`t3-extension/fixture-desk.mjs:103-106` matches `serving on (http:\/\/\S+)` against the accumulated stream on every data chunk. Pipe chunks need not end at a line boundary. If one ends after `http://127.0.0.1`, the promise resolves with that incomplete URL and never incorporates the port in the next chunk. The first local full test run failed at the initial `/api/repos` fetch. A separate direct call to startFixtureDesk reproduced `ECONNREFUSED 127.0.0.1:80` while the final Desk log named its actual ephemeral port. A temporary wrapper that forwards complete log lines allowed the unchanged suite to pass.

Buffer until newline before parsing the address; do not retry the fetch, since the returned URL is already wrong. This predates the change, affects test startup rather than the updater, and CI on the reviewed code passed, so it belongs in separate test-harness work and does not block this item.
