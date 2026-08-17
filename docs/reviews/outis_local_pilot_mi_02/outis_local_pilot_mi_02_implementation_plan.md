~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot P1 MI-02 Implementation Plan

Status: `APPROVED`
Classification: implementation planning; no code authorized by this artifact
Date: 2026-08-17
Specification: `docs/specs/outis_local_pilot_SPEC.md`, approved through S1-22C
Author pre-audit: `PASSED`
Peer audit: `PEER_AUDIT_PASSED`

## 1. Goal and authorization boundary

Implement only `MI-02`: two independent dependency-free Rust transformations
that accept one already validated UTF-8 `DocumentText` surface and its 32-byte
source-snapshot SHA-256 identity, then return ordered telephone or IBAN
`SensitiveCandidateV1` records or one typed all-or-nothing candidate-limit
error.

This plan does not authorize code. Implementation may begin only after:

1. the user explicitly approves this exact plan;
2. the approved plan and its planning-status documents are committed;
3. the worktree is clean;
4. that clean commit is recorded as `P1_MI02_BASE_COMMIT`; and
5. the complete S1-21 offline toolchain preflight passes again.

Approval authorizes only the paths, behavior, tests, commands, risks, and
rollback boundary below. A variance stops implementation and returns to the
specification or plan stage.

## 2. Scope, zones, and non-goals

MI-02 remains inside the Human Zone. Inputs and candidate records are
caller-owned private memory. The transformations perform no I/O, logging,
telemetry, persistence, network access, model execution, extraction, entity
resolution, tokenization, vault access, agent-repository access, or
publication.

MI-02 does not create:

- a combined or configurable detector;
- cross-class overlap or ordering behavior;
- extraction, path-component, model, entity, token, vault, export, application,
  Swift, Xcode, FFI, Finder, benchmark, fixture-tree, or service behavior;
- a dependency, feature, build script, manifest change, lockfile change,
  environment setting, runtime knob, compatibility layer, trait, plugin
  system, cache, thread, or future-facing abstraction; or
- a claim of complete discovery, national telephone-plan validity, account
  existence, privacy, security, anonymization, performance, or pilot
  readiness.

The Section 8 10 MiB per-document normalized UTF-8 ceiling remains the caller
contract. Neither entrypoint adds a second input-size check.

## 3. Entry gates and plan-authoring baseline

Required implementation entry gates:

- S1-22 through S1-22C remain approved.
- The MI-02 author pre-audit remains `PASSED`.
- The MI-02 peer audit remains `PEER_AUDIT_PASSED`.
- This plan is explicitly approved and committed.
- `git status --short --branch` is clean.
- `P1_MI02_BASE_COMMIT=$(git rev-parse HEAD)` records that clean commit.
- The Section 12 preflight matches every exact required identity.
- No user-owned or unrelated change is present.

Plan-authoring evidence:

- branch: `main`;
- commit: `f1801ae41ba4acad819c8292641f91c1fd5c963e`;
- current implementation surface: validated MI-01 only;
- current email oracle: seven tests;
- current worktree: documentation changes in progress plus a preserved
  user-owned `.gitignore` change; and
- current code, manifests, lockfile, toolchain, inventory, and generated
  surfaces: unchanged during planning.

The current dirty state is not an implementation baseline. It must be resolved
through an ordinary committed documentation state before code begins. The
implementation rollback identity is the future `P1_MI02_BASE_COMMIT`, not the
plan-authoring commit above.

## 4. Exact public API and compatibility contract

`SensitiveClassV1` gains exactly:

~~~text
TelephoneNumber
Iban
~~~

This is intentional exhaustive-enum API evolution. It does not claim source or
binary compatibility for downstream exhaustive matches. Current repository
source has no exhaustive match over `SensitiveClassV1`.

`candidate.rs` gains exactly:

~~~text
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StructuredDiscoveryErrorV1 {
    CandidateLimitExceeded { limit: usize },
}
~~~

`detect.rs` exposes exactly:

~~~text
pub const MAX_STRUCTURED_CANDIDATES_PER_SURFACE: usize = 65_536;

pub fn detect_telephone_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1>

pub fn detect_iban_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1>
~~~

`lib.rs` publicly re-exports only the new class variants through the existing
enum, `StructuredDiscoveryErrorV1`, the shared constant, and the two
entrypoints in addition to the unchanged MI-01 exports.

The existing `EmailDiscoveryErrorV1`,
`MAX_EMAIL_CANDIDATES_PER_SURFACE`, `detect_email_candidates`, record fields,
derives, variants, module privacy, and seven email expectations remain
unchanged. No alias, generic error, compatibility shim, duplicate record,
alternate enum version, or combined entrypoint is permitted.

Each new entrypoint has an independent 65,536-record ceiling. A requested
65,537th record returns:

