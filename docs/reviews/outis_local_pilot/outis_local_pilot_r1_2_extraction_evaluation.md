~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis R1.2 Extraction and OCR Evaluation

Slug: outis_local_pilot_r1_2_extraction_evaluation
Status: R1_2_SELECTED_FOR_S1_RATIFICATION
Date: 2026-08-17
Classification: research and documentation only
Code authorization: none

## Decision

The smallest evaluated path covering every required first-slice format on
macOS is:

| Source | Selected candidate | Owner |
|---|---|---|
| `.txt`, `.md` | strict UTF-8 decoding with no document-conversion dependency | Rust runtime |
| legacy `.doc`, `.docx` | AppKit `NSAttributedString` document import | Swift application |
| text-bearing `.pdf` | PDFKit page inspection and `PDFPage.string` | Swift application |
| scanned or image-only PDF page | PDFKit rendering, Core Graphics, and Vision text recognition revision 3 | Swift application |

This selection uses frameworks supplied by macOS and adds no document parser,
office suite, Java runtime, OCR engine, or OCR language-data package to the app.
It is an S1 candidate, not dependency or code approval.

The fidelity probes do not justify automatic publication of binary formats.
Every `.doc`, `.docx`, text-PDF, and OCR result therefore has
`needs_review` status in the funding-demo slice. Strict `.txt` and `.md` may
proceed without format review when every source and UTF-8 check passes.

## Scope and Measured Object

The measured object was local extraction of representative synthetic Italian,
German, and French content on one Mac. It included visible text, a list, a
table, page boundaries, a two-column page, image-only pages, a degraded
small-text scan, corruption, signature mismatch, PDF access restrictions,
deterministic replay, warm latency, and process memory.

It did not measure real legal documents, handwriting, photographs, rotated
pages, complex Word revisions, embedded objects, form fields, annotations,
footnotes, headers, accessibility order, every font, every scanner, every PDF
producer, cold-start latency, energy, or a signed sandboxed Outis application.

## Candidate Comparison

| Candidate | Required-format coverage | Added product surface | R1.2 finding |
|---|---|---|---|
| AppKit, PDFKit, Vision, Core Graphics | all required formats on the target Mac | no bundled extraction runtime or language artifact | selected for S1 ratification |
| Pandoc 3.6.4 | `.docx`, Markdown, and text; no legacy `.doc` reader | measured local executable was about 261 MiB | rejected for the first slice |
| Tesseract | OCR with separate Italian, German, and French trained data | engine, language artifacts, packaging, and conversion glue | rejected for the first slice |
| LibreOffice headless conversion | Word conversion | external office process, bundle, sandbox, lifecycle, and update surface | rejected for the first slice |
| Apache Tika | broad parsing; OCR delegates to external Tesseract | Java runtime and parser dependency graph | rejected for the first slice |
| remote conversion or OCR | potentially broad | plaintext network boundary and remote retention behavior | prohibited by pilot scope |

Pandoc converted the simple synthetic `.docx` in about 0.02 seconds with a
38.5 MB maximum resident set in one local run. This does not offset its missing
legacy `.doc` input or the extra bundled executable. Alternative rejection is
specific to the minimal macOS pilot, not a general quality ranking.

## Selected API and Configuration

### Input classification

Rust first binds the lowercase extension to one required class and checks:

- `.doc`: first eight bytes are `D0 CF 11 E0 A1 B1 1A E1`;
- `.docx`: first four bytes are one of the ZIP signatures `50 4B 03 04`,
  `50 4B 05 06`, or `50 4B 07 08`;
- `.pdf`: a `%PDF-` header occurs within the first 1,024 bytes;
- `.txt` and `.md`: optional leading UTF-8 BOM followed by strict UTF-8 with
  no NUL.

The selected first slice does not parse the Word package or Compound File
structure independently. AppKit must also accept the exact declared document
type. Attachment evidence exposed by the imported attributed string blocks the
job. These checks do not prove that a Word document has no hidden, revised, or
embedded content; mandatory review and the synthetic-only claim boundary
remain necessary. An adversarial compressed `.docx` is an unmeasured resource
risk and a stop condition if the platform importer cannot remain inside the S1
bounds.

