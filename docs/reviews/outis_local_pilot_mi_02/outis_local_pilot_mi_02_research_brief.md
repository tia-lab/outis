~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-02 Research Brief

Status: `COMPLETE_FOR_APPROVED_S1_22C_AND_PASSED_AUDITS`
Classification: research only; no code
Date: 2026-08-17

## Goal

Evaluate the next smallest model-independent Outis capability after `MI-01`:
deterministic telephone-number and IBAN candidate discovery from one already
validated UTF-8 document-text surface.

The proposed increment does not extract documents, run a model, resolve
entities, assign tokens, write a vault, publish an agent repository, or create
a macOS application.

## Source materials

Repository evidence:

- `docs/specs/outis_local_pilot_SPEC.md`, especially Sections 10, 11, 12, 33,
  37, and 40;
- `docs/reviews/outis_local_pilot/outis_local_pilot_r1_source_discovery_contract.md`;
- `docs/reviews/outis_local_pilot/outis_local_pilot_r1_acceptance_evidence.md`;
- `docs/reviews/outis_local_pilot/outis_local_pilot_result_review.md`;
- the committed `crates/outis-core` MI-01 source and unit oracle at
  `f1801ae41ba4acad819c8292641f91c1fd5c963e`.

External primary evidence:

- SWIFT, [IBAN Registry, Release 102, June 2026](https://www.swift.com/swift-resource/9606/download).
  SWIFT is the ISO 13616 registration authority. The registry records the
  fixed CH, DE, FR, and IT structures already used by S1.
- ITU-T, [Recommendation E.164, in force February 2026](https://www.itu.int/rec/T-REC-E.164/).
  It defines the international public-telecommunication numbering framework
  and the 15-digit maximum used by the S1 telephone grammar.

External standards do not define Outis candidate boundaries, review policy,
equality keys, or supported-country subset. Those remain product contracts.

## Measured object

The object is a pure Rust transformation with these fixed inputs:

- one 32-byte source-snapshot SHA-256 identity; and
- one already validated UTF-8 `document_text` string.

The result is either:

- ordered `SensitiveCandidateV1` records for one requested structured class;
  or
- a typed all-or-nothing candidate-limit error.

The increment measures exact ranges, observed bytes, class, equality key,
detector identity, status, order, replay equality, and fixed-limit behavior on
synthetic strings. It does not measure extraction, national telephone-plan
validity, bank-account existence, privacy, security, or end-to-end latency.

## Candidate architecture

Extend the existing dependency-free `outis-core` detector surface without
creating a new crate or generalized detector framework:

~~~text
crates/outis-core/src/candidate.rs
  add only the telephone_number and iban class variants and one proposed
  MI-02 shared typed limit error while preserving the MI-01 error

crates/outis-core/src/detect.rs
  retain the email entrypoint and add one class-specific entrypoint for
  telephone candidates and one for IBAN candidates

crates/outis-core/src/detect/telephone.rs
  deterministic scan, classification, equality key, and ceiling

crates/outis-core/src/detect/telephone/tests.rs
  private synthetic telephone oracle

crates/outis-core/src/detect/iban.rs
  deterministic scan, country structure, streaming MOD-97, equality key,
  and ceiling

crates/outis-core/src/detect/iban/tests.rs
  private synthetic IBAN oracle
~~~

Only the handwritten component inventory and generated root inventory would
change outside those Rust files. `Cargo.toml`, `Cargo.lock`, toolchain files,
Swift, Xcode, runtime, FFI, fixtures, models, and build configuration would not
change.

## Trust zones and data ownership

| Surface | Zone | Contract |
|---|---|---|
| input text and source identity | Human Zone | private caller-owned input |
| emitted candidate records | Human Zone | private review and later-tokenization input |
| agent repository | AI Zone | not read or written by MI-02 |
| private vault | Key Zone | not read or written by MI-02 |

The detector must perform no I/O, logging, telemetry, persistence, network
access, or model execution. Candidate records must not enter the agent-facing
repository.

## Sensitive-data classes

MI-02 covers only:

- `telephone_number`; and
- `iban`.

Italian, German, and French remain required document languages. These two
structured grammars are largely language-independent. The approved country
subset is deliberately CH, DE, FR, and IT; that subset is not a claim of
language detection or complete geographic coverage.

## Candidate telephone policy

The existing S1 acceptance rule is suitable for a small implementation:

- accepted values start with `+`;
- the normalized digits start with `33`, `39`, `41`, or `49`;
- the complete number contains 8 through 15 digits including country code;
- after `+`, only digits, ASCII space, U+00A0, `.`, `-`, `(`, and `)` occur;
- the accepted equality key is `+` followed by all digits; and
- missing international form, unsupported country code, and declared
  extensions become `needs_review` with no equality key.

The smallest proposed scanner works one logical line at a time. It starts only
at `+` or an ASCII digit after a line start, whitespace, or opening punctuation,
then consumes the allowed formatting set. A national-form candidate must
contain at least one declared separator or parenthesis; an unformatted 8- to
15-digit value alone is not classified as telephone-like in this increment.
This reduces generic identifier false positives but creates a declared false-
negative boundary.

The exact opening-punctuation set, terminal trimming, right boundary, and
extension-cue table must be approved in S1-22 before code. National numbering-
plan validation remains out of scope.

## Candidate IBAN policy

The S1 IBAN rule remains the candidate:

- two ASCII letters and two ASCII check digits begin the value;
- the candidate stays on one logical line;
- internal ASCII alphanumerics and declared Unicode inline whitespace are
  permitted;
- normalization uppercases ASCII letters and removes inline whitespace;
- a streaming MOD-97 calculation must produce remainder one for acceptance;
  and
- only the exact CH, DE, FR, and IT lengths and country structures can be
  accepted.

Supported-country values with the wrong length, structure, or checksum and
plausible unsupported-country values become `needs_review` with no equality
key. Accepted values use the uppercase whitespace-free IBAN as their equality
key. The local contextual model cannot validate or override an IBAN.

S1-22 must still bind the minimum plausible length, exact inline-whitespace
set, supported-country stopping rule, unsupported-country stopping rule, and
adjacent-alphanumeric behavior. These are product-policy decisions, not
settled by ISO 13616 or the SWIFT registry.

## Determinism boundary

The proposed detector identity binds:

- Rust 1.89 Unicode-scalar behavior where used;
- logical-line and inline-whitespace tables;
- left and right candidate boundaries;
- telephone body and extension tables;
- supported telephone country-code order;
- IBAN country lengths and BBAN structures;
- the MOD-97 algorithm;
- class-specific equality keys;
- detector identifiers and versions;
- fixed candidate ceilings; and
- ascending UTF-8 byte order.

No clock, locale, random source, thread count, environment value, filesystem,
database, model, or platform API may influence the result.

## Correctness and privacy oracle candidates

The synthetic unit tables should bind:

- every accepted telephone country code in compact and formatted forms;
- national, unsupported-country, extension, too-short, too-long, punctuation,
  adjacent-Unicode, and non-ASCII-digit telephone cases;
- valid compact and print-format CH, DE, FR, and IT IBANs from synthetic values;
- wrong checksum, wrong country structure, wrong length, lowercase,
  unsupported-country, adjacent-text, cross-line, and Unicode-whitespace IBAN
  cases;
- exact source identity, observed bytes, half-open UTF-8 ranges, class,
  equality key, detector identity, evidence, status, and order;
- empty and marker-free input;
- three byte-identical replays;
- exactly 65,536 results and the all-or-nothing 65,537th-result error for each
  entrypoint; and
- static searches proving absence of I/O, logging, unsafe code, model,
  database, process, clock, random, environment, and network surfaces.

The declared accepted-subset precision and recall target is exactly `1.0` only
for the bound synthetic grammar table. No broader discovery claim follows.

## Evidence table

| Question | Evidence | Finding |
|---|---|---|
| Can the existing core host another pure detector? | MI-01 code and result review | Yes for a dependency-free class-specific increment; broader integration is unproved. |
| Are the four IBAN layouts current? | SWIFT Registry Release 102 | Yes as of June 2026 for CH, DE, FR, and IT. |
| Is the 15-digit telephone ceiling grounded? | ITU-T E.164 (02/2026) | Yes for international E.164 numbers; national-plan validity is not established. |
| Are scanning boundaries already exact? | S1 Section 11 code read | No. Acceptance grammar is defined, but maximal candidate boundaries and extension cues require an amendment. |
| Is a dependency required? | Algorithm and current code read | No dependency need is established; standard-library scanning and streaming MOD-97 are sufficient hypotheses to test. |
| Is model legal clearance relevant? | S1 Section 12 | No for this pure structured increment; it remains mandatory for every model-specific surface. |

## Hypotheses

- A standard-library implementation can remain small enough for the existing
  `outis-core` compile surface.
- Separate class-specific entrypoints avoid introducing unused orchestration
  or overlap logic before the complete discovery pipeline exists.
- Streaming MOD-97 avoids large integer or cryptographic dependencies.
- Conservative telephone boundaries will create reviewable false positives
  and some declared false negatives without claiming national-plan coverage.

These hypotheses require code-read and run evidence after an approved plan.
No speed, memory, privacy, or coverage result is predicted here.

## Unknowns requiring S1-22 decisions

1. Exact telephone left boundary, trailing trimming, and right boundary.
2. Exact trilingual extension-cue table and extension digit bound.
3. Whether plain unformatted national numbers are ignored or emitted for
   review.
4. Exact Unicode inline-whitespace table for IBAN scanning.
5. Minimum length for a plausible invalid IBAN.
6. How unsupported-country IBAN candidates stop without embedding the full
   international registry.
7. Whether the existing email-specific error becomes a source-compatible
   alias of one shared structured-discovery error or remains unchanged beside
   one new shared error for MI-02.
8. Whether each class retains its own 65,536 ceiling or MI-02 shares one
   combined ceiling; separate entrypoints favor class-specific ceilings.

## Risks

- Permissive telephone scanning can create many review candidates from dates,
  matter numbers, or other digit strings.
- Restrictive telephone scanning can miss unformatted national numbers.
- An IBAN scanner can consume adjacent prose if whitespace and stopping rules
  are not exact.
- MOD-97 success does not prove that an account exists or belongs to a person.
- An E.164-shaped number does not prove that it is assigned or reachable.
- Public API migration can break future callers if aliases and limits are not
  specified before implementation.
- Combining classes now would require overlap and cross-class ordering behavior
  not needed by class-specific detectors.

## Required decisions before specification approval

- approve or revise the conservative telephone scanner boundary;
- approve an exact extension table;
- approve the IBAN whitespace, plausibility, and stopping rules;
- approve the public error and ceiling migration;
- approve the exact file and test boundary; and
- retain MI-02 as pure Human-Zone Rust with no extraction, model, vault,
  tokenization, application, or publication work.

## Recommended next phase

S1-22 through S1-22C were approved on 2026-08-17. S1-22A records the
exhaustive-enum API evolution, S1-22B defines one maximal non-empty telephone-
extension digit run with exact 1-through-6 and 7-or-more classification, and
S1-22C establishes the Section 8 10 MiB caller ceiling. The amended author
pre-audit and separate peer audit passed. The exact MI-02 implementation plan
was explicitly approved on 2026-08-17. The next gates are the clean committed
baseline and repeated offline preflight. No MI-02 code is authorized until
those remaining gates pass.
