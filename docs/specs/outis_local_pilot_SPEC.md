~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot Specification

## 1. Identification

- Slug: `outis_local_pilot`
- Product: Outis
- Version: 0.1 working draft
- Date: 2026-08-17
- Classification: specification authoring; no code
- Measured slice: local macOS synthetic funding demo
- Research input: `docs/reviews/outis_local_pilot/outis_local_pilot_r1_decision_closure.md`
- Decision method: sequential S1 packets, each requiring explicit user approval

## 2. Status

Status: `APPROVED_S1_21_PEER_AUDIT_PASSED_P1_01_APPROVED`

Approved decision packets:

- `S1-01` product boundary, approved 2026-08-17.
- `S1-02` source and filesystem contract, approved 2026-08-17.
- `S1-03` extraction and normalized-Markdown contract, approved 2026-08-17.
- `S1-04` sensitive classes and deterministic-discovery contract, approved
  2026-08-17.
- `S1-05` local contextual-model and model-artifact contract, approved
  2026-08-17 with the legal-clearance stop gate below.
- `S1-06` entity-resolution and review-decision contract, approved
  2026-08-17.
- `S1-07` tokenization and redaction contract, approved 2026-08-17.
- `S1-08` private-vault and plaintext trust-boundary contract, approved
  2026-08-17.
- `S1-09` agent-repository and atomic-publication contract, approved
  2026-08-17.
- `S1-10` macOS application-flow and job-state contract, approved 2026-08-17.
- `S1-11` macOS build, sandbox, signing, and Swift/Rust boundary contract,
  approved 2026-08-17.
- `S1-12` exact FFI, wire-schema, ownership, error, and codegen contract,
  approved 2026-08-17.
- `S1-13` exact dependency, feature, license, and supply-chain contract,
  approved 2026-08-17.
- `S1-14` whole-pipeline failure, recovery, retention, deletion, backup, and
  audit contract, approved 2026-08-17.
- `S1-15` end-to-end runtime performance, resource, application-size, and
  compile-time budget contract, approved 2026-08-17.
- `S1-16` correctness, extraction, detection, entity-resolution, privacy,
  vault-isolation, and agent-boundary oracle contract, approved 2026-08-17.
- `S1-17` benchmark methodology, synthetic-fixture, test-matrix, command, and
  evidence-artifact contract, approved 2026-08-17.
- `S1-18` final code, build, generated-artifact, model-acquisition,
  dataset-generation, and review binding contract, approved 2026-08-17.
- `S1-19` model-specific legal-gate partition, approved 2026-08-17.
- `S1-20` first complete pre-model implementation increment, approved
  2026-08-17.
- `S1-21` exact installed-toolchain resolution and offline preflight,
  approved 2026-08-17.

All sequential S1 design packets and gate amendments are explicitly approved.
`S1-20` supersedes the broad pre-model implementation permission in `S1-19`;
`S1-21` resolves the toolchain name without changing the compiler identity or
complete-pilot architecture. The S1-21 author pre-audit and separate peer audit
have passed.

Conditional model stop gate: model-weight, base-model, training-corpus,
commercial redistribution, attribution, notice, and source-obligation
clearance remains pending. It blocks model-specific dependencies, source,
tests, acquisition, bundling, integration, distribution, and complete-pilot
acceptance. It does not block the exact `S1-20` email-discovery capability
after the author-pre-audit, peer-audit, and implementation-plan gates pass.

Platform prerequisite resolved: Xcode 26.6 build `17F113`, macOS SDK 26.5,
Swift 6.3.3, and macOS 26.5 build `25F71` were observed with first-launch
status complete at `/Applications/Xcode.app/Contents/Developer`. A disposable
arm64 probe linked and ran Swift against a Rust 1.89 static library while
importing every selected Apple framework. This is toolchain integration
evidence, not an Outis application build.

This specification alone does not authorize code, dependencies, targets,
generated bindings, model artifacts, schemas, fixtures, or build changes. The
S1-21 author pre-audit and separate peer audit passed. The original `MI-01`
implementation plan and its P1-01 inventory-validation correction are
approved. MI-01 starts only after the amended plan is committed on a clean
baseline and its exact offline preflight passes. Model-specific work also
requires the qualified legal clearance above.

## 3. Purpose

Define the smallest complete Outis funding-demo slice: one authorized user runs
one local foreground job on one supported Mac to transform a bounded document
folder into a separate tokenized Markdown repository while keeping sensitive
mappings in a separate private local vault.

The slice exists to demonstrate and measure the complete local workflow on
synthetic data. It is not evidence of suitability for confidential data.

## 4. Non-goals

The first slice excludes:

- real confidential or production data;
- Intel Mac, universal binaries, non-macOS platforms, and macOS before 14;
- Finder extension or Finder contextual action;
- daemon operation, continuous watching, and synchronization;
- remote inference or services, including Swiss-hosted verification;
- Agent Service, remote Key Service, RAG, embeddings, chat, response
  rendering, and token-reversal UI;
- spreadsheets, presentations, email containers, archives, and standalone
  images;
- claims of anonymity, perfect discovery, proved security, extraction
  completeness, or confidential-data readiness.

These exclusions are typed rejection or unavailable behavior, not scaffolding
requirements.

## 5. Measured object

On one arm64 Mac running macOS 14 or later, the authorized user selects a local
source folder and a separate export destination, then starts one foreground
job in the Outis application.

The accepted source classes are `.doc`, `.docx`, text-bearing PDF, scanned or
image-only PDF, UTF-8 `.txt`, and UTF-8 `.md`. Italian, German, and French are
the supported human languages. Extraction, automatic sensitive-entity
discovery, cross-document entity resolution, review, and tokenization execute
locally.

For every successfully processed source document, the job produces one
tokenized `.md` document at the corresponding approved relative location in a
separate agent-facing repository named `outis`. The original source, the
agent-facing repository, and the private local vault are separate stores. The
private entity graph, token dictionary, sensitive mappings, and secret material
must not be stored in the agent-facing repository.

Completion means only that the job reached a terminal success state under the
approved contract. It does not mean the output is anonymous, safe, complete,
perfectly detected, secure, or ready for confidential information.

## 6. Source ownership and authorization contract

The first slice accepts synthetic data only. For each job, the user must
confirm that they are authorized to process the selected folder and that it
contains no real confidential data. Outis records this confirmation locally;
it does not determine or claim to verify ownership.

The user selects exactly one source directory and one export-parent directory
through separate native macOS folder panels. Outis creates the final agent
repository at `<export-parent>/outis`. Source access is read-only. Folder access
exists only for the active job: no security-scoped bookmark or other persistent
folder authorization is stored.

Source paths, filenames, and snapshot records are potentially sensitive Human
Zone data. They must not enter the agent-facing repository.

## 7. Source snapshot, enumeration, and filesystem contract

### Placement

The source, export parent, final `outis` repository, staging directory, and
private vault must not be equal, ancestors, descendants, aliases, or physical
references to the same directory. The only exception is that the final and
staging directories are direct children of the export parent.

Source and destination roots must be real local directories. A root that is a
symlink, Finder alias, package, network location, or unmaterialized cloud
placeholder blocks the job.

### Enumeration

Outis recursively traverses ordinary directories. It accepts regular `.doc`,
`.docx`, `.pdf`, `.txt`, and `.md` files using an ASCII-case-insensitive
extension comparison, subject to the content inspection contract in Section
8. A hidden file with a supported extension is processed normally. Empty
directories produce no Markdown artifact. `.DS_Store` is the only silently
ignored entry.

The complete job blocks if the tree contains:

- any other file type or extension;
- a symlink or regular file with multiple hard links;
- a Finder alias or macOS package;
- a socket, device, or FIFO;
- a mount-point crossing;
- a nested `.git`, `.hg`, or `.svn` repository;
- an unreadable entry;
- a non-local or unmaterialized cloud entry;
- a nonempty resource fork; or
- any extended attribute other than `com.apple.quarantine` and
  `com.apple.metadata:kMDItemWhereFroms`.

The two allowed extended attributes are ignored without reading or copying
their values. Filesystem ownership, permissions, timestamps, ACLs, Finder
metadata, resource forks, and extended attributes are never copied into the
agent-facing repository.

### Bounds and paths

The source tree is bounded by:

- at most 1,000 supported documents;
- at most 2,048 traversed entries, excluding `.DS_Store`;
- at most 32 directory levels below the selected root;
- valid UTF-8 relative paths of at most 1,024 UTF-8 bytes;
- path components of at most 255 UTF-8 bytes; and
- no empty, `.`, `..`, or NUL-containing component.

Exceeding a bound blocks the complete job. Outis does not truncate, silently
omit, or partially process the source tree.

Relative path bytes are preserved without Unicode normalization. Files are
processed in ascending raw UTF-8 relative-path byte order.

### Snapshot and mutation

For every traversed directory and document, the source snapshot records its
relative path, filesystem type, device, inode, link count, byte size, and
modification time with nanosecond precision. It additionally records SHA-256
of every regular file's contents. SHA-256 serves only as snapshot identity and
review binding; it is not encryption or a confidentiality control.

Entries are opened read-only with no-follow behavior. File identity is checked
before and after reading, and hashing and extraction bind to the same validated
contents.

Immediately before publication, Outis re-enumerates the complete tree and
compares it with the original snapshot. An added, removed, renamed, replaced,
or changed entry blocks publication as `source_changed`. Outis must not publish
a mixed snapshot.

Any source-contract violation blocks the complete job with a typed private
error. No document is published separately and no source file is modified.
Partial-staging cleanup and last-valid-output preservation are controlled by
the closed Section 20 contract.

## 8. Source format and extraction contract

### Ownership and fallback

Rust owns source validation, content signatures, UTF-8 extraction,
normalized-document validation, and size enforcement. Swift owns native binary
extraction through macOS frameworks. The first slice has no Pandoc,
LibreOffice, Tika, Tesseract, remote converter, external extraction process, or
fallback extractor. Failure blocks explicitly.

### Content classification

Before extraction, Rust binds the extension to exactly one class and validates:

- `.doc`: the first eight bytes are `D0 CF 11 E0 A1 B1 1A E1`;
- `.docx`: the first four bytes are `50 4B 03 04`, `50 4B 05 06`, or
  `50 4B 07 08`;
- `.pdf`: `%PDF-` occurs within the first 1,024 bytes;
- `.txt` and `.md`: an optional leading UTF-8 BOM followed by strict UTF-8
  without NUL.

Extension and signature disagreement blocks as `format_signature_mismatch`.
Outis does not guess a type or correct an extension.

### Text and Markdown

For `.txt` and `.md`, Rust removes exactly one leading UTF-8 BOM when present
and preserves every remaining UTF-8 byte until token replacement. Line endings,
whitespace, and Markdown syntax are not rewritten. Empty files are valid.
Invalid UTF-8 or any NUL blocks. Markdown is scanned as text, including prose,
front matter, code, link targets, and tables.

### Word

Swift imports Word documents through AppKit `NSAttributedString` using the
declared document type `.docFormat` for `.doc` and `.officeOpenXML` for
`.docx`. AppKit must accept that type. The visible `NSAttributedString.string`
is the extraction result.

An exposed attachment, reported lossy conversion, corrupt input,
password-protected input, or rejected import blocks. Document metadata is not
exported.

Comments, revisions, headers, footers, tables, lists, columns, and embedded
content can flatten or remain undiscovered. Every Word result therefore
requires review and must not be described as complete or layout-faithful.

### PDF

Swift opens PDFs with PDFKit. A zero-page, encrypted, locked,
password-required, or copy-disallowed PDF blocks. A page with nonzero rotation
blocks. A detected annotation, form widget, attachment, or embedded-content
feature blocks. Document metadata and outlines are excluded.

Every page is inspected independently. A `PDFPage.string` containing at least
one scalar outside this whitespace set uses native PDF text:

- U+0009 through U+000D;
- U+0020, U+0085, U+00A0, and U+1680;
- U+2000 through U+200A; and
- U+2028, U+2029, U+202F, U+205F, and U+3000.

An otherwise empty page uses OCR. Native text and OCR are never combined on
one page. Image text can therefore be missed on a page that also exposes
native text; mandatory review controls this declared limitation.

### OCR

OCR uses exactly:

- Vision `VNRecognizeTextRequestRevision3`;
- `.accurate` recognition;
- `it-IT`, `de-DE`, and `fr-FR` in that order;
- automatic language detection enabled;
- language correction disabled;
- no custom words;
- `minimumTextHeight = 0`;
- one PDF page and one Vision request at a time; and
- the top candidate only.

Before the first OCR operation, Outis verifies that all three languages are
available. Missing required language support blocks.

Each page media box is rendered at 200 DPI on opaque white in sRGB, with eight
bits per component, 32 bits per pixel, and premultiplied-last RGBA. Each pixel
dimension is `ceil(points * 200 / 72)`. A non-finite or non-positive dimension
blocks. Outis does not silently downscale.

OCR observations require finite normalized bounding boxes inside `[0,1]` with
positive width and height. They are ordered by decreasing midpoint Y,
increasing minimum X, decreasing maximum Y, increasing width, then original
Vision result index. No geometric tolerance or row-merging heuristic applies.
Ordered top-candidate strings are joined with one LF. An empty candidate,
invalid box, or page without observations blocks.

### Resource limits

- `.txt` and `.md`: at most 10 MiB per file.
- `.doc`, `.docx`, and `.pdf`: at most 50 MiB per file.
- Total source content: at most 250 MiB per job.
- Normalized UTF-8: at most 10 MiB per document and 100 MiB per job.
- PDF pages: at most 100 per document.
- OCR pages: at most 200 per job.
- OCR rendering: at most 4,096 pixels on either axis and 16,777,216 pixels per
  page.
- Extraction and OCR concurrency: one operation.

Exceeding a limit blocks. There is no truncation, downscaling, partial
extraction, or fallback. These are funding-demo guardrails, not capacity
claims.

### Extraction review and outcomes

Every `.doc`, `.docx`, and PDF result, including native-text PDF, is
`needs_review`. The application displays the original in an embedded
`QLPreviewView`, the complete normalized text beside it, and PDF page number
and extraction mode. The user either confirms the exact extraction or rejects
it.

Confirmation binds source SHA-256, normalized SHA-256, extraction identity,
and review-schema version. Any change invalidates confirmation. Rejection
blocks the complete job. A whitespace-only Word result can be confirmed only
after inspection and is never represented as proof of complete extraction.
Quick Look is a review aid, not extraction evidence. Its unmeasured helper and
cache behavior remains incompatible with real-confidential-data authorization.

Valid `.txt` and `.md` are `ready` without extraction review, although later
discovery review can still apply. Input or policy violations, unsupported
features, failed review, and exceeded limits are `blocked`. Unexpected platform
or internal extraction errors are `failed`. User cancellation observed between
bounded operations is `cancelled`.

Cancellation is checked between files, pages, renders, OCR requests, and
extraction submissions. One AppKit, PDFKit, or Vision call may be
non-preemptible; no cancellation-latency claim is made.

The application creates no plaintext extraction file or disk cache. Extracted
text, rendered pixels, and framework objects remain in bounded job memory, and
page buffers are released after submission. Source text, sensitive paths, OCR
output, entity values, and mappings are prohibited from normal logs and error
payloads. Apple-framework internal caching and crash artifacts are not claimed
to be controlled; the pilot therefore remains synthetic-only.

## 9. Normalized-document contract

### Text profile

For `.txt` and `.md`, the normalized bytes are the exact BOM-free source bytes.

For Word and each PDF page:

1. CRLF becomes LF.
2. Remaining CR, U+2028, and U+2029 become LF.
3. Tabs, spaces, and all other scalars are preserved.
4. Trailing LF bytes are removed.
5. PDF pages are joined with exactly two LF bytes.
6. The complete binary-derived document ends with exactly one LF.

Outis adds no headings, page labels, filenames, timestamps, comments, or front
matter. The result is text-oriented Markdown, not a layout replica.

### `NormalizedDocumentV1`

The normalized-document record contains:

- source SHA-256;
- source class;
- normalized UTF-8 bytes;
- normalized SHA-256;
- extraction identity; and
- ordered provenance segments.

Each segment contains a zero-based half-open UTF-8 byte range and exactly one
origin:

- `source_utf8`, including the source byte offset after an optional BOM;
- `appkit_visible_text`, with source offsets explicitly unavailable;
- `pdf_text`, with zero-based page index;
- `vision_ocr`, with zero-based page index, observation index, bounding box,
  and confidence; or
- `generated_page_separator`.

Ranges end on Unicode-scalar boundaries and cover the normalized bytes without
overlap or unexplained gaps. Provenance, confidence, bounding boxes, paths, and
diagnostics remain private and must not enter the agent-facing repository.

Detection and token replacement operate on zero-based half-open ranges in the
normalized UTF-8 bytes and only at Unicode-scalar boundaries.

### Extraction replay identity

Extraction identity binds:

- source bytes and ordering;
- macOS product version and build;
- CPU architecture and framework environment;
- extraction API and declared document type;
- Vision revision and every option;
- OCR-language availability result and order;
- PDF page box and render configuration;
- observation ordering; and
- normalized-document schema version.

Byte-identical replay is required only under the same complete identity. No
equality across different macOS builds or machines is promised. A changed OS
build invalidates prior extraction approval and requires the synthetic
regression suite to pass again.

## 10. Sensitive-data classification contract

### Active classes

The first slice supports exactly these classes, in this order:

1. `person`
2. `organization`
3. `postal_address`
4. `email_address`
5. `telephone_number`
6. `iban`
7. `matter_identifier`

Aliases are entity properties, not separate classes.

Government identifiers, credentials, dates, events, and generally
confidential passages are unsupported. This is a known coverage limit, not a
classification that those values are non-sensitive.

### Discovery surfaces

Discovery runs on the complete normalized document text, every source
directory-name component, and the source filename without its extension. Each
path component is scanned independently. A candidate cannot cross path
components or cross between path and document content.

Line-sensitive rules recognize LF, CRLF, CR, U+2028, and U+2029 as line
boundaries without rewriting source bytes. All observed values, path
components, and evidence remain private.

## 11. Automatic discovery contract

### `SensitiveCandidateV1`

Every discovery result contains:

- source snapshot identity;
- surface `document_text` or `path_component`;
- path-component index when applicable;
- zero-based half-open UTF-8 byte range;
- proposed sensitive class;
- exact observed bytes;
- class-specific equality key when available;
- detector identifier and version;
- evidence; and
- status `accepted`, `needs_review`, or `conflict`.

