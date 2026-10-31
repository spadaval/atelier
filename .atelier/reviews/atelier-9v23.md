---
schema: atelier.review
schema_version: 1
id: atelier-9v23
title: finding on atelier-nl9a @963c75a7
target: atelier-nl9a
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:34.878701133Z
event:
  event: finding
  recommendation: no_action
---

R9: the failing proof on a scratch branch needs --since to bypass the checkout rule, which Unknowns understates. Base rules run only when branch == base (session.rs:535); an unowned branch without the base ref stops at land_work_untraceable (session.rs:111, reproduced locally on a scratch branch and on a detached HEAD), so "accept a detached HEAD on the base commit" would not reach it. No action because only one reading satisfies the Done when and the worker meets it on the first dispatched run; Scope already names session.rs. Note when next edited: --since applies base rules to the checkout whatever its ref.
