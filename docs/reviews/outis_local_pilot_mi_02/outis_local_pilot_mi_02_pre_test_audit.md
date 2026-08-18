~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-02 Pre-Test Audit

Classification: `PRE_TEST_AUDIT_PASSED`
Date: 2026-08-18
Branch: `main`
Baseline: `da40fa21c4596dc43f240bf9ad1e68fdcbb7bcd1`
Target: approved P1 MI-02 boundary amended by approved P1-04
Post-P1-04 compilation and unit-test status: not run before this rerun classification

## Result

The corrected implementation and four approved P1-04 literal substitutions
match the MI-02 path, API, grammar, failure, compile-surface, inventory, and
oracle bindings closely enough to restart the ordered validation commands.
The audit retains the closed telephone resume-path finding and closes the IBAN
limit-fixture contradiction exposed by the first validation attempt. No
production behavior, expectation, grammar, path, dependency, or limit was
weakened.

This classification authorizes only the validation sequence in the approved
plan. It does not prove runtime correctness, complete discovery, privacy,
security, anonymization, performance, or pilot readiness.

## Entry evidence

- The exact implementation plan was explicitly approved on 2026-08-17.
- P1-04 was explicitly approved on 2026-08-18. It changes only the separator
  in Sections 10.6 and 10.7 and the two bound IBAN limit-test inputs from ASCII
  space to `|`.
- Approved documentation commit and `P1_MI02_BASE_COMMIT`:
  `da40fa21c4596dc43f240bf9ad1e68fdcbb7bcd1`.
- The baseline worktree was clean. The unrelated user-owned `.gitignore`
  change was preserved in named Git stash
  `preserve user-owned .gitignore before P1 MI-02`.
