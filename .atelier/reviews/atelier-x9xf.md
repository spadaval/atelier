---
schema: atelier.review
schema_version: 1
id: atelier-x9xf
title: finding on atelier-5tl2 @9dfcaccb
target: atelier-5tl2
identity: Independent reviewer (Claude subagent)
commit: 9dfcaccbf99308df440da029d47a86b6013cb749
created_at: 2026-10-28T02:06:44.463466969Z
event:
  event: finding
  recommendation: defer
---

low: CHANGELOG.md:7 — the first-release entry is under [Unreleased]; dist builds release notes from the section matching the tag version, so the heading must be renamed to [<version>] - <date> when the version is chosen, or the release gets generic notes. Also latest/download skips prereleases, so the first tag should be stable.
