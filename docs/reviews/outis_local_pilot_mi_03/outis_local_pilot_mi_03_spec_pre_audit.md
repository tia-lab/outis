~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-03 Specification Author Pre-Audit

Status: `BLOCKED_AFTER_S1_23B`
Classification: corrected S1-23 through S1-23B author pre-audit; no code
Date: 2026-08-18
Target: `docs/specs/outis_local_pilot_SPEC.md`, Section 40 S1-23 through
S1-23B
Commit inspected: `7faf40e71652fcc483ae364043bb3f397bde784a`
Dirty state at audit evidence capture: approved documentation changes and
MI-03 review artifacts only; no product source, test source, fixture,
manifest, lockfile, toolchain, build configuration, inventory, or
generated-artifact change

## Result

`BLOCKED`

S1-23B closes the previously reported identity and serialization defect.
S1-23A binds the
measured dependency closure, non-circular fixture bootstrap, and independent
Vision oracle. S1-23B binds the previously missing extraction identities,
canonical JSON and SHA-256 preimages, literal metadata/oracle schemas, and
production serializer ownership.

The complete author rerun found a further observable data-contract gap: the
specification does not assign an exact blocked or failed domain code and
competing-condition precedence to every bound rejection case. An
implementation plan would have to invent those values. S1-23C is therefore
required before a separate peer audit or implementation plan.

This result does not approve an implementation plan or authorize code,
dependencies, manifests, lockfile changes, Swift sources, fixtures, generated
artifacts, or builds.

## Findings closed

### MI03-AUD-04: exact dependency closure

Closed in design. The spec records exactly two local packages, 20 registry
packages, every version and registry checksum, seven custom-build packages,
zero duplicate versions, the candidate lockfile SHA-256, and the distinction
between direct and transitive ownership. Direct `libc`,
`unicode-normalization`, and `caseless` remain absent.

The approved S1-23B offline production-serialization probe kept the same
package set and lock hash. Its candidate root, core, and runtime manifest
hashes are bound in the spec. Its release time, RSS, and `.rlib` size are
planning observations only.

### MI03-AUD-05: first fixture creation

Closed in design. Test-only `--probe` writes source candidates and identities
only below a fresh ignored root. Handwritten normalized outputs and oracle
records are authored after the probe. `--write` regenerates source bytes in
memory, proves equality with the probe and oracle identities, validates every
handwritten input, and only then creates the committed generated paths.
`--check` retains an independent fresh ignored root.

### MI03-AUD-06: independent Vision oracle

Closed in design. Synthetic observations own exact ordering, coordinate and
confidence conversion, normalization, provenance, identity, and canonical
metadata expectations. Actual Vision fixtures own independently authored
visible text, exact status and page modes, bounded geometry, full provenance
coverage, and three-run metadata replay. An observed actual-Vision hash is run
evidence and cannot define its own correctness oracle.

### S1-23-PA-07: exact deterministic extraction identity

Closed in design by S1-23B. The spec defines:

- complete flat `rust_text`, `appkit_word`, and `pdfkit_vision` identity
  variants with literal keys, values, array orders, and forbidden fields;
- exact platform, language-support, Vision, OCR-render, page, provenance,
  bounding-box, native-metadata, and oracle schemas;
- compact UTF-8 JSON with recursively raw-UTF-8-sorted keys, contract-order
  arrays, shortest integers, lowercase literals, no floats, whitespace,
  omitted keys, or trailing LF;
- `extraction_identity_sha256` over the identity object alone and the oracle's
  canonical-native-submission hash over metadata alone;
- lower-hex digest encoding and independent Rust recomputation; and
- `serde_json =1.0.151` as an `outis-runtime` production dependency for the
  approved identity and metadata encodings.

The identity strings are restricted to visible ASCII. That makes Foundation's
sorted-key encoding and the raw UTF-8 key order identical for this schema. A
disposable representative nested `appkit_word` probe produced byte-identical
Rust `serde_json` and Foundation `JSONSerialization` output: 355 bytes,
including LF from the probe print, with SHA-256
`e4177eb2a3c957780820d0480e2377acb82d9a61358ca74f96f08eddf098cf19`.
The probe directory and outputs were removed. This confirms the selected
encoding mechanism for the measured representative object; complete oracle
agreement remains an implementation test.

