~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Repository Structure

Status: P1-02 approved; clean baseline and preflight pending
Scope: local macOS pilot repository

## Purpose

This document records the S1-bound source-code ownership boundaries for the
Outis pilot. It is not an implementation plan and does not authorize creation,
renaming, movement, dependency selection, code generation, model conversion, or
Xcode targets.

The exact path allowlist is incorporated by Section 37 of the working
specification and recorded in
`docs/roadmaps/outis_local_pilot_file_architecture.json`. Creation, removal, or
modification still requires the amended author pre-audit, a passed peer audit,
and an approved implementation plan.

## Current Workspace Shape

~~~text
Cargo.toml
  package: wuthier-terminal
  current root Rust binary scaffold

src/main.rs
  current executable stub

bin/
  repository maintenance tools

docs/
  architecture, invariants, protocols, and future lifecycle artifacts

AGENTS.md
  active Outis repository contract

initial-intake.md
  approved pilot intake

architecture.md
  candidate local pilot architecture

ROADMAP.json
  canonical phase and gate roadmap

inventory.md
  generated repository inventory; not manually editable
~~~

No Swift application, Xcode project, Finder extension, extraction adapter,
model adapter, private vault, or agent-export implementation exists.

## S1-Bound Workspace Shape

The phase sequence is defined in the root `ROADMAP.json`. The complete proposed
first-slice file map and per-file ownership are defined in
`docs/roadmaps/outis_local_pilot_file_architecture.json`. The latter is the
normative S1 path and responsibility allowlist through specification Section
37. This document owns the enduring
repository and compile-surface guardrails.

The proposed high-level shape is:

~~~text
crates/outis-core/
  dependency-light deterministic domain engine

crates/outis-runtime/
  approved local filesystem, strict text extraction, model, vault, and
  publication effects

crates/outis-ffi/
  narrow versioned Rust-to-Swift boundary

apps/macos/
  one Outis application target with native Word/PDF/OCR adapters and one
  unit-test target

generated/ffi/
  reproducible generated header when approved

models/
  approved model manifest and acquisition script; verified local artifacts are
  ignored and require prior legal clearance

fixtures/outis_local_pilot/v1/
  versioned synthetic extraction, detection, model, entity, publication,
  performance, and privacy oracles

tests/acceptance/
  test-only full-suite runner and controlled sandbox-boundary probe

tests/fixture_generation/
  three test-only Swift sources for deterministic Word and PDF fixtures

docs/
  architecture, invariants, protocols, roadmaps, specs, reviews, and evidence

artifacts/
  generated evidence only when a spec binds paths and retention

bin/
  repository maintenance tools only
~~~

The first slice has no Finder extension, CLI product, remote service,
format-specific crate, response renderer, or post-quantum service module. Rust
runtime owns strict `.txt`/`.md` extraction. Small Swift files in the existing
application target own AppKit `.doc`/`.docx`, PDFKit PDF, Core Graphics render,
and Vision OCR adapters. No extra extraction target or crate is created. No
directory may be created merely because it appears in an architecture
document. The specification, peer-audit, and implementation-plan gates remain
mandatory even for an allowlisted path.

The final workspace has three members. S1-20 stages that transition without
empty placeholders: the first implementation plan may create only
`crates/outis-core` and list only that member. Later crates enter the workspace
only with an approved complete capability that uses them.

The S1-20 root migration also creates `rust-toolchain.toml`, commits and stops
ignoring the dependency-free format-4 `Cargo.lock`, removes the obsolete root
`src/main.rs`, and removes stale `mbt_cache`-oriented `Makefile.toml` and
`release.toml`. S1-21 fixes the rustup locator to installed alias `stable` but
accepts it only when the Section 40 preflight observes the exact Rust 1.89.0
commit, Cargo identity, required components, and arm64 host and target while
distribution endpoints are redirected to unreachable loopback tripwires.
`inventory.md` is regenerated
only through its existing generator. These remain future implementation-plan
actions, not current documentation edits. The existing user-owned deletion of
`architecture-public.md` remains preserved.

## First Complete Implementation Increment

`MI-01` is exactly the pure Rust email-discovery capability in specification
Section 40. It accepts validated UTF-8 text plus its source-snapshot identity
and returns ordered email `SensitiveCandidateV1` records under the Section 11
grammar. Its production surface is limited to:

~~~text
crates/outis-core/Cargo.toml
crates/outis-core/docs/inventory.md
crates/outis-core/src/lib.rs
crates/outis-core/src/candidate.rs
crates/outis-core/src/detect.rs
crates/outis-core/src/detect/email.rs
crates/outis-core/src/detect/email/tests.rs
~~~

The component inventory is the handwritten input to the existing generated
root inventory. Unit tests live in the grammar's private
`detect/email/tests.rs` module; no integration-test or fixture path is created.
The fixed 65,536-record ceiling returns a typed all-or-nothing error before a
65,537th candidate can be retained. The increment has no registry
dependency and no application, Xcode, Swift, runtime, FFI, extraction, other
detector, entity, review, token, vault, export, production or test publication, generated
binding, model, job, or funding-demo behavior. It is a complete domain
capability with an exact input, output, terminal return, and oracle; it is not a
horizontal foundation. A later increment must pass its own applicable gates.

## Runtime Data Is Outside the Source Tree

Production or pilot runtime data must not be committed to this repository.

Conceptual runtime locations are:

~~~text
user-selected source repository
  Human Zone original binary and text input

separate generated agent repository
  AI Zone Markdown-only output rooted at outis/

private application storage
  Key Zone vault and private entity graph

temporary staging
  trusted local publication boundary
~~~

The source repository, agent repository, private vault, and staging location
are distinct. The export mirrors the source-relative tree and base document
names after required path tokenization, and changes every target extension to
`.md`. Exact macOS paths, containers, permissions, sandbox rules, and cleanup
behavior are bound by the working specification. Persistent folder bookmarks
are absent from the first slice.

The private vault must never be placed in:

- the agent repository;
- the original source repository;
- this source-code repository;
- fixtures, benchmarks, logs, or evidence artifacts.

## Ownership Model

### Outis Core

Candidate ownership:

- validated normalized-document input;
- deterministic ordering;
- candidate-span validation;
- span conflict resolution;
- entity and alias resolution;
- token contracts;
- deterministic serialization;
- publication validation;
- typed errors.

The core must not own macOS UI, Finder behavior, Keychain APIs, model runtimes,
format-heavy extraction dependencies, benchmarks, or real user data unless a
spec explicitly proves the boundary.

### Outis Runtime

Candidate ownership:

- approved local source enumeration, signature validation, and strict UTF-8
  text/Markdown extraction effects;
- approved on-device model adapter;
- approved private-vault adapter;
- staging, filesystem publication, cleanup, and recovery effects;
- composition of one local job from the deterministic core and approved
  adapters.

The runtime must not own macOS UI, duplicate core domain rules, expose agent
routes, or pre-create adapters for deferred formats and services. A separate
adapter crate requires measured dependency or isolation evidence.

### macOS Application

Candidate ownership:

- user interaction;
- folder selection and approved access;
- application and job lifecycle;
- progress and cancellation presentation;
- embedded Quick Look original-versus-normalized extraction review;
- Finder dispatch coordination only when later approved;
- sandbox, signing, and entitlement coordination;
- AppKit Word import, PDFKit page inspection/text, Core Graphics page
  rendering, and Vision revision 3 OCR;
- platform-adapter invocation and bounded extraction submission to Rust.

The application must not make platform UI behavior part of the deterministic
document result unless the spec defines it as an input.

### Conditional Future Finder Extension

Candidate ownership:

- validate the Finder invocation shape;
- pass an approved selection request to the application;
- report dispatch failure.

It must not:

- process the document repository;
- access or render the token dictionary;
- read Keychain secret material;
- publish partial output.

### Extraction Adapters

Candidate ownership:

- Rust runtime: content-signature and extension validation, source snapshot,
  strict `.txt` and `.md` decoding, normalized-document validation,
  deterministic source-to-target path mapping, and typed policy failure;
- Swift application: AppKit `.doc` and `.docx` import, PDFKit page inspection
  and text, Core Graphics fixed page render, Vision revision 3 OCR, private
  provenance, and platform failure;
- FFI: polling extraction request and bounded status/provenance plus separate
  UTF-8 text submission, with no callback or retained borrowed pointer.

Adapters must not silently discard unsupported content. Every Word and PDF
result requires review; detected embedded content and incomplete OCR block.
Exact selection and measured limits are in the R1.2 extraction evaluation.

### Model Adapters

Candidate ownership:

- approved model loading;
- model input and output conversion;
- artifact identity validation;
- inference configuration;
- explicit runtime failure.

Model output remains an untrusted candidate set.

