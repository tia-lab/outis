~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Codegen and Generated Artifacts

Version: 1.1
Status: active
Scope: generated code, bindings, schemas, fixtures, model conversions,
manifests, and adapters

## Purpose

Define how approved source contracts become generated code or artifacts. This
protocol does not approve a generator, binding tool, model converter, schema,
model, or runtime.

Generated tokenizers, detectors, rule tables, Rust-to-Swift bindings, storage
bindings, model artifacts, manifests, fixtures, future service clients, and
evidence reports require a spec, peer audit, and approved implementation plan.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- docs/protocols/spec_protocol.md;
- docs/protocols/implementation_protocol.md;
- approved spec, peer audit, and implementation plan;
- upstream generator, format, model, schema, platform, and dependency
  documentation.

## Source Contract

The source of truth must be explicit. Candidates include:

- normalized-document schemas;
- sensitive-data class and deterministic rule tables;
- token and manifest schemas;
- private-vault schemas and migrations;
- Rust-to-Swift interface definitions;
- platform entitlement or build inputs;
- approved model source artifacts and conversion manifests;
- synthetic fixture definitions and evaluation oracles;
- future service interface definitions.

The generator must receive explicit input, output, root, configuration, version,
and contract-hash inputs. It must not discover security intent from
unstructured implementation code.

## Generated Artifact Rules

1. Generated files and model artifacts are not edited by hand.
2. Each artifact has exactly one source contract and owner.
3. Each artifact has an exact reproducibility command.
4. Generated text includes a deterministic header when the format permits it.
5. Binary artifacts have recorded source identity, conversion command, hash,
   toolchain, configuration, and reproducibility limitations.
6. Generated artifacts contain no surface absent from the spec.
7. Unsupported inputs fail before use.
8. Checked and trusted boundaries remain distinct.
9. Deterministic ordering is preserved when equality, replay, audit, or
   benchmarks depend on it.
10. Generated files must not copy source documents, plaintext sensitive values,
    private-vault records, or secrets into source control or agent-facing
    artifacts.
11. Generated bindings must preserve the approved ownership, encoding, offset,
    error, concurrency, cancellation, panic, exception, and cleanup contract.
12. Generated model artifacts must preserve the approved input, output,
    privacy, licensing, and evaluation contract.
13. Generated fixtures must remain synthetic or explicitly approved.
14. Generated code must not compile unrelated adapters or future services.
15. Generated future-service code must preserve Agent Service plaintext
    exclusion and Key Service isolation.

## Model Artifact Rules

When conversion or packaging is in scope, define:

- original source and license;
- exact version and immutable identity;
- acquisition and integrity verification;
- preprocessing, vocabulary, tokenizer, or feature inputs;
- converter and version;
- target representation;
- precision or quantization changes;
- input and output names, shapes, encodings, and limits;
- runtime and platform assumptions;
- deterministic or variable inference behavior;
- network, tool, training, cache, logging, and persistence behavior;
- evaluation datasets and result paths;
- artifact hash and size;
- rollback and removal.

Model conversion is codegen. A model file appearing in the repository without
this evidence is unapproved.

## Binding Rules

Generated Rust, Swift, C, or IPC bindings must define:

- API version;
- symbol and type ownership;
- allocation and deallocation;
- UTF-8, Unicode, byte, and offset rules;
- nullable and empty behavior;
- error and status mapping;
- progress and cancellation;
- callback and thread behavior;
- panic and exception containment;
- generated-file location and build-target ownership;
- compatibility checks.

Bindings may not become a second source of domain or security rules.

## Compile and Application-Surface Rules

Specs must measure or bound:

- generated source lines;
- macro-expanded lines when relevant;
- Rust check and build time for affected crates;
- Swift and Xcode clean-build time for affected targets;
- dependency graph changes;
- generated binary size;
- model artifact size;
- final application and extension size;
- symbols or modules exposed across FFI;
- unrelated targets affected.

## Option and Rule Requirements

New generated options, annotations, detector rules, format rules, or model
settings require:

- name and source target;
- allowed values and defaults;
- invalid combinations;
- parser, descriptor, or converter behavior;
- runtime behavior;
- trust-zone and storage behavior;
- test cases;
- compatibility and migration risk.

## Stop Gates

Stop if:

- output cannot be regenerated;
- generated output requires manual edits;
- source, license, converter, version, configuration, or hash is missing;
- an option, rule, binding, or model contract is ambiguous;
- an unsupported input can reach runtime as valid;
- security behavior is inferred without an explicit contract;
- generated code contains handwritten domain branches;
- plaintext, vault data, or secrets could enter an agent-facing or committed
  artifact;
- model evaluation or privacy behavior is absent;
- compile, application-size, or dependency impact is unknown for a wide
  generated surface.