Ranges end on Unicode-scalar boundaries. Candidate records must not enter the
agent-facing repository.

### Email address

Automatic acceptance requires:

- exactly one ASCII `@`;
- a local part of 1 through 64 bytes using ASCII letters, digits, and
  `!#$%&'*+/=?^_{|}~-.`;
- no leading, trailing, or repeated local-part dot;
- domain labels of 1 through 63 ASCII bytes using letters, digits, and
  internal hyphens;
- no leading or trailing domain hyphen;
- an alphabetic final label of 2 through 63 bytes; and
- no more than 254 bytes overall.

The span covers the complete address. Its equality key preserves the local
part and ASCII-lowercases the domain. Any maximal non-whitespace value
containing `@` that fails this grammar is `needs_review`. Quoted, commented,
internationalized, and domain-literal addresses are not automatically
accepted.

### Telephone number

Automatic acceptance requires a leading `+`, country code `33`, `39`, `41`,
or `49`, only digits, ASCII spaces, U+00A0, `.`, `-`, `(`, and `)` after the
plus sign, and 8 through 15 digits in the complete number including country
code.

The span covers the complete formatted number. Its equality key is `+`
followed by all digits. A same-line telephone-like span containing 8 through
15 digits but missing the required international form, using another country
code, or carrying an extension is `needs_review`. Outis does not claim complete
national-numbering-plan validation.

### IBAN

An IBAN candidate begins with two ASCII letters and two digits, remains on one
logical line, and contains only ASCII letters, digits, and Unicode inline
whitespace. Validation uppercases it, removes whitespace, and requires MOD-97
remainder one.

Automatic acceptance is limited to:

| Country | Length | Structure after country and check digits |
|---|---:|---|
| `CH` | 21 | 5 digits and 12 alphanumeric characters |
| `DE` | 22 | 18 digits |
| `FR` | 27 | 10 digits, 11 alphanumeric characters, and 2 digits |
| `IT` | 27 | 1 letter, 10 digits, and 12 alphanumeric characters |

The equality key is the uppercase whitespace-free IBAN. The span includes the
original internal whitespace. A plausible value failing length, country
structure, or checksum is `needs_review`. An otherwise valid unsupported
country is also `needs_review`.

The deterministic detector exclusively owns IBAN classification. The local
model cannot accept or validate an IBAN.

### Postal address

Postal-address candidates always require review. An address occupies one
logical line and begins with either a standalone street cue followed by one
through six street-name words, or one German street word ending in an approved
joined suffix. It is followed by:

- a house number of 1 through 5 digits with an optional ASCII letter;
- an optional comma;
- a four-digit Swiss or five-digit German, Italian, or French postcode; and
- a city of 1 through 4 words and at most 64 Unicode scalars.

Street and city words contain Unicode letters and may contain internal U+0027,
U+2019, or U+002D. The candidate stops before `.`, `;`, `:`, or the line end.

Approved cues are:

- Italian: `via`, `viale`, `corso`, `piazza`, `largo`, `vicolo`, `strada`;
- German: `straße`, `strasse`, `weg`, `platz`, `gasse`, `allee`, `ring`,
  `ufer`;
- French: `rue`, `avenue`, `boulevard`, `chemin`, `route`, `place`, `quai`,
  `impasse`.

Cue matching is case-insensitive after NFC normalization, with the candidate
span mapped to the original bytes. The span covers the complete address. Local
model `LOC` output can provide overlapping evidence but never creates or
accepts an address by itself.

### Matter identifier

A matter identifier requires a same-line cue followed within 24 Unicode
scalars by an identifier of 3 through 64 characters. The identifier must
contain at least one digit and only ASCII letters, digits, `/`, `.`, `_`, and
`-`.

Approved cues are:

- Italian: `fascicolo`, `pratica`, `contratto`;
- German: `aktenzeichen`, `az.`, `akte`, `vertrag`;
- French: `dossier`, `affaire`, `contrat`.

Only the identifier is sensitive, not its cue. Every matter identifier
requires review. Its equality key is NFC-normalized, trimmed,
Unicode-lowercased, and whitespace-collapsed; punctuation remains significant.

### Person and organization boundary

The deterministic layer does not infer people or organizations from
capitalization, titles, legal suffixes, email names, or surrounding addresses.
Person and organization candidates come from the local contextual model
defined by Section 12 and always require review.

### Decisions, ordering, and overlap

Valid email addresses, international telephone numbers, and supported-country
IBANs are automatically `accepted`. Postal addresses, matter identifiers,
people, organizations, plausible but invalid or unsupported structured values,
and unresolved overlaps are `needs_review`. Unresolved review blocks
publication.

Candidates are ordered by source-document order, path components from root to
leaf before document text, start byte, end byte, active-class order, detector
order, then detector-evidence identity. Identical class-and-span evidence is
deduplicated.

Structured detection runs before the contextual model. A model span completely
contained by an accepted email, telephone, or IBAN span is discarded. An
accepted structured candidate outranks other overlapping same-class evidence.
Every other non-identical, nested, partial, or cross-class overlap becomes
`conflict`; it is not silently merged or independently replaced.

### Determinism and claim boundary

Deterministic-discovery identity binds normalized-document schema and hash,
path-component bytes, detector version, cue and grammar tables, Unicode
normalization and case-mapping version, active-class order, ordering rules, and
equality-key rules. Identical inputs and identity must produce byte-identical
ordered candidates.

Outis measures false negatives, false positives, unresolved candidates, and
conflicts on approved synthetic Italian, German, and French fixtures. Passing
those fixtures does not prove complete discovery. Unsupported classes and
values outside the declared grammars can remain undetected.

## 12. Local model and model-artifact contract

### Selection and identity

The contextual detector is fixed to:

- model `Davlan/bert-base-multilingual-cased-ner-hrl`;
- upstream revision `e756de7f7b8f64fea0c3d7c3872f1322fab747b1`;
- publisher artifact `onnx/model.onnx`;
- publisher tokenizer `onnx/tokenizer.json`;
- ONNX Runtime 1.28.0;
- `CPUExecutionProvider`; and
- arm64 macOS.

Approved artifact identities are:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `model.onnx` | 709,345,293 | `6c018415dc8129b358e9d629543c17481ad067ad02f9a6b8750473f161f9c5bd` |
| `tokenizer.json` | 2,919,362 | `bf1b59b7b11c95f194f51708d918eea378e09d05f84c0e1656dc5180e8117088` |
| ONNX Runtime archive | 32,396,562 | `1268b359718099bde2cedb55787f182a130067bc4f31e8c88478c445b850d3d8` |
| Runtime dylib | 39,312,136 | `dc19bbcb2f5c9fb3c68b4f9248aa0a35065ff702c5dbeae75eac54a74da97b6d` |

The acquisition sources are fixed to:

~~~text
https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl/resolve/e756de7f7b8f64fea0c3d7c3872f1322fab747b1/onnx/model.onnx?download=true
https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl/resolve/e756de7f7b8f64fea0c3d7c3872f1322fab747b1/onnx/tokenizer.json?download=true
https://github.com/microsoft/onnxruntime/releases/download/v1.28.0/onnxruntime-osx-arm64-1.28.0.tgz
~~~

The combined measured payload is 751,576,791 bytes. It is not described as a
small disk artifact.

Primary publisher references are the
[model card](https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl)
and [label configuration](https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl/blob/main/config.json).

### Legal stop gate

Technical selection is not legal clearance. Before any model-specific
dependency, source, test, manifest, acquisition script, artifact, Xcode phase,
bundle resource, integration, execution, or redistribution is created or
modified, a written review must clear the model-weight licence, base-model
terms, training-corpus terms, commercial distribution, attribution and
notices, and modification or source-availability obligations.

Until then, the only implementation increment permitted by this specification
is `MI-01`, the complete deterministic email-discovery capability defined in
Section 40. It has no model, application, FFI, extraction, vault, tokenization,
export, publication, fixture, or generated-artifact surface. It must not add
`ort`, `tokenizers`, `local_entity_model.rs`, `model_contract.rs`, `models/`,
the ONNX Runtime dylib, `fixtures/`, model resources, model build phases, or a
temporary model interface, stub, fallback, feature flag, or substitute.

`MI-01` neither publishes nor simulates an agent repository, and it has no job
or `completed` status. Any later pre-model increment requires a separately
approved specification amendment or an already exact complete-capability
binding plus its own approved implementation plan. `S1-20` is not blanket
authorization for other model-independent surfaces.

If clearance fails, model selection returns to research. There is no temporary,
automatic, or undocumented substitute.

### Packaging and startup

Logical locations are:

- manifest `models/manifests/entity_detector.json`;
- uncommitted acquisition area `models/artifacts/entity_detector_v1/`;
- bundled model
  `Outis.app/Contents/Resources/Models/EntityDetector/model.onnx`;
- bundled tokenizer
  `Outis.app/Contents/Resources/Models/EntityDetector/tokenizer.json`; and
- runtime
  `Outis.app/Contents/Frameworks/libonnxruntime.1.28.0.dylib`.

Startup resolves artifacts only from the signed application bundle, verifies
all approved SHA-256 values before session creation, and validates all names,
types, and shapes. The model must expose `input_ids`, `attention_mask`, and
`token_type_ids` as `int64` and one `float32` logits output shaped
`[batch, sequence, 9]`. A mismatch blocks.

Runtime download, conversion, repair, discovery, and substitution are
forbidden.

### Label mapping

The numeric mapping is exact:

| ID | Label | Outis handling |
|---:|---|---|
| 0 | `O` | no candidate |
| 1 | `B-DATE` | unsupported private date evidence |
| 2 | `I-DATE` | unsupported private date evidence |
| 3 | `B-PER` | `person` candidate |
| 4 | `I-PER` | `person` continuation |
| 5 | `B-ORG` | `organization` candidate |
| 6 | `I-ORG` | `organization` continuation |
| 7 | `B-LOC` | location evidence only |
| 8 | `I-LOC` | location-evidence continuation |

Any other count, name, or order is an artifact-contract failure.

### Runtime configuration

ONNX execution is sequential with graph optimization `ORT_ENABLE_ALL`, batch
size one, two intra-operation threads, one inter-operation thread, one
inference call at a time, and one immutable session per active job. Documents
and windows are not processed concurrently. There is no thread selector,
automatic tuning, or alternative performance mode. The session is released at
terminal job status.

The first slice uses CPU only. Core ML, GPU, Metal, and Neural Engine providers
are excluded. The two-thread setting neither reserves nor pins two physical
cores.

### Input and windowing

After deterministic structured detection, NER processes path components from
root to leaf, the filename without extension, and complete normalized document
text. The pinned cased tokenizer is used unchanged without locale-sensitive
lowercasing and retains original UTF-8 byte offsets.

Windowing is exact:

1. tokenize the complete surface without special tokens;
2. take 510 content tokens;
3. overlap 64 content tokens;
4. advance by 446 tokens;
5. add `[CLS]` and `[SEP]`;
6. process in source-byte order; and
7. stop after the first window containing the final token.

One document can contain at most 32,768 content tokens and one job at most
131,072. Exceeding either bound blocks without truncation. These are demo
guardrails, not capacity claims.

### Candidate assembly

For each content token, Outis selects the greatest finite logit; a numeric tie
selects the lowest label ID. Invalid offsets, non-scalar boundaries, NaN, or
infinite logits block. Special and padding tokens cannot create candidates.

`B-*` starts a span and a matching `I-*` continues it. An orphaned or
cross-class `I-*` blocks as `model_invalid_bio`. The span runs from the first
token's start byte through the last token's end byte. Entity confidence is the
minimum softmax probability across its tokens.

Exact duplicate `(label, start, end)` results from overlapping windows are
deduplicated while retaining maximum candidate confidence. Non-identical
overlaps are not merged. A result touching a nonterminal window boundary gains
boundary-risk evidence and requires review.

### Thresholds and decisions

| Output | Threshold | Result |
|---|---:|---|
| `PER` | at least 0.50 | `person`, always `needs_review` |
| `ORG` | at least 0.50 | `organization`, always `needs_review` |
| `LOC` | at least 0.50 | evidence only when overlapping a deterministic address; always reviewed |
| `DATE` | at least 0.50 | unsupported private review item |
| supported label | below 0.50 | not promoted; no safety conclusion |
| unknown label | any | block |

Confidence is not calibrated probability and never establishes correctness,
completeness, or safety.

Structured email, telephone, and IBAN detection runs first. A NER result fully
contained within an accepted structured span is discarded. A partial,
enclosing, or cross-boundary overlap becomes `conflict`. NER cannot create or
validate an IBAN, email, or telephone number, and `LOC` alone cannot create a
postal address.

### Privacy, failure, and cancellation

The model is trusted local Human Zone preprocessing. Plaintext does not leave
the Mac. Runtime networking, download, telemetry, tools, training,
fine-tuning, and persistent input, token, logits, output, or plaintext caches
are forbidden. ONNX Runtime log level is `ERROR`; only non-plaintext Outis
codes can enter logs. Input and output buffers are released after the document,
and the session is not intentionally retained while Outis is idle. Model output
remains untrusted candidate evidence.

The job blocks on a missing or altered artifact, wrong runtime or provider,
invalid input or output contract, tokenizer or offset failure, invalid BIO,
unknown label, non-finite output, model token limit, or unavailable signed
runtime. An unexpected inference failure is `failed`. Cancellation is checked
before and after each window; an active window is not interrupted. There is no
fallback model or remote inference.

### Replay, budgets, and evidence

Replay identity binds model revision and hashes, tokenizer hash, runtime and
dylib identity, CPU provider, thread counts, execution and optimization modes,
application build, windowing, label mapping, confidence calculation, offset
conversion, and structured-overlap policy. Byte-identical logits and
candidates are required only under the same complete identity. Cross-machine
or cross-runtime equality is not claimed.

Budgets are:

| Measure | First-slice budget |
|---|---:|
| Model, tokenizer, and runtime | at most 760,000,000 bytes |
| Complete signed application | at most 850,000,000 bytes |
| Peak NER process RSS | at most 2,000,000,000 bytes |
| Warm 510-token p95 on the reference M4 Pro | at most 125 ms |
| Model session load on the reference M4 Pro | at most 2,000 ms |
| Runtime network requests | zero |

The repository-owned synthetic suite must reproduce the 30 Italian, German,
and French smoke cases and record exact spans, false positives, false
negatives, overlaps, and unresolved cases. It gates only the funding demo. A
larger legal-document corpus is mandatory before real confidential documents
can be authorized.

## 13. Entity and alias resolution contract

### Resolution scope and equality

Resolution occurs only within one repository and one active class. Equality
keys are:

- `person`, `organization`, and `postal_address`: NFC, trim surrounding
  Unicode whitespace, collapse internal Unicode whitespace to one ASCII space,
  and Unicode-lowercase;
- `email_address`: preserve the local part and ASCII-lowercase the domain;
- `telephone_number`: `+` followed by digits for automatically accepted
  international numbers, and digits only for reviewed noninternational
  numbers;
- `iban`: uppercase with whitespace removed; and
- `matter_identifier`: NFC, trim, Unicode-lowercase, and collapse whitespace.

Punctuation, accents, apostrophes, hyphens, and legal suffixes remain
significant. Equality never crosses repositories.

### Automatic linking and same-key ambiguity

Exact equality automatically links email addresses, telephone numbers, IBANs,
and matter identifiers.

Repeated exact person, organization, or postal-address keys form a proposed
entity group requiring `confirm_same_entity`. If the user states that an exact
key represents different entities, the first slice blocks. Same-key entity
splitting is outside the first slice and cannot be simulated with hidden
disambiguators.

### Explicit alias merge

Different equality keys are never merged automatically. `merge_entities` is
allowed only when both entities are individually reviewed, have the same active
class, have no unresolved candidate or conflict, and at most one already has a
persistent token.

The entity with the earliest resolved-candidate order becomes canonical. All
observed forms remain private aliases with provenance. Merging two already
tokenized entities blocks because token retirement and rotation are outside the
first slice.

Outis never merges through fuzzy similarity, surname, initials, titles,
honorifics, abbreviation expansion, legal-suffix removal, shared address,
nearby occurrence, confidence, or language-specific inference.

### Private graph

`EntityV1` contains repository identity, private entity identity, active class,
lifecycle status, canonical private display value, creation and update
evidence, and token reference after allocation.

`AliasV1` contains entity identity, equality key, exact observed bytes, source
and span provenance, detector or manual-review origin, and review decision.
The canonical display value is the first value in resolved-candidate order.
Neither record enters the agent-facing repository.

Review decisions and entity mutations become effective only after their
private-vault transaction commits. Vault failure leaves the decision
unaccepted, allocates no token, publishes nothing, and reports a typed private
failure. Review state is not stored in the source or agent repository.

Entity resolution is complete only when every extraction review is confirmed,
every required candidate is confirmed, corrected, or excluded, every overlap
is resolved, every repeated person, organization, or address key is confirmed,
every merge is valid, no unsupported-sensitive item remains, no entity has
conflicting classes, and every accepted span maps to exactly one entity.

## 14. Uncertainty and review contract

### Review order

Review order is binary extraction, candidate and conflict review in candidate
order, cross-document entity-group review, then the final unresolved-item
check. A later stage cannot bypass an unresolved earlier stage.

Allowed candidate actions are:

| Action | Effect |
|---|---|
| `confirm` | accept proposed class and exact span |
| `adjust` | change the span at Unicode-scalar boundaries |
| `reclassify` | select another active class |
| `exclude_false_positive` | reject this exact candidate as non-sensitive |
| `add_missed_candidate` | select a missed span and assign an active class |
| `mark_unsupported_sensitive` | confirm unsupported sensitive content and block |

`add_missed_candidate` is required so a reviewer can correct an observed false
negative. There is no bulk approval, global safe-list, or ignore-all action.

### Span and conflict rules

A final span is nonempty, stays within one document or path component, begins
and ends on UTF-8 scalar boundaries, excludes generated PDF page separators,
does not overlap another final span, and has one active class. A reviewer can
confirm a structured value outside its automatic grammar, but it is recorded
as human-confirmed rather than automatically validated.

Unsupported dates and general confidential passages cannot be assigned a
false class. They block through `mark_unsupported_sensitive`.

An overlap is resolved only by confirming one and excluding the others,
adjusting to non-overlapping spans, reclassifying and adjusting, or blocking as
unsupported sensitive content. Model confidence cannot choose a cross-class
winner.

### `ReviewDecisionV1`

