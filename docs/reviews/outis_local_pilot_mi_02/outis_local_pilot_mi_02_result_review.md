~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-02 Result Review

Classification: `MI_02_VALIDATION_PASSED`
Date: 2026-08-18
Branch: `main`
Baseline: `da40fa21c4596dc43f240bf9ad1e68fdcbb7bcd1`
Plan: approved P1 MI-02 implementation plan amended by approved P1-04

## Result

The complete post-P1-04 validation sequence passed. The observed result is
limited to the approved dependency-free Rust telephone and IBAN transformations
and their synthetic grammar oracle. Exactly 21 library tests passed: seven
unchanged MI-01 email tests, seven telephone tests, and seven IBAN tests.

Format, Clippy, metadata, dependency-tree, lockfile, preserved-surface,
generated-inventory replay, static, line-budget, whitespace, and changed-path
checks also passed. This result does not prove complete sensitive-data
discovery, privacy, security, anonymization, extraction, application behavior,
performance, or pilot readiness.

## Completed entry gates

- The original exact implementation plan was explicitly approved on
  2026-08-17.
- Approved documentation was committed at the clean baseline above.
- P1-04 was explicitly approved on 2026-08-18. It replaces only the ASCII-
  space separator with `|` in plan Sections 10.6 and 10.7 and the two bound
  IBAN candidate-limit tests.
- The unrelated user-owned `.gitignore` change remains preserved in named Git
  stash `preserve user-owned .gitignore before P1 MI-02`.