### Word

Swift calls the AppKit attributed-string document initializer with the exact
declared type:

- `.docFormat` for a content-signature-validated Compound File Binary `.doc`;
- `.officeOpenXML` for a content-signature-validated ZIP/Open XML `.docx`.

Outis consumes the imported visible string and records the declared type,
source digest, operating-system build, framework identity, import result, and
whether `NSConvertedDocumentAttribute` reports lossy conversion. A missing or
zero converted attribute is not proof of complete extraction.

### PDF text

Swift opens the document with PDFKit, rejects zero-page, encrypted, locked, or
copy-disallowed documents, and inspects every page. A page whose PDFKit string
is non-empty after whitespace inspection uses that string. A page whose string
is empty uses the OCR path.

The first slice does not merge native PDF text with OCR text on one page.
Images can contain text even when a page also exposes native text, so every PDF
remains `needs_review`.

### OCR

The selected configuration is exact:

- `VNRecognizeTextRequestRevision3`;
- `.accurate` recognition;
- recognition languages in fixed order `it-IT`, `de-DE`, `fr-FR`;
- automatic language detection enabled;
- language correction disabled;
- no custom words;
- `minimumTextHeight = 0`;
- PDF media box rendered on opaque white, in sRGB, at 200 DPI, eight bits per
  component, 32 bits per pixel, premultiplied-last RGBA;
- one page render and one Vision request at a time;
- top candidate only;
- observations ordered by decreasing finite normalized bounding-box midpoint
  Y, then increasing minimum X, decreasing maximum Y, increasing width, and
  original Vision result index. Invalid boxes block the page.

The disposable benchmark rendered the media box with this color and pixel
contract. The measured host reported Italian, German, and French support for
revision 3 at the accurate recognition level. S1 must require the same runtime
query and block an unsupported required language.

Language correction is disabled because it changed a correct synthetic email
from `giulia.bianchi@...` to `giulia bianchi@...` in the degraded scan while
reporting confidence 1.0. Confidence is evidence for review; it is never an
automatic correctness decision.

## Normalized Markdown Profile

One successfully processed source produces one `.md` file under the existing
tokenized mirrored-path contract. Normalization is deliberately text-oriented,
not layout-preserving.

1. `.md` preserves the BOM-free source Markdown bytes until token replacement.
2. `.txt` preserves the BOM-free source text; plain text is valid Markdown.
3. Invalid UTF-8 or any NUL in `.txt` or `.md` blocks the job.
4. No source is NFC-normalized, lowercased, spell-corrected, translated, or
   reflowed by Rust.
5. Binary-derived text converts CRLF, CR, U+2028, and U+2029 to LF, joins PDF
   pages with exactly two LF bytes, and ends with exactly one LF byte.
6. Binary-derived tabs and spaces are preserved. Outis injects no headings,
   page labels, timestamps, filenames, comments, or front matter.
7. Ordered private provenance binds normalized byte ranges to the source file
   and, for PDF, page number and extraction mode `pdf_text` or `vision_ocr`.
   Word block-level provenance is unavailable in the selected plain-text
   profile and this absence is recorded.
8. Detection and token replacement operate on zero-based half-open UTF-8 byte
   ranges at Unicode-scalar boundaries.
9. Private provenance, bounding boxes, confidence, paths, and extraction
   diagnostics never enter the agent repository.

Lists, tables, columns, headers, footers, notes, tracked changes, and reading
order may flatten or be omitted. The export is normalized Markdown text, not a
faithful office-layout replica.

## Status and Failure Contract

| Condition | Job status before publication |
|---|---|
| valid `.txt` or `.md`, with all discovery decisions closed | may proceed |
| any successfully imported `.doc` or `.docx` | `needs_review` until the exact extraction review closes |
| any PDF using native text or OCR | `needs_review` until the exact extraction review closes |
| empty or whitespace-only binary-document result | `needs_review`; it cannot be represented as complete |
| unresolved extraction or discovery review | `needs_review`; publication forbidden |
| extension/signature mismatch, corrupt or zero-page input | `blocked` |
| encrypted, locked, password-required, or copy-disallowed PDF | `blocked` |
| detected attachment, embedded object, or omitted content-bearing feature | `blocked` |
| required OCR language unavailable, page has no OCR observation, or render exceeds a bound | `blocked` |
| source mutation or source-to-target collision | `blocked` |
| user cancellation before publication | `cancelled`; staging removed and prior output preserved |
| platform/API/internal error not caused by declared input policy | `failed` with a non-sensitive private diagnostic |

