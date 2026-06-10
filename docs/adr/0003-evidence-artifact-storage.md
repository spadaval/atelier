# ADR 0003: Evidence Artifact Storage

## Status

Accepted.

## Context

Evidence records need durable metadata that can validate issues, milestones,
missions, workflow transitions, and other records. Some evidence is small and
repo-native, such as command output summaries or short reports. Other evidence
can be large, binary, proprietary, or produced by external systems, such as
screenshots, videos, benchmark artifacts, CI logs, or hosted reports.

Atelier is local-first and Git-backed. Storing every artifact directly in the
repository would make history heavy and create avoidable merge pressure. Adding
an external artifact service as the first backend would add operational
requirements before the core record model is stable.

## Decision

The first artifact backend is metadata-only evidence records in
`.atelier-state/evidence/`.

Evidence records may reference:

- a repository-relative path for small artifacts intentionally committed to the
  repository;
- an external URI for artifacts hosted by CI, object storage, issue trackers, or
  other durable systems;
- no separate artifact when the evidence summary itself is sufficient.

Atelier does not copy, upload, hash, retain, or garbage-collect artifact
payloads in the first backend. Evidence records preserve metadata in canonical
state: kind, result, summary, path or URI, producer, timestamp, and typed
validation links. Size, hash, retention policy, and external storage adapters
are deferred until an implementation issue defines the backend contract.

## Consequences

- Evidence records are immediately useful for workflow validators and Mission
  Control without introducing a storage service.
- Small artifacts can be committed intentionally by the repository owner, but
  the CLI does not decide which files belong in Git.
- Large artifacts remain outside the repository and are referenced by URI.
- Rebuild only needs canonical evidence metadata, not artifact payloads.
- Validators can require attached evidence metadata before artifact integrity
  checks exist.
- Future backends can add hash, size, retention, upload, and fetch behavior
  without changing the first-class evidence record identity model.
