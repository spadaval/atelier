---
schema: atelier.review
schema_version: 2
id: atelier-l7mt
title: finding on atelier-45gr @859f41a2
target: atelier-45gr
identity: plan-reviewer-g6rp
commit: 859f41a2454c4278bc33d18868729b139e7e90f7
created_at: 2026-11-08T05:58:07.941144166Z
event:
  event: finding
  recommendation: fix_now
---

[low] The plan does not say how the binary fetches the latest release, and no crate in the workspace has an HTTP client today (no reqwest, ureq or hyper in any Cargo.toml). A TLS HTTP stack is a large dependency for one daily check. 'git ls-remote --tags https://github.com/spadaval/atelier' (tags v0.3.0 and v0.3.1 exist) needs no new crate and avoids the 60-calls-an-hour anonymous API limit. Also, skills 1.7.6 sends telemetry to skills.sh on every check unless DISABLE_TELEMETRY or DO_NOT_TRACK is set (src/telemetry.ts:76), and this check would run daily from a detached refresh in every repository. Fix: name the fetch method, or make it an Unknown, and set DISABLE_TELEMETRY=1 when running npx skills.