Extraction review is exact and local:

- Outis presents the original in an embedded Quick Look UI `QLPreviewView` and
  the complete normalized text with page/mode evidence;
- the user may `confirm_extraction` for the exact source SHA-256, normalized-
  text SHA-256, extractor identity, and review-schema version, or reject;
- confirmation records that a human inspected that exact result. It does not
  prove hidden-content completeness, privacy, or safety;
- source or normalized bytes changing after confirmation invalidate the
  decision and return to `needs_review`;
- rejection blocks the complete job. The first slice cannot exclude a source,
  because every accepted source must map to one Markdown target.

Quick Look is a native local review candidate, not extraction evidence. Apple
documents previews for Microsoft Office documents and PDFs, but Outis has not
measured preview fidelity, helper-process behavior, caches, or diagnostics.
S1 must bind those boundaries; any unbounded plaintext persistence blocks
confidential-data use.

Cancellation is checked between files, pages, render operations, extraction
submissions, and later pipeline stages. Vision cancellation may be requested,
but no response-time bound is claimed. Word import and one PDFKit operation are
treated as bounded, non-preemptible calls; S1 must bind file and page limits.

No error payload, diagnostic, or log may contain source text, sensitive paths,
OCR observations, extracted spans, entities, or mappings.

## Candidate Resource Bounds for S1

These guardrails bound the first slice; they are not capacity or denial-of-
service proofs:

- at most 1,000 supported files and 32 directory levels;
- `.txt` and `.md`: at most 10 MiB each;
- `.doc`, `.docx`, and `.pdf`: at most 50 MiB each;
- at most 250 MiB total source bytes;
- at most 100 PDF pages per document;
- at most 200 OCR pages per job;
- at most 100 MiB normalized UTF-8 across a job;
- fixed 200 DPI OCR render, at most 4,096 pixels on either axis and
  16,777,216 pixels per page;
- block rather than silently downscale or truncate when a limit is exceeded;
- one extraction and one OCR page request at a time.

S1 must ratify these numbers or keep implementation blocked. The measured
one-page and ten-page results are too narrow to prove the limits safe for
adversarial input.

## Synthetic Fixtures

The disposable fixture contained only invented names and values. Recorded
SHA-256 identities were:

| Fixture | SHA-256 |
|---|---|
| `.doc` | `b8e0745d71667c1cf28d4c183ca1da0cce8366b680d890621dae1fc86dca55f0` |
| `.docx` | `2823f36647ef03a61469ea7f9de09f55396cc83678c312bd4738e8d58b4d07a8` |
| text PDF | `71c9bd1c0b79a14abc986be6c6308231d6775293927b7b36e50f1a05cef48247` |
| scanned PDF | `605456f7f1a5c5738def9fa2bc9376cb24c63ba02f9751ecc8038c23ca955486` |
| degraded scan | `e1b07b2ad88b22e52e5ac1c348eed5bf9c01cfccd76f75a729a74e05fd024320` |
| `.txt` | `cef2eac45931c23fc7c7b8a7192582e07346f59c3e8fe4d8dc584b0d48f70cdd` |
| `.md` | `88a3f852651f7572da3f37d51f296c0e6090099e44b62fabeca3a14f1ed5fdf6` |

Repository-owned fixtures still need to be specified and approved before they
are created. The temporary research files are not product artifacts.

## Observed Fidelity

- AppKit produced the same 499 UTF-8 bytes of visible text for the simple
  `.doc` and `.docx` fixtures.
- List semantics were flattened. Table-style evidence appeared for the `.doc`
  import but not for the `.docx` import despite the same visible table text.
- `NSConvertedDocumentAttribute` was absent for both imports, which did not
  establish completeness.
