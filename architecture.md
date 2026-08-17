~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local macOS Pilot Architecture

Status: R1 complete; ready for S1 drafting, no code authorization
Scope: documentation and research for the local macOS pilot

## Purpose

This document defines the candidate architecture that research and a later spec
must evaluate. It does not approve dependencies, code paths, data formats,
model artifacts, storage engines, cryptography, performance targets, or
security claims.

The pilot transforms a user-selected document repository containing at least
`.doc`, `.docx`, `.pdf`, `.txt`, and `.md` sources into a separate
Markdown-only repository named `outis`. Extraction, sensitive-entity discovery,
entity resolution, path tokenization, and content tokenization occur locally on
one Mac. The private entity graph and token dictionary remain in a separate
local Key Zone.

## Current Scope

The candidate pilot owns:

- a local macOS application;
- user-initiated one-shot processing;
- local source enumeration and extraction;
- automatic sensitive-entity discovery;
- corpus-wide entity and alias resolution;
- deterministic repository-scoped token assignment;
- generation and validation of a separate Markdown repository;
- private local entity-graph and token-dictionary coordination;
- progress, cancellation, review-required, failure, and completion states.

The candidate pilot does not own:

- continuous watching or synchronization;
- remote preprocessing;
- remote Agent Service or Key Service;
- RAG, embeddings, retrieval, chat, conversation memory, or response rendering;
- a Swiss-hosted verification model;
- final legal advice;
- production identity, authorization, retention, legal hold, backup, or
  disaster recovery.

## Architectural Terms

Source repository:

- the user-selected folder containing original binary or text documents;
- immutable from the pilot's perspective;
- inside the Human Zone;
- prohibited as an agent workspace.

Normalized document:

- a local intermediate carrying extracted UTF-8 and private source/page or
  extraction-mode provenance;
- a plain-text Markdown profile for binary formats, not a layout-faithful
  office-document representation;
- plaintext while it is inside trusted preprocessing;
- never an agent-facing artifact.

Private entity graph:

- discovered entities, aliases, relationships, provenance, decisions, and
  tokens;
- created and maintained automatically from the corpus;
- stored in the private local vault.

Agent repository:

- the generated Markdown-only repository named `outis`;
- an AI Zone store;
- physically separate from the source repository and private vault.

Publication:

- replacement of the current valid agent repository with a completely
  validated staged generation;
- never a partial in-place update unless a later spec proves equivalent
  correctness and recovery.

## Trust Zones

~~~text
+---------------------------- HUMAN ZONE -----------------------------+
| authorized user                                                   |
| original source repository                                        |
| Outis macOS application                                           |
| trusted local extraction and entity discovery                     |
| deterministic local tokenization                                  |
+------------------------------+-------------------------------------+
                               |
                               | approved token requests and records
                               v
+----------------------------- KEY ZONE ------------------------------+
| private local vault                                                |
| private entity graph and aliases                                   |
| token dictionary and sensitive mappings                            |
| no secret material in the synthetic funding-demo slice             |
+------------------------------+-------------------------------------+
                               |
                               | opaque tokens only
                               v
+------------------------------ AI ZONE ------------------------------+
| separate generated outis Markdown repository                       |
| agent-visible manifest and approved metadata                        |
| local or remote agents granted access to this repository only       |
+---------------------------------------------------------------------+
~~~

The agent repository must not be inside the source repository. The private
vault must not be inside either repository. R1 selects a user-chosen export
parent containing a fixed final export and same-filesystem sibling staging
path; S1 must ratify the exact enforcement contract.

## Component Model

### macOS Application

Candidate responsibilities:

- folder selection and approved access;
- job creation and lifecycle;
- menu-bar status and review entrypoint;
- embedded Quick Look original-versus-normalized extraction review;
- AppKit Word import;
- PDFKit page inspection, text extraction, and page rendering coordination;
- Core Graphics fixed page rendering and Vision revision 3 OCR;
- local platform adapter coordination;
- application-container coordination;
- cancellation and user-visible failure reporting.

The application is a trusted Human Zone coordinator. The funding-demo slice
has no Finder extension or Keychain surface. Finder integration remains a
later lifecycle increment and must not receive vault access or implement token
rendering.

### Deterministic Engine

Candidate responsibilities:

- source enumeration under explicit filesystem rules;
- normalized-document validation;
- deterministic structured detectors;
- candidate-span normalization;
- conflict and overlap resolution;
- corpus-wide entity and alias resolution;
- stable token assignment through a vault interface;
- Markdown and manifest serialization;
- staged-output validation;
- atomic-publication coordination.

Rust is the candidate implementation language. No crate split, dependency, API,
or foreign-function interface is approved.