The final author rerun also removed three internal ambiguities without
changing the approved design: Section 9 now distinguishes platform-independent
Rust text identity from native identity; `ExtractionIdentityV1` is described
as one tagged object rather than incorrectly called flat despite its nested
records; and every exact native outcome, including blocked, failed, or
cancelled, requires an exact canonical-metadata oracle hash. No implementation
choice remains in those statements.

### S1-23-PA-08: exact failure-code and precedence matrix is incomplete

The spec defines exhaustive version-one domain-code vocabularies and names
many individual extraction mappings, but several MI-03 cases still state only
that they block. At minimum it does not unambiguously bind:

- invalid UTF-8 and NUL text input;
- input-byte, normalized-byte, PDF-page, render-axis, and render-pixel limit
  violations to `source_limit_exceeded` versus `extraction_limit_exceeded`;
- password-protected Word input;
- an AppKit import rejection for a signature-valid but unreadable Word file;
- a non-finite or non-positive PDF media box;
- native page access, render, and serialization failures where a platform
  operation fails without proving corrupt input; or
- precedence when source-hash mismatch, signature/class mismatch, size,
  cancellation, and native-submission contradictions coexist.

These codes enter `NativeExtractionMetadataV1`, `ExtractionOutcomeV1`, the
handwritten oracle, deterministic replay, and result evidence. Choosing among
`format_signature_mismatch`, `document_corrupt`, `document_encrypted`,
`extraction_incomplete`, `extraction_limit_exceeded`,
`ocr_geometry_invalid`, `extraction_platform_failed`, and
`internal_invariant_failed` is observable behavior, not an internal Rust or
Swift representation detail.

Required S1-23C correction:

1. bind one exact ordered validation and terminal-condition precedence for
   the Rust text path, native adapter, and Rust native-submission validator;
2. assign one exact domain code to every MI-03 rejection, failure, and limit
   case in the approved oracle matrix;
3. state whether cancellation wins before validation begins and at which
   existing native boundaries an already-started result wins;
4. distinguish proven malformed/corrupt input from an unexpected Apple API or
   serialization failure; and
5. require tests for every mapping and the minimal competing-condition pairs
   needed to prove precedence.

The exact proposed S1-23C mapping is:

| Condition | Exact outcome and code |
|---|---|
| `.txt`/`.md` or binary source exceeds its per-file byte ceiling | `Blocked { source_limit_exceeded }` |
| supplied source SHA-256 differs after the size gate | `Blocked { source_changed }` |
| declared format signature disagrees, including invalid UTF-8 or any NUL for text/Markdown | `Blocked { format_signature_mismatch }` |
| signature-valid Word import is rejected as malformed, corrupt, or truncated | `Blocked { document_corrupt }` |
| Word or PDF reports password, encryption, lock, or copy restriction | `Blocked { document_encrypted }` |
| Word attachment/lossy-conversion evidence or PDF rotation, annotation, form, attachment, or embedded-content evidence | `Blocked { document_feature_unsupported }` |
| PDF open rejects malformed/truncated bytes or exposes zero pages | `Blocked { document_corrupt }` |
| PDF exceeds 100 pages, normalized UTF-8 exceeds 10 MiB, or a render exceeds either pixel ceiling | `Blocked { extraction_limit_exceeded }` |
| a declared page is unavailable, OCR has no non-empty top candidate, or normalized/page/provenance coverage is incomplete | `Blocked { extraction_incomplete }` |
| any required OCR language is unavailable before the first OCR page | `Blocked { ocr_language_unavailable }` |
| media-box geometry is non-finite or non-positive, or an OCR box is non-finite, out of range, non-positive, or fails fixed-point bounds | `Blocked { ocr_geometry_invalid }` |
| an Apple import, page, render, Vision, allocation, or JSON operation fails without evidence for a blocked condition | `Failed { extraction_platform_failed }` |
| valid Rust-owned values cannot be canonically serialized or an otherwise unreachable Rust invariant fails | `Failed { internal_invariant_failed }` |
| native metadata is malformed or contradicts source, status, adapter, identity, buffer, normalization, ordering, ranges, hashes, or coverage | `Blocked { extraction_incomplete }` unless an earlier source or declared-limit gate already won |
| cancellation is observed at an approved native boundary | `Cancelled` with no code or document |

The exact proposed precedence is:

