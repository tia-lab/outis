~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-03 Research Brief

Status: `COMPLETE_FOR_APPROVED_S1_23B_AUTHOR_AUDIT_REQUIRES_S1_23C`
Classification: research and specification input; no code
Date: 2026-08-18

## Decision

The user approved S1-23, S1-23A, and S1-23B on 2026-08-18: define the smallest model-independent
local extraction increment covering `.doc`, `.docx`, text-bearing PDFs,
scanned or image-only PDFs, UTF-8 `.txt`, and UTF-8 `.md`. The increment ends
with an in-memory normalized document or an explicit extraction outcome. It
does not implement the Outis application, FFI, review UI, detection
orchestration, model, entity graph, tokenization, vault, agent export, or
publication.

S1-23A records the measured 20-package transitive Cargo closure, the isolated
test-only fixture probe, and the split between exact synthetic-observation
oracles and actual Vision validity and replay evidence. S1-23B defines the
literal text/native extraction identities, canonical JSON and SHA-256
preimages, and production serializer ownership. Neither correction changes
the measured object or authorizes implementation.

## Source materials

- `AGENTS.md` and `docs/invariants/core_invariants.md`;
- the active lifecycle, research, spec, peer-audit, testing, and documentation
  protocols;
- `docs/reviews/outis_local_pilot/outis_local_pilot_r1_2_extraction_evaluation.md`;
- `docs/reviews/outis_local_pilot/outis_local_pilot_r1_decision_closure.md`;
- Sections 8, 9, 17, 24 through 26, 28 through 38, and 40 of
  `docs/specs/outis_local_pilot_SPEC.md`;
- `docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_result_review.md`;
- current commit `7faf40e`, the dependency-free `outis-core` source, 21-test
  grammar oracle, manifests, lockfile, toolchain locator, and inventories; and
- the Apple and Microsoft primary sources cited by the R1.2 evaluation.

No new platform or format selection is needed. R1.2 already measured and S1
already approved the exact native extraction candidates. Runtime behavior in
Outis remains unproved because no extraction implementation exists.

## Measured object

The increment contains two permanent, format-specific transformations joined
by one byte-exact data contract:

1. Rust accepts one declared `.txt` or `.md` class plus stable source bytes and
   returns `ExtractionOutcomeV1`.
2. Swift accepts stable binary source bytes plus one declared `.doc`, `.docx`,
   or PDF class and returns `NativeExtractionSubmissionV1`. Rust validates a
   submission with the same source identity and returns
   `ExtractionOutcomeV1`.

The output is exactly one of:

- `ready` with `NormalizedDocumentV1` for valid `.txt` or `.md`;
- `needs_review` with `NormalizedDocumentV1` for every successful Word or PDF
  extraction;
- `blocked` with one existing stable blocked-domain code and no document;
- `failed` with `extraction_platform_failed` and no document; or
- `cancelled` with no document.

The Swift harness and Rust validator use the same independently authored
synthetic oracle. MI-03 does not claim a live Swift-to-Rust call, application
workflow, or end-to-end job.

## Candidate architecture

Rust owns:

- source-class and source-identity values;
- extension/signature, UTF-8, NUL, hash, normalized-size, range, coverage, and
  status validation;
- strict `.txt` and `.md` extraction;
- binary-derived line and page normalization validation;
- `NormalizedDocumentV1`, ordered provenance, extraction identity, and typed
  outcomes.

Swift owns:

- AppKit `NSAttributedString` import for `.doc` and `.docx`;
- PDFKit access checks, page inspection, and native PDF text;
- Core Graphics fixed 200-DPI page rendering;
- Vision revision 3 OCR with the approved Italian, German, and French
  configuration;
- deterministic native observation ordering; and
- one bounded in-memory `NativeExtractionSubmissionV1`.

The four permanent Swift adapter files are compiled with a test-only native
harness by the installed Xcode Swift compiler. They are not attached to a
placeholder application target. No temporary product API, callback, IPC,
generated header, Swift package, framework target, command-line product, or
format-specific crate is introduced.

## Source ownership and formats

All sources are repository-owned synthetic fixtures. Real or confidential
documents are prohibited. The allowlist is exactly:

- legacy Compound File Binary `.doc`;
- ZIP/Open XML `.docx`;
- text-bearing, image-only, and mixed-page `.pdf`;
- optional-BOM strict UTF-8 `.txt`; and
- optional-BOM strict UTF-8 `.md`.

The one-source capability accepts bytes, not a folder or arbitrary path. File
enumeration, symlinks, aliases, packages, mounts, nested repositories,
security-scoped access, and source-to-output path mapping remain outside
MI-03. A supplied source identity that does not match the bytes is
`source_changed`.

## Normalized Markdown contract

- `.txt` and `.md` preserve exact BOM-free UTF-8 bytes.
- Binary-derived CRLF, CR, U+2028, and U+2029 become LF.
- Trailing LF is removed per page, PDF pages are joined with exactly two LF
  bytes, and the complete binary-derived document ends with exactly one LF.
- Other spaces, tabs, scalars, and extracted reading order are preserved.
- No heading, page label, filename, timestamp, comment, front matter,
  translation, correction, normalization, or layout claim is added.
- Private provenance covers every normalized byte without overlap or
  unexplained gaps and never becomes agent-facing content.

## Trust zones and sensitive data

| Surface | Zone | MI-03 behavior |
|---|---|---|
| synthetic source bytes and native framework objects | Human Zone | in memory only |
| normalized text and private provenance | Human Zone | in memory and test comparison only |
| AI Zone agent repository | not touched | no directory or file exists |
| Key Zone vault | not touched | no database, mapping, token, or secret exists |

MI-03 performs no sensitive-entity discovery. Italian, German, and French are
required OCR languages, not detection-quality claims. Existing email,
telephone, and IBAN behavior is regression-tested but is not invoked by the
extraction transformation.

## Determinism boundary

Text extraction is byte-deterministic for the declared source bytes and schema
version. Native replay binds source bytes, macOS product and build, CPU,
AppKit/PDFKit/Vision/Core Graphics environment, Vision revision and all
options, supported-language query, PDF media box and render identity,
observation order, normalization schema, and provenance schema. No equality
across macOS builds or machines is claimed.

## Candidate dependency and build surface

The exact already-approved direct Rust versions used by MI-03 are:

- `serde` `1.0.229` in core and runtime;
- `serde_json` `1.0.151` in runtime production identity/metadata serialization
  and handwritten-oracle handling; and
- `sha2` `0.11.0` in runtime.

No other direct registry dependency is justified. Swift uses only Foundation,
AppKit, PDFKit, Vision, and Core Graphics supplied by macOS. MI-03 creates no
Xcode project, application bundle, generated C binding, model artifact,
database, cryptographic selection, or package-manager surface.

## Candidate path boundary

The proposed source boundary is limited to:

~~~text
Cargo.toml
Cargo.lock
inventory.md

crates/outis-core/Cargo.toml
crates/outis-core/docs/inventory.md
crates/outis-core/src/lib.rs
crates/outis-core/src/error.rs
crates/outis-core/src/source.rs
crates/outis-core/src/document.rs

crates/outis-runtime/Cargo.toml
crates/outis-runtime/docs/inventory.md
crates/outis-runtime/src/lib.rs
crates/outis-runtime/src/extraction.rs
crates/outis-runtime/src/extraction/text.rs
crates/outis-runtime/tests/extraction_contract.rs

apps/macos/Outis/Extraction/DocumentExtractor.swift
apps/macos/Outis/Extraction/WordExtractor.swift
apps/macos/Outis/Extraction/PDFExtractor.swift
apps/macos/Outis/Extraction/VisionOCR.swift

tests/native_extraction/Main.swift
tests/fixture_generation/Main.swift
tests/fixture_generation/WordFixtures.swift
tests/fixture_generation/PDFFixtures.swift

