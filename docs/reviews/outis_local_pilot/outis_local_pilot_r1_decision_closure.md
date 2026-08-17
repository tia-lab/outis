~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot R1 Decision Closure

Slug: outis_local_pilot_r1_decision_closure
Status: R1_COMPLETE_READY_FOR_S1_DRAFT
Date: 2026-08-17
Classification: research and documentation only
Code authorization: none

## Finding

R1.1 selected the contextual detector. R1.2 has now selected a complete
candidate extraction path for `.doc`, `.docx`, text PDFs, scanned PDFs,
`.txt`, and `.md` on the target Mac. R1 is closed for S1 drafting.

The selections are research inputs. They do not approve dependencies, source
files, targets, build settings, generated bindings, model distribution,
database schema, fixture creation, or product code. S1 must ratify or reject
each exact contract, then pass peer audit before P1 can plan implementation.

The funding demo uses synthetic data and a plaintext SQLite vault. It cannot be
represented as suitable for confidential data, protected from a local account
or disk attacker, anonymous, secure, perfectly detected, or extraction-
complete.

## Closed R1 Scope

Included:

- one arm64 Mac and one application-owned job;
- `.doc`, `.docx`, `.pdf`, `.txt`, and `.md` input;
- local OCR for image-only or scanned PDF pages;
- Italian, German, and French;
- strict UTF-8 text/Markdown extraction in Rust;
- AppKit Word import, PDFKit PDF inspection/text, Core Graphics rendering, and
  Vision revision 3 OCR in Swift;
- a plain-text normalized Markdown profile;
- mandatory review of every Word, PDF, OCR, contextual, or ambiguous result;
- deterministic structured detectors and the selected offline NER candidate;
- repository-local entity graph and stable tokens;
- a separate generated repository named `outis` containing one tokenized
  Markdown document per successfully processed source;
- mirrored source-relative directories and base names after sensitive path
  tokenization, with explicit output-collision blocking;
- application and menu-bar progress;
- explicit cancellation, failure, staging, and atomic publication.

Excluded:

- real confidential input;
- spreadsheets, presentations, email containers, archives, standalone images,
  and unsupported embedded Word content;
- layout-faithful Word or PDF conversion;
- Finder extension, daemon, watching, and synchronization;
- Keychain, vault encryption, response rendering, and token-reversal UI;
- remote services, inference, RAG, embeddings, and chat;
- Intel Mac, universal binary, and non-macOS targets;
- perfect detection, anonymity, production security, and readiness claims.

The deferred Swiss verification direction remains only in `architecture.md`.

## Decision Map

| Surface | R1 selection | Exact artifact |
|---|---|---|
| Extraction | Rust strict UTF-8 plus native AppKit, PDFKit, Core Graphics, and Vision revision 3; every binary format requires review. | `outis_local_pilot_r1_2_extraction_evaluation.md` |
| Normalized Markdown | One `.md` per source; source Markdown/text preservation; stable plain-text profile for binary documents; private page/mode provenance. | `outis_local_pilot_r1_2_extraction_evaluation.md` |
| OCR | Accurate revision 3, ordered IT/DE/FR languages, automatic language detection, correction off, 200 DPI, sequential pages. | `outis_local_pilot_r1_2_extraction_evaluation.md` |
| Source and discovery | Required formats, sensitive taxonomy, narrow structured grammars, fixed reviewed NER, exact equality, no fuzzy merge. | `outis_local_pilot_r1_source_discovery_contract.md` |
| Token | Class-bearing repository-local sequence grammar; persistent transactional reuse; token-like source blocks. | `outis_local_pilot_r1_token_vault_contract.md` |
| Vault | Bundled SQLite candidate, one writer, exact PRAGMAs, plaintext synthetic-only store, no Keychain. | `outis_local_pilot_r1_token_vault_contract.md` |
| Agent repository | Separate `outis` tree with one `.md` per source; tokenized relative paths and base names; no vault, mappings, or Git metadata. | `outis_local_pilot_r1_publication_platform_contract.md` |
| Publication | Same-parent staging, first rename, `RENAME_SWAP` replacement, no copy fallback, old-output preservation. | `outis_local_pilot_r1_publication_platform_contract.md` |
| macOS and FFI | macOS 14 arm64, Swift 6 app, sandbox, no Finder target, Rust static library and polling C ABI with extraction submission. | `outis_local_pilot_r1_publication_platform_contract.md` |
| Oracles and budgets | Synthetic extraction, structured, NER, privacy, replay, failure, and resource evidence. | `outis_local_pilot_r1_acceptance_evidence.md` |
| NER | Pinned mBERT ONNX candidate, ONNX Runtime 1.28 CPU, fixed two-thread configuration. | `outis_local_pilot_ner_evaluation.md` |

## Evidence Position

R1 includes:

- primary Apple, Microsoft, SQLite, Unicode, SWIFT, ITU, Swift, model,
  runtime, extraction-alternative, and cbindgen sources;
- synthetic native Word, PDF text, OCR, corruption, encryption, reading-order,
  replay, latency, and memory probes on one M4 Pro;
- pinned model, tokenizer, runtime, artifact hashes, smoke corpus, and
  one/two/four-thread NER benchmark;
- a disposable bundled-SQLite build and transaction probe;
- a disposable APFS directory-exchange probe.

It does not include an Outis app, repository-owned extraction fixtures, vault
schema, generated ABI, sandbox run, signed bundle, crash injection, full
confidential corpus, or complete job. Apple-managed OCR artifact bytes cannot
be pinned independently, so replay binds the macOS build and exact Vision and
render configuration. Evidence details and limits are in the R1 acceptance and
R1.2 evaluation artifacts.

## S1 Ratification Requirements

S1 must bind:

1. exact source, normalized-document, extraction request, detector, span,
   review, entity, token, vault, manifest, FFI, state, cancellation, and failure
   schemas;
2. native extraction APIs, content signatures, format inspection, mandatory
   binary review, normalized Markdown rules, resource bounds, and no-truncation
   behavior;
3. exact OCR revision, options, language availability, page render, ordering,
   artifact identity, OS-build regression, and failure contract;
4. exact crate, Swift file/target, generated-header, model, fixture, evidence,
   and command paths;
5. exact dependency versions, features, locks, licenses, alternatives, and
   owners;
6. exact Xcode, SDK, signing, entitlement, sandbox, Quick Look preview and
   persistence boundaries, dylib embedding, and distribution settings;
7. exact SQLite schema, migration SQL, transaction, permissions, retention,
   deletion, and recovery behavior;
8. exact manifest bytes and agent-tree allowlist;
9. exact tests, per-format oracles, metrics, commands, budgets, and expected
   outputs;
10. model legal-review disposition or an explicit implementation block;
11. approval status and pre-audit closure.

## Recommendation

Draft `docs/specs/outis_local_pilot_SPEC.md` from the closed R1 contracts.
Then conduct A1 peer audit and write the P1 minimal implementation plan. Do not
create crates, Xcode targets, dependencies, generated headers, fixtures, model
artifacts, schemas, or product code before those gates pass.