Each decision binds repository identity, source snapshot and SHA-256,
normalized-document SHA-256, path or document surface, original candidate,
exact observed bytes and range, detector and model identity, original and
resulting class and range, action, review-schema version, actor
`authorized_local_user`, and audit timestamp. Timestamp is audit evidence and
does not affect tokens or deterministic output.

A decision is reusable only when every bound identity is equal. A source,
extraction, model, detector, span, or observed-byte change invalidates it.

`exclude_false_positive` is restricted to the exact repository, source
snapshot, normalized document, surface, byte range, observed value, and
detector identity. It does not classify the same text as globally safe and
does not create a glossary.

Completion records human decisions but does not prove complete discovery or
factually correct identity resolution.

## 15. Tokenization contract

### Grammar and disclosure

The exact token grammar is:

~~~text
{{<class>.<class>_<sequence>}}
~~~

Both class fields are identical and name one active class. Counters are
independent per class and begin at one. Values one through 9,999 use four
digits; larger values grow naturally without leading zeroes. Zero is never
issued. Counter overflow blocks as `token_space_exhausted`.

A token exposes sensitive class and approximate repository-local encounter
order. It contains no source value, value hash, source path, repository or
vault identity, or cryptographic protection. It is a pseudonym, not encryption
or anonymity evidence.

### Scope, allocation, and transaction

Token equality exists only inside one repository vault. The same resolved
entity uses the same token in content and paths; distinct entities use distinct
tokens. Coincidentally identical token text in different repositories has no
guaranteed relationship.

After review and entity resolution close, allocation reuses existing mappings,
groups new occurrences by entity, orders new entities by earliest final
candidate, allocates from each class counter, and persists all new mappings in
one vault transaction. Path candidates precede document-content candidates as
defined by Section 11.

The transaction enforces unique tokens, entity/token relationships, class and
sequence, and class equality keys where same-key splitting is unsupported. Any
collision or vault error rolls back the allocation unit.

Mappings commit before staging. If later publication fails or is cancelled,
allocated tokens remain reserved and are reused on retry. They are never
reassigned, so sequence gaps are allowed.

### Stability and unsupported lifecycle

A token remains stable while repository identity, entity identity, vault, and
supported schema remain unchanged. An entity absent from a later snapshot
retains its private mapping and reuses it if its approved identity reappears.

The first slice has no rotation, revocation, same-key split, merge of two
already-tokenized entities, selective token deletion, cross-repository
migration, counter reset, or token reuse. A request requiring one of these
operations blocks.

### Reserved source namespace

Before discovery, Outis scans document text and path components. A valid active
Outis token blocks, as does `{{<active-class>.` followed by closing `}}` within
128 UTF-8 bytes even when malformed. Other unrelated brace syntax is ordinary
source text.

### Content replacement

Each final accepted occurrence uses its entity token. Replacement applies to
the exact zero-based half-open UTF-8 range, in descending start-byte order,
with non-overlapping scalar-aligned spans. There is exactly one pass; generated
tokens are not rescanned. Every byte outside accepted spans is preserved and
the result must remain valid UTF-8.

Outis does not repair Markdown after replacement. A token in prose, code,
front matter, or a link target remains literal token text.

### Path replacement

Sensitive ranges in reviewed directory components and filename bases use the
same entity tokens as content. Nonsensitive bytes are preserved. The source
extension is not scanned and the target extension becomes `.md`. Replacements
within one component occur in descending byte order.

An unchanged component means only that no accepted candidate covered it. It
is not evidence that the component is non-sensitive. Target-path collisions
block under Section 18.

### Validation, limits, and replay

Before publication, every token must be canonical, map to one active entity in
the current vault, and match that entity's class. Unknown, forged, unmapped, or
malformed reserved-namespace values and plaintext mappings beside tokens block.
The first slice has no stale or revoked state.

One job permits at most 10,000 final sensitive occurrences and 5,000 new
entities. Exceeding a limit blocks without truncation. These are demo
guardrails, not capacity claims.

Byte-identical output requires identical source and normalized bytes, final
spans and classes, entity resolution, candidate order, initial vault state,
repository identity, counters, grammar version, replacement version, and path
rules. A different vault is not expected to produce identical token values.

Successful tokenization proves only that final accepted spans were replaced
consistently. It does not prove complete discovery, factually correct entity
resolution, resistance to contextual correlation, anonymity, or protection
from a caller that can access the vault.

## 16. Redaction contract

There is no independent irreversible-redaction mode in the first slice. Every
accepted sensitive span is tokenized, and unsupported sensitive content
blocks. Outis does not delete spans, mask with asterisks, or emit generic
`[REDACTED]` replacements.

There is no response-rendering or token-reversal interface. Technical
documentation calls the operation tokenization or pseudonymization. The
application action may say `Anonymize with Outis`.

## 17. Trust-zone and plaintext-copy contract

### Human Zone

Approved Human Zone plaintext copies are limited to original source files,
validated source and path buffers, AppKit/PDFKit/Vision/Quick Look processing,
in-memory extracted and normalized text, local NER inputs/token IDs/outputs,
review UI, and bounded Swift/Rust transfer buffers. The application creates no
plaintext extraction cache or temporary document.

Quick Look helpers, Apple-framework caches, and crash artifacts remain
unproved local copy points and reinforce the synthetic-only restriction.

### Key Zone

The Key Zone contains the plaintext vault and rollback journal, private entity
graph, aliases and equality keys, token dictionary, path mappings, and
vault-operation memory. Rust owns the vault connection and schema. Swift,
Finder surfaces, agents, and the generated repository receive no database
handle, SQL interface, or vault query capability.

### AI Zone

The AI Zone can receive only validated tokenized Markdown, tokenized or
reviewed relative names, and approved manifest data. It receives no source
binary, source or normalized plaintext, private provenance, entity or alias
record, equality key, token dictionary, vault path or identifier, review
decision, or Keychain material. Only the final published repository may be
granted to an agent.

Filesystem placement and sandboxing are not represented as protection from the
same macOS user, root, malware, backups, disk recovery, or an unsandboxed agent.

## 18. Agent-repository content and access contract

### Tree and modes

The final AI Zone tree is exactly:

~~~text
outis/
  outis-manifest.json
  <tokenized-relative-directories>/
    <tokenized-source-base-name>.md
~~~

Every accepted source has one `.md`. Empty source directories have no output.
Directories use mode `0700` and files `0600`. The tree has no Git metadata,
source binary, vault or journal, mapping, hidden application file, temporary
file, symlink, alias, package, socket, device, ACL, extended attribute, or
resource fork. Only regular directories, Markdown files, and the root manifest
are allowed.

Source-relative structure is mirrored after path tokenization. Output
components are valid UTF-8 of at most 255 bytes; complete relative paths are at
most 1,024 bytes and contain no empty, `.`, `..`, or NUL component.

Collision keys apply NFC normalization, Unicode default case folding, then
UTF-8 byte comparison, independently of filesystem behavior. Any file or
directory collision blocks. Outis never overwrites, adds an undisclosed suffix,
changes capitalization, invents a directory, or omits a colliding document.

The tree permits at most 1,000 Markdown files, one manifest, 128 MiB total, and
a 4 MiB manifest. Nothing is truncated or partially published.

### Manifest

`outis-manifest.json` has exact top-level key order:

~~~json
{"schema_version":1,"pipeline_version":"outis-local-pilot-v1","status":"complete","export_id":"<32 lowercase hexadecimal characters>","pipeline_identity_sha256":"<64 lowercase hexadecimal characters>","model_identity":{"model_sha256":"<64 lowercase hexadecimal characters>","tokenizer_sha256":"<64 lowercase hexadecimal characters>","runtime":"onnxruntime-1.28.0-cpu"},"document_count":0,"documents":[],"occurrence_counts":{"person":0,"organization":0,"postal_address":0,"email_address":0,"telephone_number":0,"iban":0,"matter_identifier":0},"document_tree_sha256":"<64 lowercase hexadecimal characters>"}
~~~

Each document object uses key order `path`, `bytes`, `sha256`. Documents are
ordered by raw UTF-8 output-path bytes.

The manifest is BOM-free UTF-8, compact JSON without optional whitespace,
fixed-key-order, lowercase-hexadecimal hashes, decimal integers without leading
zeroes, literal UTF-8 for non-ASCII, standard required JSON escaping, and one
final LF. It has no timestamp, machine path, private repository or vault
identity, source metadata, entity, alias, equality key, mapping, confidence,
review text, OS identity, or error information.

### Document-tree and export identities

`document_tree_sha256` hashes the bytes `OUTIS-DOCUMENT-TREE-V1\0` followed by
each manifest-ordered document framed as eight-byte big-endian path length,
raw UTF-8 path, eight-byte big-endian document length, and 32 raw document
SHA-256 bytes. The manifest is excluded to avoid self-reference.

`export_id` is the first 16 bytes of SHA-256 over
`OUTIS-EXPORT-ID-V1\0`, the raw document-tree SHA-256, and the raw pipeline-
identity SHA-256, encoded as lowercase hexadecimal. It is deterministic,
non-secret, and unrelated to the private repository identifier.

### Access and interpretation

After publication, Outis can open the final tree in Finder. The user separately
grants only that tree to an agent. Manifest completion means only that the
declared tree checks passed. It is not complete-detection, anonymity,
agent-safety, authenticity, or contextual-non-correlation evidence.

## 19. Private-vault, secret, and storage contract

### Store and repository identity

The first slice selects `rusqlite` 0.40.2 with default features disabled and
`bundled` enabled, `libsqlite3-sys` 0.38.2, and bundled SQLite 3.53.2. It uses
one connection, one writer, no pool, no shared cache, and no SQLCipher or other
encryption layer. Dependency-wide approval remains controlled by Section 25.

Each Outis project receives one random 128-bit repository identifier from
macOS `SecRandomCopyBytes`, encoded as 32 lowercase hexadecimal characters.
It is opaque but not secret and never appears in tokens or the agent
repository. Existing projects are listed by opaque identifier and creation
date, without a persisted source bookmark or source path in preferences.

A vault binds to source-volume identifier, source-root filesystem identifier,
first approved source snapshot, and repository identifier. Reuse requires the
user to select the existing project and source folder again. Mismatched
filesystem identity blocks as `wrong_repository_source`. Moving a folder while
preserving filesystem identity can retain the vault; copying it to a new
identity requires a new project and tokens.

### Location and permissions

The logical sandbox-container location is:

~~~text
Library/Application Support/Outis/Vaults/<repository-id>/outis-vault.sqlite3
~~~

The application bundle identifier determines the absolute container path.
`Vaults` and the repository directory have mode `0700`; the database and
rollback journal have mode `0600`. The vault cannot be user-selected or placed
in source, export, staging, a network volume, a symlink, an alias, or a package.
Modes and physical identities are checked at every open.

### Schema version one

The application schema contains exactly:

| Table | Purpose |
|---|---|
| `vault_meta` | singleton schema, repository, and version identity |
| `jobs` | state, authorization confirmation, and non-plaintext outcome |
| `snapshots` | complete source-snapshot identity |
| `source_items` | private paths, filesystem identity, and hashes |
| `extractions` | normalized hash, extraction identity, and private provenance |
| `review_decisions` | source-bound actions and observed-value hashes |
| `entities` | private entity graph nodes |
| `aliases` | equality keys and plaintext sensitive values |
| `occurrences` | source and path spans linked to aliases |
| `tokens` | entity-to-token mappings |
| `class_counters` | next sequence per active class |
| `path_mappings` | source item to tokenized export path |
| `audit_events` | ordered non-plaintext lifecycle evidence |

All tables are `STRICT` with mandatory foreign keys. Unknown columns, triggers,
views, virtual tables, or application tables block schema validation.

`vault_meta` has exactly one row. Repository identity is a 16-byte BLOB,
SHA-256 values are 32-byte BLOBs, and device and inode values use fixed
eight-byte BLOB encoding. Relative paths and sensitive values use UTF-8 BLOBs,
not database collation. IDs and audit sequences are positive integers. One
class equality key maps to at most one entity, one entity has at most one
token, and token text, class/sequence, and entity relationship are independently
unique. Occurrences and path mappings reference source items. Audit rows
contain codes and object identities, never plaintext. SQLite collation does not
determine equality.

The vault can persist plaintext aliases and canonical values, equality keys,
private relative paths, path mappings, source and normalized hashes, reviewed
spans and provenance, detector/model identities, token mappings, and decisions.
It does not persist complete normalized documents, OCR images, complete
extracted plaintext, model input/token/logit caches, Quick Look previews,
source binaries, or application logs. An alias value is stored once;
occurrences reference it.

### Exact version-one columns and indexes

The migration at
`crates/outis-runtime/migrations/0001_private_vault.sql` is mandatory and
contains exactly these columns:

~~~text
vault_meta(
  singleton_id INTEGER, schema_version INTEGER, repository_id BLOB,
  created_at_ms INTEGER, deletion_state TEXT, source_volume_id BLOB,
  source_root_device BLOB, source_root_inode BLOB,
  initial_snapshot_sha256 BLOB
)

jobs(
  id INTEGER, job_id BLOB, pipeline_identity_sha256 BLOB, state TEXT,
  terminal_outcome TEXT, domain_code TEXT, retry_class TEXT,
  private_subject_kind TEXT, private_subject_id INTEGER,
  platform_domain TEXT, platform_numeric_code INTEGER,
  synthetic_confirmation INTEGER, created_at_ms INTEGER,
  terminal_at_ms INTEGER, intended_export_id BLOB,
  published_export_id BLOB, export_parent_device BLOB,
  export_parent_inode BLOB, staging_name BLOB, publication_state TEXT
)

snapshots(
  id INTEGER, job_id INTEGER, source_volume_id BLOB,
  source_root_device BLOB, source_root_inode BLOB, tree_sha256 BLOB,
  item_count INTEGER, source_bytes INTEGER, created_at_ms INTEGER
)

source_items(
  id INTEGER, snapshot_id INTEGER, order_index INTEGER,
  relative_path BLOB, source_class TEXT, device_id BLOB, inode_id BLOB,
  link_count INTEGER, source_bytes INTEGER, source_sha256 BLOB
)

extractions(
  id INTEGER, source_item_id INTEGER, status TEXT, domain_code TEXT,
  adapter TEXT, extraction_identity_sha256 BLOB, normalized_sha256 BLOB,
  normalized_bytes INTEGER, provenance_json BLOB, review_status TEXT
)

review_decisions(
  id INTEGER, job_id INTEGER, source_item_id INTEGER,
  decision_order INTEGER, action TEXT, surface TEXT,
  path_component_index INTEGER, original_start_byte INTEGER,
  original_end_byte INTEGER, original_class TEXT, detector_id TEXT,
  observed_sha256 BLOB, result_start_byte INTEGER,
  result_end_byte INTEGER, result_class TEXT,
  decision_identity_sha256 BLOB, actor TEXT, decided_at_ms INTEGER
)

entities(
  id INTEGER, active_class TEXT, lifecycle_status TEXT,
  canonical_alias_id INTEGER, created_job_id INTEGER,
  created_order INTEGER, updated_job_id INTEGER
)

aliases(
  id INTEGER, entity_id INTEGER, active_class TEXT, equality_key BLOB,
  observed_value BLOB, origin TEXT, first_job_id INTEGER,
  first_source_item_id INTEGER, first_start_byte INTEGER
)

occurrences(
  id INTEGER, job_id INTEGER, source_item_id INTEGER, entity_id INTEGER,
  alias_id INTEGER, surface TEXT, path_component_index INTEGER,
  start_byte INTEGER, end_byte INTEGER, candidate_order INTEGER
)

tokens(
  id INTEGER, entity_id INTEGER, active_class TEXT, sequence INTEGER,
  token_text BLOB, created_job_id INTEGER
)

class_counters(active_class TEXT, next_sequence INTEGER)

path_mappings(
  source_item_id INTEGER, output_relative_path BLOB,
  output_collision_key BLOB, output_bytes INTEGER, output_sha256 BLOB
)

audit_events(
  sequence INTEGER, job_id INTEGER, unix_time_milliseconds INTEGER,
  event_code TEXT, stage TEXT, terminal_outcome TEXT, object_kind TEXT,
  object_id INTEGER, count_a INTEGER, count_b INTEGER, domain_code TEXT,
  platform_domain TEXT, platform_numeric_code INTEGER
)
~~~

All tables are `STRICT`. Repository and job identifiers are 16-byte BLOBs;
hashes are 32-byte BLOBs; device and inode identities are eight-byte BLOBs;
paths, equality keys, observed values, and tokens are UTF-8 BLOBs. IDs,
sequences, counts, sizes, and non-null timestamps are nonnegative, with IDs and
issued sequences strictly positive. Boolean integers are zero or one.

Primary keys, mandatory foreign keys, and the Section 19 uniqueness rules are
declared in the migration. Enumerated `TEXT` columns use exact `CHECK`
constraints from Sections 8, 10, 14, 21, 29, and 30. Nullable fields are
allowed only before the owning state produces their value. At every committed
entity-resolution boundary, `canonical_alias_id` is non-null and belongs to
that entity. `initial_snapshot_sha256` is null only before the first approved
snapshot and never changes afterward.

The only explicit indexes are source items by snapshot and order, review
decisions by job and order, occurrences by source item and candidate order,
and audit events by job and sequence. Primary-key and unique-constraint
autoindexes are allowed. There is no trigger, view, virtual table, migration
framework, or general SQL extension.

### SQLite configuration

Every connection verifies:

~~~text
foreign_keys = ON
journal_mode = DELETE
synchronous = FULL
temp_store = MEMORY
secure_delete = ON
trusted_schema = OFF
busy_timeout = 0
locking_mode = NORMAL
mmap_size = 0
~~~

Initial creation sets UTF-8 encoding, page size 4,096,
`application_id = 0x4F555449`, and `user_version = 1`. Extension loading is
never enabled. WAL and shared-cache modes are forbidden. `secure_delete` is not
physical-erasure evidence.

### Transactions and opening

Schema creation, review decisions, entity mutations, and token allocation use
explicit `BEGIN IMMEDIATE`, `COMMIT`, and error-path rollback. No transaction
waits for user review or spans agent-repository publication. Cancellation
during a transaction completes commit or rollback before acknowledgement.
`SQLITE_BUSY` becomes `vault_busy`; there is no hidden retry. SQL and bound
parameters are not logged.

