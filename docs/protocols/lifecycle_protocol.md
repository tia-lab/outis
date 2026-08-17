~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Lifecycle

Version: 1.1
Status: active
Scope: entire repository

## Purpose

Define the mandatory lifecycle for Outis research, design, implementation,
validation, and review.

The active pilot is a local macOS document-pseudonymization system. Prototype
status does not permit unclear source or output contracts, unverifiable
detection or privacy claims, undocumented token behavior, manual
generated-artifact edits, unbound platform entitlements, or plaintext leakage
into agent-facing surfaces.

Future remote services remain conditional and require their own approved
contracts.

## Required Reads

Before any non-trivial task:

- AGENTS.md;
- docs/invariants/core_invariants.md;
- this protocol;
- the task-specific protocol;
- relevant architecture;
- existing specs and reviews;
- target source and generated-artifact paths when implementation is requested;
- upstream platform, format, model, storage, service, data, and dependency
  contracts when behavior matters.

## Phase 0: Intake

Required output:

- goal restatement;
- task class;
- required reads and inputs;
- source-data owner;
- affected Human, AI, and Key Zone surfaces;
- affected sensitive-data classes;
- format, language, model, storage, platform, and generated-artifact surfaces;
- unknowns;
- stop/go risks.

No code is allowed.

## Phase 1: Context Read

Required output:

- protocols read;
- architecture and documentation read;
- specs and reviews read;
- source and generated paths read;
- platform, format, model, storage, service, data, and dependency contracts read
  when behavior matters;
- observed constraints;
- open hypotheses.

No code is allowed.

## Phase 2: Research Brief

Output path:

- docs/reviews/[slug]/[slug]_research_brief.md

For the local pilot, research must identify:

- measured source-to-agent-repository transformation;
- source formats and extraction fidelity;
- target languages and sensitive-data classes;
- automatic discovery and entity-resolution candidates;
- tokenization and determinism inputs;
- local model and model-artifact boundary when relevant;
- source, staging, agent-repository, and private-vault separation;
- macOS, Swift, Rust, FFI, Finder, sandbox, Keychain, and storage candidates;
- correctness, privacy, recovery, and publication oracle candidates;
- evidence required before spec.

No code is allowed.

## Phase 3: Spec

Output path:

- docs/specs/[slug]_SPEC.md

The spec must pass docs/protocols/spec_protocol.md and close every applicable
source, extraction, detection, entity, token, storage, platform, FFI,
generated-artifact, model, publication, failure, test, and evidence contract.

Conditional future Agent Service, Key Service, RAG, chat, rendering, or remote
model sections are required only when those surfaces are in scope; otherwise
the spec must mark them out of scope.

No code is allowed.

## Phase 4: Peer Audit

Output path:

- docs/reviews/[slug]/[slug]_peer_audit.md

The audit must try to falsify the spec and classify exactly:

- PEER_AUDIT_PASSED
- BLOCKED

No code is allowed.

## Phase 5: Implementation Plan and Approval

Output path:

- docs/reviews/[slug]/[slug]_implementation_plan.md

The plan binds every:

- file to edit or create;
- crate, Swift package, Xcode target, and build file;
- FFI or IPC surface;
- entitlement, signing, sandbox, app-group, Finder, and Keychain artifact;
- extraction, model, storage, and future service adapter;
- generated file and model artifact;
- test, benchmark, synthetic dataset, and evidence path;
- validation command and expected output;
- risk and rollback boundary.

No code is allowed until the plan is explicitly approved.

## Phase 6: Implementation

Code changes may start only after Phase 5 approval.

Implementation must stay within the approved spec and plan. New platform,
format, model, storage, dependency, configuration, generated-artifact, or
service behavior requires lifecycle restart at the applicable earlier phase.

## Phase 7: Testing and Benchmarking

Testing follows docs/protocols/testing_benchmark_protocol.md.

For the pilot, extraction correctness, detection measurement, private-vault
isolation, agent-repository plaintext exclusion, deterministic replay,
atomic-publication, failure, cancellation, and recovery evidence precede
performance claims.

## Phase 8: Result Review

Output path:

- docs/reviews/[slug]/[slug]_result_review.md

The result review states:

- what was built and measured;
- source, dataset, model, platform, storage, and build identities;
- what passed and failed;
- what is proved;
- what remains unproved;
- applicable interpretation limits;
- whether work should continue, be redesigned, be abandoned, or be promoted.

## Stop Gates

Stop if any applicable condition holds:

- required reads are incomplete;
- the measured object is unclear;
- source ownership, source formats, snapshot, or enumeration rules are unclear;
- extraction completeness and failure behavior are unclear;
- sensitive-data classes, languages, uncertainty, or entity-resolution
  behavior are unclear;
- tokenization, normalization, equality, collision, scope, or rotation is
  unclear;
- Human, AI, or Key Zone boundaries are unclear;
- source, staging, agent-repository, or private-vault separation is unclear;
- local model identity, runtime, supported inputs, network, logging,
  persistence, evaluation, or failure behavior is unclear when relevant;
- macOS version, Xcode target, Swift/Rust boundary, sandbox, entitlement,
  signing, Finder, Keychain, or application lifecycle is unclear when relevant;
- storage, migration, recovery, deletion, or audit behavior is unclear;
- generated-code or model-artifact ownership is unclear;
- correctness, privacy, publication, or recovery oracle is missing;
- spec is missing or unapproved;
- peer audit is missing or blocked;
- implementation plan is missing or unapproved;
- benchmark method cannot prove the intended claim;
- compile-surface or application-size impact is relevant but unmeasured.

For conditional future services, also stop when Agent Service plaintext
exclusion, Key Service isolation, remote model processing, authorization,
rendering, network, credential, or remote-storage behavior is unclear.
