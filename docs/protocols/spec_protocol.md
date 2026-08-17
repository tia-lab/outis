~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Spec Authoring

Version: 1.2
Status: active
Scope: docs/specs/*_SPEC.md

## Purpose

The spec is the source of truth. Implementation may not contain behavior,
dependency, platform configuration, generated artifact, model artifact,
storage schema, security claim, benchmark, or interpretation absent from the
approved spec.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- docs/protocols/research_protocol.md;
- this protocol;
- target research brief;
- relevant architecture and prior specs;
- relevant source and generated paths;
- relevant platform, format, model, storage, service, data, and dependency
  documentation.

## Entry Conditions

Spec authoring may start only when:

- slug and measured object are known;
- target source, formats, languages, and sensitive-data classes are bounded;
- candidate approach is known;
- affected Human, AI, and Key Zone surfaces are identified;
- major unknowns and evidence gaps are listed;
- no code or generated-artifact change is required to finish the spec.

## Mandatory Location

- docs/specs/[slug]_SPEC.md

## Minimal-Surface Requirements

Every pilot spec must define the smallest complete vertical slice that can be
implemented and tested end to end.

The spec must:

- use an explicit allowlist for formats, languages, sensitive-data classes,
  commands, routes, states, and outputs;
- define typed rejection for every adjacent unsupported category that can
  reach the slice;
- distinguish required edge behavior from deferred behavior;
- include edge behavior only when it is inside the supported contract or is
  required for privacy boundaries, determinism, vault integrity, atomic
  publication, cancellation, or recovery;
- exclude speculative frameworks, plugin systems, compatibility layers,
  future-service adapters, optional modes, feature flags, caches, concurrency,
  and extension points;
- justify every production file, target, interface, abstraction,
  configuration item, dependency, and non-trivial branch through a current
  requirement, invariant, acceptance test, or evidence need;
- define the accepted path, explicit rejection path, and applicable privacy
  and failure-boundary tests;
- state measurable performance and resource budgets without assuming an
  optimization strategy.

A wider roadmap is not part of the implementation contract.

## Mandatory Sections

Use this section order:

1. Identification
2. Status
3. Purpose
4. Non-goals
5. Measured object
6. Source ownership and authorization contract
7. Source snapshot, enumeration, and filesystem contract
8. Source format and extraction contract
9. Normalized-document contract
10. Sensitive-data classification contract
11. Automatic discovery contract
12. Local model and model-artifact contract
13. Entity and alias resolution contract
14. Uncertainty and review contract
15. Tokenization contract
16. Redaction contract
17. Trust-zone and plaintext-copy contract
18. Agent-repository content and access contract
19. Private-vault, secret, and storage contract
20. Staging, validation, and atomic-publication contract
21. macOS application and lifecycle contract
22. Finder dispatch contract
23. Sandbox, signing, entitlement, and Keychain contract
24. Rust, Swift, FFI, IPC, crate, and target contract
25. Dependency contract
26. Codegen, generated-binding, and generated-artifact contract
27. Conditional future-service contract
28. Determinism contract
29. Failure, cancellation, retry, and recovery contract
30. Retention, deletion, backup, and audit contract
31. Compile-surface and application-size budget
32. Runtime performance and resource budget
33. Correctness, extraction, detection, and entity-resolution oracle
34. Privacy, vault-isolation, and agent-boundary oracle
35. Benchmark methodology
36. Test plan
37. Code and build bindings
38. Generated, model, dataset, and evidence artifact bindings
39. Review artifact bindings
40. Implementation-plan requirement
41. Approval and pre-audit closure checklist
42. Open questions

A non-applicable section must say not in scope and identify the controlling
non-goal. It must not be silently deleted.

## Pre-Audit Closure Gate

Before the first peer audit, the author must verify:

- mandatory section order is exact;
- every prior approved spec was searched and preserved, locally superseded, or
  cited as out of scope;
- every command and UI dispatch surface is exact;
- source, staging, agent-repository, private-vault, temporary, model, and
  evidence paths have one owner;
- every supported and unsupported source format has explicit behavior;
- every generated binding, model artifact, manifest, schema, fixture, and
  evidence artifact has one owner and reproducibility command;
- no artifact is bound to incompatible commands or trust zones;
- every Rust, Swift, C, Xcode, extension, adapter, storage, and dispatch path is
  listed in code and build bindings;
- every sandbox, signing, entitlement, app-group, Keychain, and persisted-access
  decision is explicit;
- FFI or IPC ownership, encoding, offsets, errors, threading, cancellation, and
  cleanup are explicit;
- every test encoding old behavior is preserved or explicitly migrated;
- no design decision is deferred to the implementation plan when it belongs in
  the spec;
- privacy, extraction, detection, vault-isolation, deterministic replay,
  atomic-publication, and recovery proof commands are defined;
- compile-surface and application-size evidence commands are defined for
  generated, model, extraction, database, or platform-heavy work;
- future-service sections are explicitly out of scope or fully closed;
- the slice is the smallest complete path satisfying the measured object;
- every planned production surface has a current necessity binding;
- deferred behavior is rejected rather than represented by scaffolding.

The approval section must include this checklist. A missing or false item
blocks peer audit.

## Source and Extraction Requirements

The spec must define:

- owner and authorization assumptions;
- snapshot identity and mutation behavior;
- deterministic enumeration;
- path, symlink, hard-link, alias, package, mount, archive, hidden-file, and
  nested-repository behavior;
- accepted format variants and maximum sizes;
- metadata, headers, footers, tables, comments, revisions, annotations, forms,
  attachments, and embedded-content handling;
- text-bearing, image-only, OCR, encrypted, corrupt, partial, and unsupported
  behavior;
- normalized text, structure, encoding, offset, and provenance semantics;
- temporary plaintext and cleanup;
- extraction correctness oracle.

## Discovery and Model Requirements

The spec must define:

- sensitive-data classes, languages, and policy;
- deterministic and structural signals;
- entity, alias, span, class, relationship, conflict, and uncertainty behavior;
- automatic publication, review-required, block, and failure thresholds;
- false-negative, false-positive, unresolved, and extraction-failure measures;
- model source, identity, hash, conversion, runtime, configuration, inputs,
  outputs, networking, tools, training, logging, caches, persistence,
  cancellation, licensing, and evaluation when a model is in scope;
- deterministic validation of model candidates.

Perfect detection may not be an acceptance criterion.

## Token and Data-Boundary Requirements

The spec must define:

- token grammar, escaping, class semantics, normalization, repository scope,
  equality, stability, collision, missing, malformed, forged, stale, rotation,
  merge, split, deletion, and migration behavior;
- every plaintext, normalized, candidate, tokenized, redacted, encrypted, and
  derived-data copy point;
- allowed Human, AI, and Key Zone callers and callees;
- agent-repository contents and forbidden contents;
- private-vault schema, secrets, transactions, journals, migrations, backup,
  restore, retention, deletion, recovery, audit, corruption, and denied
  callers;
- staging validation, atomic-publication, cancellation, and last-valid-output
  behavior.

## macOS and FFI Requirements

The spec must define:

- minimum macOS version;
- Xcode, Swift, Rust, and target versions;
- application, extension, package, crate, feature, and linkage structure;
- Finder action and application dispatch;
- menu-bar, progress, notification, review, failure, and cancellation behavior;
- sandbox, signing, notarization, entitlements, app groups, security-scoped
  access, application container, and Keychain;
- FFI or IPC request, response, memory, string, byte, offset, error,
  concurrency, cancellation, panic, exception, and cleanup behavior;
- universal-binary, packaging, migration, and update behavior when relevant.

## Conditional Future-Service Requirements

When a future Agent Service, Key Service, remote detector, RAG, embedding, chat,
conversation, model-provider, or rendering surface is in scope, define:

- data classes transmitted and stored;
- plaintext exclusion;
- route, network, credential, and storage isolation;
- allowed caller and callee zones;
- authorization and audit;
- temporary plaintext and destruction;
- remote model training, tools, logs, caches, and persistence;
- retry, offline, degraded, cancellation, wrong-tenant, wrong-matter,
  stale-token, missing-token, and unauthorized-rendering behavior.

For the local pilot, this section must identify all these surfaces as out of
scope.

## Required Bindings

Bind exact paths for:

- Rust, Swift, C, and support implementation files;
- Cargo, Xcode, package, entitlement, signing, and build configuration;
- application and extension targets;
- schemas and migrations;
- source fixtures and evaluation datasets;
- model inputs, manifests, generated artifacts, and converted outputs;
- codegen inputs and generated bindings;
- tests and benchmarks;
- review and evidence artifacts;
- temporary, staging, vault, source, output, and security-boundary test
  directories.

An intentionally deferred path must name its blocker.

## Generated and Model Artifact Requirements

Define:

- source contract and owner;
- source and destination paths;
- exact generator or conversion command;
- deterministic formatting or conversion;
- artifact identity and hash;
- supported and rejected inputs;
- runtime API and configuration;
- licensing and provenance;
- privacy constraints;
- compile and application-size budget;
- reproducibility and check commands.

## Approval Checklist

A spec is not implementation-ready until:

- required reads are complete;
- measured object is precise;
- every applicable mandatory section is closed;
- pre-audit closure gate passed before audit;
- correctness and privacy oracles are defined;
- benchmark method is defined when performance is claimed;
- compile and application-size budgets are defined when relevant;
- code, build, test, generated, model, dataset, review, and evidence bindings
  are exact;
- the minimal-surface requirements and necessity bindings are complete;
- peer audit passed;
- implementation plan remains required.

## Stop Gates

Stop if:

- pre-audit closure is incomplete;
- upstream format, platform, model, storage, or dependency behavior is assumed
  without evidence;
- extraction, discovery, entity, token, trust-zone, vault, agent-repository,
  publication, macOS, FFI, generated-artifact, failure, or recovery behavior is
  ambiguous;
- correctness or privacy oracle is weak or missing;
- benchmark does not isolate the measured object;
- compile or application-size impact is ignored;
- bindings are incomplete;
- conditional future-service boundaries are missing when those services are in
  scope;
- peer audit has not passed.