Open verifies path, ownership, modes, SQLite header, `application_id`,
`user_version`, exact schema, foreign keys, PRAGMAs, and `PRAGMA quick_check`.
A mismatch, corruption, partial migration, unknown version, or unexpected
sidecar blocks. Outis never repairs, recreates, or deletes a vault
automatically. Version one has no migration predecessor.

Rollback journals are Key Zone plaintext beside the database with mode `0600`.
SQLite removes them after clean transactions. A crash-left journal is processed
only through SQLite recovery. WAL, shared-memory files, disk temporary tables,
and application-created database copies are forbidden.

### Bounds, retention, and deletion

The main database is limited to 256 MiB and the complete vault directory,
including rollback journal, to 512 MiB. A measurable excess blocks before
another write. Nothing is removed or compacted to meet the bound. These are
unmeasured demo guardrails.

The vault remains until the Section 30 explicit project-deletion procedure
completes. `vault_meta` contains a deletion state whose only values are
`active` and `deletion_pending`. There is no backup, export, restore, selective
mapping deletion, automatic expiry, compaction, or cloud synchronization.
Source and agent repositories are not automatically deleted, and no SSD,
snapshot, or backup erasure is claimed.

### Secret and claim boundary

The funding demo has no vault encryption, password, data-encryption key,
Keychain item, key rotation, or recovery key. The repository identifier is not
a secret. `Private vault` means structurally excluded from source and agent
repositories, not protected from local attackers.

The permitted claim is limited to the tested configuration storing private
mappings outside the tested source and generated repositories and excluding
them from the generated tree under the declared oracle. The vault cannot be
called encrypted, locally attack-resistant, or appropriate for confidential
documents.

## 20. Staging, validation, and atomic-publication contract

### Staging

Staging is a new same-parent sibling named
`.outis-staging-<32-lowercase-hex-job-id>` with mode `0700`. The export volume
must be local APFS and report atomic directory-exchange capability. Staging and
export parent have the same device identity. Name and parent identity are
recorded privately in the vault. An existing path of that name blocks.

Staging contains only the candidate agent tree, is never granted to an agent,
and contains no plaintext intermediate or private diagnostic.

### Complete validation

Before publication, validation proves:

- every expected source has exactly one Markdown output;
- file, directory, size, hash, and occurrence counts match the manifest;
- every path passes the allowlist, bounds, and collision rules;
- every entry is a regular file or directory with the required mode;
- no link, alias, package, mount crossing, ACL, extended attribute, or resource
  fork exists;
- every Markdown file is valid UTF-8;
- every token is canonical and maps to an active same-class vault entity;
- no malformed, forged, or unmapped token exists;
- source snapshots still match;
- no unresolved, blocked, failed, or cancelled state exists; and
- the known-plaintext oracle passes.

The known-plaintext scan covers every stored alias and canonical value,
sensitive observed path span, UTF-8 equality key, private repository and vault
identifier, absolute source and vault path, SQLite header, and prohibited
mapping artifact across every staged name and byte. A match blocks. This oracle
proves only absence of declared known values, not unknown false negatives or
anonymity.

### Initial and replacement publication

If final `outis` is absent, Outis validates and synchronizes staging, rechecks
absence, renames staging to `outis` on the same filesystem, synchronizes the
parent, and verifies the published tree. There is no copy publication.

If final `outis` exists, it must validate as an intact prior Outis repository.
An edited, malformed, replaced, or unrelated target blocks. Outis then
validates and synchronizes staging, performs
`renameatx_np(..., RENAME_SWAP)`, synchronizes the parent, verifies the new
final tree, and removes the old tree occupying the staging path. There is no
delete-then-rename, copy fallback, or overwrite of an unexpected target. A
modified agent repository must be moved or removed manually.

Every generated file receives checked `F_FULLFSYNC`; generated directories are
checked with `fsync` from leaves to root; the export parent is synchronized
before and after publication. This is a procedure, not proof against every
power-loss or filesystem failure.

### Failure, cancellation, and recovery

Failure before publication removes staging and preserves the previous valid
output. Committed tokens remain reserved. Cleanup failure is private and
explicit. After a successful swap, old-tree removal failure is a private
warning: the new final remains published and the old staging tree is not
agent-accessible.

Cancellation before the critical section removes staging and preserves old
output. During rename or swap, Outis completes or rejects the atomic operation
before acknowledging cancellation. After successful publication,
cancellation is too late and the job completes. A partial final repository is
never exposed.

When the user reselects an export parent, Outis checks vault-recorded staging:

- final absent and staging present: validate, remove staging, require rerun;
- old final intact and new candidate in staging: remove candidate, require
  rerun;
- intended new export ID final and validated prior repository in staging:
  treat swap as complete, remove old staging, complete audit;
- invalid or ambiguous state: block without deleting either tree.

Outis deletes only staging whose name, parent identity, job record, and contents
bind to the current vault. It never auto-publishes crash-left staging.

## 21. macOS application and lifecycle contract

### Surface and project selection

The first slice has one native foreground application, one main window, one
`MenuBarExtra`, one active job, and English UI. Document processing covers
Italian, German, and French. There is no Finder extension, daemon, background
agent, login item, notification permission, or automatic updater. A second
launch activates the existing application.

The main window offers `New Outis Project` and `Open Existing Outis Project`.
New-project vault identity is created after folder validation. Existing-project
use requires opaque-project selection and reselection of source and export
folders. No folder bookmark is remembered.

Before every job, the user must check `I confirm that all document content is
synthetic and that I am authorized to process it.` This cannot be remembered
and is stored in the private audit record. The window permanently displays
`Funding demo: synthetic data only. Do not use confidential documents.`

The primary `Anonymize with Outis` action is enabled only with a valid project,
source, export, separation checks, per-job confirmation, and no active job.

### State machine

Persisted nonterminal states are `validating`, `extracting`, `detecting`,
`needs_review`, `tokenizing`, `validating_export`, and `publishing`. Terminal
states are `completed`, `cancelled`, `blocked`, and `failed`. `idle` is an
application state only.

Allowed transitions are:

~~~text
idle -> validating
validating -> extracting | blocked | failed | cancelled
extracting -> detecting | blocked | failed | cancelled
detecting -> needs_review | tokenizing | blocked | failed | cancelled
needs_review -> tokenizing | blocked | failed | cancelled
tokenizing -> validating_export | blocked | failed | cancelled
validating_export -> publishing | blocked | failed | cancelled
publishing -> completed | blocked | failed
~~~

Only empty-review-queue execution can skip `needs_review`. A terminal job does
not resume; retry creates a new job. Exact reusable decisions and tokens retain
their existing contracts.

State responsibilities are validation and snapshot; extraction; deterministic
plus model detection; review; entity/token persistence and replacement;
staged-tree validation; atomic publication; then terminal audit and resource
release. The NER session is released after detection and before waiting for
review.

### Progress and menu bar

Progress reports stage and exact completed/total units, such as entries,
documents/pages, windows, review items, entities, or export documents. Counts
are monotonic inside a stage. There is no invented overall percentage or ETA.
Menu-bar progress contains no filename, path, entity, or sensitive value.

`MenuBarExtra` exposes current stage/count, `Show Outis`, `Cancel Job` when
allowed, and `Quit Outis`. `needs_review`, `blocked`, and `failed` show
`Attention required` and activate the window. It is not a separate process and
does not survive application exit.

### Review UI

Word and PDF review shows embedded `QLPreviewView`, complete scrollable
normalized text, page/mode evidence, and only `Confirm Extraction` or `Reject
Extraction`. Rejection blocks.

Candidate review shows private context, exact span, proposed class, detector,
applicable confidence, conflict, and boundary evidence, with Section 14
actions and no bulk confirmation. Confidence is labelled `Model evidence only
— not a correctness or safety score.` Complete normalized text can be opened
to add a missed candidate, but manual inspection is not represented as
mandatory or complete.

Repeated people, organizations, and addresses show every occurrence with
`Confirm Same Entity` or `Different Entities — Block This Demo`. Alias merging
appears only when Section 13 permits it.

The review queue is fixed after detection except for manually added candidates
and edit-created conflicts. The app leaves review only after extraction,
candidate, conflict, entity, and unsupported-sensitive checks close. It then
continues through tokenization and publication without a second button.

### Cancellation, close, quit, and sleep

Cancellation sets one atomic request; repeats have no extra effect. The UI
shows `Cancelling…` until a boundary acknowledges it. Review cancellation waits
for current transaction commit or rollback. Publication uses Section 20. No
fixed latency is claimed.

Closing the main window does not cancel an active job. Explicit quit exits when
idle; otherwise it offers `Cancel Job and Quit` or `Keep Running`. It waits for
bounded cleanup before exit, and defers termination during the publication
critical section. Forced termination or crash is not clean cancellation.

Outis has no background service and does not prevent system sleep. Suspension
pauses work, removes timeout claims, and requires source revalidation before
publication.

### Terminal presentation and retry

Completion displays:

~~~text
Completed under the synthetic funding-demo contract.
Detection can miss sensitive information.
Review the generated repository before granting access.
~~~

It offers `Open Outis Repository`, non-sensitive document and occurrence
counts, and evidence location when available. It never uses `safe`, `secure`,
`anonymous`, `perfectly detected`, or `ready for confidential data`.

`blocked` is a declared input, policy, review, or integrity condition; `failed`
is an unexpected platform or internal failure. The Human Zone window can show
a private relative path and typed code; menu-bar text, normal logs, and agent
output cannot.

There is no automatic retry. A new job follows user correction. Crash recovery
starts only after project, source, and export reselection.

## 22. Finder dispatch contract

Not in scope under Section 4. The first slice has no Finder extension, Finder
contextual action, or Finder dispatch route.

## 23. Sandbox, signing, entitlement, and Keychain contract

Keychain is explicitly absent from the synthetic funding-demo vault under
Section 19.

The first slice is a locally run, non-distributed, ad-hoc-signed application.
Its product name is `Outis`, bundle identifier is `com.outis.localpilot`,
marketing version is `0.1.0`, and build number is `1`. Changing the bundle
identifier requires a specification amendment. Distribution signing,
notarization, App Store packaging, and external distribution are out of scope
and require a future specification.

Hardened Runtime and App Sandbox are enabled. The application has exactly:

- `com.apple.security.app-sandbox = true`; and
- `com.apple.security.files.user-selected.read-write = true`.

Network client, network server, application group, Keychain access group,
automation, hardware, broad filesystem, temporary-exception, JIT, and
disable-library-validation entitlements are absent. No nested target inherits
entitlements implicitly.

The user selects the source and export-parent folders through the application.
Every successful security-scoped-resource start has a matched stop. Source
access is read-only by Outis policy even though the platform entitlement is
read-write; output writes are confined to the selected export parent. The
application persists no security-scoped bookmark. Access ends at a terminal
job state. Quick Look access occurs only while the applicable source scope is
active.

The application container owns only the private vault and evidence artifacts.
It does not own or copy the source corpus, normalized extraction or OCR cache,
or agent repository.

The ONNX Runtime dynamic library is signed before the enclosing app, and the
app is signed last. The Rust static library is linked into the executable and
is not a nested signing object. Validation checks the dynamic library and app
separately; signing must not use `--deep`. Runtime library validation remains
enabled.

## 24. Rust, Swift, FFI, IPC, crate, and target contract

### Apple toolchain and application targets

The selected build toolchain is full Xcode 26.6 with macOS SDK 26.5, Swift
compiler 6.3, and Swift 6 language mode. The deployment target is macOS 14 and
the only architecture is `arm64`. The active developer directory must be
`/Applications/Xcode.app/Contents/Developer`; Command Line Tools alone are not
an accepted build environment.

The Xcode project has exactly two targets:

- `Outis`, a macOS application target using SwiftUI and AppKit; and
- `OutisTests`, its test target.

There is no Finder extension, command-line executable, daemon, XPC service,
remote-service client, renderer target, framework target, or Swift Package
Manager package in the first slice.

The application may link only these Apple frameworks: SwiftUI, AppKit,
Foundation, PDFKit, Vision, CoreGraphics, QuickLookUI, and Security. A new
framework requires a specification amendment.

### Rust workspace and linkage

Rust is fixed at release 1.89.0, full commit
`29483883eed69d5fb4db01964cdf2af4d86e9cb2`, with Cargo 1.89.0, edition 2024,
and host and target `aarch64-apple-darwin`. The installed rustup locator is
`stable`; it is not a floating compiler acceptance rule. Every approved
Rust-family command sets `RUSTUP_TOOLCHAIN=stable` and first passes the exact
identity, component, and target preflight in Section 40. A different release,
commit, host, component set, or target blocks even if the `stable` alias
resolves successfully.

The repository commits `rust-toolchain.toml` and `Cargo.lock`; approved build
commands use `--locked`. Nightly Rust is absent. Release builds use
`panic = "unwind"` so the FFI boundary can contain Rust panics.

The complete-pilot Rust workspace contains exactly:

- `outis-core`, the dependency-light deterministic domain and policy layer;
- `outis-runtime`, the side-effecting extraction coordination, model, vault,
  and publication layer; and
- `outis-ffi`, the versioned language boundary.

The dependency direction is macOS app to generated C header to `outis-ffi` to
`outis-runtime` to `outis-core`. Reverse dependencies and feature-specific
micro-crates are absent.

`MI-01` is the staged exception defined in Section 40: its workspace contains
only `outis-core`. It creates no empty `outis-runtime` or `outis-ffi` member.
The exact final three-member statement applies after later approved complete
capabilities create and use those crates.

`outis-ffi` builds `liboutis_ffi.a` as a Rust `staticlib`. There is no C++
interop. The only non-system dynamic library is
`libonnxruntime.1.28.0.dylib`, loaded as
`@rpath/libonnxruntime.1.28.0.dylib` with
`@executable_path/../Frameworks` as the application runtime search path. Outis
must not load a dynamic library from an environment-controlled path, current
directory, source repository, or export directory.

The complete-pilot Xcode build invokes:

~~~text
env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo build --locked --offline --release \
  --package outis-ffi --target aarch64-apple-darwin
~~~

That build performs no network access or artifact download, declares its
inputs and outputs to Xcode, verifies that the generated header is current,
enforces the model legal-clearance gate, and copies artifacts only into the
approved application-bundle locations. `MI-01` does not create or compile a
Swift target, Xcode project, application bundle, runtime crate, FFI crate,
generated header, static library, or application build phase. It runs only the
exact `outis-core` checks in Section 40. Exact complete-pilot repository paths
and Xcode build-phase bindings remain in Section 37.

### Swift concurrency and failure surface

Swift strict concurrency checking is `complete`. UI state is isolated to
`MainActor`; one non-main actor serially owns the Rust engine and job handle.
Runtime, extraction, persistence, and FFI paths contain no force unwrap,
`try!`, unchecked cast, `fatalError`, or `preconditionFailure`. Resource and
scope cleanup is deterministic on success, failure, and cancellation.

Objective-C exceptions from imported platform frameworks are not declared
recoverable. If one escapes, the process may terminate; the next launch follows
the Section 21 recovery contract. This limitation is accepted only for the
synthetic first slice.

### ABI source, versions, and types

The ABI version and JSON wire-schema version are both unsigned integer `1`.
Rust `#[repr(C)]` declarations are the only ABI source. C is the
interoperability language. There is no C++, IPC, callback, re-entrant call, or
generated Swift surface. Every exported symbol begins with `outis_`.

The exact C surface is:

~~~c
typedef struct OutisEngine OutisEngine;
typedef struct OutisJob OutisJob;

typedef struct {
    const uint8_t *ptr;
    size_t len;
} OutisSlice;

typedef struct {
    uint8_t *ptr;
    size_t len;
} OutisBuffer;

typedef struct {
    uint32_t code;
    OutisBuffer error_json;
} OutisCallResult;

typedef struct {
    uint32_t kind;
    uint32_t private_kind;
    uint64_t sequence;
    OutisBuffer metadata_json;
    OutisBuffer private_payload;
} OutisEvent;

uint32_t outis_abi_version(void);

OutisCallResult outis_engine_create(
    OutisSlice resources_root_utf8,
    OutisSlice vault_root_utf8,
    OutisEngine **out_engine,
    OutisBuffer *out_project_catalog_json
);

OutisCallResult outis_engine_release(OutisEngine **engine);

OutisCallResult outis_job_start(
    OutisEngine *engine,
    OutisSlice request_json,
    OutisSlice source_root_utf8,
    OutisSlice export_parent_utf8,
    OutisJob **out_job
);

OutisCallResult outis_job_poll(
    OutisJob *job,
    OutisEvent *out_event
);

OutisCallResult outis_job_submit_extraction(
    OutisJob *job,
    uint64_t request_id,
    OutisSlice result_json,
    OutisSlice extracted_utf8
);

OutisCallResult outis_job_submit_review(
    OutisJob *job,
    uint64_t request_id,
    OutisSlice decision_json
);

OutisCallResult outis_job_cancel(OutisJob *job);
OutisCallResult outis_job_release(OutisJob **job);
uint32_t outis_buffer_release(OutisBuffer *buffer);
~~~

These ten functions are the complete version-one export surface.

### Buffer and handle ownership

An input slice is borrowed only for its call. Rust copies and validates any
input it retains before returning. Strings are pointer-and-length UTF-8, never
NUL-terminated strings. An empty buffer is exactly `{NULL, 0}`; a nonempty
buffer requires a non-null pointer. Output structures are zeroed before work
begins.

Rust owns every returned buffer until Swift releases it exactly once through
`outis_buffer_release`. Successful release resets the caller's structure to
`{NULL, 0}`; releasing that cleared structure again is a no-op. Copying one
owned buffer and releasing both copies violates the caller contract. No file,
SQLite, vault, model-runtime, Apple-framework, or security-scope handle crosses
the ABI.

Exact wire limits are:

| Surface | Maximum |
|---|---:|
| Absolute path input | 4 KiB |
| Start-request JSON | 4 KiB |
| Review-decision JSON | 64 KiB |
| Extraction-provenance JSON | 32 MiB |
| Extracted-document UTF-8 | 10 MiB |
| Event-metadata JSON | 4 MiB |
| Private event payload | 32 MiB |
| Error JSON | 4 KiB |
| Project catalog | 1 MiB and 1,024 projects |

Exceeding a limit returns `LIMIT_EXCEEDED`. No ABI surface truncates.

### Stable call-result codes

