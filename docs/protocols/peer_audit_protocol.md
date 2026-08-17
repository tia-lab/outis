~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Peer Audit

Version: 1.1
Status: active
Scope: spec and result audits

## Purpose

Peer audit tries to falsify the work before code starts or before claims are
accepted. Default mode is findings-first and no-edit.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- docs/protocols/spec_protocol.md;
- target research brief and spec;
- relevant architecture and prior specs;
- relevant source, generated, model, build, test, and evidence paths;
- upstream platform, format, model, storage, service, data, and dependency
  documentation when material.

## Audit Lenses

Challenge:

- pre-audit closure completeness;
- measured-object and non-goal clarity;
- source ownership and authorization;
- source snapshot, enumeration, mutation, filesystem, and path boundaries;
- supported and unsupported format behavior;
- extraction completeness and normalized-document contract;
- filename, metadata, comments, revisions, annotations, attachment, embedded
  content, temporary-file, log, and crash-artifact leakage;
- sensitive-data taxonomy and language coverage;
- automatic discovery, structure signals, entity resolution, aliases, spans,
  classes, relationships, conflicts, and uncertainty;
- local model trust status, artifact provenance, identity, runtime,
  configuration, network, tool, training, persistence, logging, evaluation, and
  failure behavior;
- perfect-detection or overgeneralized quality claims;
- token grammar, normalization, equality, repository scope, stability,
  collision, missing, malformed, forged, stale, rotation, merge, split, and
  token-like input behavior;
- Human, AI, and Key Zone boundaries and every plaintext copy;
- source, staging, agent-repository, and private-vault separation;
- agent-repository allowlist and forbidden contents;
- private-vault schema, secret ownership, callers, denied callers, migrations,
  journals, temporary files, corruption, recovery, retention, and deletion;
- staging validation, atomic publication, cancellation, crash consistency, and
  last-valid-output preservation;
- macOS, Finder, application lifecycle, sandbox, persisted access, signing,
  entitlements, app groups, Keychain, progress, and retry;
- Rust, Swift, C, FFI, IPC, crate, feature, Xcode target, build, and dependency
  boundaries;
- memory ownership, encoding, offsets, concurrency, error, panic, exception,
  cancellation, and cleanup;
- generated binding and model-artifact reproducibility;
- compile-surface and application-size containment;
- correctness, extraction, detection, entity, privacy, vault-isolation,
  publication, recovery, and deterministic replay oracles;
- benchmark isolation and resource budgets;
- code, build, test, dataset, generated, model, review, and evidence binding
  completeness;
- operator interpretation safety.

For conditional future services, also challenge:

- Agent Service plaintext exclusion;
- Key Service route and credential isolation;
- remote model plaintext, training, logging, cache, and destruction behavior;
- RAG, embedding, conversation, retrieval-log, and telemetry storage;
- authorization, rendering, wrong-tenant, wrong-matter, stale-token, offline,
  degraded, and retry behavior.

## Output

Write:

- docs/reviews/[slug]/[slug]_peer_audit.md

Classify exactly one:

- PEER_AUDIT_PASSED
- BLOCKED

## Stop Gates

Block the spec if:

- pre-audit closure is missing or false;
- a core assumption is unproved;
- source, extraction, detection, model, entity, token, trust-zone,
  agent-repository, vault, platform, FFI, publication, failure, recovery, or
  future-service behavior is ambiguous;
- a privacy or correctness oracle can pass while the intended property fails;
- benchmark cannot prove the intended claim;
- compile-surface or application-size impact is ignored;
- code, build, artifact, test, dataset, review, or evidence bindings are
  incomplete;
- dependency, platform, model, or format behavior is assumed but unverified;
- the operator could interpret processed or completed as perfect detection or
  proved safety without supporting evidence.
