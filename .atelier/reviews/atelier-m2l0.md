---
schema: atelier.review
schema_version: 3
id: atelier-m2l0
title: finding on branch release-0.4.0 @6dd196be
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 6dd196be861e871264a466b5892360a9f8218715
created_at: 2026-11-09T20:56:14.055804468Z
event:
  event: finding
  recommendation: fix_now
---

medium: CHANGELOG.md:41 The lifecycle summary omits the user-visible start changes from atelier-md0v. start now creates a bare issue-ID branch on the base, adopts an existing unowned current branch, and accepts start --base only as a deprecated, ignored flag. In 0.3.1 --base selected the branch to create from and allowed creation from detached HEAD; that invocation now refuses when no recorded branch exists. Automation depending on type-prefixed branches or --base needs an upgrade note. Add these changes and say the merge target comes from policy base_branch, not the recorded branch_base. Do not describe --base as removed outright: this release retains the hidden compatibility flag.