| Value | Name |
|---:|---|
| `0` | `OK` |
| `1` | `NO_EVENT` |
| `2` | `NULL_ARGUMENT` |
| `3` | `INVALID_BUFFER` |
| `4` | `INVALID_UTF8` |
| `5` | `INVALID_JSON` |
| `6` | `SCHEMA_MISMATCH` |
| `7` | `LIMIT_EXCEEDED` |
| `8` | `INVALID_STATE` |
| `9` | `STALE_REQUEST` |
| `10` | `DUPLICATE_RESPONSE` |
| `11` | `BUSY` |
| `12` | `ABI_MISMATCH` |
| `13` | `PANIC_CONTAINED` |
| `14` | `INTERNAL_FAILURE` |

These codes describe ABI calls, not job-domain outcomes. Error JSON uses
exactly `schema_version`, `code`, and `operation`, as in:

~~~json
{"schema_version":1,"code":"ffi.invalid_utf8","operation":"outis_job_start"}
~~~

It contains no path, document text, hash, span, entity, alias, mapping, or
model output.

### Concurrency, polling, and lifecycle

One Swift actor owns one engine and at most one job. It serializes every ABI
call except `outis_job_cancel`. Rust runs one job thread. Swift polls every 100
ms. Progress events may be coalesced; extraction, review, surface-view, and
terminal events are never dropped.

`outis_job_release` accepts only a terminal job and otherwise returns
`INVALID_STATE`. `outis_engine_release` returns `BUSY` while a job exists.
Cancellation is idempotent and returns immediately. Every exported Rust
function contains a panic boundary; no Rust panic or unwind crosses the ABI.

### Start and project wire schemas

The start request is exactly one of:

~~~json
{"schema_version":1,"project_mode":"new","synthetic_confirmation":true}
~~~

~~~json
{"schema_version":1,"project_mode":"existing","repository_id":"<32 lowercase hex>","synthetic_confirmation":true}
~~~

The project catalog contains only opaque repository identifiers and creation
timestamps, sorted by raw repository-identifier bytes. It contains no source
or export path.

### Poll events and private payloads

Event kinds are numbered in this order:

1. `progress`;
2. `extraction_request`;
3. `extraction_review`;
4. `candidate_review`;
5. `entity_review`;
6. `surface_view`; and
7. `terminal`.

Private-payload kinds are numbered in this order:

0. none;
1. source-relative path;
2. normalized document or path surface;
3. bounded review context; and
4. private entity-context JSON.

Every event has a strictly increasing job-local `sequence`. Event metadata
contains the schema version, event kind, sequence, and only the typed IDs,
stage, counts, adapter, class, detector, confidence in integer parts per
million when applicable, conflict code, byte offsets, allowed actions, status,
and non-plaintext terminal code applicable to that event. It contains no
plaintext document content or sensitive path. FFI metadata remains local and
is prohibited from normal logs and the agent repository.

An extraction request carries request ID, document ID, selected adapter
`appkit_word` or `pdfkit_vision`, and source class in metadata. Its validated
source-relative path is the separate private payload.

### Native extraction submission

Swift submits the request ID, typed status and provenance JSON, and extracted
text as separate buffers. Successful provenance contains ordered pages and
the Section 9 `NormalizedDocumentV1` segments. Bounding-box coordinates and
Vision confidence use integers from zero through 1,000,000, calculated by
rounding the bounded value multiplied by 1,000,000 to nearest with ties away
from zero. Floating-point values do not enter JSON.

Rust validates complete segment coverage, UTF-8 scalar boundaries, page and
observation order, normalization, adapter and source-class identity, status,
and size limits before accepting the submission. A `blocked`, `failed`, or
`cancelled` result has an empty extracted-text buffer, empty page and segment
arrays, and a mandatory typed code.

### Review wire behavior

Review submissions contain IDs, actions, classes, and byte offsets only. They
never copy sensitive text into decision JSON. The allowed action vocabulary is
exactly:

- `confirm_extraction`;
- `reject_extraction`;
- `confirm`;
- `adjust`;
- `reclassify`;
- `exclude_false_positive`;
- `add_missed_candidate`;
- `mark_unsupported_sensitive`;
- `confirm_same_entity`;
- `different_entities_block`; and
- `request_full_surface`.

`request_full_surface` and `add_missed_candidate` do not consume the pending
request. `adjust` and `reclassify` consume it and cause Rust to issue a new
request identifier. Final decisions consume it. Default candidate context
contains the complete span and up to 1,024 UTF-8 bytes on each side, aligned to
Unicode-scalar boundaries. Complete document or path text crosses only after
`request_full_surface`.

### Duplicate, stale, and cancellation races

Extraction and review request identifiers increase monotonically within the
job. An accepted response cannot be accepted again: an already consumed ID
returns `DUPLICATE_RESPONSE`; an unknown or cancellation-invalidated ID returns
`STALE_REQUEST`.

Cancellation and submission linearize against the same request state. If
cancellation wins, submission returns `STALE_REQUEST`. If submission wins,
Rust completes copying and validation, then observes cancellation before
further processing. Accepted plaintext made unnecessary by cancellation is
discarded from job memory.

## 25. Dependency contract

### Exact production dependencies and ownership

All direct production versions use exact Cargo `=` requirements. The complete
approved set is:

| Dependency | Owner | Features |
|---|---|---|
| `serde` `1.0.229` | core, runtime, FFI | defaults off; `derive`, `std` |
| `serde_json` `1.0.151` | runtime, FFI | defaults off; `std` |
| `unicode-normalization` `0.1.25` | core | defaults off; `std` |
| `caseless` `0.2.2` | core | defaults off; no features |
| `sha2` `0.11.0` | core, runtime | defaults off |
| `libc` `0.2.183` | runtime publication | defaults off |
| `rusqlite` `0.40.2` | runtime vault | defaults off; `bundled` |
| `ort` `2.0.0-rc.13` | runtime model | defaults off; `std`, `api-28`, `load-dynamic` |
| `tokenizers` `0.23.1` | runtime model | defaults off; `fancy-regex` |

No other direct production dependency is permitted. `libsqlite3-sys` 0.38.2
and bundled SQLite 3.53.2 are accepted transitively through `rusqlite`.

The table is the final complete-pilot set. Each complete implementation
increment adds only the direct dependencies that its bound behavior uses.
`MI-01` uses the Rust standard library and adds no registry dependency; its
format-4 `Cargo.lock` therefore contains only the `outis-core` package. Later
approved increments may add only their used entries from the final table.
After legal clearance, the model-specific plan adds `ort` and `tokenizers` and
regenerates the lock under the commands and ceilings below. This is a staged
repository transition, not a runtime option: no Cargo feature, alternate
manifest, fallback dependency, or model-free release configuration is created.

`serde` and `serde_json` own versioned wire, manifest, evidence, and private
provenance serialization. `unicode-normalization` and `caseless` own the
declared normalization and collision-equality operations. `sha2` owns approved
source, artifact, manifest, tree, and replay identities. `libc` owns the
approved low-level publication and filesystem calls. `rusqlite`, `ort`, and
`tokenizers` own only the vault, model-runtime, and publisher-tokenizer
adapters respectively.

### Unicode identity correction

The Section 18 collision key is explicitly NFC using
`unicode-normalization` 0.1.25 with Unicode 17 data, followed by full
non-locale default case folding using `caseless` 0.2.2 with Unicode 16 data,
followed by UTF-8 byte comparison. The version mismatch is explicit and part
of the pipeline identity.

`unicode-casefold` 0.2.0 was rejected because it contains Unicode 9 data.
ICU4X was rejected for the first slice because its provider and data surface
are materially larger. Handwritten case-fold tables were rejected because
they would create a separately generated and maintained Unicode-data system.

### Model and tokenizer feature exclusions

`ort` excludes binary download, model fetching, HTTP, TLS, Core ML, CUDA,
other GPU providers, training, telemetry, tracing, automatic dylib discovery,
and copied or statically downloaded runtime binaries. Only the Section 12
explicit bundle path and CPU provider are allowed.

`tokenizers` excludes HTTP and Hugging Face Hub, progress output, Oniguruma,
the `esaxx_fast` C++ feature, training, and tokenizer mutation. Outis calls
`tokenizers::utils::parallelism::set_parallelism(false)` before tokenizer
construction and verifies the disabled state. Tokenization is serial. Only the
approved ONNX Runtime inference uses two worker threads.

### Explicitly absent direct dependencies

The first slice adds no direct `regex`, `thiserror`, `anyhow`, asynchronous
runtime, `rayon`, UUID, random, time-formatting, temporary-file, directory-walk,
Keychain, security-framework, cryptographic-vault, or Swift Package Manager
dependency. Structured detectors use bounded deterministic scanners;
concurrency and integer Unix time use standard-library surfaces.

Repository and job identifiers use the already-approved Security framework
`SecRandomCopyBytes` through one bounded runtime declaration. A custom ONNX
wrapper, custom BERT tokenizer, system SQLite, Swift-owned hashing, and
handwritten JSON were rejected because they would add unmeasured correctness
or platform-version surfaces.

### Registry source identities

| Crate | Registry SHA-256 |
|---|---|
| `caseless` `0.2.2` | `8b6fd507454086c8edfd769ca6ada439193cdb209c7681712ef6275cccbfe5d8` |
| `libc` `0.2.183` | `b5b646652bf6661599e1da8901b3b9522896f01e736bad5f723fe7a3a27f899d` |
| `ort` `2.0.0-rc.13` | `4336a1e2b38848325241c72889086886004e589b7c74f335e60a8e8db5138a0b` |
| `rusqlite` `0.40.2` | `23f2a97da3e3873c73cb2a2e71b35c40ff95e0b1eefa8d72d8499a6928c3b5b3` |
| `serde` `1.0.229` | `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba` |
| `serde_json` `1.0.151` | `c841b55ecdae098c80dcae9cf767f6f8a0c2cdb3416bbef72181df4d0fe73f14` |
| `sha2` `0.11.0` | `446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4` |
| `tokenizers` `0.23.1` | `44e5bea67576e04b6ff8564c5d9e09c2ef0cf476502245f2f120e497769d3112` |
| `unicode-normalization` `0.1.25` | `5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8f` |

### Development tools

The separate, non-application tools are `cbindgen` 0.29.4,
`cargo-audit` 0.22.2, and `cargo-deny` 0.20.2. They are installed with their
exact version and `--locked`; none is a workspace dependency or linked into
Outis.

### Lock, source, feature, and compile-surface policy

The repository commits Cargo lockfile format version four. Only crates.io
registry packages and the three local workspace crates are allowed. Git
dependencies, unapproved path dependencies, patches, alternate registries,
and build-time or runtime downloads are forbidden. Product builds use the
committed lock and Cargo offline mode. Vendoring is deferred because it would
duplicate a large source tree.

A dependency-resolution probe using Rust 1.89 on 2026-08-17 observed 114
registry packages, 25 packages with build scripts, and only `syn` duplicated
across major versions two and three. These are maxima. It observed no enabled
HTTP, runtime-download, GPU, Core ML, SQL extension-loading, SQLCipher, or
tokenizer C++ feature. A mismatch blocks and requires review.

Updating a direct dependency requires a specification amendment. Updating the
transitive lock requires new source, license, advisory, feature,
compile-surface, and deterministic-replay evidence.

### Advisory and license policy

`cargo-audit` 0.22.2 found zero known vulnerabilities in the disposable
resolution on 2026-08-17. It reported only `RUSTSEC-2024-0436`, because
transitive `tokenizers -> paste 1.0.15` is unmaintained. This one informational
warning is accepted because `paste` is a compile-time procedural macro, no
vulnerability is reported, and replacement requires patching or forking the
selected tokenizer. No other advisory or warning is ignored.

Permitted resolved licenses are Apache-2.0, Apache-2.0 with LLVM exception,
BSD-2-Clause, BSL-1.0, ISC, MIT, Unicode-3.0, Unlicense, and Zlib. MPL-2.0 is
permitted only for the non-shipped cbindgen tool. An `OR` expression is
acceptable when at least one approved branch applies. GPL, AGPL, SSPL,
noncommercial, unknown-source, or unlicensed packages block.

The complete application contains
`apps/macos/Outis/Resources/THIRD_PARTY_NOTICES.txt` covering distributed Rust,
SQLite, tokenizer, and ONNX Runtime components. The model's AFL-3.0,
base-model, corpus, notice, and redistribution clearance remains the separate
Section 2 legal stop gate.

### Dependency validation

The exact required commands are:

~~~text
env RUSTUP_TOOLCHAIN=stable cargo metadata --locked --offline --format-version 1
env RUSTUP_TOOLCHAIN=stable cargo tree --locked -e features
env RUSTUP_TOOLCHAIN=stable cargo tree --locked -d
env RUSTUP_TOOLCHAIN=stable cargo deny check bans licenses sources
env RUSTUP_TOOLCHAIN=stable cargo audit --deny warnings --ignore RUSTSEC-2024-0436
env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo build --locked --offline --release \
  --package outis-ffi --target aarch64-apple-darwin
~~~

Any source, checksum, feature, license, advisory, package-count, build-script,
duplicate, or network mismatch blocks. This approved contract still does not
authorize a Cargo manifest or lockfile change.

## 26. Codegen, generated-binding, and generated-artifact contract

Rust `#[repr(C)]` declarations under `crates/outis-ffi/src/` are the source for
the version-one C header. `crates/outis-ffi/cbindgen.toml` is the generator
configuration. Exactly `cbindgen` 0.29.4 writes
`generated/ffi/outis.h`. The header is a committed production input, is never
edited manually, exposes at most the ten Section 24 functions, and is limited
to 300 logical lines.

`apps/macos/Outis/Engine/module.modulemap` is a small reviewed handwritten
module map, not generated Swift. There is no other generated binding.

The explicit generation command, run from the repository root, is:

~~~text
env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cbindgen crates/outis-ffi \
  --config crates/outis-ffi/cbindgen.toml \
  --output generated/ffi/outis.h
~~~

The byte-clean verification command is:

~~~text
env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cbindgen crates/outis-ffi \
  --config crates/outis-ffi/cbindgen.toml \
  --output generated/ffi/outis.h \
  --verify
~~~

`CARGO_NET_OFFLINE` is fixed to `true` for these build operations and is not a
runtime or operator configuration knob. Xcode runs verification and never
regenerates the header. Version mismatch, generation failure, changed bytes,
unexpected symbols, or line-budget excess blocks the build.

Validation covers header byte equality, the symbol allowlist, C and
Swift imports, arm64 size and alignment, null and empty buffers, invalid UTF-8
and JSON, limits, ownership and releases, handle lifecycle, stale and duplicate
submissions, concurrent cancellation, panic containment, plaintext exclusion,
and absence of callbacks and unexpected exports. Sections 36 and 37 bind the
exact test files and commands. This contract does not authorize generation.

## 27. Conditional future-service contract

Not in scope under Section 4. The first slice has no Agent Service, remote Key
Service, remote detector, Swiss-hosted verification, RAG, embeddings, chat,
conversation memory, model-provider route, response rendering, or remote
storage. `architecture.md` is the sole location for the deferred Swiss
verification direction.

## 28. Determinism contract

Extraction replay is approved in Section 9, structured discovery in Section
11, model replay in Section 12, entity ordering in Section 13, token allocation
and replacement in Section 15, repository serialization in Section 18, vault
state in Section 19, and publication in Section 20. JSON serialization uses
declared Rust structure order, rejects unknown fields, and emits the exact
encoding owned by the applicable schema. SQLite row order is never implicit;
every replay query has an explicit approved key order.

The Section 33 complete-tree oracle and Section 36 acceptance command close
whole-pipeline replay. Three runs require byte-identical ordered candidates,
entities, tokens, Markdown files, manifest, tree hash, and export identifier
under identical complete identity and valid vault state. Audit timestamps,
private job identifiers, and private evidence-directory names are explicitly
outside agent-repository equality. A changed bound input invalidates the replay
comparison rather than permitting a tolerance.

## 29. Failure, cancellation, retry, and recovery contract

### Terminal outcomes and precedence

The only terminal outcomes are:

- `completed`: publication committed, the final tree passed validation, and no
  earlier terminal condition won;
- `cancelled`: a cancellation request was acknowledged before a
  non-preemptible operation committed a different result;
- `blocked`: a declared input, policy, review, integrity, capacity, or contract
  condition prevents safe continuation; and
- `failed`: an unexpected platform, I/O, runtime, or internal operation failed.

Before every bounded operation, the worker checks the single atomic
cancellation request. If it is set, cancellation wins. Once a declared
non-preemptible transaction, model call, extraction call, synchronization, or
publication operation starts, that operation's result wins over a later
cancellation request. A declared contract violation becomes `blocked`; an
unexpected operational failure becomes `failed`. Publication commit followed
by successful final-tree validation becomes `completed`. A later cleanup or
audit warning does not reclassify it. A job records one terminal outcome
exactly once.

### Stable blocked domain codes

The complete version-one `blocked` taxonomy is:

~~~text
authorization_missing
source_selection_invalid
export_selection_invalid
zone_separation_violation
source_entry_unsupported
source_entry_unsafe
source_unreadable
source_limit_exceeded
source_path_invalid
source_changed
format_signature_mismatch

document_unsupported
document_corrupt
document_encrypted
document_feature_unsupported
extraction_incomplete
extraction_limit_exceeded
ocr_language_unavailable
ocr_geometry_invalid
extraction_rejected

model_artifact_invalid
model_runtime_incompatible
model_invalid_bio
model_output_invalid
review_unresolved
review_rejected
unsupported_sensitive_content
entity_conflict
entity_operation_unsupported

reserved_token_namespace
token_space_exhausted
token_collision
token_operation_unsupported
wrong_repository_source
vault_path_invalid
vault_busy
vault_corrupt
vault_schema_mismatch
vault_limit_exceeded

export_path_collision
export_target_invalid
export_manifest_invalid
known_plaintext_detected
publication_capability_unavailable
prior_export_invalid
recovery_state_ambiguous
~~~

Existing owning-section meanings remain authoritative. No implementation may
rename, merge, or add a domain code without a specification amendment.

### Stable failed domain codes and warnings

The complete version-one `failed` taxonomy is:

~~~text
extraction_platform_failed
model_inference_failed
vault_io_failed
vault_transaction_failed
publication_io_failed
randomness_failed
ffi_internal_failed
internal_invariant_failed
process_interrupted
~~~

A Rust panic contained at the FFI boundary records `ffi_internal_failed`; no
panic payload crosses the boundary or enters persistence. The only
nonterminal warnings are `staging_cleanup_failed`,
`old_export_cleanup_failed`, and `post_publication_audit_failed`. A warning is
private and cannot turn an incomplete publication into `completed`.

### Private error schema

`DomainErrorV1` contains exactly:

~~~text
schema_version
job_id
stage
terminal_outcome
domain_code
retry_class
private_subject_kind
private_subject_id
platform_domain
platform_numeric_code
~~~

`retry_class` is exactly one of:

~~~text
new_job_after_user_action
new_job_after_relaunch
project_deletion_only
not_retryable_in_first_slice
~~~

The schema contains no raw error message, SQL, document text, path, filename,
hash, entity, alias, mapping, model output, or panic content. While a valid
security-scoped source or export selection remains active, the Human Zone
window may separately display a validated private relative path. Error JSON,
menu-bar status, the agent repository, and normal runtime output may not.

### Retry and crash recovery

There is no automatic retry, fallback extractor, fallback model, alternate
vault, copy publication, or other hidden fallback. User correction or relaunch
starts a new job. Only decisions whose complete Section 14 identity still
matches may be reused. Committed token allocations remain persistent.

After abnormal termination, SQLite performs only its configured rollback-
journal recovery. Outis then validates the vault and permits at most one prior
nonterminal job. That job is closed as `failed` with `process_interrupted`.
The user must reselect the source and export folders, and all authorization,
identity, snapshot, and separation checks run again. Section 20 staging
recovery then classifies and cleans only a recognized state; it never
automatically publishes crash-left staging. Any conflicting prior job,
unrecognized artifact, or ambiguous filesystem state blocks as
`recovery_state_ambiguous` without deletion.

## 30. Retention, deletion, backup, and audit contract

### Runtime evidence store

The private vault is the only Outis runtime audit store. The first slice writes
no separate runtime log, diagnostic file, analytics record, or evidence file.
`AuditEventV1` contains exactly:

~~~text
sequence
job_id
unix_time_milliseconds
event_code
stage
terminal_outcome
object_kind
object_id
count_a
count_b
domain_code
platform_domain
platform_numeric_code
~~~

Its `event_code` is exactly one of:

~~~text
project_created
job_created
authorization_confirmed
validation_completed
extraction_completed
detection_completed
review_requested
review_decision_recorded
entity_resolution_completed
token_commit_completed
export_validation_completed
publication_started
publication_committed
cancellation_requested
cancellation_acknowledged
recovery_started
recovery_resolved
terminal_recorded
project_deletion_requested
~~~

Audit events contain no plaintext, path, filename, hash, entity value, alias,
mapping, model output, SQL, or raw platform message. Sequences are strictly
monotonic within one vault. A state change and its audit event commit in the
same SQLite transaction. Counts and private object identifiers summarize
bounded operations instead of emitting span- or token-level event streams;
review details remain in `review_decisions` and related private tables.

Publication filesystem commit necessarily precedes its corresponding vault
audit transaction. If the final tree is committed and validates but its final
audit write fails, the recovered outcome remains `completed` with
`post_publication_audit_failed`; recovery recognizes the export and attempts
no republish. Release builds emit no Outis-initiated normal runtime log. This
contract does not claim control over Apple or operating-system diagnostics,
crash reports, swap, snapshots, or third-party backup behavior.

### Vault and agent-repository retention

The vault is retained until explicit project deletion. Version one has no
expiry, compaction, selective mapping or token deletion, backup, export,
restore, migration, or synchronization feature. Reaching a bound blocks; it
does not evict data. No mapping is removed automatically. Removing the Outis
application is not claimed to remove its sandbox container.

The published agent repository is user-controlled after publication. Outis
replaces it only after the Section 20 validation and atomic-publication
contract succeeds. The first slice has no delete-export action. Deleting a
vault does not delete the source or agent repository. Tokens in retained
Markdown then cannot be rendered through Outis, but the Markdown remains.
The user may remove an agent repository manually in Finder.

### Staging and memory retention boundary

Outis removes only a staging tree recognized under Section 20. It preserves
ambiguous state and blocks. Extracted plaintext, OCR images, model inputs,
logits, and tokenized document buffers exist only in process memory for the
bounded stage that owns them; Outis creates no plaintext cache or temporary
file. This is not evidence of memory zeroization or exclusion from swap, crash
capture, or operating-system diagnostics.

### Explicit project deletion

Project deletion is allowed only with no active job and follows this exact
procedure:

1. The user selects the exact opaque project and confirms permanent loss of
   its token mappings.
2. Outis changes `vault_meta` from `active` to `deletion_pending`, appends
   `project_deletion_requested` in the same transaction, commits, and
   synchronizes the database and vault directory.
3. Outis closes SQLite, validates the repository directory identity, modes,
   ownership, and complete entry allowlist, and removes recognized rollback
   sidecars.
4. Outis removes the database last, synchronizes the vault directory, removes
   the now-empty repository directory, and synchronizes its parent.

At relaunch, a present database with `deletion_pending` exposes only `Finish
Project Deletion`. An absent database in an otherwise empty recognized project
directory causes that directory to be removed. An unknown entry, identity
mismatch, nonempty unrecognized directory, or ambiguous state blocks without
deletion. No audit survives successful deletion; the Human Zone UI reports
that local Outis project deletion completed. Outis makes no independent-audit
or physical-erasure claim.

### Backup boundary

Outis provides no backup feature and makes no deliberate vault copy. It does
not claim exclusion from Time Machine, APFS or other snapshots, third-party
backup, deleted-file recovery, the same macOS user, root, malware, crash
capture, or swap. Source and agent repositories may be backed up by the user's
environment. The synthetic-only warning states this limitation explicitly.

## 31. Compile-surface and application-size budget

The complete signed-application and model-payload limits are approved in
Section 12. The production surface is limited to one app target, one app-test
target, three Rust crates, the listed Apple frameworks, one Rust static
library, one ONNX Runtime dynamic library, one generated C header of at most
300 logical lines, and ten exported C functions. The dependency graph is
bounded by Section 25 at 114 registry packages and 25 build-script packages.

Production files have one semantic responsibility. The review thresholds are
300 logical lines for an ordinary Rust or Swift production file, 200 for an
FFI or security-boundary file, 220 for a UI view, and 400 for a test file.
Crossing a threshold requires responsibility review. Artificial compression,
generic modules, or splitting a cohesive responsibility merely to meet a
number is forbidden. A production file or abstraction without a specific
specification responsibility is forbidden.

The build acceptance budgets are:

| Measurement | Budget |
|---|---:|
| Clean Rust release static library | at most 120 seconds |
| Clean complete signed application | at most 180 seconds |
| No-change application rebuild | at most 15 seconds |
| One Rust production-file incremental application rebuild | at most 45 seconds |
| One Swift production-file incremental build | at most 20 seconds |
| Peak build resident memory | at most 6 GB |
| Cargo target plus Xcode DerivedData after the clean build | at most 4 GB |

A clean build removes product target output and project DerivedData but retains
the verified local Cargo registry and model-artifact caches. It starts with
all approved inputs already present, runs offline, and includes the Rust
static-library build, generated-header verification, Swift build, linkage,
resource copy, and ad-hoc signing applicable to the measured application.
Initial dependency or model acquisition is not build time.

Each time measurement uses three runs and gates on the median while reporting
every result and failure. The build profile, command, input identity, cache
state, wall/user/system time, peak memory, output size, package graph, target
set, and dirty state are evidence. Section 35 must bind exact commands and
instrumentation before these budgets can pass.

## 32. Runtime performance and resource budget

### Claim and reference boundary

These values are unproved acceptance gates, not current performance claims.
Acceptance applies only to a release, ad-hoc-signed, sandboxed application
without a debugger on the following reference environment:

- Mac model `Mac16,7`, Apple M4 Pro, arm64;
- 25,769,803,776 bytes physical memory;
- macOS 26.5 build `25F71`;
- full Xcode 26.6 and macOS SDK 26.5;
- Rust 1.89 release build; and
- local APFS source, vault, staging, and export locations.

Full Xcode is verified, but no Outis application is implemented, so no
complete-application budget currently has acceptance evidence. Passing on this
host establishes no performance claim for another Mac, operating-system build,
architecture, storage device, or dataset.

### `PERF-SMOKE-V1`

The performance corpus contains exactly 18 synthetic source documents: one
`.txt`, `.md`, `.doc`, `.docx`, text PDF, and scanned PDF for each of Italian,
German, and French. Each Word or PDF source has one page, giving three OCR
pages. Every active sensitive class occurs in each language. The complete
corpus is at most 10 MB of source data, 1 MB of normalized Markdown, and
12,000 model content tokens across document and path surfaces.

The corpus has an initial-publication case using a fresh valid vault and absent
destination and an intact-replacement case using a valid prior Outis export.
Section 35 must bind the exact source bytes, hashes, counts, annotations,
review decisions, vault state, and prior export before a result can pass.

Active machine time begins when `Anonymize with Outis` is accepted and ends at
the terminal result, excluding only time persisted in `needs_review` while
waiting for the user. Folder selection and human review dwell are not runtime.
Engine processing of submitted review decisions remains included.

### Latency and responsiveness

| Measurement | Budget |
|---|---:|
| Cold application launch to usable window | p95 at most 2 seconds |
| Start action to first progress event | p95 at most 250 ms |
| Complete `PERF-SMOKE-V1` active machine time | p95 at most 10 seconds |
| Engine event to visible window or menu-bar update | p95 at most 250 ms |
| Warm non-OCR extraction per document | p95 at most 100 ms |
| Warm selected 200-DPI OCR per page | p95 at most 250 ms |
| Model-session load | at most 2 seconds |
| Warm 510-token model inference | p95 at most 125 ms |
| Runtime network requests | exactly zero |

Each runtime p95 uses nearest-rank over at least 20 measured runs of the exact
subject, and the report includes every result, the maximum, every failure, and
cold or warm state. Initial and replacement publication are reported
separately. Section 8 maximum input limits remain guardrails rather than speed
claims: maximum-bound workloads must be measured and reported but do not
inherit the ten-second corpus budget.

At a cancellable boundary, worker acknowledgement has a p95 budget of 250 ms.
There is no wall-time ceiling while a declared non-preemptible AppKit, PDFKit,
Vision, SQLite, synchronization, model-window, or atomic-publication operation
is active. The UI must present `Cancelling` within 100 ms of accepting the user
action.

### CPU and concurrency

There is one active job, one Rust job worker, one extraction operation, one OCR
page, and one model inference at a time. ONNX Runtime uses two intra-operation
threads and one inter-operation thread only while the model session exists.
Mean model-inference CPU time divided by wall time is at most `2.25` on the
reference workload. The model session is released at terminal state.

After 30 seconds without a job, Outis-initiated CPU use averages at most one
percent over the following 60 seconds. Apple frameworks may schedule internal
workers; the two-thread ONNX setting is not a hard two-core limit for AppKit,
PDFKit, Vision, the operating system, or the complete process.

### Memory

| Measurement | Budget |
|---|---:|
| Peak model-stage Outis-process RSS | at most 2,000,000,000 bytes |
| Peak complete Outis-process RSS | at most 2,500,000,000 bytes |
| Outis-process RSS 30 seconds after terminal cleanup | at most 750,000,000 bytes |

Attributable Apple helper-process memory is recorded separately. No helper-
process threshold or complete-system memory claim is made because those
services can be shared and operating-system managed.

### Storage and bundle size

Bundle logical size is the sum of `st_size` for every entry without following
links. Section 12 remains authoritative for the 760,000,000-byte combined
model, tokenizer, and runtime limit. The complete signed application is at
most 850,000,000 logical bytes. All remaining application content combined is
at most 90,000,000 logical bytes.

The production bundle contains no debug symbols, tests, fixtures, benchmarks,
evaluation data, unused architecture, or model-acquisition artifact. The
agent repository remains limited to 128 MiB. Initial publication has one
candidate tree; replacement has at most two 128-MiB agent trees at any instant.
Section 19 remains authoritative for the 256-MiB database and 512-MiB complete
vault-directory limits. Outis creates no persistent runtime store outside the
approved vault and agent-repository locations.

### Measurement and failure rule

Correctness, extraction, detection, privacy, deterministic replay,
publication, and recovery checks pass before a performance result can support
a claim. Evidence separates enumeration, extraction, deterministic discovery,
model load and inference, entity resolution, vault access, tokenization,
serialization, validation, publication, and cleanup. It records wall, user,
system, peak memory, CPU ratio, storage, output size, cold/warm state, thermal
state, and failures against the identical logical payload.

Energy use is recorded, but version one has no energy threshold or efficiency
claim because no complete application has been measured. A missed budget
blocks acceptance. It cannot be silently relaxed or hidden; changing a budget,
workload, or measurement requires evidence and specification amendment. No
benchmark-only runtime mode or user performance knob is permitted.

## 33. Correctness, extraction, detection, and entity-resolution oracle

### Oracle record and evidence boundary

All acceptance data is synthetic and repository-owned. Comparisons are byte-
exact unless explicitly identified as model-quality measurements. Passing an
oracle proves behavior only for the pinned artifacts, fixtures, configuration,
Mac, and macOS build.

`OracleCaseV1` contains exactly:

~~~text
schema_version
case_id
corpus_id
corpus_sha256
source_sha256
pipeline_identity_sha256
language
source_format
expected_status
expected_domain_code
normalized_utf8_sha256
normalized_utf8_artifact
expected_provenance
expected_candidates
expected_review_decisions
expected_entities
expected_aliases
expected_occurrences
expected_tokens
expected_output_path
expected_output_bytes
expected_output_sha256
expected_manifest_values
~~~

An inapplicable field is an empty array, empty object, or empty string as fixed
by the Section 38 schema; fields are never omitted. Oracle files may contain
synthetic plaintext. They are Human Zone test inputs and are forbidden from
production bundles and generated agent repositories.

### Source and extraction oracle

The extraction corpus covers the 18 `PERF-SMOKE-V1` documents and at least one
contract case for each of:

- strict `.txt` and `.md`, with and without one leading UTF-8 BOM;
- `.doc` and `.docx` visible-text import;
- text-bearing, scanned, and multipage PDFs;
- exact two-LF PDF page joining;
- invalid UTF-8, NUL, corruption, and truncation;
- encrypted or locked PDF and extension/signature mismatch;
- Word attachment or lossy-conversion evidence;
- PDF rotation, annotation, form, attachment, and embedded-content evidence;
- OCR language unavailability, invalid geometry, empty observations, and
  resource-limit failure.

A successful case matches exact normalized UTF-8 bytes, page order, segment
ranges, extraction mode, adapter identity, provenance, review status, and
hashes. A rejected case produces the exact terminal class and domain code with
no accepted partial extraction. OCR equality applies only to the pinned Vision
revision, configuration, architecture, and macOS build. A changed OS build
requires a new approved regression result before acceptance.

### Deterministic-discovery oracle

The table-driven corpus covers every approved address and matter cue; valid
and invalid CH, DE, FR, and IT IBANs; every accepted telephone country code;
accepted and plausible-invalid email forms; and Unicode, punctuation, line,
path-component, scalar-boundary, overlap, and reserved-token behavior.

Every expected deterministic candidate matches its exact surface, UTF-8 byte
range, observed bytes, class, equality key, evidence, status, and order. For
the declared deterministic grammar corpus, accepted-subset precision and
recall are each exactly `1.0`. Every plausible-invalid value becomes the
expected `needs_review`. Three replays produce byte-identical ordered
candidates. This establishes only the declared grammars.

### Contextual-model regression oracle

The pinned 30-case Italian, German, and French corpus reproduces the approved
candidate list and these integer results:

| Language | Class | Gold | Predicted | Exact matches |
|---|---|---:|---:|---:|
| Italian | Person | 6 | 6 | 6 |
| Italian | Organization | 6 | 7 | 6 |
| German | Person | 6 | 7 | 5 |
| German | Organization | 6 | 7 | 6 |
| French | Person | 6 | 6 | 6 |
| French | Organization | 6 | 5 | 5 |

Person overlap recall remains `18/18`; organization overlap recall remains
`17/18`. Each language's three postal addresses receives overlapping `LOC`
evidence, while the model produces zero complete postal-address spans. The
known false positives, split German person, and missed French organization
remain explicit evidence and are not hidden or relabelled. Known missed spans
enter the final-span oracle only through prescribed `add_missed_candidate`
decisions.

Under the complete Section 12 model identity, the 16, 128, 256, and 510-token
raw-logit replay inputs match the four approved SHA-256 values in
`outis_local_pilot_ner_evaluation.md`. Any candidate, count, metric, or replay-
hash regression blocks. These values are a small synthetic regression gate,
not a general model-quality or perfect-discovery claim.

### Review, entity, and token oracle

After the prescribed synthetic decisions:

- final spans exactly equal the annotated final-span set;
- no unresolved or overlapping span remains;
- every accepted span maps to exactly one same-class entity;
- repeated exact structured keys link automatically;
- repeated exact person, organization, and address keys use the prescribed
  `confirm_same_entity` decision;
- different keys remain separate unless the prescribed same-class merge is
  valid;
- canonical aliases follow earliest resolved-candidate order;
- cross-class merges and merges of two tokenized entities block;
- entity, alias, occurrence, and token rows match the oracle exactly;
- allocation follows per-class counters and final-candidate order;
- retry retains committed tokens and permitted sequence gaps;
- content and path occurrences of one entity use the same token; and
- reserved or malformed Outis-like source tokens block.

### Complete-tree and replay oracle

Every successful source has exactly one expected `.md` output. The complete
agent tree, Markdown bytes, manifest bytes, paths, modes, hashes, occurrence
counts, document-tree hash, and export identifier match the expected
repository byte-for-byte.

Three same-identity replays using the same valid vault state produce identical
agent repositories. Private audit timestamps may differ and are outside the
agent-repository replay identity. Cancellation, source mutation, validation
failure, corrupt prior output, and publication failure preserve the last valid
export byte-for-byte.

## 34. Privacy, vault-isolation, and agent-boundary oracle

### Known-plaintext set and scan

`KnownPlaintextSetV1` contains every annotated sensitive document and path
value, alias, canonical value, equality key, manually added value, private
repository identifier, absolute source and vault path, private path mapping,
vault filename, SQLite header, and synthetic unsupported-sensitive sentinel.

The oracle scans every exported path and byte, manifest field, staging tree,
Outis-created runtime file, captured stdout and stderr, and Outis-initiated log
capture. Passing requires:

- zero forbidden-value matches;
- zero source binary, vault, journal, mapping, review record, cache, log, or
  temporary file in the agent tree;
- exact conformance to the Section 18 tree allowlist;
- every expected token present and canonical; and
- no unknown, malformed, forged, or unmapped token.

