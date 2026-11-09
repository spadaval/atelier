---
schema: atelier.review
schema_version: 2
id: atelier-iq33
title: finding on atelier-zpmb @7a392500
target: atelier-zpmb
identity: gpt-6.1-sol-r2
commit: 7a3925008cafd4f69b701fc80b5934c1175f0d39
created_at: 2026-11-09T17:09:53.237323977Z
event:
  event: finding
  recommendation: fix_now
---

R2 medium: crates/atelier-site/src/discovery.rs:167-176 suppresses a failed config-blob read into slug=None, and crates/atelier-site/src/lib.rs:268-269 caches those CommitFacts whenever the policy read succeeded. Reproduced at the pinned head in a scratch repository whose directory is directory-name but whose committed project_slug is project: make only the config's loose blob unreadable for the first GET /api/repos, then restore it without changing the base commit. The next listing loads the records successfully and has error=null, but still calls the repository directory-name instead of project. Its committed URL name never recovers until the base moves or the server restarts, and callers using the committed slug get an unknown-repository 404. This is another remaining path of atelier-cuq9: the cache retains the result of a transient failure. Distinguish a genuinely absent or invalid config from failed Git I/O, and do not retain facts when a required read failed. Add the config-only recovery case to the tests. Recommendation: fix_now.