1. The Rust text transformation has no cancellation input in MI-03. It checks
   source-byte limit, source SHA-256, declared format/content signature, text
   validity, normalization limit, and canonical result construction in that
   order.
2. Before the first Swift operation, cancellation wins without reading or
   validating native content. Otherwise Rust source-byte limit, source
   SHA-256, and declared binary signature have already passed in that order.
3. Word then evaluates import result, reported encryption, corrupt/rejected
   import, attachment/lossy evidence, normalized limit, and metadata encoding
   in that order. When one framework error contains more than one classified
   input condition, encryption wins over corrupt/rejected import.
4. PDF then evaluates open/API failure, access restriction, zero-page or
   corrupt structure, page-count limit, and whole-document unsupported-feature
   evidence in that order. It performs no text or OCR work before this
   preflight completes.
5. Each PDF page then checks cancellation, page availability, native-text
   classification, and the normalized limit. An OCR page additionally checks
   required-language availability once before the first OCR page, media-box
   geometry, render limits, render result, cancellation, Vision result,
   observation validity/order, normalized limit, and coverage in that order.
6. A non-preemptible Apple call runs to return. If it returns a classified
   blocked or failed result, that result wins over cancellation requested
   during the call. If it succeeds, the immediately following cancellation
   check wins and all partial in-memory output is discarded.
7. Rust native-submission acceptance checks source-byte limit, source SHA-256,
   binary signature, metadata JSON/schema/status consistency, declared
   normalized limit, identity and identity hash, buffer presence/length/hash
   and normalization, page/observation/provenance order and coverage, then
   canonical metadata equality. A contradiction at one of those steps is
   `extraction_incomplete`; valid-value serialization failure is
   `internal_invariant_failed`.

The exact minimal competing-condition oracle adds these pairs:

| Pair | Required winner |
|---|---|
| pre-start cancellation plus oversized native source | `cancelled` |
| oversized source plus source-hash mismatch | `source_limit_exceeded` |
| source-hash mismatch plus signature mismatch | `source_changed` |
| signature mismatch plus malformed native metadata | `format_signature_mismatch` |
| PDF encryption plus excessive page count | `document_encrypted` |
| zero-page PDF plus unsupported-feature evidence | `document_corrupt` |
| excessive page count plus annotation | `extraction_limit_exceeded` |
| unsupported PDF feature plus later invalid OCR geometry | `document_feature_unsupported` |
| unavailable OCR language plus invalid media box | `ocr_language_unavailable` |
| invalid media box plus render-axis excess | `ocr_geometry_invalid` |
| render-axis excess plus injected render failure | `extraction_limit_exceeded` |
| declared normalized-size excess plus normalized-hash mismatch | `extraction_limit_exceeded` |
| successful non-preemptible page operation plus cancellation observed immediately after it | `cancelled` and no partial document |
| failed non-preemptible page operation plus cancellation requested during it | that operation's exact blocked or failed result |

S1-23C may use only the already approved status and code vocabularies. It must
not add a format, fallback, retry, dependency, path, recovery behavior, or
claim.

## Pre-audit closure matrix

| Required closure | Result | Bound evidence |
|---|---|---|
| Goal and non-goals | Pass | one-source extraction only; application, FFI, model, vault, export, and publication absent |
| Prior-spec compatibility | Pass | S1-23 through S1-23B amend Section 40 without weakening Sections 8, 9, 24 through 29, or MI-01/MI-02 |
| Source and format contract | Pass | stable synthetic bytes; exact `.txt`, `.md`, `.doc`, `.docx`, and PDF signatures and limits |
| Sensitive-data and zones | Pass | synthetic-only Human Zone; AI and Key zones untouched |
| Outcome variants | Pass | `ready`, `needs_review`, `blocked`, `failed`, and `cancelled` are exhaustive and all-or-nothing |
| Failure-code mapping and precedence | Blocked | several bound invalid, limit, Word, PDF, platform, and competing-condition cases lack one exact observable result |
| Identity and determinism | Pass | exact schemas, preimages, hashes, platform identity, observation order, and replay boundary |
| Storage and logging | Pass | bounded memory only; no source, extracted text, render, cache, or diagnostic output path |
| Dependency and compile surface | Pass | exact manifests, closure, features, checksums, license set, build targets, and absent dependencies |
| Product and command surfaces | Pass | exact Rust, Swift, harness, generator, fixture, and command allowlists; no live integration claim |
| Generated-artifact ownership | Pass | Cargo owns lockfile, current generator owns root inventory, fixture generator owns only declared generated fixture paths |
| Fixture bootstrap | Pass | isolated probe, independent handwritten inputs, create-only write, and independent check flow |
| Dispatch path | Pass | two permanent transformations are invoked separately by bound Rust and Swift harnesses |
| Test migration and regression | Pass | 21 MI-01/MI-02 tests remain unchanged; MI-03 adds only exact bound extraction cases |
| Correctness and privacy proof plan | Pass | exact/synthetic and actual-Vision oracles are separated; static and path checks are bound |
| Performance methodology | Pass | cold/warm commands, profiles, payloads, machine identity, time, RSS, and no unsupported claim are bound |
| Exact implementation bindings | Blocked | paths are exact, but test expectations cannot be completed without the missing failure matrix |
| Implementation-plan entry | Blocked | author closure and the separate peer audit have not passed |