~~~text
Err(StructuredDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
~~~

No partial vector is observable.

## 5. Common candidate-record contract

Every new candidate copies `SOURCE_IDENTITY = [0xA5; 32]` in tests and has:

~~~text
surface = CandidateSurfaceV1::DocumentText
path_component_index = None
evidence = CandidateEvidenceV1::StructuredGrammar
detector_version = 1
~~~

Telephone records additionally have:

~~~text
sensitive_class = SensitiveClassV1::TelephoneNumber
detector_id = "outis.telephone.e164_subset"
~~~

IBAN records additionally have:

~~~text
sensitive_class = SensitiveClassV1::Iban
detector_id = "outis.iban.swift_subset"
~~~

Every half-open byte range must be within the original `text`, begin and end on
UTF-8 scalar boundaries, and select exactly `observed`. Results are ascending
and non-nested within their class-specific call. `Accepted` records have only
the exact class equality key below. Every `NeedsReview` record has no equality
key. Neither function produces `Conflict`.

Production code uses a direct forward scan. It may allocate the required
result vector, each emitted `observed` string, and an equality key only for an
accepted record. It must not copy the complete input, create an intermediate
candidate list, normalize through locale APIs, or retain a rejected partial
result after error.

## 6. Exact implementation path ledger

The complete product and generated-path allowlist is:

| Path | Action | Exact responsibility |
|---|---|---|
| `crates/outis-core/docs/inventory.md` | modify | retain five MI-01 purposes, update the shared candidate/module purposes, and add four one-line MI-02 purposes |
| `crates/outis-core/src/lib.rs` | modify | add only the approved public re-exports |
| `crates/outis-core/src/candidate.rs` | modify | add two class variants and the new typed error |
| `crates/outis-core/src/detect.rs` | modify | add two private modules, the shared constant, two delegating entrypoints, and only genuinely shared line-boundary logic |
| `crates/outis-core/src/detect/telephone.rs` | create | telephone scanner, classification, equality key, and ceiling |
| `crates/outis-core/src/detect/telephone/tests.rs` | create | seven private telephone unit tests |
| `crates/outis-core/src/detect/iban.rs` | create | IBAN scanner, country structure, streaming MOD-97, equality key, and ceiling |
| `crates/outis-core/src/detect/iban/tests.rs` | create | seven private IBAN unit tests |
| `inventory.md` | regenerate | direct output of the existing immutable strict generator |

The implementation lifecycle may additionally create only:

- `docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_pre_test_audit.md`;
  and
- `docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_result_review.md`.

No file is removed. No other source, test, documentation, generated, root,
manifest, lockfile, toolchain, build, package, model, fixture, application, or
evidence path may change during implementation.

## 7. Preserved files, baseline hashes, and line budgets

These plan-authoring SHA-256 values document the inspected surface:

| Path | SHA-256 | Implementation rule |
|---|---|---|
| `Cargo.toml` | `c5be7f8ead089b9c4d03b6dbaceb67bfa1610c3e093f4c5f9da56b63b9a7d26a` | byte-identical |
| `Cargo.lock` | `ba6b438e2751e14a2299aa53232e83e21767a08a8e54f5d20cee3575220fc163` | byte-identical; no regeneration |
| `rust-toolchain.toml` | `5d660b0669d5123f6528cdaa959c51a202abda7bf8ef373f8ad7047391ef03f3` | byte-identical |
| `crates/outis-core/Cargo.toml` | `f592e6d4c4a04d78ba6e1a3b4c1006b4ab49cd22ac53e3475bca8eb65680f8c7` | byte-identical |
| `crates/outis-core/src/detect/email.rs` | `5ec601a31f7826e4878f1f82761652ad82070daa375f6c8aabcde1941ae1745b` | byte-identical |
| `crates/outis-core/src/detect/email/tests.rs` | `8e4b729220bda3aeb20bcd505d038c52ce4e58e464e1ecc3ab55d7e05469a4be` | byte-identical |
| `bin/generate_global_inventory.rs` | `9535d1196c2e5f5aadfae7ab27219059b2aa0eaf783eaefff848a617618f91cd` | byte-identical |

Files intentionally modified have these rollback-reference hashes:

| Path | Plan-authoring SHA-256 |
|---|---|
| `crates/outis-core/docs/inventory.md` | `8c2edce6f94305b829435bc410692b73ccb41748e2af1aacd520a76139f993dd` |
| `crates/outis-core/src/lib.rs` | `b27b2d61c788a131766cafc4339357c51b19662f561835dff40dbbfd85679537` |
| `crates/outis-core/src/candidate.rs` | `fc568f5e4d3daad847858ee71d38afe31e1ac385abc842dd95a5ae7a618dc7d6` |
| `crates/outis-core/src/detect.rs` | `d1483a77656441815e6ae7b490d13102147139f76fed67e5e2bdcab864952772` |
| `inventory.md` | `a6c534e37440abc61cb59d2418982e5ad93b6b8187f027c1c0614f4821933ae5` |

The future clean baseline is authoritative if documentation commits change
these hashes before implementation. Any product-surface difference from the
values above must be explained and rebound before code.

Maximum logical lines after formatting:

| File | Maximum |
|---|---:|
| `src/lib.rs` | 30 |
| `src/candidate.rs` | 80 |
| `src/detect.rs` | 80 |
| `src/detect/telephone.rs` | 300 |
| `src/detect/telephone/tests.rs` | 400 |
| `src/detect/iban.rs` | 300 |
| `src/detect/iban/tests.rs` | 400 |

The component inventory remains concise and contains exactly nine source
purposes. Crossing a budget stops implementation and requires plan amendment;
artificial compression or responsibility fragmentation is forbidden.

## 8. Exact handwritten and generated inventory contract

The handwritten component inventory retains its legal notice and heading and
contains exactly these nine one-physical-line entries:

~~~text
- `src/lib.rs`: intentional public domain API exports.
- `src/candidate.rs`: MI-01 and MI-02 candidate records, enums, and typed limit errors.
- `src/detect.rs`: MI-01 and MI-02 detector module ownership and public entrypoints.
- `src/detect/email.rs`: deterministic email scanner, grammar, equality key, and fixed output ceiling.
- `src/detect/email/tests.rs`: private MI-01 unit oracle.
- `src/detect/telephone.rs`: deterministic telephone scanner, classification, equality key, and fixed output ceiling.
- `src/detect/telephone/tests.rs`: private MI-02 telephone unit oracle.
- `src/detect/iban.rs`: deterministic IBAN scanner, structure, MOD-97, equality key, and fixed output ceiling.
- `src/detect/iban/tests.rs`: private MI-02 IBAN unit oracle.
~~~

The purpose text must not wrap onto Markdown continuation lines. The existing
generator remains immutable and regenerates root `inventory.md`. The generated
output must contain one `crate::outis-core` component, these exact nine paths
and complete purposes, and no generated `INVENTORY GAP` record.

The generator's timestamp is the only accepted variable. Reproducibility is
proved by normalizing only the `Generated:` line in two direct outputs and
requiring all other bytes to match.

## 9. Telephone implementation and exact oracle

The implementation follows the S1-22 scanner literally. It uses no regex,
national-plan table, locale, Unicode normalization, or telephone dependency.
The supported country-code comparison order is `33`, `39`, `41`, `49`.
Accepted equality is ASCII `+` followed by all ASCII digits in the retained
base span.

The implementation must distinguish:

- the maximal same-line telephone body;
- trailing body characters removed by the approved trimming rule;
- an optional exact extension cue plus one maximal non-empty ASCII digit run;
  and
- trailing text that prevents automatic acceptance.

The overlapping `ext` and `ext.` cues are tested as complete cue-plus-digit
forms. A cue qualifies only when its complete exact form is followed, after
optional ASCII space or U+00A0, by a non-empty ASCII digit run. A failed
shorter cue attempt must not prevent the longer `ext.` form from matching.

All telephone tests use `SOURCE_IDENTITY = [0xA5; 32]` and assert complete
records and scalar boundaries.

### 9.1 `accepted_country_subset_and_ranges_are_exact`

Each row emits one `Accepted` record.

| ID | Input | Range | Equality key |
|---|---|---:|---|
| T-A33 | `+33 1 23 45 67 89` | `0..17` | `+33123456789` |
| T-A39 | `+39.02.12345678` | `0..15` | `+390212345678` |
| T-A41 | `+41 (44) 668 18 00` | `0..18` | `+41446681800` |
| T-A49 | `+49-30-12345678` | `0..15` | `+493012345678` |
| T-NBSP | `+41 44 668 18 00` using four U+00A0 scalars | `0..20` | `+41446681800` |
| T-B08 | `+41123456` | `0..9` | `+41123456` |
| T-B15 | `+411234567890123` | `0..16` | `+411234567890123` |
| T-PUN | `Call: +41 44 668 18 00,` | `6..22` | `+41446681800` |

`observed` is exactly the selected range. T-PUN excludes the trailing comma.
The test also covers the exact 8- and 15-digit accepted boundaries.

### 9.2 `review_rejection_and_boundaries_are_exact`

These rows emit one `NeedsReview` record without an equality key:

| ID | Input | Observed range | Observed |
|---|---|---:|---|
| T-NAT | `079 123 45 67` | `0..13` | full input |
| T-CC | `+44 20 7946 0958` | `0..16` | full input |
| T-TXT | `+41 44 668 18 00abc` | `0..16` | `+41 44 668 18 00` |
| T-CUE0 | `+41 44 668 18 00 ext.` | `0..16` | `+41 44 668 18 00` |
| T-WORD | `+41 44 668 18 00extensionist 4` | `0..16` | `+41 44 668 18 00` |

These rows emit no candidate:

| ID | Input | Reason |
|---|---|---|
| T-B07 | `+4112345` | seven ASCII digits |
| T-B16 | `+4112345678901234` | sixteen ASCII digits; no interior suffix |
| T-PLAIN | `0791234567` | unformatted national digit string |
| T-NONASCII | `+٤١ ٤٤ ٦٦٨ ١٨ ٠٠` | no ASCII telephone digit run |
| T-ADJ | `é+41 44 668 18 00` | start is adjacent to a non-delimiter scalar |

Trailing ` `, `.`, `-`, and unmatched opening `(` variants are table-driven
from the base `+41 44 668 18 00`. Each emits one accepted record with
`observed` and range `0..16` and equality `+41446681800`. No trimmed character
enters the record.

### 9.3 `extension_cues_and_digit_runs_are_exact`

Every row emits one complete `NeedsReview` record spanning the full input with
no equality key:

| Cue case | Exact input | End byte |
|---|---|---:|
| `x` with U+00A0 spacing | `+41 44 668 18 00 x 1` | 22 |
| ASCII-case-insensitive `ext` | `+41 44 668 18 00 EXT 12` | 23 |
| `ext.` | `+41 44 668 18 00 ext. 123` | 25 |
| `extension` | `+41 44 668 18 00 extension 4` | 28 |
| `interno` | `+41 44 668 18 00 interno 5` | 26 |
| `int.` | `+41 44 668 18 00 int. 6` | 23 |
| `durchwahl` | `+41 44 668 18 00 durchwahl 7` | 28 |
| `dw` | `+41 44 668 18 00 dw 8` | 21 |
| `poste` | `+41 44 668 18 00 poste 9` | 24 |
| declared six-digit shape | `+41 44 668 18 00 ext 123456` | 27 |
| invalid seven-digit shape | `+41 44 668 18 00 ext. 1234567` | 29 |

The one-digit and six-digit rows cover the declared extension-shape endpoints.
The seven-digit row covers the first invalid shape. The record always retains
the maximal complete digit run; it is never truncated or split.

### 9.4 `unicode_lines_empty_and_marker_free_are_exact`

Exact assertions:

- `é +41 44 668 18 00` is 19 UTF-8 bytes and emits one accepted record at
  `3..19` with observed `+41 44 668 18 00` and equality `+41446681800`.
- Empty input returns `Ok(Vec::new())`.
- `plain text without telephone marker` returns `Ok(Vec::new())`.

For each delimiter LF, CRLF, CR, U+2028, and U+2029, the input is the exact
concatenation:

~~~text
"+41 44 668" + delimiter + "18 00"
~~~

It emits no candidate because the first line has seven ASCII digits and the
second has four. No candidate crosses the logical line. The complete UTF-8 byte
lengths are respectively 16, 17, 16, 18, and 18.

### 9.5 `replay_is_exact_across_three_runs`

The exact 32-byte input is:

~~~text
+41 44 668 18 00 | 079 123 45 67
~~~

It emits, in order:

1. accepted range `0..16`, observed `+41 44 668 18 00`, equality
   `+41446681800`; and
2. review range `19..32`, observed `079 123 45 67`, no equality key.

Three calls with the same source identity must return exactly equal vectors.

### 9.6 `candidate_limit_accepts_exact_ceiling`

The input is:

~~~text
"+41123456 ".repeat(65_535) + "+41123456"
~~~

It is 655,359 bytes and requests exactly 65,536 accepted records. Candidate
`n`, zero-based, has range `(n * 10)..(n * 10 + 9)`, observed `+41123456`,
and equality `+41123456`. The first range is `0..9` and the last is
`655350..655359`.

### 9.7 `candidate_limit_rejects_next_without_partial_output`

The input is:

~~~text
"+41123456 ".repeat(65_536) + "+41123456"
~~~

It is 655,369 bytes and requests 65,537 records. The exact return is the shared
typed error with limit 65,536. No vector is observable.

These seven named functions are the complete telephone test set.

## 10. IBAN implementation and exact oracle

The implementation uses direct ASCII classification, Rust 1.89 whitespace
semantics, and streaming MOD-97. It uses no registry dependency, regex,
Unicode normalization, big integer, cryptographic primitive, or locale API.
Only CH, DE, FR, and IT can be accepted. Equality is the uppercase
whitespace-free normalized value and is allocated only after exact structure
and MOD-97 acceptance.

All IBAN tests use `SOURCE_IDENTITY = [0xA5; 32]` and assert complete records
and scalar boundaries.

### 10.1 `registry_compact_and_print_forms_are_exact`

Each row emits one full-range `Accepted` record:

| Country/form | Input | End byte | Equality key |
|---|---|---:|---|
| CH compact | `CH9300762011623852957` | 21 | `CH9300762011623852957` |
| CH print | `CH93 0076 2011 6238 5295 7` | 26 | `CH9300762011623852957` |
| DE compact | `DE89370400440532013000` | 22 | `DE89370400440532013000` |
| DE print | `DE89 3704 0044 0532 0130 00` | 27 | `DE89370400440532013000` |
| FR compact | `FR1420041010050500013M02606` | 27 | `FR1420041010050500013M02606` |
| FR print | `FR14 2004 1010 0505 0001 3M02 606` | 33 | `FR1420041010050500013M02606` |
| IT compact | `IT60X0542811101000000123456` | 27 | `IT60X0542811101000000123456` |
| IT print | `IT60 X054 2811 1010 0000 0123 456` | 33 | `IT60X0542811101000000123456` |

The values and structures are the four public examples bound to SWIFT IBAN
Registry Release 102.

### 10.2 `lowercase_and_unicode_whitespace_normalize_exactly`

The lowercase compact CH, DE, FR, and IT strings below are accepted with the
uppercase keys from Section 10.1 and full ranges of 21, 22, 27, and 27 bytes:

~~~text
ch9300762011623852957
de89370400440532013000
fr1420041010050500013m02606
it60x0542811101000000123456
~~~

The exact thin-space input:

~~~text
CH93 0076 2011 6238 5295 7
~~~

uses five U+2009 scalars, is 36 UTF-8 bytes, emits one accepted range `0..36`,
preserves the input in `observed`, and has equality
`CH9300762011623852957`.

### 10.3 `checksum_structure_and_supported_lengths_are_exact`

Each row emits one full-range `NeedsReview` record with no equality key:

| ID | Input | End byte | Contract |
|---|---|---:|---|
| I-CHECK | `CH9400762011623852957` | 21 | wrong MOD-97 checksum |
| I-CH-S | `CH93A0762011623852957` | 21 | CH five-digit bank-position violation |
| I-DE-S | `DE89A70400440532013000` | 22 | DE digit-only violation |
| I-FR-S | `FR14A0041010050500013M02606` | 27 | FR leading ten-digit segment violation |
| I-IT-S | `IT6010542811101000000123456` | 27 | IT leading-letter violation |
| I-MIN8 | `CH00ABCD` | 8 | supported-country minimum review length |
| I-UNDER | `CH9300762011` | 12 | supported-country underlength |
| I-OVER | `CH9300762011623852957A` | 22 | contiguous supported-country overlength |

`CH00ABC` is seven normalized characters and emits no candidate.

### 10.4 `unsupported_adjacency_punctuation_and_lines_are_exact`

Unsupported-country rows emit one full-range `NeedsReview` record:

| ID | Input construction | End byte |
|---|---|---:|
| I-U15 | `GB00ABCDEFGHIJK` | 15 |
| I-U34 | `GB00` followed by 30 ASCII `A` | 34 |
| I-U35 | `GB00` followed by 31 ASCII `A` | 35 |
| I-PROSE | `GB00ABCDEFGHIJK text` | 20 |

I-PROSE deliberately locks the unsupported-country maximal same-line scan,
including inline whitespace and following ASCII letters.

Additional exact assertions:

- `xCH9300762011623852957` emits nothing because the possible start is
  preceded by ASCII alphanumeric `x`.
- `(CH9300762011623852957)` is 23 bytes and emits one accepted record at
  `1..22` with observed and equality `CH9300762011623852957`.
- `CH9300762011623852957 ordinary` emits one accepted record at `0..21`;
  whitespace after the exact supported-country length is the boundary.

For each delimiter LF, CRLF, CR, U+2028, and U+2029, the input is:

~~~text
"CH93 0076" + delimiter + "2011 6238 5295 7"
~~~

It emits exactly one `NeedsReview` record at `0..9` with observed
`CH93 0076` and no equality key. The second line does not start with two ASCII
letters plus two ASCII digits. No candidate crosses the line.

### 10.5 `empty_marker_free_and_replay_are_exact`

Empty input and `plain text without IBAN marker` each return
`Ok(Vec::new())`.

The exact replay input is:

~~~text
CH9300762011623852957|GB00ABCDEFGHIJK
~~~

It emits:

1. accepted CH range `0..21` with equality `CH9300762011623852957`; and
2. unsupported-country review range `22..37` with no equality key.

Three calls with the same identity return exactly equal vectors.

### 10.6 `candidate_limit_accepts_exact_ceiling`

The input is:

~~~text
"GB00ABCDEFGHIJK ".repeat(65_535) + "GB00ABCDEFGHIJK"
~~~

It is 1,048,575 bytes and requests exactly 65,536 review records. Candidate
`n` has range `(n * 16)..(n * 16 + 15)` and observed
`GB00ABCDEFGHIJK`. The first range is `0..15` and the last is
`1048560..1048575`.

### 10.7 `candidate_limit_rejects_next_without_partial_output`

The input is:

~~~text
"GB00ABCDEFGHIJK ".repeat(65_536) + "GB00ABCDEFGHIJK"
~~~

It is 1,048,591 bytes and requests 65,537 records. The exact result is the
shared typed error with limit 65,536 and no observable partial vector.

These seven named functions are the complete IBAN test set.

## 11. Test-count and MI-01 regression contract

After implementation the library has exactly:

- seven unchanged email tests;
- seven telephone tests named in Section 9; and
- seven IBAN tests named in Section 10.

The single-thread library test result must report exactly 21 passed, zero
failed, zero ignored, zero measured, and zero filtered out. No test may be
renamed, ignored, conditional, flaky, network-backed, clock-backed, or
dependent on process-global state.

`crates/outis-core/src/detect/email.rs` and
`crates/outis-core/src/detect/email/tests.rs` remain byte-identical to
`P1_MI02_BASE_COMMIT`. Running the seven MI-01 tests unchanged is the complete
email regression contract.

## 12. Exact offline toolchain preflight

Run after recording `P1_MI02_BASE_COMMIT` and before any Cargo, rustc, rustfmt,
or Clippy operation:

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

Accept only:

- rustc 1.89.0, commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, host
  `aarch64-apple-darwin`;
- Cargo 1.89.0, commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`, host
  `aarch64-apple-darwin`;
- Clippy 0.1.89 commit `29483883ee`;
- rustfmt 1.8.0-stable commit `29483883ee`;
- installed cargo, Clippy, rustc, rustfmt, and arm64 Rust standard-library
  components; and
- installed `aarch64-apple-darwin` target.

Additional components and targets are allowed. Any missing or mismatched
required item stops implementation. The loopback endpoints and Cargo offline
mode prove only that remote toolchain resolution was not required on this host.

## 13. Ordered implementation procedure

1. Confirm explicit approval of this exact plan.
2. Commit the approved plan and planning-status documents, require a clean
   worktree, record branch and `P1_MI02_BASE_COMMIT`, and verify every
   plan-authoring product hash against the clean baseline.
3. Run and record the complete Section 12 preflight. Stop on any mismatch.
4. Modify `candidate.rs` with only the two class variants and typed error.
5. Modify `detect.rs` and `lib.rs` with only the approved modules, shared
   constant, delegating entrypoints, intentional exports, and minimal shared
   logical-line helper.
6. Create `telephone.rs` and its private test module. Run no test yet.
7. Create `iban.rs` and its private test module. Run no test yet.
8. Update the handwritten component inventory with the exact nine lines.
9. Run `cargo fmt --all` as the sole mechanical source rewrite, then read every
   formatted file and verify all line budgets.
10. Compile the immutable inventory generator, generate `inventory.md`
    strictly, and inspect the direct output.
11. Author the MI-02 pre-test audit. It must inspect the complete diff against
    `P1_MI02_BASE_COMMIT` and classify exactly `PRE_TEST_AUDIT_PASSED` or
    `BLOCKED`. A blocked result stops before compilation or tests.
12. If the pre-test audit passes, execute Section 14 in order. Stop at the
    first failure and diagnose without weakening a test or expectation.
13. Author the result review from observed command outputs, including every
    failed attempt. Classify only the bound MI-02 result.
14. Re-read every changed file, rerun final path and whitespace checks, and
    report the exact worktree. Do not commit implementation unless the user
    separately requests it.

No parallel mutation is permitted because later steps depend on the exact
artifact produced by earlier steps.

## 14. Validation commands and expected outputs

Every Rust-family command uses the Section 12 environment.

### 14.1 Format, lint, test list, and unit oracle

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
  -p outis-core --lib -- --list
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo test --locked --offline \
  -p outis-core --lib -- --test-threads=1
~~~

Expected:

- every command exits zero;
- format produces no diff;
- Clippy produces no warning;
- test listing contains exactly the seven unchanged email names and the
  fourteen names in Sections 9 and 10;
- the test run reports exactly 21 passed and no failed, ignored, measured, or
  filtered test; and
- no network or process-global input is required.

### 14.2 Workspace, dependency, and preserved-surface checks

~~~text
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo metadata --locked --offline \
  --no-deps --format-version 1
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo tree --locked --offline -p outis-core
git diff --quiet "$P1_MI02_BASE_COMMIT" -- \
  Cargo.toml Cargo.lock rust-toolchain.toml \
  crates/outis-core/Cargo.toml \
  crates/outis-core/src/detect/email.rs \
  crates/outis-core/src/detect/email/tests.rs \
  bin/generate_global_inventory.rs
~~~

Expected: one `outis-core` 0.1.0 library target, no dependency or feature,
tree output containing only `outis-core v0.1.0`, and no diff in any listed
preserved file.

Direct lockfile checks:

~~~text
rg -n '^version = 4$|^name = "outis-core"$|^version = "0.1.0"$' Cargo.lock
test "$(rg -c '^\[\[package\]\]$' Cargo.lock)" -eq 1
! rg -n '^(source|checksum|dependencies) = ' Cargo.lock
~~~

Expected: format four, one local package, and no source, checksum, or dependency
entry.

### 14.3 Inventory generation and replay

~~~text
mkdir -p target/tools
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true rustc -O bin/generate_global_inventory.rs \
  -o target/tools/generate_global_inventory_mi02
target/tools/generate_global_inventory_mi02 --repo-root "$(pwd)" \
  --out "$(pwd)/inventory.md" --strict
cp inventory.md target/tools/inventory.mi02.first.md
target/tools/generate_global_inventory_mi02 --repo-root "$(pwd)" \
  --out "$(pwd)/inventory.md" --strict
cp inventory.md target/tools/inventory.mi02.second.md
sed '/^Generated:/d' target/tools/inventory.mi02.first.md \
  > target/tools/inventory.mi02.first.normalized.md
sed '/^Generated:/d' target/tools/inventory.mi02.second.md \
  > target/tools/inventory.mi02.second.normalized.md
cmp target/tools/inventory.mi02.first.normalized.md \
  target/tools/inventory.mi02.second.normalized.md
~~~

Expected: strict generation exits zero twice; normalized outputs are
byte-identical; the tracked file is the unedited direct second output.

Validate:

~~~text
rg -F '# `outis` - Global Inventory (GENERATED; DO NOT EDIT)' inventory.md
test "$(rg -c '^- `crate::outis-core`$' inventory.md)" -eq 1
test "$(rg -c '^- `src/.*`:' crates/outis-core/docs/inventory.md)" -eq 9
! rg -F ': INVENTORY GAP (add 1-line purpose in ' inventory.md
~~~

Expected output contains the title, one component marker, all nine exact paths
and complete purposes, and no gap record.

### 14.4 Failure-surface, external-surface, and line-budget audit

~~~text
! rg -n 'unwrap\(|expect\(|panic!|todo!|unimplemented!|unreachable!|unsafe|dbg!|println!|eprintln!|#\[allow' \
  crates/outis-core/src/lib.rs \
  crates/outis-core/src/candidate.rs \
  crates/outis-core/src/detect.rs \
  crates/outis-core/src/detect/telephone.rs \
  crates/outis-core/src/detect/telephone/tests.rs \
  crates/outis-core/src/detect/iban.rs \
  crates/outis-core/src/detect/iban/tests.rs
! rg -n 'std::(fs|net|env|process|thread|time)|extern crate|cfg\(feature|include_bytes!|include_str!' \
  crates/outis-core/src/lib.rs \
  crates/outis-core/src/candidate.rs \
  crates/outis-core/src/detect.rs \
  crates/outis-core/src/detect/telephone.rs \
  crates/outis-core/src/detect/iban.rs
~~~

Expected: no match. Production code must also contain no filesystem, clock,
randomness, locale, model, database, platform, FFI, process, or thread access.

Line budgets:

~~~text
test "$(wc -l < crates/outis-core/src/lib.rs)" -le 30
test "$(wc -l < crates/outis-core/src/candidate.rs)" -le 80
test "$(wc -l < crates/outis-core/src/detect.rs)" -le 80
test "$(wc -l < crates/outis-core/src/detect/telephone.rs)" -le 300
test "$(wc -l < crates/outis-core/src/detect/telephone/tests.rs)" -le 400
test "$(wc -l < crates/outis-core/src/detect/iban.rs)" -le 300
test "$(wc -l < crates/outis-core/src/detect/iban/tests.rs)" -le 400
~~~

Expected: all assertions exit zero.

### 14.5 Changed-path and whitespace audit

~~~text
git diff --check
git diff --cached --check
git status --short --untracked-files=all
git status --porcelain=v1 --untracked-files=all | cut -c4- | LC_ALL=C sort
git diff --name-status "$P1_MI02_BASE_COMMIT" --
~~~

Before the result review exists, the sorted changed-path set is exactly:

~~~text
crates/outis-core/docs/inventory.md
crates/outis-core/src/candidate.rs
crates/outis-core/src/detect.rs
crates/outis-core/src/detect/iban.rs
crates/outis-core/src/detect/iban/tests.rs
crates/outis-core/src/detect/telephone.rs
crates/outis-core/src/detect/telephone/tests.rs
crates/outis-core/src/lib.rs
docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_pre_test_audit.md
inventory.md
~~~

After the result review is authored, the same set contains exactly one
additional path:

~~~text
docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_result_review.md
~~~

The name-status command is a tracked-path cross-check and omits untracked
files. The porcelain path set is the complete oracle. Both whitespace checks
must exit zero.

## 15. Pre-test audit and result-review contracts

The pre-test audit is findings-first and checks:

- exact path allowlist and responsibilities;
- public API, enum evolution, module privacy, and unchanged MI-01 surface;
- telephone and IBAN grammar-to-code traceability;
- exact 14 new test names and table bindings;
- typed all-or-nothing resource failure;
- Human-Zone-only execution and absence of I/O or logging;
- standard-library-only dependency and compile surface;
- inventory source and generated ownership;
- forbidden constructs, line budgets, and no speculative abstraction; and
- claim limits.

It classifies exactly `PRE_TEST_AUDIT_PASSED` or `BLOCKED`. Testing does not
begin on `BLOCKED`.

The result review records:

- baseline commit, branch, final dirty state, and changed paths;
- exact toolchain identities and offline-resolution result;
- format, Clippy, test listing, 21-test output, metadata, tree, lockfile,
  inventory replay, static audits, and line-budget outputs;
- every failed attempt and correction;
- preserved MI-01 hashes and regression result;
- supported deterministic conclusions only for the bound synthetic grammar;
  and
- unproved extraction, coverage, privacy, security, performance, and product
  claims.

No benchmark or performance result is produced.

## 16. Failure and rollback boundary

Stop immediately if:

- a required product change falls outside the nine-path ledger;
- a grammar or expected record conflicts with S1-22 through S1-22C;
- an exact test cannot be implemented without changing its expectation;
- MI-01 source or expectation changes;
- a manifest, lockfile, dependency, feature, toolchain, generator, or build
  change appears;
- code requires I/O, unsafe Rust, unchecked failure, a general framework, or a
  new configuration surface;
- the candidate ceiling can expose a partial vector;
- the inventory requires manual editing or generator modification; or
- any validation command fails.

Rollback is bounded to the nine product/generated paths:

1. preserve the pre-test or result-review evidence of the failure;
2. use file-specific `apply_patch` edits to reverse the four modified source
   and inventory-input files;
3. use file-specific `apply_patch` deletions for only the four newly created
   detector/test files;
4. regenerate `inventory.md` from the restored handwritten inventory using the
   immutable generator;
5. compare the restored product surface with `P1_MI02_BASE_COMMIT`; and
6. verify that only the lifecycle failure artifact remains changed.

Do not use `git reset --hard`, `git checkout --`, `git clean`, a recursive
delete, glob deletion, or an unvalidated rollback target. No source document,
vault, agent repository, user data, dependency cache, or unrelated file is
inside the rollback boundary.

## 17. Risks and bounded responses

| Risk | Bound response |
|---|---|
| Telephone false positives from formatted identifiers | national forms and unsupported codes remain `NeedsReview`; no completeness claim |
| Telephone false negatives for unformatted national values | explicit no-candidate contract and test |
| Telephone cue overlap | complete cue-plus-digit matching and exact `ext`/`ext.` tests |
| Unicode byte-range error | every record asserts exact observed slice and scalar boundaries |
| IBAN scan absorbs unsupported-country prose | exact I-PROSE review test preserves the declared risk |
| False acceptance from structure alone | accepted subset also requires streaming MOD-97 remainder one |
| MOD-97 overflow | digit-by-digit remainder; no large integer construction |
| Excess candidate allocation | independent fixed ceilings and all-or-nothing error tests |
| Public exhaustive-enum break | intentional API evolution disclosed; workspace compilation and MI-01 regression required |
| Regression hidden by test migration | email source and seven-test file remain byte-identical |
| Compile-surface growth | no dependency, feature, manifest, lockfile, build, or crate change |
| Generic abstraction added for future detectors | forbidden; direct class-specific modules only |
| Generated inventory drift | immutable generator, strict mode, normalized two-run replay |
| Overstated result | result review limits claims to the exact synthetic grammar and recorded commands |

## 18. Explicit approval gate

This plan was explicitly approved by the user on 2026-08-17.

Implementation remains forbidden until the approved documentation is committed
on a clean baseline and the S1-21 preflight passes again. Approval does not
authorize any path, behavior, test migration, dependency, artifact, model,
application surface, or claim not listed here.
