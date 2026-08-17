~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local macOS Pilot Intake

Status: approved intake; R1 complete and ready for S1 drafting
Product: Outis
Scope: local macOS pilot

## Purpose

Outis is intended to let an authorized macOS user create an agent-facing
representation of a local document repository without intentionally providing
the agent with plaintext sensitive values or the private token dictionary.

The pilot is a one-shot local transformation:

~~~text
source folder containing .doc, .docx, .pdf, .txt, and .md
  -> local extraction
  -> normalized local plaintext
  -> local automatic sensitive-entity discovery
  -> local entity resolution
  -> local deterministic tokenization
  -> separate Markdown-only repository named outis
~~~

This intake records product intent. It is not a specification, implementation
plan, security proof, dependency approval, or performance claim.

## Problem

A normal local document folder may contain:

- text and office documents;
- text-bearing or scanned PDFs;
- filenames and folder names containing client or matter information;
- document metadata, headers, footers, comments, tables, annotations, and
  attachments;
- repeated people, organizations, addresses, matters, identifiers, and aliases.

An agent can use document meaning and relationships without necessarily
receiving the real identifying values. Outis is intended to create a stable
symbolic representation for that use.

The central research problem is automatic discovery. The user is not expected
to provide a complete glossary before processing. Outis must propose and
maintain the private entity graph from the corpus itself.

## Pilot Goal

The pilot goal is to measure whether a local macOS application can:

1. accept a user-selected document folder containing at least `.doc`, `.docx`,
   `.pdf`, `.txt`, and `.md` sources;
2. extract supported content locally;
3. discover candidate sensitive entities automatically;
4. resolve aliases and repeated entities across documents;
5. assign stable repository-scoped tokens;
6. produce Markdown containing tokens instead of accepted plaintext spans;
7. publish the Markdown to a separate agent-facing repository;
8. store the private entity graph and token dictionary outside that repository;
9. report unsupported, failed, uncertain, or partially processed content;
10. reproduce the same output under a defined deterministic input contract.

The pilot does not assume that sensitive-entity discovery is perfect. Detection
quality must be measured on declared datasets, languages, formats, models,
rules, and thresholds.

## Pilot Non-goals

The pilot does not include:

- remote Agent Service;
- remote Key Service;
- RAG, embeddings, retrieval, or vector storage;
- chat, prompt tokenization, conversation memory, or response rendering;
- remote synchronization or continuous repository watching;
- Swiss-hosted model review;
- final legal advice;
- production authorization or organization-wide identity policy;
- final retention, legal-hold, backup, or disaster-recovery policy;
- approved database, cryptographic, OCR, model-runtime, or extraction
  dependency selections.

## Terminology

Original source repository:

- the user-selected folder containing original binary or text documents;
- part of the Human Zone;
- never an agent workspace.

Sensitive entity:

- a value classified under an approved sensitive-data class;
- examples may include people, organizations, addresses, contacts, matters,
  account identifiers, government identifiers, secrets, and confidential
  labels.

Private entity graph:

- the local record of discovered entities, aliases, relationships, provenance,
  confidence, review state, and assigned tokens;
- an output of discovery rather than a required user-authored input.

Token:

- an opaque symbolic identifier carrying only approved class and equality
  semantics;
- intended to remain stable inside one repository scope;
- not intended to reveal the plaintext value.

Agent-facing repository:

- a separate generated repository containing Markdown and approved
  non-sensitive metadata;
- part of the AI Zone;
- never a location for the private vault.

Private local vault:

- the Key Zone store for the private entity graph, token dictionary, sensitive
  source mappings, and approved audit or recovery data;
- physically and logically separate from the agent-facing repository.

Technical documents use tokenization or pseudonymization because the mapping is
intended to be reversible inside the Key Zone. The macOS interface may use the
action label Anonymize with Outis.

## User Flow

The intended pilot interaction is:

1. The user selects a folder in the Outis application.
2. The user selects a physically separate export parent.
3. Outis obtains approved local access to both locations.
4. Outis scans and extracts supported content without modifying the source.
5. The user reviews every Word/PDF extraction locally against its original;
   confirmation binds the exact source and normalized hashes, while rejection
   blocks the job.