- PDFKit exactly exposed the simple vector PDF text, including its line wraps.
- PDFKit returned empty text for the image-only PDF.
- On a two-column page, PDFKit returned the complete left column before the
  right column. Vision returned row-interleaved left/right observations. Both
  were deterministic in the probes, but neither order is a general semantic
  oracle.
- Clean 150, 200, and 300 DPI OCR returned the expected visible text in the
  small fixture. With language correction disabled, the tested degraded scan
  also matched at all three resolutions.
- Truncated `.doc`, `.docx`, and PDF inputs failed explicitly. Forced
  extension/type mismatches failed explicitly. A generated encrypted/locked
  PDF was rejected.

These observations establish usable candidate behavior on the recorded
fixtures only. They do not prove complete extraction.

## Benchmark Evidence

Environment:

- Apple M4 Pro, arm64, 24 GiB RAM;
- macOS 26.5, build 25F71;
- Command Line Tools SDK macOS 15.2;
- Objective-C harness compiled with Apple Clang `-O2`, calling the same
  AppKit, PDFKit, Vision, and Core Graphics APIs selected for Swift;
- warm process, synthetic one-page fixture, 20 iterations for non-OCR and 10
  for each OCR resolution after warm-up.

Command shape:

~~~text
/usr/bin/time -l <temporary-probe> benchmark <temporary-fixture-directory> 20
~~~

Observed milliseconds:

| Path | Minimum | Median | p95 | Mean |
|---|---:|---:|---:|---:|
| `.txt` | 0.015 | 0.016 | 0.022 | 0.017 |
| `.md` | 0.014 | 0.014 | 0.016 | 0.015 |
| `.doc` | 0.134 | 0.148 | 0.344 | 0.182 |
| `.docx` | 0.562 | 0.577 | 0.597 | 0.589 |
| text PDF | 0.542 | 0.584 | 0.776 | 0.639 |
| OCR 150 DPI | 103.527 | 110.818 | 125.762 | 113.052 |
| OCR 200 DPI | 98.876 | 107.176 | 117.722 | 106.038 |
| OCR 300 DPI | 138.573 | 147.244 | 173.106 | 149.853 |

The multi-resolution process reached about 812 MB maximum RSS because Vision
caches for three resolutions remained in one process. A separate selected
200-DPI ten-page OCR run completed in 1.22 seconds wall time with about 383 MB
maximum RSS. A ten-page text-PDF extraction completed in 0.04 seconds with
about 20 MB maximum RSS.

Fresh-process parallel OCR probes were dominated by Vision service start-up
and contention, taking 17 to 24 seconds each. They are not accepted as a cold
baseline. Cold start, concurrency, energy, signed-app behavior, and product
memory remain unmeasured. The selection forbids concurrent OCR.

Two attempts to compile one monolithic disposable Swift probe did not finish
within approximately 2.5 and 1.5 minutes and were terminated. This is evidence
only that the probe shape was unsuitable in the measured Command Line Tools
environment; it is not Outis compile-time evidence. The API behavior was then
measured through the disposable Objective-C harness.

## Replay and Artifact Identity

Five fresh-process runs produced one stable SHA-256 of complete probe stdout
per tested path:

- `.docx`:
  `fc4bbcb58d0b0adf315ee11790c6f80dceb54f7aa3e462e7b02c2d452b599076`;
- PDF text:
  `1838be63eedd9fb27d4b5e2c6a49c215340f7b82cb8d9f48dc96deed07fd2e64`;
- probe OCR:
  `35a234540b9e5b655c1e07d4383c01b16e90dae800eae0e3a949c09afafaa662`.

These are disposable-probe output identities, not fixture-distribution
checksums. Repository-owned replay evidence must reproduce its own exact
outputs.

The disposable harness grouped vertical coordinates with a pairwise 0.005
tolerance before horizontal ordering. That comparator is not the selected
total-order contract above. It produced the recorded simple-fixture output,
but the OCR hash is not a final normalized-output oracle. S1 must test the
selected total order on repository-owned multi-column and tie fixtures.

Vision OCR is an operating-system-managed artifact and cannot be separately
pinned or distributed by Outis. Replay identity must bind:

- source bytes and source ordering;
- macOS product version and build, CPU architecture, SDK deployment contract,
  and AppKit/PDFKit/Vision framework environment;