### Extraction Adapters

Candidate responsibilities:

- identify supported formats;
- extract text and approved structure locally;
- report complete, partial, unsupported, encrypted, corrupt, or failed input;
- expose source provenance without publishing plaintext paths;
- identify metadata, comments, annotations, attachments, and other content
  surfaces relevant to completeness.

Binary extraction remains separate from the deterministic core because it is
platform-owned and version-sensitive. R1.2 selects AppKit for `.doc` and
`.docx`, PDFKit for PDF inspection and native text, Core Graphics for fixed
page rendering, and Vision revision 3 for OCR. These adapters are small Swift
files inside the application target. Rust owns content-signature and snapshot
validation, strict UTF-8 `.txt` and `.md`, normalized-document validation, and
all later deterministic policy. S1 must ratify the exact APIs and schemas.

### On-device Detection Adapter

R1 selects one exact embedded local model candidate for S1 ratification. It is
not approved for implementation or redistribution. The candidate:

- runs on the user's Mac;
- receives normalized plaintext only inside trusted preprocessing;
- emits candidate spans and classes rather than trusted final decisions;
- has no agent tools;
- has no network egress, training, or persistent plaintext logs under the pilot
  contract;
- is identified by source, artifact hash, conversion history, model version,
  runtime version, configuration, and supported-input contract;
- is evaluated on declared languages, formats, and datasets.

The deterministic engine validates and resolves model candidates. Model output
does not bypass failure, uncertainty, or publication gates.

### Private Local Vault

Candidate responsibilities:

- repository identity;
- private entity graph;
- aliases and provenance;
- token assignment and reuse;
- collision records;
- sensitive source mappings;
- extraction, detector, normalization, model, token, and schema versions;
- approved audit and recovery records.

R1 selects bundled SQLite as the S1 candidate for the synthetic funding-demo
vault. The candidate vault contains plaintext mappings and has no Keychain
secret; it is not suitable for confidential data. Encryption and Keychain
protection require a later approved research and specification cycle.

The vault must not expose a general token-rendering tool to agents. The pilot
does not include response rendering.

### Agent Repository

Candidate contents:

- tokenized or redacted Markdown;
- source-relative directories and base names after required path
  tokenization;
- an approved manifest;
- approved non-sensitive processing metadata.

Forbidden contents:

- original source files;
- plaintext sensitive source-relative path or filename components;
- document metadata not explicitly classified for publication;
- private entity graph or token dictionary;
- plaintext sensitive mappings;
- vault database or secret material;
- plaintext extraction intermediates;
- sensitive logs;
- incomplete files represented as complete.

## End-to-End Flow

~~~text
1. user selects source folder
2. application validates source and destination boundaries
3. application obtains local access for the job
4. engine enumerates a deterministic source snapshot
5. format adapters extract .doc, .docx, PDF, .txt, and .md into normalized documents
6. deterministic detectors produce candidate spans
7. the selected offline on-device detector produces additional untrusted candidate spans
8. engine resolves spans, entities, aliases, classes, and conflicts
9. vault assigns or reuses repository-scoped tokens
10. engine mirrors the tokenized relative tree and serializes one .md per source
11. publication oracle validates the full staging repository
12. application atomically publishes or preserves the previous valid output
13. private job and audit state is committed under the storage contract
14. application reports the terminal job state
~~~

Every plaintext copy, temporary file, IPC message, FFI buffer, model input, log,
and error surface must be identified by the future spec.

## Job State Model

Candidate states:

~~~text
requested
  -> validating
  -> extracting
  -> detecting
  -> needs_review when required
  -> tokenizing
  -> validating_export
  -> publishing
  -> completed
~~~

Terminal or interrupting states:

~~~text
needs_review
blocked
cancelled
failed
~~~

R1 selects these labels for S1. Transition persistence, restart behavior, and
exact cancellation points still require S1 bindings.

Review-required must not be equivalent to published. Completed must not be
described as proof of perfect discovery.

## Source Enumeration

The source contract must define:

- snapshot identity;
- deterministic path ordering;
- required `.doc`, `.docx`, `.pdf`, `.txt`, and `.md` variants;
- hidden files and packages;
- symlinks, hard links, Finder aliases, mounts, and path traversal;
- nested repositories;
- source changes during a run;
- source permissions and stale access;
- maximum file, repository, and extracted-content sizes;
- exclusion of source and output destinations from each other;
- treatment of filenames and folder names as possibly sensitive.

The pilot must not follow an unapproved link outside the selected source
boundary.

## Extraction Contract

The earlier `.txt` and `.md`-only source candidate was incorrect and is
withdrawn. R1.2 selects:

