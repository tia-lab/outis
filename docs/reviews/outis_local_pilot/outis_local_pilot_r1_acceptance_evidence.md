~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis R1 Acceptance and Evidence

Status: R1_COMPLETE_READY_FOR_S1_DRAFT
Date: 2026-08-17
Classification: research only
Code authorization: none

## Determinism Boundary

Byte-identical export is required when all identities are unchanged:

- source bytes and relative paths;
- extractor mode and configuration, macOS product/build, framework
  environment, OCR revision, supported-language result, render configuration,
  and normalized-document schema;
- private vault state before the job;
- completed review decisions;
- pipeline and schema versions;
- model, tokenizer, runtime, detector, normalization, and dependency versions;
- arm64 CPU configuration;
- taxonomy and manifest schema.

Cross-machine, cross-runtime, cross-operating-system-build, new-vault, and
changed-review identity are not promised. R1.1 observed bitwise-identical
logits and R1.2 observed stable extraction output in their bounded probes; the
complete export replay test does not exist.

## Synthetic Funding-Demo Corpus

S1 must bind synthetic fixtures that a later approved implementation plan can
create. Required format coverage includes:

- legacy `.doc` documents;
- `.docx` documents covering paragraphs, lists, tables, headers, footers,
  notes, tracked changes, and embedded-content failure;
- text-bearing PDF documents covering page order and multi-column ambiguity;
- scanned or image-only PDF documents covering Italian, German, and French
  local OCR, mixed text/image pages, confidence, and incomplete page coverage;
- `.txt` and `.md` covering BOM, CRLF, LF, Unicode composition, invalid UTF-8,
  NUL, and Markdown syntax;
- all required formats covering corruption, encryption, size limits,
  unsupported features, mutation, cancellation, extraction provenance, and
  source-to-target collisions;
- binary review covering exact source/normalized hash confirmation, rejection,
  and invalidation after mutation;
- the 30 R1.1 multilingual NER cases;
- per language, at least 10 positive and 10 negative cases for email,
  telephone, IBAN, postal address, and matter identifier;
- at least 12 entity cases covering exact reuse, near-match separation,
  explicit merge, class conflict, overlap, and token stability;
- publication cases for first publish, replacement, every cancellation stage,
  corrupt old output, unsupported exchange, write or sync failure, cleanup
  failure, and source mutation.

Funding-demo acceptance candidates:

- exact precision and recall 1.000 for the declared structured accepted
  subsets;
- exact normalized values, spans, token grammar, entity reuse, manifest, and
  output bytes;
- every uncertain/contextual oracle item enters review;
- no unresolved item publishes;
- the 30 NER cases remain a per-language and per-class regression smoke gate,
  not a production threshold;
- every oracle sensitive value, sensitive path component, and mapping is absent
  from exported names and the complete exported byte tree;
- every successfully processed source has one target `.md` at the declared
  tokenized relative path and no source is silently omitted;
- vault path and database bytes are absent from export;
- repeated runs under the complete identity are byte-identical;
- failure and cancellation preserve the last valid output.

Passing these checks does not prove perfect discovery, formal anonymity,
production security, or suitability for confidential data.

## Provisional Resource Gates

On the recorded M4 Pro:

- selected model, tokenizer, runtime, and notices: no more than 752 MB;
- peak process RSS during NER evaluation: no more than 2 GB;
- one active job and one inference call;
- 510-token warm NER: mean no more than 100 ms and p95 no more than 110 ms
  using the fixed two-thread configuration;
- menu-bar observation: no more than 250 ms after an engine event;
- cancellation observation: no more than 250 ms outside a current NER window,
  SQLite commit, full sync, or atomic rename;
- clean release build, final app size, energy, and full-job latency are measured
  and reported without an unproved R1 pass threshold.

Extraction guardrail candidates for S1 are:

- `.txt` and `.md` at most 10 MiB each;
- `.doc`, `.docx`, and `.pdf` at most 50 MiB each;
- at most 250 MiB source bytes and 100 MiB normalized UTF-8 per job;
- at most 100 PDF pages per document and 200 OCR pages per job;
- sequential 200-DPI OCR, at most 4,096 pixels on either axis and 16,777,216
  pixels per page;
- no truncation, silent downscaling, or concurrent OCR.

## Evidence Table

