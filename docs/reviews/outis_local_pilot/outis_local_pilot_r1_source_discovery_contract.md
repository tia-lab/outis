~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis R1 Source and Discovery Contract

Status: R1_SELECTED_FOR_S1_RATIFICATION
Date: 2026-08-17
Classification: research only
Code authorization: none

## Source Selection and Access

1. The user selects one source directory and one distinct export-parent
   directory with native folder panels.
2. Both security-scoped URLs exist only for the active job. Every successful
   access call has a matching stop call.
3. No security-scoped bookmark is persisted.
4. The sandbox uses user-selected read/write access because Outis creates the
   export. Outis opens source files read-only and never mutates the source.
5. Source, export parent, final export, staging, and vault cannot be equal,
   ancestors, or descendants of one another, except that final export and
   staging are children of the export parent.

## Required Source Tree

The first pilot must accept regular files in these source classes:

- legacy Word `.doc`;
- Open XML Word `.docx`;
- text-bearing `.pdf`;
- image-only or scanned `.pdf` through local OCR;
- UTF-8 `.txt`;
- UTF-8 `.md`.

R1 previously restricted input to `.txt` and `.md`. That restriction was
incorrect and is withdrawn. R1.2 selected strict Rust UTF-8 decoding plus
native AppKit, PDFKit, Core Graphics, and Vision candidates. Exact evidence,
limitations, configuration, and alternatives are in
`outis_local_pilot_r1_2_extraction_evaluation.md`.

Candidate tree and resource conditions for S1 ratification are:

- at most 1,000 supported files;
- `.txt` and `.md` are at most 10 MiB each;
- `.doc`, `.docx`, and `.pdf` are at most 50 MiB each;
- source bytes total at most 250 MiB;
- normalized UTF-8 totals at most 100 MiB;
- at most 100 pages per PDF and 200 OCR pages per job;
- each fixed 200-DPI OCR render is at most 4,096 pixels on either axis and
  16,777,216 pixels total;
- at most 32 directory levels;
- relative paths are valid UTF-8, at most 1,024 UTF-8 bytes, and contain no
  empty, `.` or `..` component;
- every included item is a regular file whose extension and inspected content
  identify one required source class;
- `.DS_Store` is the only silently ignored item;
- every other regular file, symlink, Finder alias, package, hard-link
  duplicate, device, socket, FIFO, mount crossing, or nested repository blocks
  the job with a typed private error;
- source and export have distinct device and inode identities;
- each file's device, inode, size, modification time, and SHA-256 remain stable
  from snapshot read through the pre-publication recheck.

The limits are unproved denial-of-service candidates, not capacity claims.
S1 must ratify or reject them. Exceeding a limit blocks; Outis never silently
truncates or downscales.

## Normalized Document

- Every accepted source yields one normalized document or an explicit
  `needs_review` or blocked result; it is never silently skipped.
- `.txt` and `.md` require valid UTF-8. One leading UTF-8 BOM is omitted; a NUL
  scalar or invalid UTF-8 blocks the job. Other Unicode scalars, whitespace,
  and line endings are preserved.
- Word, PDF, and OCR conversion emits a plain-text Markdown profile. CRLF, CR,
  U+2028, and U+2029 become LF; PDF pages join with exactly two LF bytes; the
  result has exactly one final LF. Tabs and spaces are preserved.
- Binary extraction emits ordered private provenance binding normalized byte
  ranges to the source and, for PDF, page and extraction mode. It emits no
  source metadata into the agent repository.
- Text-bearing and image-only PDF pages are distinct cases. A page with
  non-whitespace PDFKit text uses native text; an empty page uses local OCR.
  The first slice does not merge native page text with OCR text.
- Source text is not globally normalized or lowercased.
- Offsets are zero-based half-open ranges in the BOM-free UTF-8 bytes. Every
  span lands on scalar boundaries.
- Markdown is unparsed text. Front matter, code blocks, link targets, and prose
  are all scanned.
- One successfully processed source file becomes one `.md` export. The export
  mirrors the source-relative directory and base document name after sensitive
  path components are tokenized, then replaces the source extension with
  `.md`.
- If two sources resolve to the same case-insensitive target path, publication
  blocks with a typed collision; Outis never overwrites or invents an
  undisclosed suffix.