6. Outis discovers candidate entities and resolves corpus-wide aliases.
7. Outis assigns or reuses stable tokens through the private vault.
8. Outis mirrors the source-relative tree into a staged repository named
   `outis`, preserves each base name after path tokenization, and changes every
   target document extension to `.md`.
9. Outis verifies the staged artifact against the approved publication oracle.
10. Outis publishes atomically or leaves the previous valid output unchanged.
11. The menu-bar surface reports progress, completion, cancellation, review
   requirement, or failure.

R1 defines progress, ephemeral security-scoped access, application lifecycle,
and cancellation candidates for S1 ratification. Finder placement is deferred
until the application path is validated.

## Source and Output Separation

The source repository and generated `outis` repository must be physically
separate. The target is not created inside the source tree.

The agent-facing repository must not contain:

- original source files;
- private-vault files;
- token-to-plaintext mappings;
- secret material;
- plaintext sensitive path or filename components;
- unreviewed document metadata;
- plaintext temporary extraction artifacts;
- logs containing sensitive source values.

The target relative tree and base-name preservation are product requirements.
Sensitive path components require the same tokenization and review discipline
as document content. Two sources that would produce the same target `.md` path
block publication rather than overwrite. Manifest bytes, repository identity,
and exact publication mechanics remain spec decisions.

## Automatic Entity Discovery

R1 combines exact structured subsets for email, telephone, IBAN, postal
address, and matter identifier with the selected offline NER candidate for
person, organization, and address-location evidence. Every NER, address,
matter, overlap, or ambiguous result requires review.

Automatic cross-document resolution uses only exact same-class normalized
equality. Fuzzy similarity, surname matching, abbreviation, and legal-suffix
removal do not merge entities automatically. Government identifiers,
credentials, dates, and context-sensitive passages are unsupported in the
funding-demo slice.

No remote model receives plaintext in the pilot.

## Entity Resolution

Review may associate supported references such as:

~~~text
Jane Example
J. Example
Ms Example
jane.example@example.invalid
~~~

when the user confirms one entity. Automatic resolution preserves separate
entities unless same-class normalized keys are identical.

The private entity graph should record:

- opaque entity identity;
- sensitive-data class;
- accepted aliases;
- document and span provenance;
- detector evidence;
- confidence or decision state;
- assigned token and scope;
- model, rule, extraction, and normalization versions;
- review history when review is in scope.

The exact schema and storage engine are not approved.

## Tokenization Direction

R1 selects this S1 candidate grammar:

~~~text
{{<class>.<class>_<zero-padded-sequence>}}
~~~

Tokens are stable only inside one private vault and contain no plaintext or
source-derived hash. S1 must bind:

- token grammar and escaping;
- workspace or repository scope;
- normalization by sensitive-data class;
- equality and alias semantics;
- stability and rotation;
- collision detection;
- missing, stale, forged, and malformed token behavior;
- treatment of source text that already resembles a token;
- cross-repository unlinkability.

Deterministic output depends on more than the input documents. The contract must
identify the source snapshot, extraction version, rule version, model artifact,
model runtime configuration, normalization version, private-vault state,
serializer version, and output configuration.

## Uncertainty

Automatic discovery can produce false positives, false negatives, ambiguous
classes, unresolved aliases, and uncertain passages.

The pilot must define:

- which conditions publish automatically;
- which conditions require local review;
- which conditions block a document or entire repository;
- whether a partially successful repository may ever be published;
- how an operator distinguishes processed, review-required, blocked, and
  published states.

Completion must not be presented as proof that all sensitive values were found.

## Trust Zones

Human Zone:

- authorized user;
- original source repository;
- Outis application;
- local extraction;
- local trusted entity discovery and tokenization.

AI Zone:

- generated agent-facing repository;
- agents allowed to read that repository;
- no private token dictionary or plaintext rendering authority.

Key Zone:

- private local vault;
- token dictionary;
- entity graph;
- sensitive mappings;
- no secret material in the synthetic funding-demo slice.

An unsandboxed process running as the same macOS user may have broader
filesystem access than the agent-facing repository. The pilot threat model and
agent sandbox requirement therefore remain open and must be proved before an
agent-inaccessibility claim.