| Type | Available evidence | Limit |
|---|---|---|
| Code read | Current package is a dependency-free Hello World. | No pilot behavior. |
| Run | macOS 26.5, arm64, Rust 1.89, Swift 6.0.3 on one M4 Pro. | No Outis app run. |
| Model | R1.1 revision, hashes, labels, tokenizer, runtime, thresholds, and smoke corpus. | Small corpus; no commercial legal clearance. |
| Benchmark | R1.1 balanced 1/2/4 thread comparison; two-thread 510-token mean/p95 89.54/94.03 ms. | Temporary harness and synthetic token payload. |
| Storage | Bundled SQLite 3.53.2 PRAGMAs, rollback, commit, quick check, and `0600` files observed. | One happy path; no crash test. |
| Storage build | Clean release dependency build 14.73 s; warm operation 1–3 ms. | Disposable probe, not product build. |
| Publication | APFS sibling `RENAME_SWAP` probe succeeded and preserved both trees. | One volume and happy path. |
| Platform | Apple documentation and local SDK support candidate APIs. | No Outis sandbox, entitlement, or signing run. |
| FFI | Swift interoperability and cbindgen upstream contracts read. | No Outis ABI generated or compiled. |
| Extraction | R1.2 selected Rust UTF-8 plus AppKit, PDFKit, Core Graphics, and Vision revision 3. Synthetic Word, PDF, OCR, failure, replay, latency, and memory probes ran on one M4 Pro. | Small disposable corpus; no Outis implementation, repository fixture, real-document completeness, cold-start baseline, or cross-OS-build replay. |
| Security | Zone and export contracts documented. | No denied-access evidence; no security claim. |
| End-to-end | None. | No complete job. |

## Rejected First-Slice Choices

- remote or Swiss-hosted LLM: adds a plaintext transfer boundary;
- LLM-only discovery: lacks deterministic structured and review contracts;
- model-free contextual discovery: does not meet the selected person and
  organization direction;
- Core ML, GPU, or Neural Engine: conversion, caching, numeric, packaging, and
  replay behavior are unmeasured;
- Finder extension: unnecessary before the application path is proved;
- remote document-conversion or OCR services: violate the pilot local-only
  boundary;
- Pandoc for the first slice: no legacy `.doc` input and a measured 261 MiB
  executable despite acceptable simple `.docx` conversion;
- Tesseract, LibreOffice, and Apache Tika for the first slice: add engine,
  artifact, process, runtime, or dependency surfaces not needed by the selected
  native path;
- WAL and multiple SQLite connections: unnecessary sidecar and concurrency
  surface;
- Keychain without vault encryption: a secret alone does not protect plaintext
  SQLite mappings;
- source-derived or hashed tokens: leak equality or guessing surface;
- fuzzy automatic entity merge: unproved false-merge risk;
- copy publication fallback: cannot preserve an old-or-new visible tree;
- automatic Git initialization: no first-slice need.

## Remaining Unknowns and Risks

- model redistribution and training-data obligations need qualified legal
  review;
- full Xcode, SDK, signing, embedded-runtime signing, Hardened Runtime, and
  notarization are not measured;
- App Sandbox and security-scoped access are not exercised in Outis;
- Quick Look preview fidelity, helper-process access, caches, and diagnostics
  are unmeasured;
- the plaintext vault is unsuitable for confidential data;
- Time Machine, snapshots, Spotlight, antivirus, crash reports, swap, and
  diagnostics may copy or observe plaintext;
- aliases, packages, extended attributes, resource forks, and mount boundaries
  are not tested;
- Word/PDF layout, notes, revisions, embedded content, mixed image/text pages,
  real scan quality, rotation, handwriting, accessibility order, and field
  completeness remain unproved; every binary result therefore requires
  review;
- Apple-managed Vision artifacts cannot be independently pinned; OCR replay is
  scoped to an exact macOS build and configuration;
- OCR cold start, energy, concurrent behavior, and selected configuration in a
  signed sandboxed Swift app remain unmeasured;
- `F_FULLFSYNC`, directory `fsync`, crash points, and exchange recovery are not
  failure-injection tested;
- the NER corpus is small and template-like;
- address and matter lexicons intentionally miss valid forms;
- same-user agents may reach source or vault unless separately sandboxed;
- memory, app size, compile surface, energy, full-job latency, and cross-machine
  replay remain unproved.

## Stop Conditions

S1 or implementation stops if:

- model legal redistribution review fails without an approved replacement;
- full-Xcode signing or runtime packaging cannot satisfy the local-only
  contract;
- exact source, mapping, vault, or export boundaries cannot be tested;
- an implementation shortcut would publish unresolved or partial content;
- synthetic-only evidence is used to claim confidential-data readiness;
- a dependency or file lacks a necessary S1 and P1 binding.

R1.2 details are in
`outis_local_pilot_r1_2_extraction_evaluation.md`. Completion of this research
artifact authorizes S1 drafting only, not implementation.