- The complete S1-21 offline preflight passed before source mutation:
  Rust 1.89.0 commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, Cargo 1.89.0 commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`, Clippy 0.1.89, rustfmt
  1.8.0-stable, host and target `aarch64-apple-darwin`, and every required
  installed component.
- Rustup distribution endpoints were redirected to closed loopback port 9 and
  Cargo offline mode was enabled. This proves only local tool resolution on
  this host.

## Finding and correction

### MI02-PTA-01: rejected telephone run could expose an interior suffix

Initial status: blocking.

The first source read found that a `+` adjacent to a non-delimiter scalar was
rejected at that byte but its later whitespace-separated digits could be
rescanned as a national-form candidate. That contradicted the approved
no-interior-suffix rule and the exact `é+41 44 668 18 00` no-candidate oracle.

Correction: the scanner now consumes the complete maximal telephone body for
every potential `+` or ASCII-digit start. A valid predecessor invokes
classification; an invalid predecessor advances past the same body without
emission. The test expectation and approved grammar were unchanged. The file
was formatted and reread, strict inventory generation was repeated, and the
complete pre-test audit restarted.

Final status: closed.

### MI02-PTA-02: unsupported-country limit fixture formed one maximal span

Initial status: blocking, observed by the first single-thread validation run.

The original two IBAN limit fixtures separated unsupported-country values with
ASCII space. The approved scanner consumes ASCII letters, ASCII digits, and
inline whitespace as one maximal same-line unsupported-country span. Each
fixture therefore requested one record, not 65,536 or 65,537 records. Splitting
on spaces would have contradicted the passing unsupported-prose oracle.

Correction: approved P1-04 replaces only those two plan literals and two test
literals with the one-byte `|` separator. `|` terminates the span and is not
ASCII alphanumeric, so the next start remains eligible. The approved input
lengths, ranges, record fields, limit, scanner behavior, and all other tests
remain unchanged. The complete pre-test audit was rerun before any post-P1-04
compilation or test.

Final status: closed.

## Path and ownership audit

The product and generated path set is exactly:

~~~text
crates/outis-core/docs/inventory.md
crates/outis-core/src/candidate.rs
crates/outis-core/src/detect.rs
crates/outis-core/src/detect/iban.rs
crates/outis-core/src/detect/iban/tests.rs
crates/outis-core/src/detect/telephone.rs
crates/outis-core/src/detect/telephone/tests.rs
crates/outis-core/src/lib.rs
inventory.md
~~~

The approved implementation-plan amendment, this pre-test audit, and the result
review are the only additional planning or lifecycle paths. `git status
--porcelain=v1 --untracked-files=all` and the diff against the baseline show no
other product, generated, manifest, lockfile, toolchain, build, fixture, model,
Swift, Xcode, runtime, FFI, vault, tokenization, export, or publication path.

## API and compatibility audit

- `SensitiveClassV1` gains only `TelephoneNumber` and `Iban`.
- `StructuredDiscoveryErrorV1` has only the approved typed candidate-limit
  variant.
- `MAX_STRUCTURED_CANDIDATES_PER_SURFACE` is exactly 65,536.
- The two public entrypoints have the exact approved names, parameters, and
  result type.
- `lib.rs` re-exports only the approved additions.
- The exhaustive enum change is intentional. No compatibility shim, duplicate
  enum, alternate record, combined detector, trait, feature, configuration, or
  future-facing abstraction was added.
- The MI-01 email implementation, private seven-test oracle, API, error, and
  limit are byte-identical to the baseline.

## Telephone traceability

The source implements the approved start delimiters, five logical-line
boundaries, ASCII-digit counting, exact body alphabet, trailing trimming,
8-through-15 digit interval, formatted-national requirement, accepted
`33`/`39`/`41`/`49` subset, equality key, direct trailing-text review rule,
nine extension cues, maximal non-empty extension digit run, and no-interior-
suffix resume behavior.

The seven private tests bind complete records for the approved country,
national, unsupported-code, punctuation, adjacency, Unicode, extension,
boundary, replay, empty, marker-free, line-ending, 65,536-record success, and
65,537th-record failure cases. Every accepted record has its exact equality
key; every review record has none.

## IBAN traceability

The source implements the approved contiguous start, ASCII predecessor rule,
Rust 1.89 inline-whitespace behavior, five logical-line boundaries, exact
CH/DE/FR/IT lengths and structures, supported-country stopping, contiguous
overlength handling, unsupported-country maximal scan, minimum review lengths,
ASCII uppercase normalization, streaming MOD-97, and equality key.

The seven private tests bind compact and print registry examples, lowercase,
U+2009, checksum and structure failures, supported underlength and overlength,
unsupported 15/34/35-character boundaries, unsupported prose, adjacency,
punctuation, line endings, replay, empty, marker-free, `|`-separated
65,536-record success, and 65,537th-record failure cases.

## Failure, trust-zone, and allocation audit

- Both entrypoints return an all-or-nothing typed error before retaining a
  65,537th record. A partial vector is not returned.
- Byte ranges select `observed` directly from the input and remain scalar
  aligned by construction and oracle.
- Accepted equality keys are allocated only after class-specific acceptance.
- The transformations perform no I/O, logging, telemetry, persistence,
  filesystem, network, environment, clock, randomness, locale, model,
  database, process, or thread access.
- Inputs and outputs remain caller-owned Human-Zone memory. No source value or
  candidate reaches a vault, agent repository, model, or service.
- No `unwrap`, `expect`, `panic!`, placeholder panic, `unsafe`, debug print, or
  unchecked suppression appears in the approved source and test boundary.

## Compile surface and inventory

Cargo manifests, `Cargo.lock`, `rust-toolchain.toml`, the email source and
tests, and the inventory generator are byte-identical to the baseline hashes
bound by the plan. The handwritten inventory has exactly nine complete
one-line purposes. Strict generation produced one `outis-core` component, all
nine source paths, and no `INVENTORY GAP` record.

Formatted logical line counts remain below every approved maximum:

| File | Observed | Maximum |
|---|---:|---:|
| `src/lib.rs` | 11 | 30 |
| `src/candidate.rs` | 48 | 80 |
| `src/detect.rs` | 40 | 80 |
| `src/detect/telephone.rs` | 227 | 300 |
| `src/detect/telephone/tests.rs` | 243 | 400 |
| `src/detect/iban.rs` | 229 | 300 |
| `src/detect/iban/tests.rs` | 257 | 400 |

The test-source count is exactly seven unchanged email tests, seven telephone
tests, and seven IBAN tests. This is a source audit, not a test result.

## Remaining validation

The following remain unproved until their exact commands restart after P1-04
and are recorded:

- compilation and Clippy acceptance;
- the exact 21-test listing and single-thread result;
- runtime agreement with every literal record oracle;
- metadata, dependency-tree, and lockfile results;
- two-run normalized inventory replay;
- final static, line-budget, changed-path, and whitespace checks; and
- the final MI-02 result classification.

## Classification

`PRE_TEST_AUDIT_PASSED`