S1 selects `Davlan/bert-base-multilingual-cased-ner-hrl` at the pinned
revision recorded in
`docs/reviews/outis_local_pilot/outis_local_pilot_ner_evaluation.md`, with an
ONNX Runtime 1.28 CPU execution binding. Model acquisition and bundling remain
blocked until the qualified model legal review is `CLEARED`. Before clearance,
the exact model source, tests, fixtures, dependencies, artifacts, bundle phases,
and temporary substitutes remain absent. The only pre-clearance implementation
capability is `MI-01`. It may proceed only after the amended author pre-audit,
repeated peer audit, and its exact implementation plan pass.

### Vault Adapters

Candidate ownership:

- approved storage schema;
- transactions and migrations;
- repository identity;
- entity and token records;
- secret-provider interface;
- corruption, recovery, retention, and deletion behavior.

The adapter must not expose a general agent-facing lookup or rendering route.

### FFI Surface

Candidate ownership:

- a narrow versioned boundary between Rust and Swift;
- explicit memory, encoding, offset, error, cancellation, and concurrency
  contracts;
- generated bindings only if approved codegen owns them.

FFI must not become a second domain model or duplicate business rules.

## Compile-Surface Rules

- Runtime users must not compile unrelated extraction, model, database,
  benchmark, or future-service adapters.
- The Finder extension must not link private-vault or model internals unless a
  spec proves necessity and isolation.
- The deterministic core must not require Swift, AppKit, SwiftUI, Finder,
  Keychain, SQLite, OCR, or model dependencies by default.
- Benchmarks, fixtures, synthetic generators, and evaluation tooling must not
  enter production targets.
- Model files and generated bindings must not be compiled into unrelated
  targets.
- Xcode target and Rust feature relationships require compile-surface evidence.
- Universal macOS binaries and deployment targets require explicit build
  evidence when supported.

## Schema and Artifact Ownership

Every schema or generated artifact requires one source of truth and one owner.

Potential schemas include:

- normalized-document schema;
- candidate-span and detector-evidence schema;
- private entity-graph schema;
- token and vault schema;
- agent-manifest schema;
- job-state and audit schema;
- Rust-to-Swift interface schema;
- model manifest;
- synthetic evaluation-oracle schema.

No Rust annotation, Swift type, database migration, model metadata file, or
generated binding may independently define security-critical behavior.

Generated files:

- are not edited by hand;
- have an approved input and generator;
- are reproducible by a checked command;
- record applicable contract or artifact identity;
- remain outside production targets unless required.

## Test and Evidence Ownership

Candidate separated surfaces:

~~~text
fixtures/outis_local_pilot/v1/extraction/
  synthetic format fixtures and extraction oracles

fixtures/outis_local_pilot/v1/detection/
  synthetic multilingual entity and span oracles

fixtures/outis_local_pilot/v1/publication/
  synthetic source, vault, and agent-repository boundary cases

tests/fixture_generation/
  test-only Word and PDF fixture generation and byte-replay checks

crates/outis-core/tests/
  deterministic detector, entity, token, and export contract tests

crates/outis-runtime/tests/
  extraction, vault, publication, cancellation, and recovery tests

crates/outis-ffi/tests/
  ABI and ownership contract tests

apps/macos/OutisTests/
  application-state, entity review, extraction review, FFI-client,
  folder-access, Word/PDF, OCR, and extraction-submission tests

tests/acceptance/
  argument-free acceptance orchestration and controlled sandbox probe

docs/reviews/
  research, peer audit, implementation plan, and result review
~~~

Real sensitive documents must not be added as fixtures.

## Conditional Future Services

Remote Agent Service, Key Service, RAG, embeddings, chat, rendering, and
Swiss-hosted review are outside the pilot. Their repositories, crates, clients,
schemas, routes, credentials, deployment manifests, and storage must not be
pre-created by the pilot.

A later approved spec may add them only with separate compile, network,
credential, storage, authorization, audit, and plaintext-exclusion boundaries.

## Non-goals

- No undocumented stack lock-in.
- No crate or Xcode target beyond the approved three Rust crates, one
  application target, and one unit-test target.
- No agent access to the source repository or private vault.
- No vault or source data in the generated agent repository.
- No benchmark or fixture support in production targets.
- No generated code or model artifact edited by hand.
- No real sensitive data committed as fixtures or evidence.
- No public privacy, security, performance, compile-time, or detection-quality
  claim without recorded evidence.