- Processing order is ascending raw UTF-8 relative-path bytes. Paths remain
  private.

## Extraction Selection and Review

- Rust owns content-signature and snapshot validation, strict `.txt` and `.md`
  decoding, normalized-document validation, and later pipeline stages.
- Swift owns `.doc` and `.docx` import through AppKit, PDF inspection and text
  through PDFKit, fixed rendering through Core Graphics, and OCR through
  Vision revision 3.
- OCR uses accurate recognition, languages `it-IT`, `de-DE`, `fr-FR` in that
  order, automatic language detection, no language correction, no custom
  words, `minimumTextHeight = 0`, 200 DPI, and one page request at a time.
- Every `.doc`, `.docx`, and PDF result is `needs_review` before publication.
  This includes native PDF text and OCR irrespective of confidence.
- Lists, tables, columns, headers, footers, notes, tracked changes, and reading
  order may flatten or be omitted. Non-empty text is not completeness proof.
- A detected attachment, embedded object, corrupt or signature-mismatched
  file, encrypted/locked/copy-disallowed PDF, missing required OCR language,
  page with no OCR observations, or source mutation blocks publication.
- Empty or whitespace-only binary extraction is `needs_review` and cannot be
  represented as complete.
- OCR confidence and AppKit conversion attributes are retained privately as
  evidence only. Neither can approve content automatically.
- Cancellation is checked between files, pages, renders, extraction
  submissions, and later stages. No cancellation-latency claim is made for one
  platform call.

## Active Classes

The token classes are exactly:

- `person`;
- `organization`;
- `postal_address`;
- `email_address`;
- `telephone_number`;
- `iban`;
- `matter_identifier`.

Government identifiers, credentials, dates, events, and generally
confidential passages are unsupported. That is a coverage limit, not evidence
that such values are non-sensitive.

## Email Address

Automatic acceptance is limited to:

- a 1–64 byte dot-atom local part using ASCII letters, digits, and
  `!#$%&'*+/=?^_{|}~-`;
- no leading, trailing, or repeated dot;
- one `@`;
- ASCII domain labels 1–63 bytes long with letters, digits, and internal
  hyphens;
- an alphabetic final label 2–63 bytes long;
- no leading or trailing domain hyphen and at most 254 bytes overall.

The equality key preserves the local part and ASCII-lowercases the domain.
Internationalized, literal, commented, or quoted mailboxes and plausible
strings containing `@` that fail this subset are `needs_review`.

## Telephone Number

Automatic acceptance requires:

- leading `+`;
- country code `33`, `39`, `41`, or `49`;
- only digits, ASCII spaces, non-breaking spaces, `.`, `-`, `(`, and `)` after
  the plus sign;
- 8–15 digits in the complete number after separator removal.

The equality key is `+` followed by digits. A plausible local or national
number, extension, unsupported country code, or out-of-bounds international
form is `needs_review`. E.164 supports the 15-digit maximum; the other limits
are first-slice policy, not complete national numbering-plan validation.

## IBAN

The detector removes Unicode whitespace, uppercases ASCII letters, requires
alphanumeric content and MOD-97 remainder one, then validates:

| Country | Length | Structure after country and check digits |
|---|---:|---|
| Switzerland | 21 | 5 digits, 12 alphanumeric |
| Germany | 22 | 18 digits |
| France | 27 | 10 digits, 11 alphanumeric, 2 digits |
| Italy | 27 | 1 letter, 10 digits, 12 alphanumeric |

The equality key is uppercase without spaces. A plausible `CCdd...` sequence
that fails an active rule or checksum is `needs_review`. Other countries are
unsupported and require review.

## Postal Address

The deterministic assembler requires, on one line:

1. one declared street cue and a house number;
2. a four-digit Swiss or five-digit German, Italian, or French postcode;
3. a following city phrase;
4. optional overlapping NER `LOC` evidence.

Initial case-insensitive NFC street cues are:

- Italian: `via`, `viale`, `corso`, `piazza`, `largo`, `vicolo`, `strada`;
- German: `straße`, `strasse`, `weg`, `platz`, `gasse`, `allee`, `ring`,
  `ufer`, including those strings at the end of a joined street word;
