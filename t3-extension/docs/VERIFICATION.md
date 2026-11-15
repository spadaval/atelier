# Prototype verification

Validated on macOS on November 15, 2026, against T3 Code Nightly
`0.1.10-nightly.20261114.4669` and an existing Atelier Desk server.

The isolated Chromium integration suite checks:

- Desk docking, host panel resizing, and completed rendering of real Desk data.
- Repository identity matching, manual mapping, same-repository navigation
  preservation, and hiding previous content for unmapped projects.
- Configured-slug recognition in prose and inline code, excluding fenced code
  and editors, without rewriting React's text nodes.
- Hovercards using real item data, stable references while messages stream,
  direct mission clicks before hover, and cross-project reference navigation.
- Settings reinjection without refreshing the document, subscription cleanup,
  restoration of native surfaces, and full injector removal.
- Rejection of action endpoints by the read bridge.

The actual installed T3 application was additionally used to verify private
module discovery, project context, native panel placement, item hovercards,
Settings, reinjection, and document reload during connection startup. The
official application bundle was not modified.

The tests use existing backend records; they do not cover Desk mutations.
They are not a compatibility guarantee for other T3 builds. Runtime logs,
machine configuration, screenshots, and temporary browser profiles are local
artifacts and are not versioned.