fixtures/outis_local_pilot/v1/README.md
fixtures/outis_local_pilot/v1/SHA256SUMS
fixtures/outis_local_pilot/v1/fixture_manifest.json
fixtures/outis_local_pilot/v1/extraction/**
~~~

Only `Cargo.lock`, root `inventory.md`, and the declared binary source fixtures
are generated. The implementation plan must bind their generators and exact
output identities. No directory is created as a placeholder.

## Oracle candidates

The extraction oracle must cover:

- exact success bytes and provenance for `.txt`, `.md`, `.doc`, `.docx`, text
  PDF, mixed PDF, and Italian/German/French scanned-PDF pages;
- BOM, invalid UTF-8, NUL, signature mismatch, corruption, zero-page PDF,
  protected PDF, unsupported Word/PDF feature, rotation, no OCR observation,
  invalid OCR geometry, unavailable language, source-identity mismatch, and
  every applicable size/page/render limit;
- cancellation before a page and between pages;
- three-run equality under one complete native extraction identity;
- exact `ready`, `needs_review`, `blocked`, `failed`, and `cancelled` outcomes;
- zero normal-log or diagnostic plaintext;
- the unchanged 21 MI-01/MI-02 tests; and
- strict fixture regeneration and checksum verification.

Timing and memory are recorded on the existing reference host. They support no
claim beyond the recorded implementation, fixtures, identity, and run.

## Evidence table

| Question | Evidence | Finding |
|---|---|---|
| Are all required formats covered by one local candidate set? | R1.2 evaluation | Yes on the recorded synthetic probes; product behavior remains unproved. |
| Can `.txt` and `.md` remain dependency-light? | S1 Sections 8 and 9 | Yes; strict UTF-8 and BOM handling need no parser. SHA-256 identity uses the already-approved `sha2`. |
| Is binary automatic publication justified? | R1.2 fidelity results | No; every Word and PDF result remains `needs_review`. |
| Is a local model required? | extraction contract | No. Vision OCR is a native extraction framework, not the contextual NER model. |
| Is live Swift/Rust integration required to prove adapter contracts? | approved S1-23 boundary | No. The shared byte-exact oracle proves the two permanent boundaries separately; live composition remains unproved. |
| Is the model legal gate relevant? | S1-19 | No model-specific path, dependency, artifact, fixture, or execution is included. |
| Can confidential use be claimed? | trust and cache limits | No. Apple-managed caches, helper behavior, crash artifacts, and real-document fidelity remain unproved. |

## Hypotheses and unknowns

- The selected adapters are expected to remain within the approved small-file
  latency and memory budgets, but only implementation measurements can test
  that hypothesis.
- Repository-owned Word/PDF fixture generation may vary across macOS builds;
  byte replay is required only inside the complete generator identity.
- AppKit exposure of every unsupported Word feature is not proved. A fixture
  that cannot expose the declared evidence must block rather than weaken the
  contract.
- Vision and PDFKit can use Apple helper processes or caches not controlled by
  Outis. MI-03 remains synthetic-only.
- Live Rust/Swift memory ownership, FFI, sandbox, signing, application state,
  review confirmation, and cancellation latency remain unproved and outside
  the increment.

## Risks

- Office/PDF visible text can omit hidden or semantically important content.
- PDFKit and Vision reading order can be deterministic but semantically wrong.
- A platform update can change imported or OCR bytes.
- Binary fixtures and the native compiler increase repository and compile
  surface.
- A test-only harness could be mistaken for product integration; documentation
  and result evidence must prohibit that interpretation.

## Decisions required before implementation planning

Approved S1-23 through S1-23B bind:

1. the two-transform measured object and explicit absence of live integration;
2. exact type, status, normalization, provenance, identity, and failure
   contracts;
3. exact source, fixture, generated, test, and evidence paths;
4. the three direct Rust dependencies and platform frameworks;
5. direct Swift compilation without an app or temporary target;
6. exact fixture names, generation/check commands, and independently authored
   normalized outputs;
7. correctness, privacy, replay, compile-surface, time, and memory gates;
8. literal extraction-identity, native-metadata, provenance, page, and oracle
   schemas plus their canonical JSON and SHA-256 preimages; and
9. the mandatory separate implementation plan and approval gate.

The corrected author audit found one remaining required decision: S1-23C must
assign every MI-03 rejection, failure, and limit case to one existing domain
code and define precedence when conditions compete. Until that matrix is
approved, test expectations and canonical native failure metadata cannot be
exactly planned.

## Recommendation

Approve and incorporate the bounded S1-23C failure-code and precedence matrix,
then rerun the author closure gate and separate falsifying peer audit. If both
pass, prepare a separate exact MI-03 implementation plan. Do not create code,
fixtures, targets, generated outputs, dependencies, or build changes before
that plan is explicitly approved.