- Vision request revision and every option;
- supported-language query result and fixed language order;
- PDF page box, 200 DPI, color space, pixel format, background, and render
  dimensions;
- observation ordering and normalized-document schema versions.

Byte-identical OCR is required only inside that complete identity. No cross-
macOS-build or cross-machine equality is claimed. A changed operating-system
build invalidates cached approval and requires the approved synthetic
regression before controlled use.

## Architecture and FFI Effect

Binary document extraction belongs to the Swift platform layer because the
selected APIs are native Objective-C/macOS frameworks. Rust remains the job
state and data-contract authority.

The polling ABI therefore needs one additional S1 candidate operation:

~~~text
outis_job_submit_extraction
~~~

Rust polling emits an extraction request with an opaque request identifier.
Swift performs one bounded extraction operation on a background task, then
submits the identifier, typed status/provenance JSON, and a separate
pointer-plus-length UTF-8 text buffer. Rust copies and validates all bytes
before return. The buffer is Human Zone plaintext. There are no callbacks,
borrowed retained pointers, re-entrant calls, file handles, vault handles, or
source text inside JSON diagnostics.

Candidate files are amended in the file-architecture roadmap. S1 must define
the exact request schema, byte limits, lifecycle, cancellation race behavior,
and ABI tests before any symbol is implemented.

## Required S1 Decisions

S1 must ratify or reject:

1. the four selected extraction paths and exact platform API behavior;
2. the normalized Markdown and provenance schemas;
3. mandatory binary-format review and every typed status in this artifact;
4. content signatures, feature inspection, limits, cancellation points, and
   no-truncation behavior;
5. fixed Vision revision/configuration, system-artifact identity, and OS-build
   regression gate;
6. exact synthetic fixtures and text, page, structure, OCR, failure, and replay
   oracles;
7. Swift extraction ownership and the extraction-submission ABI;
8. full-Xcode, sandbox, signing, deployment, and framework-availability checks.

Code remains blocked until S1 passes peer audit and P1 binds exact files,
commands, dependencies, expected outputs, and rollback.

## Primary Sources

Sources were read on 2026-08-17.

- [Apple: NSAttributedString document types](https://developer.apple.com/documentation/foundation/nsattributedstring/documenttype)
- [Apple: NSConvertedDocumentAttribute](https://developer.apple.com/documentation/appkit/nsconverteddocumentattribute)
- [Apple: PDFPage](https://developer.apple.com/documentation/pdfkit/pdfpage)
- [Apple: PDFDocument string](https://developer.apple.com/documentation/pdfkit/pdfdocument/string)
- [Apple: PDFDocument](https://developer.apple.com/documentation/pdfkit/pdfdocument)
- [Apple: PDFDocument allowsCopying](https://developer.apple.com/documentation/pdfkit/pdfdocument/allowscopying)
- [Apple: Recognizing text in images](https://developer.apple.com/documentation/vision/recognizing-text-in-images)
- [Apple: VNRecognizeTextRequest](https://developer.apple.com/documentation/vision/vnrecognizetextrequest)
- [Apple: Locating and displaying recognized text](https://developer.apple.com/documentation/vision/locating-and-displaying-recognized-text)
- [Apple: Quick Look UI](https://developer.apple.com/documentation/quicklookui)
- [Apple: QLPreviewView](https://developer.apple.com/documentation/quicklookui/qlpreviewview)
- [Microsoft: Word `.doc` binary format](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-doc/ccd7b486-7881-484c-a137-51170af7cc22)
- [Microsoft: Compound File Binary format](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/53989ce4-7b05-4f8d-829b-d08d6148375b)
- [Pandoc user guide](https://pandoc.org/MANUAL.html)
- [Tesseract installation and language data](https://tesseract-ocr.github.io/tessdoc/Installation.html)
- [Tesseract command-line usage](https://tesseract-ocr.github.io/tessdoc/Command-Line-Usage.html)
- [Apache Tika getting started](https://tika.apache.org/3.2.3/gettingstarted.html)
- [LibreOffice command-line parameters](https://help.libreoffice.org/latest/en-US/text/shared/guide/start_parameters.html)
