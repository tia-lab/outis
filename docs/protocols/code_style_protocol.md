~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Code Style

Version: 1.2
Status: active
Scope: Rust, Swift, C-compatible interfaces, SQL migrations, generated
artifacts, and support code

## Purpose

Define minimum code discipline for Outis production and evidence code.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- approved spec and implementation plan;
- docs/protocols/implementation_protocol.md;
- docs/protocols/codegen_protocol.md when relevant;
- docs/protocols/testing_benchmark_protocol.md.

## Universal Rules

1. Validate before use.
2. Preserve deterministic ordering.
3. Use explicit typed failures.
4. Keep plaintext copies, allocations, caches, temporary files, logs, and
   ownership transitions explicit.
5. Do not add hidden defaults or unbound configuration.
6. Do not add behavior absent from the spec.
7. Keep modules, types, targets, and interfaces small.
8. Do not create public APIs by accident.
9. Do not put benchmark or synthetic-data logic in production targets.
10. Do not put the private vault, source files, or plaintext metadata in the
    agent repository.
11. Do not silently accept partial extraction, uncertain detection, model
    failure, token failure, vault failure, or publication failure.
12. Do not make privacy, performance, or compile claims inline.
13. Keep future remote service behavior out of the pilot.
14. Omit code that has no current requirement, invariant, acceptance-test, or
    evidence binding.
15. Reject unsupported behavior explicitly rather than adding speculative
    handling.

## Minimal-Surface Discipline

- Prefer direct, readable code over a general framework when both satisfy the
  current contract.
- Do not add plugin systems, factories, generic repositories, compatibility
  layers, future adapters, optional modes, feature flags, caches, concurrency,
  or extension points before the approved slice requires them.
- Do not create an abstraction for hypothetical reuse. A single-implementation
  boundary is justified only by a current isolation, substitution, FFI, or test
  contract.
- Validate untrusted input at its external or trust boundary. Convert it to a
  validated type and avoid duplicating the same defensive check internally.
- Implement supported edge behavior and privacy- or integrity-critical failure
  behavior. Return a typed unsupported failure for deferred behavior.
- Do not retain dead code, placeholder branches, unused configuration, or
  commented-out alternatives.
- Avoid unnecessary allocation, copying, persistence, and synchronization.
  Prefer a simpler bounded path until measurement identifies a bottleneck.
- Keep tests focused on the accepted path, explicit rejection path, and the
  applicable data-boundary and failure contracts.

## Rust Shape

Preferred order in non-trivial files:

1. imports;
2. constants;
3. configuration and contract types;
4. error and result types;
5. intentionally public entrypoints;
6. validation helpers;
7. domain, persistence, platform-adapter, or runtime core;
8. artifact and evidence helpers;
9. tests only when the approved plan permits colocated unit tests.

Library roots should export modules and intentional public items only.
Non-trivial logic belongs in named modules following the approved boundaries,
such as source, extract, detect, entity, token, vault, manifest, publish, ffi,
or error.

Rust runtime, persistence, extraction, model, FFI, codegen, security-boundary,
and measurement logic must not use:

- unwrap;
- expect;
- panic;
- todo;
- unreachable;
- unwrap_or to hide failure;
- unwrap_or_default to hide failure.

Use typed errors and explicit result propagation. A panic must not cross FFI.

Unsafe Rust requires:

- an exact spec and plan binding;
- a stated invariant for every unsafe block;
- boundary-focused tests;
- review of aliasing, lifetime, alignment, initialization, thread, unwind, and
  cleanup behavior.

## Swift Shape

Candidate responsibility order:

1. imports;
2. immutable constants;
3. value and configuration types;
4. typed errors;
5. protocols;
6. public application or adapter entrypoints;
7. validation and conversion helpers;
8. platform, UI, storage, model, or FFI implementation;
9. tests under approved target bindings.

Swift runtime, platform, storage, model, FFI, and security-boundary code must
not use:

- force unwrap;
- force cast;
- try with forced success;
- fatalError;
- preconditionFailure;
- assertion as runtime validation;
- try with discarded error when failure is material;
- implicitly unwrapped optionals to avoid initialization or ownership design.

Use throwing functions, typed results, validated optionals, and explicit error
mapping.

Swift concurrency must define:

- actor or thread ownership;
- cancellation propagation;
- Sendable boundaries when relevant;
- UI isolation;
- access to shared job, vault, and security-scoped state;
- cleanup if a task or extension terminates.

UI convenience must not swallow processing, vault, sandbox, or publication
failure.

## FFI and IPC Discipline

Every boundary must define:

- ABI and version;
- caller and callee;
- ownership and lifetime;
- allocation and deallocation owner;
- UTF-8, byte, Unicode, and offset semantics;
- nullable and empty distinction;
- error representation;
- progress and cancellation;
- callback, thread, actor, and reentrancy behavior;
- panic and exception containment;
- cleanup after partial failure;
- maximum payload and backpressure;
- treatment of plaintext buffers.

Do not pass language-native object layouts across FFI. Do not duplicate domain
rules on both sides of the boundary.

Generated bindings are permitted only through approved codegen.

## SQL and Migration Discipline

- Every schema and migration is spec-bound and versioned.
- Migrations are deterministic and transactional where the storage contract
  requires it.
- Plaintext, token, ciphertext, hash, identifier, and metadata columns are
  classified.
- Journal, write-ahead log, temporary, index, backup, and error surfaces are
  part of the privacy contract.
- Queries bind parameters; user or document values are not interpolated into
  SQL text.
- Wrong-repository, stale-schema, corrupt, partial, and migration-failure
  behavior is explicit.
- Destructive migration requires a bound backup, rollback, deletion, and audit
  contract.

## Sensitive-Data Discipline

When code touches source content, tokens, models, redaction, secrets, vaults,
logs, rendering, or storage:

- identify every plaintext buffer;
- identify tokenized, redacted, encrypted, and derived buffers;
- identify borrowed and owned views;
- identify Human, AI, and Key Zone boundaries;
- identify source, staging, agent-repository, vault, cache, log, and temporary
  locations;
- keep copy and lifetime boundaries explicit;
- test plaintext exclusion and cleanup.

## Platform Discipline

- Folder access begins and ends at approved lifecycle points.
- Finder integration dispatches; it does not acquire vault authority.
- Entitlements and app groups are minimal, target-specific, and spec-bound.
- Keychain items, groups, prompts, and accessibility are spec-bound.
- Signing, notarization, sandbox, and debug exceptions are not inferred.
- Paths are validated before use and are not logged in plaintext unless
  explicitly approved.
- Application or extension termination must not create partial publication.

## Compile-Surface Discipline

- Avoid compiling unrelated platform, extraction, model, vault, future service,
  benchmark, fixture, or generated surfaces.
- Avoid dependency-heavy adapters in the deterministic core.
- Do not embed model files or generated data into unrelated targets.
- Measure Rust crate, Swift target, generated-source, model-artifact,
  application-size, and clean-build impact when relevant.
