~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Implementation

Version: 1.2
Status: active
Scope: code, generated artifacts, model artifacts, build files, platform
configuration, and adapters

## Purpose

Control implementation after a spec, peer audit, implementation plan, and
explicit approval exist.

## Locked Refresh Rule

Before implementation, reread:

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- docs/protocols/spec_protocol.md;
- docs/protocols/code_style_protocol.md;
- docs/protocols/codegen_protocol.md when generated or model artifacts are
  touched;
- docs/protocols/testing_benchmark_protocol.md;
- approved spec, peer audit, and implementation plan;
- bound upstream platform, format, model, storage, service, data, and
  dependency contracts.

Memory of prior work is not evidence.

## Entry Conditions

Implementation may start only when:

- spec exists and is approved;
- peer audit passed;
- implementation plan exists and is approved;
- exact Rust, Swift, C, Xcode, build, schema, migration, entitlement, signing,
  FFI, adapter, model, test, benchmark, and evidence paths are bound;
- generated and model artifacts are bound;
- dependency and configuration changes are bound;
- failure, cancellation, retry, and recovery contracts are bound;
- correctness and privacy oracles are bound.

## Minimal-Surface Rule

Implement the smallest complete vertical slice in the approved spec and plan.
The default for code without a current necessity binding is omission.

- Every production file, module, target, public interface, abstraction,
  configuration item, dependency, and non-trivial branch must support a bound
  requirement, invariant, acceptance test, or evidence need.
- Do not add scaffolding, generic frameworks, plugin systems, compatibility
  layers, future-service adapters, optional modes, feature flags, caches,
  concurrency, or extension points for possible later use.
- Do not implement a wide set of edge cases. Implement the supported contract
  and the cases required to preserve privacy boundaries, determinism, vault
  integrity, atomic publication, cancellation, and recovery.
- Reject unsupported inputs and behavior explicitly at the narrowest approved
  boundary.
- Validate external and trust boundaries once and represent validated state in
  types. Do not repeat checks at every internal layer without a contract reason.
- Avoid unnecessary allocations, copies, persistence, dependencies, and
  compile surface. Optimize only against measurements of the approved path.

Minimality is not permission to omit an applicable invariant or silently
accept incomplete work.

## Implementation Rules

- Implement only the approved spec and plan.
- Prefer small modules and explicit types.
- Keep the default Outis runtime small.
- Keep macOS UI, Finder, extraction, model, vault, database, FFI, future
  service, benchmark, fixture, and generated-data surfaces isolated unless the
  spec proves otherwise.
- Validate source, trust-zone, sensitive-data, storage, and publication
  boundaries before reading, adapting, storing, or transmitting content.
- Keep plaintext buffers and copy points explicit and short-lived.
- Keep hot paths allocation-conscious.
- Do not use unchecked failure in runtime, persistence, platform, model,
  codegen, security-boundary, or measurement code.
- Do not add hidden defaults, unbound configuration, dependencies, targets,
  entitlements, generated artifacts, model files, or network access.
- Do not widen supported formats, languages, classes, routes, or measured
  object.
- Do not infer complete extraction from non-empty output.
- Do not trust model output without deterministic validation.
- Do not publish the source, vault, temporary files, sensitive metadata,
  private paths, or plaintext logs to the agent repository.
- Do not publish partial output on failure or cancellation.
- Do not place the private vault inside the source or agent repository.
- Do not expose token lookup or rendering as an agent tool.
- Do not create benchmark shortcuts outside the measured path.

For approved future services:

- do not send plaintext sensitive values to Agent Service, RAG, embeddings,
  model providers, agent tools, logs, telemetry, or AI-facing stores;
- do not expose Key Service routes to Agent Service or LLM tools;
- do not store token dictionaries or decryption keys in AI-facing stores;
- do not render plaintext without approved authorization and audit.

## Platform and FFI Rules

- Xcode targets, build phases, linkage, signing, entitlements, app groups,
  sandbox rules, persisted access, Keychain groups, and bundle contents must
  match the plan.
- Finder extensions dispatch approved requests only.
- FFI and IPC must enforce the approved ownership, encoding, offset, error,
  concurrency, cancellation, panic, exception, and cleanup contract.
- Rust panics and Swift or Objective-C exceptions must not cross the language
  boundary.
- Security-scoped access must end at the approved lifecycle point.
- Model and generated artifact identity must be checked before use.
- Debug-only behavior must not silently change privacy or correctness.

## Pre-Test Audit

Before testing, audit final changes against:

- approved spec and plan;
- code-style and codegen protocols;
- bound source and generated paths;
- expected test, benchmark, and evidence commands.

Confirm:

- every changed file was bound;
- every production surface and non-trivial branch has a current necessity
  binding;
- no deferred scaffolding or speculative generality was added;
- dependencies, model files, generated artifacts, build settings, targets,
  entitlements, and signing match the plan;
- source and extraction paths match the source contract;
- model and deterministic detector paths match the discovery contract;
- entity, token, vault, and publication behavior match the spec;
- trust-zone routes and plaintext copies match the spec;
- agent repository excludes forbidden content;
- cancellation, crash, retry, and recovery behavior match the spec;
- correctness and privacy oracles can test the implemented path;
- benchmark code measures the intended object;
- no unapproved behavior was added.

## Stop Gates

Stop implementation if:

- code requires behavior absent from the spec;
- a platform, format, model, storage, service, or dependency API contradicts a
  spec assumption;
- correctness or privacy oracle cannot be implemented;
- benchmark would measure a different object;
- plaintext copy, allocation, cache, journal, temporary file, or log behavior
  cannot be bounded;
- changes cannot stay within approved file and target bindings;
- generated or model output requires manual editing;
- model, source, vault, or agent boundaries cannot match the spec;
- partial publication or silent downgrade would be required;
- future Agent Service would receive plaintext or call Key Service;
- authorization or audit cannot match an approved future rendering contract.