“Pass” means the specification states a testable contract. It is not product
run evidence.

## Rerun evidence

| Check | Observed result |
|---|---|
| Repository identity | HEAD `7faf40e71652fcc483ae364043bb3f397bde784a` |
| Dirty paths | approved documentation and new MI-03 review artifacts only |
| Specification structure | exactly Sections 1 through 42 |
| JSON syntax | both roadmap JSON documents parse with `jq` |
| Whitespace | `git diff --check` passed before this audit rewrite |
| Rust identity | 1.89.0, full commit `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, arm64 host |
| Cargo identity | 1.89.0, full commit `c24e1064277fe51ab72011e2612e556ac56addf7` |
| Rust components and target | required compiler, Cargo, rustfmt, Clippy, rust-src, rust-analyzer, docs, and `aarch64-apple-darwin` target installed |
| Offline current workspace | locked metadata and no-run library test compile passed |
| Xcode and Swift | Xcode 26.6 build 17F113; Swift 6.3.3 arm64 target |
| macOS identity | product 26.5, build 25F71; `ProcessInfo` exposed `Version 26.5 (Build 25F71)` |
| Existing regression surface | offline locked single-threaded run passed exactly 21 tests with zero failure, ignore, measure, or filter |
| S1-23B dependency probe | two local plus 20 registry packages; seven custom builds; zero duplicates; unchanged lock hash |
| Cross-language JSON probe | representative nested identity bytes and hash equal; disposable outputs removed |
| Planned MI-03 paths | runtime, Swift extraction, harness, generator, and extraction fixture roots remain absent |
| Product and generated diff | empty for Cargo, Rust, Swift, tests, fixtures, inventory, toolchain, and build paths |
| MI-03 implementation plan | absent at author-gate completion |

## Sources read

- `AGENTS.md`, core invariants, and all applicable lifecycle, research,
  specification, peer-audit, implementation, code-style, testing,
  documentation, and reusable-prompt protocols;
- the complete active S1 specification and approved S1-23 through S1-23B;
- the MI-03 research brief and prior blocked audit artifacts;
- repository structure, file architecture, and `ROADMAP.json`;
- R1.2 extraction, R1 decision closure, and MI-01/MI-02 result evidence; and
- current source, manifests, lockfile, inventories, inventory generator,
  toolchain locator, ignore rules, path existence, and git state.

## Preserved limitations

- Rust and Swift remain unconnected in MI-03.
- AppKit, PDFKit, and Vision extraction fidelity and complete schema agreement
  remain unproved until implementation validation.
- Actual Vision equality is limited to one complete recorded platform
  identity.
- Apple helper processes, caches, crash artifacts, swap, and backup remain
  uncontrolled.
- Synthetic fixtures cannot establish confidential-data readiness, privacy,
  security, real-document fidelity, or production performance.

After the audit commands completed, an unrelated user-owned import appeared in
`crates/outis-core/src/detect/telephone.rs` and an untracked `.vscode/` tree
appeared. They were not created, edited, tested, or included by this
documentation audit. Their presence means the current worktree is not an
implementation baseline.

## Gate result

The author pre-audit is blocked at `S1-23-PA-08`. S1-23C must be explicitly
approved and incorporated, then the author pre-audit must restart. Only a
passed author audit permits the separate peer audit. Only passed author and
peer audits permit writing the exact MI-03 implementation plan.

No implementation plan or implementation code is authorized.