The scan establishes absence only for the declared values and scanned
surfaces. It does not establish that unannotated sensitive information was
discovered.

### Vault oracle

The vault oracle verifies:

- exact container location, physical separation, ownership, and modes;
- exact schema, foreign keys, PRAGMAs, application ID, and version;
- no unknown table, column, trigger, view, virtual table, or sidecar;
- expected entities, aliases, decisions, mappings, tokens, and audit rows;
- absence of complete normalized documents, source binaries, OCR images,
  model buffers, and application logs;
- transaction rollback, busy behavior, corruption, schema mismatch, wrong-
  repository rejection, and rollback-journal recovery;
- token persistence after publication failure; and
- exact project deletion and `deletion_pending` recovery behavior.

Successful deletion establishes only that the recognized Outis vault
directory was removed. It does not establish SSD, snapshot, swap, diagnostic,
or backup erasure.

### Controlled agent-boundary oracle

A separate test-only sandboxed probe receives read access only to the final
agent repository. It reads every expected Markdown and manifest file and must
receive an access-denied result when it attempts to enumerate or open the
source, vault, application-container, or staging locations. It receives no
bookmark, handle, path, environment value, or IPC surface for those locations
and performs zero network requests.

This result applies only to the controlled probe and its recorded sandbox
configuration. Outis does not control an arbitrary external agent and cannot
claim that an unsandboxed process running as the same macOS user is prevented
from accessing other user files.

### State coverage and overall pass rule

The known-plaintext, tree, vault, and controlled-probe oracles run for initial
publication, replacement, cancellation, blocked input, operational failure,
and crash-recovery outcomes where applicable. A non-completed job exposes no
new agent tree; the prior valid tree and recognized staging behavior match
Sections 20 and 29 exactly.

Acceptance requires every exact oracle to pass and every model false positive,
false negative, unresolved item, extraction rejection, and privacy limitation
to remain visible in the evidence. Passing cannot be described as perfect
detection, anonymity, security against the local user, suitability for
confidential data, or representation of real legal documents.

## 35. Benchmark methodology

### Prerequisites and measured subject

Correctness, privacy, deterministic replay, publication, and recovery oracles
must pass before performance can pass. Runtime measurements use the exact
`PERF-SMOKE-V1` corpus from Section 32 and the release-configured, sandboxed
application test host using the same production engine, native adapters,
model, vault, synchronization, and publication paths. Prescribed synthetic
review decisions may be supplied by the test target; no processing stage may
be bypassed.

No benchmark crate, command-line product, UI-test target, production test
mode, feature flag, runtime environment variable, or performance setting is
permitted. The previously proposed `tools/outis-eval` crate is rejected. The
three approved Rust crates and existing `OutisTests` target own evaluation.

### Repetitions and states

- Initial and replacement publication are separate measurements.
- Runtime p95 uses nearest-rank over 20 measured runs.
- One preceding warm-up is excluded and still recorded.
- A cold launch is a fresh process with no model session. Filesystem caches are
  neither purged nor described as cold.
- Warm extraction and inference follow one excluded warm-up operation.
- Build measurements use three runs and gate on the median.
- Human `needs_review` dwell is excluded; engine work that validates and
  commits submitted decisions remains included.
- Failed, unstable, cancelled, or exceeded runs remain in evidence and are
  never discarded from interpretation.

The report records UTC interval, operator, commit, dirty state, exact command,
toolchain, signing, sandbox and entitlement state, CPU, memory, OS, filesystem,
power state, thermal warnings, corpus and artifact identities, wall/user/system
time, CPU ratio, peak RSS, output bytes, stage times, cold/warm state, every
sample, and every failure. Power and thermal state are captured before and
after a run set. Energy is reported without an acceptance threshold.

### Evidence root and contents

The acceptance runner creates exactly one new directory:

~~~text
artifacts/outis_local_pilot/<UTC-YYYYMMDDTHHMMSSZ>-<12-hex-git-sha>/
  environment.json
  commands.jsonl
  fixture_manifest.json
  correctness.json
  privacy.json
  build.json
  performance.json
  bundle.json
  evidence_manifest.json
  summary.json
  raw/
~~~

An existing run directory blocks rather than being overwritten. The evidence
root is never inside a source, vault, staging, or agent repository and is not
bundled. Raw command output replaces the run-specific temporary absolute roots
with declared placeholders before promotion. No vault database, token
dictionary, model artifact, real sensitive value, private runtime path, or
plaintext mapping is stored.

`commands.jsonl` records ordered command, exit, duration, and stdout/stderr
hashes. `evidence_manifest.json` records relative path, byte size, and SHA-256
for every other evidence file. `summary.json` has terminal status exactly
`PASS` or `BLOCKED`. `PASS` requires every mandatory command to exit zero,
every Section 33 and 34 oracle to pass, zero skipped tests, the exact
dependency and symbol allowlists, exact entitlements, zero runtime network
requests, and every Section 31 and 32 budget to pass. Missing legal clearance,
model artifacts, full Xcode, required system capability, or evidence produces
`BLOCKED`, never a partial pass.

## 36. Test plan

### Synthetic fixtures

The exact fixture root is:

~~~text
fixtures/outis_local_pilot/v1/
  README.md
  SHA256SUMS
  fixture_manifest.json
  extraction/
    text/
    doc/
    docx/
    pdf_text/
    pdf_scan/
    expected_normalized/
    oracle.json
  detection/
    it/
    de/
    fr/
    oracle.json
  model/
    ner_smoke.json
    replay_inputs.json
    oracle.json
  entity/
    source/
    oracle.json
  publication/
    source/
    expected_agent_repository/
    oracle.json
  performance/
    source/
    oracle.json
  privacy/
    known_plaintext.json
~~~

Every value is synthetic; real sensitive data is forbidden. Every fixture is
listed with byte size and SHA-256. Binary fixtures record creation tool,
version, OS build, purpose, and limitation. Expected data is independently
authored and cannot be produced through production Outis logic. No SQLite
vault is committed. Fixtures and oracles are excluded from the application
bundle. A changed fixture byte requires reviewed manifest and oracle changes.

The byte-verification command is:

~~~text
shasum -a 256 -c fixtures/outis_local_pilot/v1/SHA256SUMS
~~~

### Exact Rust test files

~~~text
crates/outis-core/tests/structured_detection.rs
crates/outis-core/tests/entity_resolution.rs
crates/outis-core/tests/tokenization.rs
crates/outis-core/tests/export_contract.rs
crates/outis-runtime/tests/job_flow.rs
crates/outis-runtime/tests/extraction_contract.rs
crates/outis-runtime/tests/model_contract.rs
crates/outis-runtime/tests/vault_contract.rs
crates/outis-runtime/tests/publication_contract.rs
crates/outis-runtime/tests/cancellation_recovery.rs
crates/outis-ffi/tests/abi_contract.rs
crates/outis-ffi/tests/ownership_contract.rs
~~~

Failure injection exists only in `#[cfg(test)]` modules. It adds no production
feature, environment variable, runtime branch, configuration item, dependency,
or exported symbol.

The files above are the final complete-pilot integration-test set, not an
allowlist for `MI-01`. The first increment creates none of them and creates no
fixture subtree. Its complete oracle is the `#[cfg(test)]` unit-test module in
`crates/outis-core/src/detect/email.rs` defined in Section 40.
`crates/outis-runtime/tests/model_contract.rs` and the model fixture subtree
remain absent until legal clearance.

### Exact Swift test files

The existing `OutisTests` target owns exactly:

~~~text
OutisTests/AppModelTests.swift
OutisTests/JobViewModelTests.swift
OutisTests/ReviewViewModelTests.swift
OutisTests/ExtractionReviewViewModelTests.swift
OutisTests/EngineClientTests.swift
OutisTests/FolderAccessTests.swift
OutisTests/DocumentExtractorTests.swift
OutisTests/WordExtractorTests.swift
OutisTests/PDFExtractorTests.swift
OutisTests/VisionOCRTests.swift
OutisTests/ExtractionSubmissionTests.swift
OutisTests/PerformanceTests.swift
~~~

There is no UI-test or third Xcode target.

### Acceptance support

The only cross-surface acceptance support files are:

~~~text
tests/acceptance/run.sh
tests/acceptance/agent_boundary.sb
tests/acceptance/agent_boundary_probe.sh
~~~

`run.sh` is the single full-suite entry point, accepts no arguments, creates a
new Section 35 evidence directory, preserves every failure, and returns
nonzero unless every mandatory check passes. The sandbox profile and probe
implement only the controlled Section 34 oracle. If the selected macOS build
lacks `/usr/bin/sandbox-exec`, that oracle is `BLOCKED`; there is no substitute
or weakened test.

### Test ownership matrix

| Contract | Owning tests |
|---|---|
| Source, snapshot, path, mutation | `job_flow`, `extraction_contract` |
| Extraction and normalized Markdown | Rust extraction plus Swift extractor tests |
| Structured detection and metrics | `structured_detection` |
| Model identity, output, replay | `model_contract` |
| Review, entity, alias | `entity_resolution` plus Swift review tests |
| Token and replacement | `tokenization`, `export_contract` |
| Vault and audit | `vault_contract` |
| Agent tree and known plaintext | export, publication, acceptance probe |
| Publication and last-valid output | `publication_contract` |
| Cancellation and crash recovery | `cancellation_recovery` |
| ABI, memory, panic containment | FFI tests, `EngineClientTests` |
| Sandbox, entitlements, folder access | `FolderAccessTests`, bundle inspection |
| Performance and resources | `PerformanceTests`, acceptance runner |
| Deferred-service absence | symbol, dependency, bundle scans |

Every supported success branch and declared block or failure branch has at
least one exact case. Acceptance permits no ignored, skipped, flaky,
quarantined, or expected-failure test.

### Narrow validation commands

~~~text
env RUSTUP_TOOLCHAIN=stable cargo fmt --all -- --check

env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo clippy --locked --offline \
  --workspace --all-targets -- -D warnings

env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo test --locked --offline \
  -p outis-core --tests -- --test-threads=1

env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo test --locked --offline \
  -p outis-runtime --tests -- --test-threads=1

env RUSTUP_TOOLCHAIN=stable CARGO_NET_OFFLINE=true \
  cargo test --locked --offline \
  -p outis-ffi --tests -- --test-threads=1
~~~

Swift tests use:

~~~text
env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer xcodebuild \
  -project apps/macos/Outis/Outis.xcodeproj \
  -scheme Outis -configuration Debug \
  -destination 'platform=macOS,arch=arm64' \
  -derivedDataPath target/xcode-tests test
~~~

Release performance tests use:

~~~text
env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer xcodebuild \
  -project apps/macos/Outis/Outis.xcodeproj \
  -scheme Outis -configuration Release \
  -destination 'platform=macOS,arch=arm64' \
  -derivedDataPath target/xcode-performance \
  -only-testing:OutisTests/PerformanceTests test
~~~

The release application build uses:

~~~text
env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer xcodebuild \
  -project apps/macos/Outis/Outis.xcodeproj \
  -scheme Outis -configuration Release \
  -destination 'platform=macOS,arch=arm64' \
  -derivedDataPath target/xcode-release clean build
~~~

The full acceptance command is:

~~~text
/bin/sh tests/acceptance/run.sh
~~~

It also invokes the Section 25 dependency and advisory commands, Section 26
header verification, fixture hashes, symbol allowlist, signature,
entitlements, bundle contents, known-plaintext scan, controlled-agent probe,
runtime network observation, and Section 31 and 32 measurements. Section 37
binds the exact lower-level bundle inspection commands and expected values.

## 37. Code and build bindings

### Normative path allowlist and root migration

`docs/roadmaps/outis_local_pilot_file_architecture.json` is incorporated here
as the normative path and responsibility allowlist. An unlisted production,
test, generated, model, fixture, acceptance, or review path is forbidden until
a specification amendment. In particular, the core allowlist includes
`crates/outis-core/src/detect/postal_address.rs`, and the migration and
`rust-toolchain.toml` are mandatory.

Across the approved implementation sequence:

- root `Cargo.toml` first becomes the one-member `MI-01` workspace and later
  reaches the exact three-member complete-pilot workspace without empty
  placeholders;
- `Cargo.lock` becomes committed format version four and is no longer ignored;
- the obsolete root `src/main.rs` is removed;
- stale `Makefile.toml` and `release.toml`, whose commands target unrelated
  `mbt_cache` products, are removed rather than migrated into Outis;
- `.gitignore` first stops ignoring `Cargo.lock`; later changes add only the
  generated Xcode output, local model artifacts, and acceptance evidence used
  by their approved complete capability;
- repository-maintenance package and commit-hook files remain outside product
  targets; and
- `inventory.md` is regenerated only through
  `bin/generate_global_inventory.rs`, never edited manually.

The user-owned `architecture-public.md` deletion is preserved.

### Xcode settings and phases

The application target fixes:

~~~text
PRODUCT_NAME = Outis
PRODUCT_BUNDLE_IDENTIFIER = com.outis.localpilot
MARKETING_VERSION = 0.1.0
CURRENT_PROJECT_VERSION = 1
MACOSX_DEPLOYMENT_TARGET = 14.0
ARCHS = arm64
SWIFT_VERSION = 6.0
SWIFT_STRICT_CONCURRENCY = complete
ENABLE_APP_SANDBOX = YES
ENABLE_HARDENED_RUNTIME = YES
CODE_SIGN_STYLE = Manual
CODE_SIGN_IDENTITY = -
DEVELOPMENT_TEAM = empty
GENERATE_INFOPLIST_FILE = NO
LD_RUNPATH_SEARCH_PATHS = @executable_path/../Frameworks
DEAD_CODE_STRIPPING = YES
ENABLE_USER_SCRIPT_SANDBOXING = YES
~~~

Release uses Swift `-O`, stripped application output, and a separate dSYM
outside the bundle. Debug uses `-Onone`. Both are arm64-only. The first slice
has no asset catalog until an actual approved visual asset exists.

Build ownership is ordered as:

1. verify the committed generated C header;
2. verify model legal clearance and model/runtime hashes;
3. build `outis-ffi` offline as an arm64 release static library;
4. compile and link Swift, Rust, and approved Apple frameworks;
5. copy model and tokenizer to `Resources/Models/EntityDetector/`;
6. copy and sign `libonnxruntime.1.28.0.dylib` in `Contents/Frameworks/`;
7. copy `THIRD_PARTY_NOTICES.txt`; and
8. sign the application last.

This is the complete-pilot phase set. `MI-01` creates none of these phases and
no Xcode project or development application. All application phases, their
inputs and outputs, and all model bundle references remain outside its first
implementation plan. Model phases additionally remain blocked until legal
clearance and the approved model-specific implementation plan.

Xcode regenerates neither bindings nor fixtures. Its script phases declare
exact repository inputs and `target` outputs and remain compatible with user-
script sandboxing.

### Bundle inspection

Within `tests/acceptance/run.sh`, the fixed release bundle paths are:

~~~text
OUTIS_APP_PATH=target/xcode-release/Build/Products/Release/Outis.app
OUTIS_EXECUTABLE_PATH=$OUTIS_APP_PATH/Contents/MacOS/Outis
OUTIS_RUNTIME_PATH=$OUTIS_APP_PATH/Contents/Frameworks/libonnxruntime.1.28.0.dylib
~~~

The script runs these exact inspection commands and preserves their complete
outputs under the active Section 35 evidence directory:

~~~text
/usr/bin/file "$OUTIS_EXECUTABLE_PATH" "$OUTIS_RUNTIME_PATH"
/usr/bin/codesign --verify --strict --verbose=4 "$OUTIS_RUNTIME_PATH"
/usr/bin/codesign --verify --strict --verbose=4 "$OUTIS_APP_PATH"
/usr/bin/codesign --display --verbose=4 "$OUTIS_APP_PATH"
/usr/bin/codesign --display --entitlements :- "$OUTIS_APP_PATH"
/usr/bin/otool -L "$OUTIS_EXECUTABLE_PATH" "$OUTIS_RUNTIME_PATH"
/usr/bin/otool -l "$OUTIS_EXECUTABLE_PATH"
/usr/bin/nm -gjU "$OUTIS_EXECUTABLE_PATH"
/usr/bin/plutil -p "$OUTIS_APP_PATH/Contents/Info.plist"
/usr/bin/find "$OUTIS_APP_PATH/Contents" -print
~~~

The runner parses these outputs and requires:

- arm64-only executable and dylib;
- valid separate dylib and application signatures without `--deep`;
- enabled Hardened Runtime and entitlement keys exactly
  `com.apple.security.app-sandbox` and
  `com.apple.security.files.user-selected.read-write`, each boolean true;
- application dependencies restricted to the selected Apple system
  frameworks, Swift system runtimes, `/usr/lib/libSystem.B.dylib`, and
  `@rpath/libonnxruntime.1.28.0.dylib`; the ONNX Runtime dylib may depend only
  on Apple system libraries;
- exactly one `LC_RPATH`, `@executable_path/../Frameworks`;
- only the ten `outis_` ABI functions across the language boundary;
- `CFBundleIdentifier=com.outis.localpilot`,
  `CFBundleShortVersionString=0.1.0`, `CFBundleVersion=1`, and
  `LSMinimumSystemVersion=14.0`;
- regular bundle files restricted to `Contents/Info.plist`,
  `Contents/MacOS/Outis`,
  `Contents/Frameworks/libonnxruntime.1.28.0.dylib`,
  `Contents/Resources/Models/EntityDetector/model.onnx`,
  `Contents/Resources/Models/EntityDetector/tokenizer.json`,
  `Contents/Resources/THIRD_PARTY_NOTICES.txt`, and signing-owned files under
  `Contents/_CodeSignature/`; and
- absence of fixtures, tests, archives, acquisition tools, evidence, dSYMs,
  vaults, logs, and unsupported architectures.

Any additional entitlement, rpath, non-system dependency, regular bundle file,
or exported `outis_` symbol blocks acceptance. The exact observed Apple and
Swift system dependency set becomes evidence; it is not treated as a stable
cross-Xcode byte identity.

Section 36 owns the exact build commands. No code or build change is
authorized by this working specification.

## 38. Generated, model, dataset, and evidence artifact bindings

### Generated C header and handwritten sources

`generated/ffi/outis.h` remains the only generated production source and has
the Section 26 source, command, verification, symbol, and size contract. The
SQLite migration, module map, entitlements, model manifest, fixture manifest,
oracles, and notices are reviewed handwritten sources rather than generated
bindings.

### Model acquisition and local artifacts

The committed model inputs are:

~~~text
models/manifests/entity_detector.json
models/acquire_entity_detector.sh
~~~

The argument-free script requires a Section 39 `CLEARED` legal review,
downloads only into a fresh `target/model-acquisition/` directory, uses the
immutable Section 12 model-revision URLs and Microsoft ONNX Runtime 1.28.0
macOS arm64 release URL, verifies every approved size and SHA-256, verifies the
archive and extracted dylib, and then atomically publishes exactly:

~~~text
models/artifacts/entity_detector_v1/model.onnx
models/artifacts/entity_detector_v1/tokenizer.json
models/artifacts/entity_detector_v1/libonnxruntime.1.28.0.dylib
~~~

`models/artifacts/` is ignored and never committed. The script cannot replace
a valid local set until the complete new set validates. It performs no model
conversion, optimization, quantization, repair, or modification. Acquisition
is a separate preparation operation; Xcode and Outis never download.

The sole acquisition command is:

~~~text
/bin/sh models/acquire_entity_detector.sh
~~~

`entity_detector.json` records schema version, upstream model and revision,
immutable URLs, license identifiers, legal-review identity, artifact names,
sizes and hashes, runtime and provider, input/output names and shapes, labels,
tokenizer, threading, bundle paths, and evaluation-corpus identity. Runtime
uses this committed source contract when verifying its signed-bundle inputs.

### Fixture generation and evidence

Test-only fixture-generation sources are exactly:

~~~text
tests/fixture_generation/Main.swift
tests/fixture_generation/WordFixtures.swift
tests/fixture_generation/PDFFixtures.swift
~~~

They read only the versioned synthetic fixture manifest, use fixed content,
geometry, metadata, ordering, and dates, import no Outis production logic, and
write only declared binary Word and PDF fixtures into a fresh directory.
Allowed modes are `--write` and `--check`. `--write` requires the approved
implementation plan; acceptance uses `--check` and blocks on any byte change.
Text, detection, entity, privacy, and expected-output oracles are independently
authored sources.

The exact compilation command is:

~~~text
env DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer xcrun \
  --sdk macosx swiftc -swift-version 6 \
  -target arm64-apple-macos14.0 -parse-as-library \
  tests/fixture_generation/Main.swift \
  tests/fixture_generation/WordFixtures.swift \
  tests/fixture_generation/PDFFixtures.swift \
  -framework AppKit -framework PDFKit -framework CoreGraphics \
  -o target/tools/outis-fixture-generation
~~~

The implementation-plan-only creation command and acceptance replay command
are, respectively:

~~~text
target/tools/outis-fixture-generation --write
target/tools/outis-fixture-generation --check
~~~

The fixture manifest and `SHA256SUMS` own output identities. The current
temporary AppKit probe observed byte-identical simple `.doc` and `.docx` pairs
on one host; complete fixture reproducibility remains unproved until `--check`
passes.

Sections 35 and 36 own evidence and dataset locations. No fixture, model,
generated header, evidence artifact, or notice may be created before the
approved implementation plan. Model-specific fixtures, sources, manifests,
scripts, artifacts, notices, and evidence additionally require legal
clearance.

## 39. Review artifact bindings

The lifecycle artifact bindings are exactly:

~~~text
docs/reviews/outis_local_pilot/outis_local_pilot_model_legal_review.md
docs/reviews/outis_local_pilot/outis_local_pilot_spec_pre_audit.md
docs/reviews/outis_local_pilot/outis_local_pilot_peer_audit.md
docs/reviews/outis_local_pilot/outis_local_pilot_implementation_plan.md
docs/reviews/outis_local_pilot/outis_local_pilot_pre_test_audit.md
docs/reviews/outis_local_pilot/outis_local_pilot_result_review.md
~~~

The model legal review must be authored or approved by a qualified human and
classify exactly `CLEARED` or `BLOCKED`. `CLEARED` addresses model weights,
base-model terms, training corpora, commercial use, redistribution, notices,
attribution, modification, and source-availability obligations. Codex cannot
supply that legal conclusion. `BLOCKED` returns model selection to research.

The author pre-audit records closure evidence without changing the spec. A
separate peer audit then classifies exactly `PEER_AUDIT_PASSED` or `BLOCKED`.
The implementation plan separately binds and requests approval for every
change. The pre-test audit checks the completed implementation, and the result
review may claim only evidence from the approved acceptance command.

## 40. Implementation-plan requirement

After this specification is complete and passes a separate peer audit, a
separate minimal implementation plan must bind every authorized change and
receive explicit approval. This draft cannot substitute for that plan.

### `MI-01`: deterministic email discovery

The first plan may bind exactly one complete capability: convert one already
validated UTF-8 `document_text` surface and its 32-byte source-snapshot SHA-256
identity into the ordered email subset of `SensitiveCandidateV1`. This is a
complete domain transformation with a direct oracle. It is not an application,
job, extractor, anonymizer, publisher, or funding demo.

The stable Rust entrypoint is:

~~~text
pub fn detect_email_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, EmailDiscoveryErrorV1>
~~~

The initial public domain types are exactly:

~~~text
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitiveCandidateV1 {
    pub source_snapshot_sha256: [u8; 32],
    pub surface: CandidateSurfaceV1,
    pub path_component_index: Option<u32>,
    pub start_byte: usize,
    pub end_byte: usize,
    pub sensitive_class: SensitiveClassV1,
    pub observed: String,
    pub equality_key: Option<String>,
    pub detector_id: &'static str,
    pub detector_version: u16,
    pub evidence: CandidateEvidenceV1,
    pub status: CandidateStatusV1,
}

pub enum CandidateSurfaceV1 { DocumentText }
pub enum SensitiveClassV1 { Email }
pub enum CandidateEvidenceV1 { StructuredGrammar }
pub enum CandidateStatusV1 { Accepted, NeedsReview }
pub const MAX_EMAIL_CANDIDATES_PER_SURFACE: usize = 65_536;
pub enum EmailDiscoveryErrorV1 {
    CandidateLimitExceeded { limit: usize },
}
~~~

Each enum has the same four derives as the struct. These public types and the
entrypoint are the intended final `outis-core` boundary for this capability,
not temporary scaffolding. Later approved complete capabilities may add
spec-bound variants without adding unused variants in `MI-01`.

The scanner visits maximal non-empty spans delimited by Rust 1.89
`char::is_whitespace` in ascending UTF-8 byte order and emits only spans
containing ASCII `@`. It emits one record per such span and preserves that
order. For every record:

- `source_snapshot_sha256` equals the input identity;
- `surface` is `document_text` and `path_component_index` is absent;
- the half-open byte range is within `text`, ends on scalar boundaries, and
  selects exactly `observed`;
- class is `email`;
- detector identifier is `outis.email.ascii`, version is `1`, and evidence is
  `structured_grammar`;
- Section 11 grammar success gives status `accepted` and equality key equal to
  the preserved local part, ASCII `@`, and ASCII-lowercased domain; and
- grammar failure gives status `needs_review` and no equality key.

The fixed `MAX_EMAIL_CANDIDATES_PER_SURFACE` is 65,536. The function returns
`CandidateLimitExceeded { limit: 65_536 }` as soon as a 65,537th output would
be required, returns no partial vector, and logs or retains no rejected
candidate. This fixed ceiling bounds record overhead for a spec-valid hostile
input; it is not configurable and is not a performance claim.

The function is deterministic, performs no I/O, logs nothing, and has no
locale, clock, random, environment, thread, model, database, or platform input.
The input `&str` proves UTF-8 validity at the Rust boundary. Empty text or text
without ASCII `@` returns `Ok` with an empty vector. The 16-MiB
normalized-document bound from Section 9 remains the caller contract; `MI-01`
does not add a second size check.

### Exact first-plan file boundary

The first plan may create, change, or remove only the following implementation
paths. Lifecycle plans and review artifacts remain governed by Section 39 and
are not product implementation paths:

~~~text
rust-toolchain.toml
Cargo.toml
Cargo.lock
.gitignore
src/main.rs
Makefile.toml
release.toml
crates/outis-core/Cargo.toml
crates/outis-core/docs/inventory.md
crates/outis-core/src/lib.rs
crates/outis-core/src/candidate.rs
crates/outis-core/src/detect.rs
crates/outis-core/src/detect/email.rs
crates/outis-core/src/detect/email/tests.rs
inventory.md
~~~

`src/main.rs`, `Makefile.toml`, and `release.toml` are removals of the current
unrelated scaffold. The root manifest becomes a workspace containing only
`crates/outis-core` in this increment. This staged membership creates no empty
runtime or FFI crate; the final architecture still contains exactly the three
crates in Section 37. `Cargo.lock` is committed in format 4 and contains no
registry package because `MI-01` has no external dependency. `.gitignore`
stops ignoring `Cargo.lock` and otherwise changes only where the plan proves a
current need. `inventory.md` is never edited manually; it is regenerated only
through `bin/generate_global_inventory.rs` after the approved migration.

The complete `MI-01` manifest and toolchain content is:

~~~text
# rust-toolchain.toml
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
profile = "minimal"
targets = ["aarch64-apple-darwin"]

# Cargo.toml
[workspace]
members = ["crates/outis-core"]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.89.0"
publish = false

[profile.release]
panic = "unwind"

# crates/outis-core/Cargo.toml
[package]
name = "outis-core"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[lib]
path = "src/lib.rs"
~~~

`stable` is only the installed rustup alias. Before any MI-01 Cargo, rustc,
rustfmt, or Clippy operation, the following exact read-only preflight runs with
rustup distribution endpoints redirected to an unreachable loopback port and
Cargo network access disabled:

~~~text
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true rustc --version --verbose
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo --version --verbose
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo clippy --version
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true rustfmt --version
env RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  rustup component list --installed \
  --toolchain stable-aarch64-apple-darwin
env RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  rustup target list --installed \
  --toolchain stable-aarch64-apple-darwin
~~~

The preflight accepts only all of these observations:

- rustc release `1.89.0`, commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, and host
  `aarch64-apple-darwin`;
- Cargo release `1.89.0`, commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`, and host
  `aarch64-apple-darwin`;
- Clippy `0.1.89` commit `29483883ee`;
- rustfmt `1.8.0-stable` commit `29483883ee`;
- installed `cargo`, `clippy`, `rustc`, `rustfmt`, and
  `rust-std-aarch64-apple-darwin` components; and
- installed `aarch64-apple-darwin` target.

Additional installed components or targets do not fail; every required item
must be present and every exact binary identity above must match. Because the
distribution endpoints are loopback tripwires and Cargo is
offline, a successful preflight proves that no remote toolchain resolution is
required on the inspected host. It does not prove that no process opened a
local socket. The loopback values are fixed validation inputs, not operator
configuration or runtime settings.

No omitted manifest section, feature, example, binary, benchmark, build script,
or package metadata is implied. The only `.gitignore` edit removes the exact
`Cargo.lock` line.

`lib.rs` declares private `candidate` and `detect` modules and publicly
re-exports only `SensitiveCandidateV1`, the four candidate enums,
`EmailDiscoveryErrorV1`, `MAX_EMAIL_CANDIDATES_PER_SURFACE`, and
`detect_email_candidates`. `detect.rs` declares private module `email` and
defines the public constant and entrypoint; the entrypoint delegates scanning
and grammar work to that private module. No module itself is public.

The handwritten component inventory contains exactly these source-purpose
entries under its legal notice and `outis-core` heading:

~~~text
- `src/lib.rs`: intentional public domain API exports.
- `src/candidate.rs`: MI-01 candidate records, enums, and typed limit error.
- `src/detect.rs`: MI-01 detector module ownership and public entrypoint.
- `src/detect/email.rs`: deterministic email scanner, grammar, equality key,
  and fixed output ceiling.
- `src/detect/email/tests.rs`: private MI-01 unit oracle.
~~~

`crates/outis-core/docs/inventory.md` is the handwritten component inventory
required by the existing generator and lists only the five Rust source files
and their responsibilities. `candidate.rs` owns only the spec-bound record
fields, enum variants, and typed limit error used by this capability.
`detect.rs` owns the public entrypoint and delegates the grammar to
`detect/email.rs`. `email.rs` owns scanning, validation, equality-key
construction, and the fixed limit. `detect/email/tests.rs` is its private
`#[cfg(test)]` unit-test module. No temporary public API, generic detector
framework, error hierarchy, configuration, feature, fixture, integration-test
file, benchmark, or unused class implementation is allowed.

All other complete-pilot paths are excluded. In particular, the plan must not
touch or create the macOS application, Xcode project, Swift, runtime crate, FFI
crate, generated header, extraction adapter, other detector, entity graph,
review surface, token contract, vault, schema, export, publication contract,
agent repository, acceptance runner, fixture, evidence directory, model path,
model dependency, build phase, or distributable artifact. There is no
test-only publication and no temporary job status, model substitute, or
development application.

### `MI-01` oracle and validation

The unit-test table must cover at least:

- accepted ASCII local-part forms, mixed-case domains, multiple domain labels,
  and lengths at the 64-byte local, 63-byte label, and 254-byte total limits;
- leading, trailing, and repeated local dots; repeated `@`; empty sides;
  domain edge hyphens; invalid ASCII; non-alphabetic or one-byte final labels;
  over-limit local parts, labels, and totals; quoted, commented,
  internationalized, and domain-literal forms, all as `needs_review`;
- adjacent Unicode text and whitespace, multiple candidates, empty input, and
  no-candidate input;
- exactly 65,536 candidates succeed, a 65,537th returns the exact typed error,
  and no partial output is observable;
- exact source identity, observed bytes, range, class, detector identity,
  evidence, status, equality key, and ascending order for every result;
- scalar-boundary validity for every range; and
- exactly equal ordered records across three replays of the same input.

No test is ignored, conditionally skipped, network-backed, time-backed, or
dependent on process-global state. The plan must bind the exact table values
before code. Validation is limited to:

~~~text
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo fmt --all -- --check
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo clippy --locked --offline \
  -p outis-core --lib --tests -- -D warnings
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo test --locked --offline \
  -p outis-core --lib -- --test-threads=1
mkdir -p target/tools
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true rustc -O bin/generate_global_inventory.rs \
  -o target/tools/generate_global_inventory
target/tools/generate_global_inventory --repo-root "$(pwd)" \
  --out "$(pwd)/inventory.md" --strict
~~~

Acceptance requires the S1-21 preflight and all MI-01 commands to exit zero,
every exact identity and required installed item to match, every bound unit
case to pass, no registry package in the lockfile, and a changed-path audit
matching the first-plan boundary. This proves only local toolchain resolution
and the declared email transformation on the bound synthetic strings. It does
not prove extraction, multilingual contextual detection, complete sensitive
discovery, privacy, anonymization, publication, application operation, or
performance.

After a qualified `CLEARED` review, a separate model-specific implementation
plan binds the excluded paths, two dependencies, lockfile transition, Xcode
phase transition, complete tests, acquisition, packaging, and validation. That
plan requires explicit approval before model-specific work.

## 41. Approval and pre-audit closure checklist

- [x] `S1-01` product boundary explicitly approved.
- [x] `S1-02` source and filesystem contract explicitly approved.
- [x] `S1-03` extraction and normalized-Markdown contract explicitly approved.
- [x] `S1-04` sensitive-class and deterministic-discovery contract explicitly
  approved.
- [x] `S1-05` local contextual-model contract explicitly approved.
- [x] `S1-06` entity-resolution and review-decision contract explicitly
  approved.
- [x] `S1-07` tokenization and redaction contract explicitly approved.
- [x] `S1-08` private-vault and plaintext trust-boundary contract explicitly
  approved.
- [x] `S1-09` agent-repository and atomic-publication contract explicitly
  approved.
- [x] `S1-10` macOS application-flow and job-state contract explicitly
  approved.
- [x] `S1-11` macOS build, sandbox, signing, and Swift/Rust boundary contract
  explicitly approved.
- [x] `S1-12` exact FFI, wire-schema, ownership, error, and codegen contract
  explicitly approved.
- [x] `S1-13` exact dependency, feature, license, and supply-chain contract
  explicitly approved.
- [x] `S1-14` whole-pipeline failure, recovery, retention, deletion, backup,
  and audit contract explicitly approved.
- [x] `S1-15` end-to-end runtime performance, resource, application-size, and
  compile-time budget contract explicitly approved.
- [x] `S1-16` correctness, extraction, detection, entity-resolution, privacy,
  vault-isolation, and agent-boundary oracle contract explicitly approved.
- [x] `S1-17` benchmark methodology, synthetic-fixture, test-matrix, command,
  and evidence-artifact contract explicitly approved.
- [x] `S1-18` final code, build, generated-artifact, model-acquisition,
  dataset-generation, and review binding contract explicitly approved.
- [x] `S1-19` model-specific legal-gate partition explicitly approved.
- [x] `S1-20` exact complete `MI-01` email-discovery increment explicitly
  approved.
- [x] `S1-21` installed-stable exact-identity and offline-resolution amendment
  explicitly approved.
- [ ] Model legal-clearance stop gate resolved; required only before
  model-specific implementation and complete-pilot acceptance.
- [x] Full Xcode 26.6 installation and active-developer-directory stop gate
  resolved.
- [x] Installed `stable` toolchain resolved with the exact S1-21 identities,
  components, and target while distribution endpoints were redirected to
  loopback and Cargo was offline.
- [x] All S1 decision packets approved.
- [x] Every mandatory section closed as a design binding.
- [x] Mandatory section order verified after closure.
- [x] Prior-spec search and disposition complete.
- [x] Exact command, UI, dispatch, path, artifact, FFI, target, dependency,
  test, migration, and evidence bindings complete.
- [x] Correctness, privacy, deterministic replay, publication, and recovery
  proof commands complete.
- [x] Compile-surface and application-size evidence commands complete.
- [x] Minimal-surface necessity bindings complete.
- [x] S1-21 author pre-audit closure passed.
- [x] Separate peer audit passed.
- [x] P1-01 amended MI-01 implementation plan explicitly approved.

Specification approval, including `S1-21`: granted by the user on 2026-08-17.

## 42. Open questions

No design decision remains intentionally open. The missing qualified model
legal review blocks only the exact model-specific surfaces and complete-pilot
acceptance defined above; it does not block MI-01 after its lifecycle gates.
The S1-21 author pre-audit, separate peer audit, and P1-01 approval passed.
MI-01 requires the amended plan's new committed clean baseline and exact
offline preflight. An implementation detail that contradicts or is absent from
this specification requires a specification amendment; it cannot be delegated
to the implementation plan.
