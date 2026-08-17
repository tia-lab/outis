~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Research

Version: 1.1
Status: active
Scope: research-only work

## Purpose

Research builds enough evidence to decide whether a design deserves a spec. It
does not authorize implementation, dependency selection, generated artifacts,
model conversion, or platform configuration.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- this protocol;
- relevant architecture, specs, and reviews;
- relevant source and generated paths;
- upstream format, platform, model, storage, service, data, and dependency
  documentation when behavior matters.

## Research Output

Write:

- docs/reviews/[slug]/[slug]_research_brief.md

The brief must include:

- status;
- source materials;
- measured object;
- candidate approach;
- source data and owner;
- affected trust zones;
- affected sensitive-data classes;
- expected formats and languages;
- extraction, detection, entity-resolution, token, storage, model, platform,
  FFI, publication, and future-service surfaces when relevant;
- evidence table;
- hypotheses;
- unknowns;
- risks;
- correctness and privacy oracle candidates;
- required decisions before spec;
- recommended next phase.

## Evidence Table

Every brief must separate:

- code-read evidence;
- run evidence;
- build evidence;
- storage evidence;
- security-boundary evidence;
- benchmark evidence;
- data-contract and extraction evidence;
- model-artifact and evaluation evidence;
- platform, signing, sandbox, and entitlement evidence;
- external-doc evidence;
- hypotheses.

Unavailable evidence must be marked unavailable or not yet measured. Empty
categories must not be presented as passed.

## Local Pilot Research Minimum

Research for the Outis local pilot must identify:

- source owner and authorization assumptions;
- source snapshot, enumeration, link, package, and mutation behavior;
- candidate document formats and extraction completeness risks;
- candidate normalized-document representation;
- filenames, paths, metadata, comments, revisions, annotations, attachments,
  and embedded-content treatment;
- target languages and sensitive-data taxonomy;
- automatic discovery, structure signals, and deterministic detector
  candidates;
- on-device model necessity and boundary when relevant;
- model source, identity, conversion, runtime, configuration, supported inputs,
  licensing, and evaluation requirements;
- entity, alias, class, span, conflict, and uncertainty semantics;
- token grammar, normalization, scope, stability, equality, collision,
  token-like input, and rotation candidates;
- Human Zone plaintext copy points;
- AI Zone agent-repository contents and access assumptions;
- Key Zone private-vault, secret, storage, migration, retention, deletion,
  recovery, and denied-caller boundary;
- staging, validation, atomic-publication, cancellation, and recovery behavior;
- macOS version, Finder, sandbox, security-scoped access, signing, entitlement,
  Keychain, application-lifecycle, and progress surfaces;
- Rust, Swift, FFI, crate, target, generated-binding, dependency, compile, and
  application-size impact;
- synthetic or approved dataset strategy;
- extraction, detection, entity-resolution, determinism, known-plaintext,
  vault-isolation, publication, and recovery oracle candidates;
- expected failure cases.

## Conditional Future-Service Minimum

When a future remote service is in scope, research must additionally identify:

- allowed caller and callee trust zones;
- plaintext, tokenized, redacted, encrypted, and derived data transmitted or
  stored;
- Agent Service plaintext exclusion;
- Key Service route and credential isolation;
- remote model training, network, logging, temporary storage, and destruction;
- RAG, embedding, conversation, retrieval-log, and telemetry boundaries;
- rendering authorization and audit;
- wrong-tenant, wrong-matter, stale-token, retry, offline, and degraded
  behavior.

These fields are marked out of scope for the local pilot; they are not silently
omitted.

## Stop Gates

Stop research escalation if:

- source ownership is unknown;
- source format and extraction risks cannot be stated;
- sensitive-data classes or target languages cannot be stated;
- automatic discovery and uncertainty cannot be bounded;
- tokenization or agent-repository contract cannot be stated;
- private-vault or agent-access threat model cannot be stated;
- model boundary or evaluation cannot be stated when a model is relevant;
- platform, FFI, storage, or generated-artifact impact is ignored;
- correctness and privacy oracle candidates are missing;
- dependency or upstream behavior is assumed but not read;
- performance baseline cannot be defined when performance is claimed;
- future service route or storage boundaries cannot be stated when those
  services are in scope.