- French: `rue`, `avenue`, `boulevard`, `chemin`, `route`, `place`, `quai`,
  `impasse`.

The house number is 1–5 digits with an optional ASCII letter. The city is 1–4
whitespace-separated words and at most 64 Unicode scalars. Each word contains
Unicode letters with optional internal U+0027, U+2019, or U+002D. The candidate
spans from the cue or joined street word through the city and stops before
`.`, `;`, `:`, or line end.

Every address is `needs_review`. NER `LOC` alone never creates or automatically
tokenizes an address.

## Matter Identifier

A candidate requires, on one line, a cue followed within 24 Unicode scalars by
a 3–64 character identifier containing at least one digit and only ASCII
letters, digits, `/`, `.`, `_`, or `-`.

Initial cues:

- Italian: `fascicolo`, `pratica`, `contratto`;
- German: `aktenzeichen`, `az.`, `akte`, `vertrag`;
- French: `dossier`, `affaire`, `contrat`.

Every matter identifier is `needs_review`. Its equality key is NFC, trimmed,
Unicode-lowercased, and whitespace-collapsed; punctuation is preserved.

## Contextual Detector

The contextual detector is the exact R1.1 selection in
`outis_local_pilot_ner_evaluation.md`, including artifact hashes, tokenizer,
chunking, label mapping, 0.50 emit threshold, fixed two-thread CPU
configuration, and no-network behavior.

- `PER` creates a `person` candidate.
- `ORG` creates an `organization` candidate.
- `LOC` is address evidence only.
- `DATE` is unsupported and creates `needs_review` without automatic
  tokenization.
- Unknown labels or artifact mismatch block the job.
- Every NER person or organization requires review, regardless of confidence.

There is no fallback model or remote inference.

## Candidate Ordering and Overlap

Candidates are ordered by document, start byte, end byte, active-class order,
detector order `structured`, `address`, `NER`, then detector evidence identity.

- Identical class and span evidence is deduplicated.
- An accepted structured span outranks same-class overlapping contextual
  evidence.
- Non-identical, cross-class, nested, or partial overlaps are never merged
  automatically; they require review.
- Replacement cannot begin until every conflict has one outcome.

## Entity Resolution

- Link only same-class candidates with byte-identical class equality keys.
- Never merge by fuzzy similarity, surname, initials, abbreviation, honorific,
  legal-suffix removal, address proximity, or confidence.
- Preserve each source form privately with document and span provenance.
- Create a new entity for every non-identical key unless the user explicitly
  merges two reviewed same-class entities.
- Never link identities across source repositories.

For person, organization, and postal address, the equality key is NFC,
trimmed, Unicode-lowercased, and whitespace-collapsed. Punctuation,
apostrophes, accents, hyphens, and legal suffixes are preserved. Structured
classes use their class rules.

## Review

Sensitive-entity review actions are exactly:

- confirm class and span;
- reclassify to an active class and adjust to scalar boundaries;
- exclude a false positive;
- explicitly merge two reviewed same-class entities.

Binary extraction review actions are exactly:

- `confirm_extraction` after inspecting the original in embedded Quick Look
  beside the complete normalized text and page/mode evidence;
- reject the extraction, which blocks the complete job.

Extraction confirmation binds the source SHA-256, normalized-text SHA-256,
extractor identity, and review-schema version. Any change invalidates it.
Confirmation records review; it is not proof of hidden-content completeness,
privacy, or safety. A source cannot be omitted from the first-slice export.

Decisions are repository-local and private. Exclusion applies only to the exact
document snapshot, span, detector version, and observed value; it creates no
global safe-list. Any unresolved candidate or conflict blocks publication.

## Primary Sources

Sources were read on 2026-08-17.

- [Apple: Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)
- [Apple: stopAccessingSecurityScopedResource](https://developer.apple.com/documentation/foundation/url/stopaccessingsecurityscopedresource%28%29)
- [Unicode Standard Annex 15](https://www.unicode.org/reports/tr15/)
- [SWIFT IBAN Registry](https://www.swift.com/sites/default/files/files/IBAN_Registry.pdf)
- [ITU-T E.164](https://www.itu.int/rec/T-REC-E.164)
- [RFC 6531](https://www.rfc-editor.org/info/rfc6531/)