- The exact S1-21 offline preflight passed before source mutation with Rust
  1.89.0 commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, Cargo 1.89.0 commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`, Clippy 0.1.89, rustfmt
  1.8.0-stable, host and target `aarch64-apple-darwin`, and every required
  installed component.
- The source-level pre-test audit was rerun after P1-04 and classified
  `PRE_TEST_AUDIT_PASSED` before post-P1-04 compilation or testing.

Rustup distribution endpoints were redirected to closed loopback port 9 and
Cargo offline mode was enabled for Rust-family commands. This proves local
tool resolution on this host, not general network isolation.

## Findings and corrections

### MI02-PTA-01: rejected telephone run exposed an interior suffix

The first source read found that a `+` rejected for an invalid predecessor
could expose later whitespace-separated digits for rescanning. The scanner was
corrected to consume the complete maximal body without emission for an invalid
start. The approved grammar and `é+41 44 668 18 00` no-candidate expectation
were unchanged. The pre-test audit restarted and passed.

### MI02-PTA-02: unsupported-country limit fixture formed one maximal span

The first single-thread validation run reported 19 passed and two failed. Both
failed IBAN limit tests used ASCII spaces between unsupported-country values.
The approved scanner correctly consumed each full input as one maximal
same-line span because inline whitespace belongs to that grammar.

Production behavior and the passing unsupported-prose oracle were retained.
Approved P1-04 changed only the two plan and two test separators to `|`, a
one-byte disallowed scanner character that terminates a candidate and permits
the next start. All input lengths, ranges, limits, record expectations, paths,
and claims remained unchanged.

## Failed-attempt record

Every observed failed attempt in this implementation chain is retained:

1. The first multi-file lifecycle-status patch before the baseline commit did
   not match one repository-structure context line. The patch applied no
   change; the same status-only edits were reapplied in smaller patches.
2. `MI02-PTA-01` was found during the source audit before library compilation
   or testing. The implementation was corrected without changing an oracle.
3. The first single-thread test command, before P1-04, reported 19 passed and
   two failed. Validation stopped; later commands were not run until P1-04 was
   explicitly approved and the pre-test audit passed again.
4. The first post-P1-04 read-only audit command had malformed shell quoting and
   exited at parse time before running a check or mutation. The simplified
   command then executed successfully.

No failed expectation was weakened and no production scanner was changed to
obtain the final pass.

## Final validation evidence

The complete post-P1-04 sequence ran in the approved order:

| Order | Command or check | Observed result |
|---:|---|---|
| 1 | `cargo fmt --all -- --check` | exit zero; no diff |
| 2 | `cargo clippy --locked --offline -p outis-core --lib --tests -- -D warnings` | exit zero; no warning |
| 3 | `cargo test --locked --offline -p outis-core --lib -- --list` | exactly 21 tests; zero benchmarks |
| 4 | `cargo test --locked --offline -p outis-core --lib -- --test-threads=1` | 21 passed; zero failed, ignored, measured, or filtered; 0.29 seconds |
| 5 | `cargo metadata --locked --offline --no-deps --format-version 1` | one local `outis-core` 0.1.0 library; no dependency or feature |
| 6 | `cargo tree --locked --offline -p outis-core` | only local `outis-core v0.1.0` |
| 7 | preserved-surface diff and direct lockfile checks | exit zero; format 4, one local package, no source, checksum, or dependency entry |
| 8 | strict inventory generation twice and normalized `cmp` | exit zero; outputs are byte-identical after removing only the `Generated:` line |
| 9 | inventory title, component, nine-purpose, and gap checks | exit zero; one component, nine paths, no gap |
| 10 | forbidden failure and external-surface searches | no match |
| 11 | seven line-budget assertions | exit zero |
| 12 | tracked and untracked path, whitespace, and stash checks | exit zero; exact amended path set; user stash preserved |

The final direct generated inventory records timestamp
`2026-08-18T06:04:30Z`. It remains an unedited generator output.

## Exact test result

The final test split is:

- seven byte-identical MI-01 email tests;
- seven telephone tests covering accepted and review subsets, boundaries,
  Unicode, all nine extension cues, five line endings, replay, and independent
  limit behavior; and
- seven IBAN tests covering CH, DE, FR, and IT registry examples, lowercase,
  U+2009, structure, MOD-97, supported and unsupported boundaries, prose,
  adjacency, five line endings, replay, and independent limit behavior.

The success fixtures request exactly 65,536 records. The failure fixtures
request a 65,537th record and return the shared typed all-or-nothing error with
limit 65,536. No partial vector is observable through the public return type.

Passing these synthetic tests supports only the bound grammar and records. It
does not establish recall or precision on real documents.

## Compile surface and preserved files

The following remain byte-identical to the baseline:

~~~text
Cargo.toml
Cargo.lock
rust-toolchain.toml
crates/outis-core/Cargo.toml
crates/outis-core/src/detect/email.rs
crates/outis-core/src/detect/email/tests.rs
bin/generate_global_inventory.rs
~~~

No dependency, feature, build script, manifest, lockfile package, configuration
knob, environment contract, model, database, Swift, Xcode, runtime, FFI,
extraction, vault, tokenization, export, or publication surface was added.

Formatted line counts are within the approved maxima:

| File | Observed | Maximum |
|---|---:|---:|
| `src/lib.rs` | 11 | 30 |
| `src/candidate.rs` | 48 | 80 |
| `src/detect.rs` | 40 | 80 |
| `src/detect/telephone.rs` | 227 | 300 |
| `src/detect/telephone/tests.rs` | 243 | 400 |
| `src/detect/iban.rs` | 229 | 300 |
| `src/detect/iban/tests.rs` | 257 | 400 |

## Final worktree boundary

The complete final changed-path set is exactly:

~~~text
crates/outis-core/docs/inventory.md
crates/outis-core/src/candidate.rs
crates/outis-core/src/detect.rs
crates/outis-core/src/detect/iban.rs
crates/outis-core/src/detect/iban/tests.rs
crates/outis-core/src/detect/telephone.rs
crates/outis-core/src/detect/telephone/tests.rs
crates/outis-core/src/lib.rs
docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_implementation_plan.md
docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_pre_test_audit.md
docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_result_review.md
inventory.md
~~~

The implementation-plan path is present only because approved P1-04 changed
the two bound separators after the clean baseline. The product/generated set
remains the approved nine paths. The implementation has not been committed;
the approved plan requires a separate user request before an implementation
commit.

## Supported conclusions

- The two new pure public entrypoints compiled and returned the exact records
  required by all bound synthetic cases.
- Three-run replay equality passed for telephone and IBAN.
- The two independent candidate ceilings and all-or-nothing errors passed.
- Every new range selected its exact UTF-8 `observed` slice in the bound cases.
- Accepted equality keys and review-without-key behavior passed for the bound
  classes.
- The seven unchanged MI-01 expectations passed after intentional exhaustive-
  enum evolution.
- Static evidence found no I/O, logging, external runtime surface, unchecked
  failure construct, or dependency in the approved production boundary.

## Unproved claims

The evidence does not prove:

- complete telephone, IBAN, or sensitive-entity discovery;
- national telephone-plan validity, assignment, or reachability;
- IBAN account existence or ownership;
- behavior on extracted or real documents;
- multilingual contextual-model behavior;
- privacy, security, anonymity, or suitability for confidential data;
- entity resolution, tokenization, vault isolation, agent export, or the macOS
  application workflow; or
- performance, capacity, or pilot readiness.

No benchmark or performance claim was produced.

## Classification

`MI_02_VALIDATION_PASSED`
