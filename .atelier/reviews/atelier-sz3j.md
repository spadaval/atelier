---
schema: atelier.review
schema_version: 1
id: atelier-sz3j
title: approval on atelier-7v9q @575cf9d0
target: atelier-7v9q
identity: Independent reviewer (Claude subagent)
commit: 575cf9d0abbb3682dda04458261b4e1ced61d016
created_at: 2026-10-28T01:52:32.126619894Z
event:
  event: approval
---

Approved at 575cf9d0. Contract pass: dist plan --tag=v0.2.0 lists the four archives, per-archive .sha256, sha256.sum, source.tar.gz and atelier-cli-installer.sh, no Windows; metadata names spadaval/atelier; PR-mode Release run and CI passed; no tag pushed. Fitness pass: dist generate --check exits 0 (release.yml unedited); tag trigger, contents: write, host job and gh release create verified; mismatched tag fails at plan; only atelier-cli is released; native runners; .gitignore fix for dist-manifest.json is the minimal correct fix for -dirty. Deferred: CI gate on tags (atelier-vy5i), build.rs rerun path (atelier-b99t). No action: every-PR artifact builds (cost of unedited workflow), atelier-cli-* artifact names, stale CHANGELOG in archives (atelier-5tl2), glibc >= 2.35 (state in S3 validation). Not run: tag-push path.
