~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Testing and Benchmarking

Version: 1.1
Status: active
Scope: tests, evaluations, benchmarks, security-boundary checks, and evidence
artifacts

## Purpose

Compilation is not proof. A detection score, privacy scan, or benchmark number
is not proof unless its extraction, correctness, dataset, trust, state, and
measurement boundaries are established.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- docs/protocols/implementation_protocol.md;
- docs/protocols/code_style_protocol.md;
- docs/protocols/codegen_protocol.md when relevant;
- approved spec, peer audit, and implementation plan;
- implemented code, build, schema, migration, model, generated, test, and
  evidence paths.

## Entry Conditions

Testing may start only when:

- implementation and pre-test audit are complete;
- test, evaluation, benchmark, dataset, model, storage, platform, and evidence
  paths are bound;
- correctness and privacy oracles are implemented or ready;
- source formats, languages, classes, versions, and threat model are recorded.

## Test Categories

Every change covers applicable categories:

- A: contract validation and typed failure;
- B: source snapshot, enumeration, mutation, path, link, package, and boundary
  behavior;
- C: per-format extraction completeness and normalized-document correctness;
- D: deterministic structured detectors;
- E: model loading, identity, configuration, supported input, output,
  cancellation, network, logging, cache, persistence, and failure;
- F: entity, alias, class, relationship, span, overlap, and conflict resolution;
- G: false-negative, false-positive, unresolved, and extraction-failure
  evaluation;
- H: token grammar, escaping, normalization, equality, scope, stability,
  collision, malformed, missing, stale, forged, rotation, merge, split, and
  token-like source input;
- I: private-vault schema, transaction, migration, journal, temporary-file,
  corruption, wrong-repository, backup, restore, deletion, and recovery;
- J: agent-repository allowlist, known-plaintext, filename, path, metadata,
  source-file, vault-file, log, cache, and temporary-file exclusion;
- K: deterministic replay with every stateful, platform, extraction, model,
  vault, token, and serializer input bound;
- L: staging, full validation, atomic publication, cancellation, crash, retry,
  cleanup, and last-valid-output preservation;
- M: macOS application, Finder dispatch, folder access, sandbox, signing,
  entitlement, app-group, Keychain, progress, lifecycle, and termination;
- N: Rust, Swift, FFI, IPC, memory, encoding, offset, error, concurrency, panic,
  exception, cancellation, and cleanup;
- O: generated-code, binding, schema, fixture, and model-artifact
  reproducibility;
- P: dependency, target, feature, compile-surface, application-size, and bundle
  contents;
- Q: runtime time, memory, storage, throughput, and cancellation when
  performance is claimed;
- R: conditional future Agent Service, Key Service, RAG, embedding, chat,
  rendering, authorization, and remote-model boundaries when approved.

## Dataset Requirements

Every dataset records:

- owner and approval;
- synthetic, pseudonymized, or real-data classification;
- immutable identity and hash;
- generation or acquisition path;
- languages and jurisdictions represented;
- document formats and variants;
- sensitive-data classes;
- span, class, entity, alias, relationship, and uncertainty annotations;
- extraction oracle;
- train, development, calibration, and test separation when a model is
  involved;
- known limitations and missing populations;
- storage, access, retention, cleanup, and audit.

Real sensitive data is prohibited unless the spec explicitly approves its
boundary.

## Detection Evaluation Requirements

Reports must record:

- exact detector rules and versions;
- model source, artifact hash, runtime, configuration, and thresholds;
- extraction adapter and normalized-document version;
- dataset identity;
- language, format, and class slices;
- true positives, false positives, false negatives, unresolved cases, and
  extraction failures under the approved oracle;
- micro, macro, per-class, per-format, and per-language measures when required;
- confidence calibration method when confidence drives publication;
- entity and alias resolution results separately from span detection;
- failed, timed-out, cancelled, and unsupported inputs;
- raw result artifact path.

A result applies only to the recorded dataset and configuration. No result
justifies a perfect-detection claim.

## Privacy and Boundary Requirements

Privacy tests must identify:

- allowed plaintext source locations and buffers;
- forbidden agent-repository values, files, paths, metadata, logs, caches, and
  temporary artifacts;
- private-vault location, schema, secret provider, allowed callers, and denied
  callers;
- tested agent process and its sandbox or filesystem authority;
- source, vault, staging, destination, and application-container permissions;
- future route callers, callees, credentials, stores, and authorization when
  relevant.

A scan for known plaintext proves only absence of the declared values in the
scanned surface. It does not prove complete discovery or absolute
inaccessibility.

## Benchmark Evidence

Record:

- UTC timestamp;
- operator;
- git commit and dirty state;
- command;
- build profile and target;
- CPU, RAM, OS, filesystem, and kernel;
- Rust, Swift, Xcode, linker, and relevant platform versions;
- application signing, sandbox, and entitlement state when relevant;
- input and dataset identity;
- source format, language, file count, byte count, page count, extracted byte
  count, entity count, and token count when relevant;
- extraction, detector, model, vault, token, and serializer versions;
- storage identity, schema, journal mode, and warm/cold state when relevant;
- model load and warm/cold state;
- result summary and raw output path;
- failures and instability.

## Compile and Application-Surface Evidence

Record:

- command and profile;
- clean or dirty state;
- Rust crate, Swift target, and enabled features;
- dependency graph condition;
- wall, user, and system time when available;
- maximum memory when available;
- generated and macro-expanded source size when relevant;
- model artifact and generated binding size;
- application, framework, library, and extension size;
- bundle contents;
- universal architectures and deployment target;
- unrelated targets compiled.

## Performance Claim Rules

- Correctness, extraction, detection, privacy, deterministic replay,
  publication, and recovery pass before speed claims.
- Report median, tail, throughput, peak memory, and output size when required.
- Separate enumeration, extraction, deterministic detection, model inference,
  entity resolution, vault access, tokenization, serialization, validation,
  publication, and cleanup costs.
- Separate cold and warm storage and model states.
- Compare systems only with identical logical payloads and oracles.
- Do not hide failed or unstable runs.
- Do not count benchmark-only fixtures as production footprint unless the spec
  includes them.

## Failure Rule

When a test, evaluation, or benchmark fails, assume first that implementation,
data, oracle, or measurement design may be wrong.

Do not change expected values, thresholds, tolerances, datasets, or scope until
independent evidence proves the original contract was wrong and the lifecycle
approves the change.

## Exit Conditions

Testing is complete only when:

- mandatory categories pass or blocked categories are justified;
- extraction, detection, entity, privacy, vault, determinism, publication, and
  recovery oracles pass for the approved scope;
- generated and model artifacts reproduce when applicable;
- compile and runtime evidence exists when claimed;
- failures and interpretation limits are recorded;
- result review is ready.