## Candidate Language Boundaries

Rust is the candidate language for:

- deterministic transformation;
- detector orchestration;
- entity and span resolution;
- token assignment;
- Markdown and manifest serialization;
- validation and atomic-publication logic.

Swift with native macOS frameworks is the candidate language and platform
surface for:

- application and menu-bar UI;
- folder selection and security-scoped access;
- application sandbox and signing;
- AppKit Word import, PDFKit inspection/text, Core Graphics page rendering, and
  Vision revision 3 OCR;
- embedded Quick Look original-versus-normalized extraction review;
- Finder integration only in a later approved increment.

R1 selects bundled SQLite as the S1 candidate for the synthetic funding-demo
vault. It is plaintext and uses no Keychain secret, so it is not an approved
confidential-data design. Encrypted storage and Keychain protection remain a
later research decision.

The Rust-to-Swift interface, memory ownership, cancellation, progress events,
error model, generated bindings, and build integration have exact R1
candidates in `outis_local_pilot_r1_decision_closure.md`; S1 must ratify them.

## Document-Format Boundary

The first pilot must accept at least:

- legacy Word `.doc`;
- Open XML Word `.docx`;
- text-bearing PDF;
- image-only or scanned PDF through approved local OCR;
- UTF-8 `.txt`;
- UTF-8 `.md`.

All successfully processed sources produce Markdown targets. Archives, email
containers, spreadsheets, presentations, and standalone images are deferred.
R1.2 selected strict Rust UTF-8 decoding and the native macOS Word/PDF/OCR
path. The exact evidence and limits are in
`docs/reviews/outis_local_pilot/outis_local_pilot_r1_2_extraction_evaluation.md`.
S1 must ratify:

- exact extensions and format versions;
- legacy `.doc` and `.docx` structure, metadata, comments, revisions,
  footnotes, headers, footers, tables, images, and embedded-object behavior;
- text-bearing versus image-only PDF, page ordering, layout, annotations,
  forms, attachments, and embedded-content behavior;
- the selected Vision revision 3 accurate OCR at 200 DPI, ordered Italian,
  German, and French languages, automatic language detection, disabled
  language correction, page coverage, and failure behavior;
- password-protected documents;
- headers, footers, tables, comments, revisions, annotations, attachments, and
  metadata;
- malformed and partially extractable inputs;
- packages, archives, symlinks, hard links, aliases, and filesystem boundaries;
- the plain-text normalized Markdown and private page/mode provenance
  representation;
- source-to-output provenance without plaintext leakage.

## Failure Direction

The pilot should fail explicitly for:

- unreadable or unauthorized source;
- unsupported or corrupt input;
- partial extraction;
- low-confidence OCR evidence, which requires review and never automatic
  acceptance;
- detector or model failure;
- conflicting or overlapping spans;
- unresolved token collision;
- private-vault corruption or unavailability;
- stale security-scoped access;
- output collision or unsafe destination;
- validation failure;
- cancellation;
- insufficient disk space;
- attempted publication inside the original source repository;
- attempted inclusion of private-vault or source artifacts.

Failure must not silently publish plaintext or replace the last valid output.

## Deferred Swiss Verification Layer

The future controlled Swiss verification direction is not part of the pilot.
Its bounded description is maintained in architecture.md. Any work on that
direction requires a separate research brief and approval chain.

## Remaining Decisions for S1

R1.1 NER and R1.2 extraction decisions are candidates ready for S1
ratification. S1 must bind the exact schemas, APIs, dependencies, targets,
resource bounds, fixtures, commands, and failure oracles. Model redistribution
review, full Xcode and signing identity, the exact SQLite schema,
repository-owned fixtures, and the complete build lock also remain unresolved.
No implementation is authorized before S1, peer audit, and an approved
implementation plan.

## Non-claims

This intake does not claim:

- perfect sensitive-data discovery;
- that the generated repository is safe for real confidential data;
- that an agent is unable to access the source or vault;
- that any document format is supported;
- that any language, database, model, OCR, cryptographic, or platform adapter
  is approved;
- privacy, security, correctness, determinism, performance, or production
  readiness.

Those claims require the lifecycle evidence chain.