- strict Rust UTF-8 decoding for `.txt` and `.md`;
- AppKit attributed-string import for legacy `.doc` and Open XML `.docx`;
- PDFKit page inspection and text for text-bearing PDF pages;
- PDFKit/Core Graphics page rendering and Vision revision 3 for pages without
  native text;
- accurate OCR at 200 DPI with `it-IT`, `de-DE`, and `fr-FR` in fixed order,
  automatic language detection enabled, language correction disabled, and one
  page request at a time.

The normalized output is a plain-text Markdown profile, not a layout-preserving
conversion. Source Markdown and text are preserved after strict BOM/UTF-8/NUL
checks. Binary-derived line endings and page joins are canonical. Page and
extraction-mode provenance remains private.

Every Word and PDF result, including OCR, requires review. The user inspects an
embedded Quick Look original beside complete normalized text, then confirms the
exact source and normalized hashes or rejects the job. Confirmation is not
completeness proof. Lists, tables, columns, headers, footers, notes, tracked
changes, annotations, forms,
attachments, and embedded content can flatten or be omitted. Detected embedded
content, corrupt/signature-mismatched input, encrypted/locked/copy-disallowed
PDF, unavailable required OCR language, empty OCR page, limit violation, or
source mutation blocks publication. No input is silently truncated,
downscaled, skipped, or called complete because it produced non-empty text.

Exact configuration, synthetic measurements, resource guardrails, and
limitations are in the R1.2 extraction evaluation. S1 must bind the final
schemas, fixtures, APIs, status types, and tests before the format is called
implemented.

## Automatic Discovery

The candidate multi-signal flow is:

~~~text
normalized document
  -> deterministic structured detectors
  -> document-structure signals
  -> selected fixed on-device detector
  -> cross-document consistency signals
  -> candidate-span set
  -> deterministic conflict resolution
  -> entity and alias resolution
  -> sensitivity and uncertainty decision
~~~

The private entity graph is produced by this flow. A complete user-authored
glossary is not an input requirement.

R1 active class candidates are person, organization, postal address, email
address, telephone number, IBAN, and matter identifier. Government identifiers,
credentials, identifying dates or events, and context-sensitive confidential
passages are unsupported in the funding-demo slice. Italian, German, and
French are required; exact rules and thresholds are in the R1 closure for S1
ratification.

## Entity and Alias Resolution

Entity resolution preserves provenance and avoids silent merging. R1 links only
byte-identical same-class equality keys. Any fuzzy, abbreviation, surname,
cross-class, or overlap decision requires explicit review. Every unresolved
candidate blocks complete publication. S1 must bind canonical identities,
review changes, provenance schema, and replay behavior.

## Tokenization

Tokens preserve approved class and repository-local equality semantics without
embedding plaintext. R1 selects class-local, persistent, zero-padded sequential
allocation after deterministic candidate ordering. S1 must define:

- grammar and escaping;
- repository scope and cross-repository unlinkability;
- class-specific normalization;
- stability and rotation;
- lookup and assignment behavior;
- collision detection;
- missing, stale, malformed, and forged tokens;
- token-like text already present in source;
- alias-to-token behavior;
- deletion and entity-split or entity-merge migrations.

Source-derived and unkeyed-digest tokens are excluded. No cryptographic token
construction is selected.

## Determinism

The deterministic measured object must bind at least:

- source snapshot and enumeration rules;
- normalized-document representation;
- extraction adapter, operating-system build, framework environment, OCR
  revision/configuration, supported-language result, and render identity;
- detector rules and versions;
- model artifact, runtime, and configuration when applicable;
- entity-resolution and normalization versions;
- private-vault state and repository scope;
- token contract;
- serializer and manifest versions;
- output configuration.

Byte-identical output may be claimed only when the spec identifies all
state-dependent and platform-dependent inputs and replay evidence passes.

Timestamps, random identifiers, filesystem order, model nondeterminism, locale,
Unicode behavior, operating-system behavior, and concurrent source mutation
must not enter the deterministic result without an explicit contract.

## Storage and Secret Boundary

The private-vault contract must define:

- application-container location;
- database schema and migrations;
- record encryption and secret ownership if approved;
- Keychain accessibility and application identity if approved;
- transactions and crash consistency;
- backup, restore, deletion, retention, and recovery;
- vault corruption and version mismatch;
- repository identity and wrong-repository behavior;
- plaintext columns, indexes, journal files, temporary files, and logs;
- allowed callers and denied callers;
- agent-boundary denial test.

Filesystem placement alone does not prove agent isolation. The threat model
must distinguish sandboxed agents, unsandboxed same-user processes, compromised
applications, and physical-device compromise.

