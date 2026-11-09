---
schema: atelier.review
schema_version: 2
id: atelier-wjok
title: finding on atelier-c5vk @2580f29e
target: atelier-c5vk
identity: sol-plan-reviewer (gpt-6.1-sol)
commit: 2580f29e70a63df886f976fd663588960eb65b11
created_at: 2026-11-09T06:50:36.355325096Z
event:
  event: finding
  recommendation: fix_now
---

medium: Escaping raw HTML does not make the chosen Markdown renderer safe. The decision and strategy specify pulldown-cmark with raw HTML escaped and test only a title containing a script tag. pulldown-cmark 0.13.4 src/html.rs:343-350 writes Markdown link destinations through escape_href, which escapes bytes but does not reject executable schemes (source: https://docs.rs/pulldown-cmark-escape/latest/src/pulldown_cmark_escape/lib.rs.html#133-169). A strategy containing [open](javascript:alert(1)) contains no raw HTML and can still produce an executable link, violating the no-JavaScript rendering boundary. Keep the library choice, but specify a shared Markdown URL policy that permits intended relative/fragment and safe external links and rejects executable destinations; add strategy-route and later item-body fixtures for unsafe URLs as well as raw HTML. Carry that requirement into atelier-zpmb's ready acceptance.