## Agent Repository Contract

The publication oracle must inspect the complete staged artifact.

Candidate checks include:

- only allowed file types and paths exist;
- no source or vault files are present;
- no known sensitive values remain under the declared oracle;
- every source document has one corresponding `.md` target;
- the relative tree and base names match after required path tokenization;
- no plaintext sensitive path or filename component is published;
- output-path collisions block rather than overwrite;
- every emitted token is syntactically valid and belongs to the repository
  scope;
- every source item has an explicit processed, ignored, review-required,
  blocked, or failed record;
- unsupported or incomplete extraction is not represented as successful;
- manifest and Markdown ordering are deterministic;
- staging and destination are on an approved publication boundary;
- cancellation or failure cannot expose a partial repository.

A known-value scan can prove only that declared known values were absent. It
cannot prove that discovery found every sensitive value.

## macOS Application Boundary

Swift with native macOS frameworks is the candidate platform language.

The S1 spec must bind:

- minimum macOS version;
- Xcode and Swift versions;
- one application and one test target;
- signing, notarization, sandbox, and entitlement decisions;
- ephemeral folder selection access with no persisted bookmark;
- job lifetime if the application terminates;
- progress event semantics;
- menu-bar and notification behavior;
- cancellation and retry;
- application-container ownership and absence of a Keychain item;
- update and migration behavior.

Finder integration is deferred. If later approved, it is a command surface,
not the processing engine, and must not receive direct vault access.

## Rust and Swift Boundary

Rust is the candidate deterministic engine language. Swift is the candidate
platform language.

R1 selects an in-process Rust `staticlib`, version-one C ABI, opaque handles,
pointer-length UTF-8 JSON messages, polling, cancellation, and explicit buffer
release for S1 ratification. S1 must define:

- ownership of every request, response, string, byte buffer, and error;
- UTF-8 and offset semantics;
- synchronous and asynchronous behavior;
- progress and cancellation;
- panic and exception containment;
- thread and actor requirements;
- model and extraction adapter calls;
- extraction polling requests and the candidate
  `outis_job_submit_extraction` entrypoint, which carries bounded plaintext as
  a separate pointer-length buffer rather than inside JSON;
- security-scoped access lifetime;
- generated-header ownership and its exact cbindgen command;
- static linkage and ONNX Runtime dynamic-library packaging;
- debug and release behavior;
- crash and recovery semantics.

R1 selects cbindgen 0.29.4 as the generated-header candidate. This is not a
dependency, generated-artifact, or build approval.

## Failure and Recovery

Explicit failures include:

- unsafe or inaccessible source or destination;
- source mutation during processing;
- unsupported, corrupt, encrypted, or partial extraction;
- detector or model failure;
- uncertain or conflicting sensitive spans;
- entity merge or split conflict;
- token collision or vault mismatch;
- vault corruption or migration failure;
- stale folder access;
- invalid token-like source text;
- output validation failure;
- cancellation;
- insufficient storage;
- crash during staging or publication;
- attempted source, vault, or temporary-artifact publication.

No failure may silently downgrade extraction, detection, tokenization, vault
isolation, or publication safety.

## Evidence and Measurement

Correctness and privacy evidence precede performance evidence.

Research and later specs must define:

- synthetic and approved evaluation datasets;
- language and document-format coverage;
- extraction completeness oracle;
- entity span, class, alias, and relationship oracle;
- false-negative, false-positive, unresolved, and failure measurements;
- deterministic replay oracle;
- known-plaintext exclusion oracle;
- vault isolation and agent-access oracle;
- atomic-publication and recovery oracle;
- compile-surface and application-size budget;
- end-to-end and stage-specific performance methodology.

Results apply only to the recorded inputs, environment, versions, and threat
model.

## Deferred Evolution

A separate future phase may research a self-hosted second-pass model on
controlled Swiss infrastructure. It may not be added to the pilot by
implementation convenience.

Remote preprocessing, Agent Service, Key Service, RAG, embeddings, chat,
conversation memory, and rendering each require separate research, specs,
audits, route isolation, authorization, storage, failure, and privacy evidence.

## Required S1 Closure

R1.1 selected the contextual detector and R1.2 selected the extraction path.
S1 may now draft the complete schemas, crates, targets, SQL, manifests,
commands, fixtures, failures, and budgets, but must ratify every R1 candidate.
Model legal clearance, full Xcode and signing identity, sandbox runs,
repository-owned fixtures, end-to-end validation, and crash-recovery evidence
remain implementation or readiness blockers. No code is authorized until S1,
A1, and P1 pass.
