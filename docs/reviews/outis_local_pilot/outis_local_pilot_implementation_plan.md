~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot P1 MI-01 Implementation Plan

Status: `APPROVED_P1_02`
Classification: implementation planning; no code authorized by this artifact
Date: 2026-08-17
Specification: `docs/specs/outis_local_pilot_SPEC.md`, approved through S1-21
Peer audit: `PEER_AUDIT_PASSED`
Prior approval: explicitly granted by the user on 2026-08-17
P1-01 amendment: inventory component-marker validation corrected after
code-read falsification
P1-01 approval: explicitly granted by the user on 2026-08-17
P1-02 amendment: full inventory-purpose emission corrected after the blocked
pre-test audit
P1-02 approval: explicitly granted by the user on 2026-08-17

## 1. Goal and authorization boundary

Implement only S1-20 `MI-01`: a dependency-free Rust library transformation
that accepts an already validated UTF-8 document-text surface and its 32-byte
source-snapshot SHA-256 identity, then returns the ordered email subset of
`SensitiveCandidateV1` or the specified typed candidate-limit error.

This plan does not authorize code until the user explicitly approves it and
the approved plan is committed on a clean baseline. Approval authorizes only
the paths, behavior, commands, and risks bound below. Any variance stops work
and returns to planning.

This increment is not an application, extractor, contextual detector,
anonymizer, token vault, tokenization pipeline, exporter, publisher, or funding
demo. It proves neither complete discovery nor privacy, security, performance,
or production readiness.

## 2. Entry gates and evidence baseline

All of these gates must hold immediately before implementation:

1. S1-21 remains approved and its separate peer audit remains
   `PEER_AUDIT_PASSED`.
2. This plan is explicitly approved and committed.
3. `git status --short --branch` is clean at the approved-plan commit.
4. That commit is recorded as `P1_BASE_COMMIT` before any implementation edit.
5. The exact S1-21 offline toolchain preflight in Section 10 passes.
6. No unrelated user change is present. If one appears, stop rather than hide,
   overwrite, stage, or include it.

Plan-authoring baseline observed before this artifact was created:

- branch: `main`;
- commit: `7ee08989db924e45449d0c4a3eb47c3bafd9fd46`;
- relation: `main` and `origin/main` both resolved to that commit; and
- worktree: clean.

The implementation rollback identity is not the plan-authoring baseline. It is
the future clean commit containing the approved plan.

## 3. Capability and public API

The only public function is:

~~~text
pub fn detect_email_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, EmailDiscoveryErrorV1>
~~~

The public record contains exactly these fields:

~~~text
source_snapshot_sha256: [u8; 32]
surface: CandidateSurfaceV1
path_component_index: Option<u32>
start_byte: usize
end_byte: usize
sensitive_class: SensitiveClassV1
observed: String
equality_key: Option<String>
detector_id: &'static str
detector_version: u16
evidence: CandidateEvidenceV1
status: CandidateStatusV1
~~~

The only variants and constant are:

~~~text
CandidateSurfaceV1::DocumentText
SensitiveClassV1::Email
CandidateEvidenceV1::StructuredGrammar
CandidateStatusV1::{Accepted, NeedsReview}
EmailDiscoveryErrorV1::CandidateLimitExceeded { limit: usize }
MAX_EMAIL_CANDIDATES_PER_SURFACE = 65_536
~~~

The struct and every enum derive exactly `Clone`, `Debug`, `Eq`, and
`PartialEq`. `lib.rs` publicly re-exports only the record, four candidate
enums, typed error, constant, and entrypoint. The `candidate`, `detect`, and
`email` modules remain private.

## 4. Deterministic transformation

The implementation performs one forward `text.char_indices()` scan and forms
maximal non-empty spans delimited by Rust 1.89 `char::is_whitespace`. It ignores
spans without ASCII `@`. Each remaining span emits exactly one record in input
byte order unless a 65,537th record would be emitted, in which case the
function returns only:

~~~text
Err(EmailDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
~~~

No partial vector is observable on error.

Every emitted record copies the input source identity and has:

~~~text
surface = CandidateSurfaceV1::DocumentText
path_component_index = None
sensitive_class = SensitiveClassV1::Email
detector_id = "outis.email.ascii"
detector_version = 1
evidence = CandidateEvidenceV1::StructuredGrammar
~~~

Its half-open range selects exactly `observed` and ends on UTF-8 scalar
boundaries. Grammar success produces `Accepted` plus an equality key that
preserves the local part and ASCII-lowercases the domain. Grammar failure
produces `NeedsReview` and no equality key.

Automatic acceptance requires all of:

- ASCII input and at most 254 total bytes;
- exactly one ASCII `@`;
- a 1 through 64 byte local part containing only ASCII letters, digits, and
  `!#$%&'*+/=?^_{|}~-.`;
- no leading, trailing, or repeated local dot;
- domain labels of 1 through 63 ASCII bytes containing only letters, digits,
  and internal hyphens;
- no leading or trailing label hyphen; and
- a 2 through 63 byte ASCII-alphabetic final label.

S1-20 does not require a dot in the domain. Therefore `a@com` is accepted.
This is a locked contract, not an implementation inference.

The function performs no I/O, logging, networking, model use, database access,
threading, clock access, randomness, locale lookup, environment lookup, or
unsafe operation. It adds no second 16-MiB input check because that bound is a
caller contract outside MI-01.

## 5. Exact implementation path ledger

The following table is the complete product and root path allowlist.

| Path | Action | Exact responsibility or result | Necessity |
|---|---|---|---|
| `rust-toolchain.toml` | create | exact S1-21 stable alias, minimal profile, Clippy, rustfmt, arm64 target | makes the approved toolchain contract repository-visible |
| `Cargo.toml` | replace | one-member resolver-3 workspace and shared package/release settings from S1-20 | removes the unrelated binary package and binds the first library member |
| `Cargo.lock` | regenerate and commit | format 4; only `outis-core` 0.1.0; no registry source or checksum | reproducible locked offline Cargo surface |
| `.gitignore` | edit | remove only the exact `Cargo.lock` line | permits the required lockfile commit |
| `src/main.rs` | remove | no root executable remains | removes the unrelated Hello World binary scaffold |
| `Makefile.toml` | remove | no stale `mbt_cache` tasks remain | prevents an unrelated command surface |
| `release.toml` | remove | no stale `mbt-cache` release configuration remains | prevents an unrelated release surface |
| `crates/outis-core/Cargo.toml` | create | exact dependency-free library manifest from S1-20 | owns the complete MI-01 compile unit |
| `crates/outis-core/docs/inventory.md` | create | handwritten legal notice, heading, and exactly five source-purpose entries | supplies the existing strict inventory generator contract |
| `crates/outis-core/src/lib.rs` | create | private modules and exact public re-exports only | defines the stable capability boundary |
| `crates/outis-core/src/candidate.rs` | create | exact record, enum variants, derives, and typed limit error | owns MI-01 domain data only |
| `crates/outis-core/src/detect.rs` | create | private email module, public constant, and delegating entrypoint | owns detector entry and fixed ceiling |
| `crates/outis-core/src/detect/email.rs` | create | scanner, grammar validation, equality construction, and ceiling enforcement | owns the deterministic transformation |
| `crates/outis-core/src/detect/email/tests.rs` | create | exact private unit oracle in Section 8 | proves only the bound transformation |
| `inventory.md` | regenerate | strict global inventory generated by the existing repository tool | records the changed repository surface |

No other product, root configuration, build, or generated path may change.
The implementation may additionally create only these S1 Section 39 lifecycle
artifacts:

- `docs/reviews/outis_local_pilot/outis_local_pilot_pre_test_audit.md`; and
- `docs/reviews/outis_local_pilot/outis_local_pilot_result_review.md`.

The approved implementation-plan artifact already exists at this plan's path
and must be committed before implementation. `ROADMAP.json`, specifications,
protocols, architecture documents, and the file-architecture JSON do not change
during MI-01 implementation.

## 6. Exact removals and preserved files

Removal is limited to `src/main.rs`, `Makefile.toml`, and `release.toml`.
Implementation uses `apply_patch`; it does not use a broad delete, glob,
`git clean`, checkout, or reset.

`bin/generate_global_inventory.rs` is executed but not modified. Existing
documentation, hooks, repository metadata, and all user-owned files not named
in Section 5 are preserved.

The old ignored lockfile is replaced through Cargo generation, not manual
editing. Its planning-baseline identity was:

~~~text
Cargo.lock
SHA-256 3b1417200a300593e389a517866b6890e778aac199879d81202492a33fab97ec
~~~

The root files used for rollback verification had these planning-baseline
SHA-256 identities:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `9710dcb2721c6dda84e720cd6a3c6e1c3b2663dbcf81d46712eedaadac7fccee` |
| `.gitignore` | `c11c05ee998c6b2717cf66af78bc6633462192b2155512042477c69be2f06161` |
| `src/main.rs` | `c8e0583694bb1e0188dbe28fe0d65ac1130723c55f968b6262b906c147f72549` |
| `Makefile.toml` | `563f4149490f9c1be446e0d7cf8225c292f7ee942306055f5068aa6b41d0f279` |
| `release.toml` | `1ea8f013fd19c302948b03d7bd5d0054b9372aa6f6f97e73c4a2fb61f2d4b649` |
| `inventory.md` | `a6964a033bac6822ce64032109fa783f3e035bde22ac0fbecfcfa3287d4d72ed` |
| `bin/generate_global_inventory.rs` | `9535d1196c2e5f5aadfae7ab27219059b2aa0eaf783eaefff848a617618f91cd` |

These hashes document the observed authoring baseline. Rollback uses the
recorded `P1_BASE_COMMIT`, not copied file content from this table.

## 7. Manifest, toolchain, and compile-surface bindings

The three manifest/toolchain files use the complete content already fixed in
S1-20. No additional section, metadata, feature, example, binary, benchmark,
build script, dependency, dev-dependency, or workspace member is allowed.

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

The handwritten component inventory has its legal notice, the `outis-core`
heading, and exactly these entries:

~~~text
- `src/lib.rs`: intentional public domain API exports.
- `src/candidate.rs`: MI-01 candidate records, enums, and typed limit error.
- `src/detect.rs`: MI-01 detector module ownership and public entrypoint.
- `src/detect/email.rs`: deterministic email scanner, grammar, equality key, and fixed output ceiling.
- `src/detect/email/tests.rs`: private MI-01 unit oracle.
~~~

Production and test code use the Rust standard library only. The expected
workspace has one package and one library target. Approximate review budgets
are:

| Handwritten Rust file | Maximum lines |
|---|---:|
| `src/lib.rs` | 30 |
| `src/candidate.rs` | 100 |
| `src/detect.rs` | 50 |
| `src/detect/email.rs` | 220 |
| `src/detect/email/tests.rs` | 400 |

Exceeding a budget requires stopping and amending the plan. The budgets are
scope controls, not quality or performance claims.

## 8. Exact unit-test bindings

All tests are private unit tests in
`crates/outis-core/src/detect/email/tests.rs`. They use the fixed source
identity `[0xA5; 32]`, perform no external I/O, and are neither ignored nor
conditional. Helpers may construct records and repeated boundary strings but
may not add a public API.

For every emitted candidate, the tests assert the complete record: source
identity, surface, absent path index, byte range, observed value, class,
detector identifier and version, evidence, status, equality key, and UTF-8
scalar-boundary validity.

### 8.1 `accepted_ascii_grammar_has_exact_records`

This test executes every row below independently. Each result is exactly one
`Accepted` record covering the full input from byte zero to the stated end.

| ID | Input | End byte | Equality key |
|---|---|---:|---|
| A01 | `a@example.com` | 13 | `a@example.com` |
| A02 | `A.B+tag@Sub.Example.COM` | 23 | `A.B+tag@sub.example.com` |
| A03 | `a!#$%&'*+/=?^_{\|}~-b@example.com` | 32 | identical to input |
| A04 | `a@com` | 5 | `a@com` |
| A05 | `a@b.co` | 6 | `a@b.co` |
| A06 | `a@`, then 63 ASCII `b` | 65 | identical to input |
| A07 | 64 ASCII `a`, then `@example.com` | 76 | identical to input |
| A08 | `a@`, 63 ASCII `b`, then `.com` | 69 | identical to input |
| A09 | 64 `a`, `@`, 63 `b`, `.`, 63 `c`, `.`, and 61 `d` | 254 | identical to input |

A02 locks internal local dots, plus, multiple domain labels, and domain-only
case folding. A03 covers every allowed local punctuation except dot, which A02
covers. A04 explicitly locks the spec's single-label-domain behavior. A05
locks the one-byte non-final and two-byte final-label minima. A06 locks the
63-byte final-label maximum. A07, A08, and A09 lock the accepted local,
non-final-label, and total byte maxima.

### 8.2 `invalid_ascii_and_unsupported_forms_need_review`

Each row emits exactly one full-span `NeedsReview` record with no equality key.

| ID | Input construction | End byte | Contract exercised |
|---|---|---:|---|
| R01 | `.a@example.com` | 14 | leading local dot |
| R02 | `a.@example.com` | 14 | trailing local dot |
| R03 | `a..b@example.com` | 16 | repeated local dot |
| R04 | `a@@example.com` | 14 | repeated `@` |
| R05 | `@example.com` | 12 | empty local part |
| R06 | `a@` | 2 | empty domain |
| R07 | `a@-example.com` | 14 | leading domain hyphen |
| R08 | `a@example-.com` | 14 | trailing domain hyphen |
| R09 | `a@exam_ple.com` | 14 | invalid domain ASCII |
| R10 | `a@example.12` | 12 | non-alphabetic final label |
| R11 | `a@example.c` | 11 | one-byte final label |
| R12 | `a@exa!mple.com` | 14 | invalid domain punctuation |
| R13 | `"a"@example.com` | 15 | quoted local form |
| R14 | `a(comment)@example.com` | 22 | commented form |
| R15 | `ä@example.com` | 14 | internationalized local form |
| R16 | `a@exämple.com` | 14 | internationalized domain form |
| R17 | `a@[127.0.0.1]` | 13 | domain literal |
| R18 | `a@.example.com` | 14 | empty first domain label |
| R19 | `a@example..com` | 14 | empty internal domain label |
| R20 | `a@example.com.` | 14 | empty final domain label |
| R21 | `a@example.com,` | 14 | maximal non-whitespace punctuation |
| R22 | 65 `a`, then `@example.com` | 77 | local part over limit |
| R23 | `a@`, 64 `b`, then `.com` | 70 | domain label over limit |
| R24 | 64 `a`, `@`, 63 `b`, `.`, 63 `c`, `.`, and 62 `d` | 255 | total over limit |

R21 locks the scanner boundary: punctuation is not stripped because only
whitespace delimits spans.

### 8.3 `whitespace_unicode_order_and_ranges_are_exact`

Input is exactly:

~~~text
Préface\na@example.com\tb@EXAMPLE.ORG\nfin
~~~

The UTF-8 input is 40 bytes. It emits, in this order:

1. accepted `a@example.com`, range `9..22`, equality `a@example.com`; and
2. accepted `b@EXAMPLE.ORG`, range `23..36`, equality `b@example.org`.

The test asserts complete fields and scalar boundaries for both records.

### 8.4 `empty_and_marker_free_inputs_emit_nothing`

Two assertions are exact:

- empty input returns `Ok(Vec::new())`; and
- `plain text without marker` returns `Ok(Vec::new())`.

### 8.5 `replay_is_exact_across_three_runs`

The exact Unicode input from Section 8.3 is evaluated three times with the
same source identity. All three ordered vectors must compare exactly equal.

### 8.6 `candidate_limit_accepts_exact_ceiling`

The input is `"@ ".repeat(65_535) + "@"`: 131,071 bytes and exactly 65,536
review candidates. The result is `Ok` with length 65,536. Its first range is
`0..1`; its last range is `131070..131071`. Every record has the fixed common
fields, `observed == "@"`, status `NeedsReview`, no equality key, and valid
scalar boundaries.

### 8.7 `candidate_limit_rejects_next_without_partial_output`

The input is `"@ ".repeat(65_536) + "@"`: 131,073 bytes and 65,537 requested
records. The exact result is:

~~~text
Err(EmailDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
~~~

The return type exposes no partial vector. The equality assertion covers the
complete error value.

These seven named tests are the complete MI-01 test set. Adding, removing,
renaming, ignoring, or weakening a test requires plan amendment and approval.

## 9. Generated-artifact contracts

### 9.1 `Cargo.lock`

Source contracts are the two approved manifests. After they exist, generate
the lockfile only with the fixed offline environment:

~~~text
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo generate-lockfile --offline
mkdir -p target/tools
cp Cargo.lock target/tools/Cargo.lock.first
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true cargo generate-lockfile --offline
cmp target/tools/Cargo.lock.first Cargo.lock
~~~

Expected semantic output:

- lockfile format `version = 4`;
- exactly one package, `outis-core` version `0.1.0`;
- no `source`, `checksum`, or dependency entry; and
- a second identical generation produces no byte diff.

`Cargo.lock` is never manually edited.

### 9.2 Root `inventory.md`

The root inventory's sources are the strict component inventory and the five
Rust source files. Its existing owner is
`bin/generate_global_inventory.rs`, which remains byte-identical. Generate it
only with:

~~~text
mkdir -p target/tools
env RUSTUP_TOOLCHAIN=stable \
  RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
  RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
  CARGO_NET_OFFLINE=true rustc -O bin/generate_global_inventory.rs \
  -o target/tools/generate_global_inventory
target/tools/generate_global_inventory --repo-root "$(pwd)" \
  --out "$(pwd)/inventory.md" --strict
~~~

Expected semantic output:

- heading ``# `outis` - Global Inventory (GENERATED; DO NOT EDIT)``;
- one `crate::outis-core` component;
- the exact five paths and purposes from Section 5; and
- no `INVENTORY GAP`.

The existing generator writes the component-list marker as
``- `crate::outis-core` `` and the detail heading as
``## `crates/outis-core` ``. This is fixed generator behavior observed at
`bin/generate_global_inventory.rs:551` and
`bin/generate_global_inventory.rs:600`. P1-01 corrects only the validation
pattern; it changes no implementation path, generated bytes, or product
behavior.

The same immutable generator parses one physical inventory line at a time at
`bin/generate_global_inventory.rs:316-346`. It does not append Markdown
continuation lines to a source purpose. The blocked pre-test audit proved that
wrapping the email-detector purpose after `equality key,` silently truncated
the generated purpose. P1-02 therefore binds that complete handwritten entry
to one physical line as shown in Section 7. It changes no Rust source,
generator, product behavior, or generated-artifact owner.

The generator writes a `Generated:` timestamp. That line is the sole accepted
variable. Reproducibility is checked by running the generator twice, copying
each output under `target/tools`, deleting only the `Generated:` line from the
two temporary copies, and comparing the normalized copies byte-for-byte. The
tracked `inventory.md` remains the direct second generator output and is never
normalized or hand-edited.

No generated binding, model artifact, fixture, benchmark result, package, or
application artifact is created.

## 10. Exact offline toolchain preflight

Run this preflight after recording `P1_BASE_COMMIT` and before any MI-01 Cargo,
rustc, rustfmt, or Clippy operation:

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

All commands must exit zero without changing the repository. Accept only:

- rustc release `1.89.0`, commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, host
  `aarch64-apple-darwin`;
- Cargo release `1.89.0`, commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`, host
  `aarch64-apple-darwin`;
- Clippy `0.1.89` commit `29483883ee`;
- rustfmt `1.8.0-stable` commit `29483883ee`;
- installed `cargo`, `clippy`, `rustc`, `rustfmt`, and
  `rust-std-aarch64-apple-darwin` components; and
- installed `aarch64-apple-darwin` target.

Additional installed components or targets are allowed. Any missing or
mismatched required identity stops implementation before file changes. The
loopback distribution endpoints plus Cargo offline mode prove only that the
commands did not require remote toolchain resolution on this host; they do not
prove absence of local socket activity.

## 11. Ordered implementation procedure

1. Confirm explicit plan approval, commit the approved plan, require a clean
   worktree, and record `P1_BASE_COMMIT` and branch.
2. Run and record the complete Section 10 preflight. Stop on any difference.
3. Create `rust-toolchain.toml`, replace the root manifest, create the
   `outis-core` manifest, and remove only the exact `Cargo.lock` ignore line.
4. Remove only `src/main.rs`, `Makefile.toml`, and `release.toml` with
   `apply_patch`.
5. Create the handwritten component inventory and five bound Rust source files
   with `apply_patch`; do not create convenience modules or placeholders.
6. Generate `Cargo.lock` offline, validate its semantic content, regenerate it,
   and require byte equality.
7. Run the following as the only mechanical source rewrite, then review every
   formatted source file:

   ~~~text
   env RUSTUP_TOOLCHAIN=stable \
     RUSTUP_DIST_SERVER=http://127.0.0.1:9 \
     RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup \
     CARGO_NET_OFFLINE=true cargo fmt --all
   ~~~

8. Compile the existing inventory generator and generate `inventory.md` once
   with the exact Section 9.2 commands. Require strict generation and inspect
   the direct output before the audit.
9. Author the pre-test audit. It must inspect the completed diff against
   `P1_BASE_COMMIT`, classify exactly `PRE_TEST_AUDIT_PASSED` or `BLOCKED`, and
   bind changed paths, API, tests, forbidden constructs, lockfile, generated
   artifacts, and claim boundary. A blocked audit stops validation.
10. Run the narrow validation in Section 12 in its listed order. Stop and
   diagnose the first failure; do not weaken tests or expected results.
11. Author the result review from observed commands and outputs. It records
    only supported claims and classifies the increment as passed or blocked.
12. Re-read every changed file, run the final path and diff audits, and report
    the exact status without committing the implementation unless separately
    requested.

No parallel mutation is permitted. The implementation is small and each step
depends on the preceding artifact or gate.

## 12. Validation commands and expected outputs

Every Rust command below uses the S1-21 environment. The preflight is rerun if
the shell, toolchain selection, or repository state changes.

### 12.1 Format, lint, and unit oracle

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
~~~

Expected: each exits zero; formatting has no diff; Clippy emits no warning;
and exactly the seven named, unignored unit tests in Section 8 pass on one test
thread.

### 12.2 Workspace and dependency surface

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
~~~

Expected: metadata reports one workspace member, `outis-core` 0.1.0, with one
library target and no dependencies. The tree contains only `outis-core
v0.1.0`. The lockfile has the exact Section 9.1 semantics.

Validate the lockfile directly with:

~~~text
rg -n '^version = 4$|^name = "outis-core"$|^version = "0.1.0"$' Cargo.lock
test "$(rg -c '^\[\[package\]\]$' Cargo.lock)" -eq 1
! rg -n '^(source|checksum|dependencies) = ' Cargo.lock
~~~

Expected: the first command prints exactly the lockfile version, package name,
and package version lines; the package-count assertion succeeds; and the
forbidden-field search finds no match.

### 12.3 Inventory generation and replay

Compile the generator once and invoke the first exact Section 9.2 generation
command. Then execute these replay steps, whose second generator invocation
also supplies the tracked final output:

~~~text
cp inventory.md target/tools/inventory.first.md
target/tools/generate_global_inventory --repo-root "$(pwd)" \
  --out "$(pwd)/inventory.md" --strict
cp inventory.md target/tools/inventory.second.md
sed '/^Generated:/d' target/tools/inventory.first.md \
  > target/tools/inventory.first.normalized.md
sed '/^Generated:/d' target/tools/inventory.second.md \
  > target/tools/inventory.second.normalized.md
cmp target/tools/inventory.first.normalized.md \
  target/tools/inventory.second.normalized.md
~~~

Expected: compiler and generator exit zero; strict mode reports no inventory
gap; the normalized outputs are byte-identical; the tracked direct second
output has one Outis Core component and the exact five source-purpose entries.

Validate the fixed inventory markers with:

~~~text
rg -F '# `outis` - Global Inventory (GENERATED; DO NOT EDIT)' inventory.md
test "$(rg -c '^- `crate::outis-core`$' inventory.md)" -eq 1
! rg -n 'INVENTORY GAP' inventory.md
~~~

Expected: the title is present, exactly one component-list marker is present,
and no gap marker is present. The detail heading is exactly
``## `crates/outis-core` ``. The five entries are then compared literally
with Section 7 while reading the generated file.

### 12.4 Failure-surface source audit

Search only the five handwritten Rust source files for:

~~~text
unwrap\(|expect\(|panic!|todo!|unimplemented!|unreachable!|unsafe|dbg!|println!|eprintln!
~~~

Use:

~~~text
! rg -n 'unwrap\(|expect\(|panic!|todo!|unimplemented!|unreachable!|unsafe|dbg!|println!|eprintln!' \
  crates/outis-core/src/lib.rs \
  crates/outis-core/src/candidate.rs \
  crates/outis-core/src/detect.rs \
  crates/outis-core/src/detect/email.rs \
  crates/outis-core/src/detect/email/tests.rs
~~~

Expected: no match. Also inspect that production code contains no filesystem,
network, clock, environment, locale, random, model, database, process, or
thread access and no linter suppression.

### 12.5 Repository and changed-path audit

~~~text
git diff --check
git diff --cached --check
git status --short --branch
git diff --name-status "$P1_BASE_COMMIT" --
~~~

Expected: both diff checks exit zero. The name-status set contains only the 15
implementation paths in Section 5 plus the pre-test audit and result review.
The already committed plan and planning-status documents are part of
`P1_BASE_COMMIT`, not the implementation diff. `target/` remains ignored. No
model, fixture, generated binding, application, Swift, runtime, FFI, vault,
export, or evidence directory appears.

The final worktree is intentionally dirty with the reviewed implementation
unless the user separately requests a commit. No unrelated path may appear.

## 13. Pre-test audit and result-review contracts

The existing `BLOCKED` pre-test audit records the P1-02 discovery and is
committed with the amended planning baseline. After implementation and
formatting are recreated, the same artifact is re-authored before the
validation suite and records:

- `P1_BASE_COMMIT`, branch, and dirty-state path list;
- exact conformance of every Section 5 action;
- the seven test bindings and expected values;
- public API and private-module conformance;
- standard-library-only and forbidden-surface inspection;
- lockfile and inventory generation contracts;
- line-budget observation; and
- classification `PRE_TEST_AUDIT_PASSED` or `BLOCKED`.

The result review is authored only after a passed pre-test audit and completed
validation. It records each command, exit status, relevant output, toolchain
identity, host, profile, dirty state, and observed result. Its conclusion may
claim only deterministic email transformation on the bound synthetic inputs,
the observed compile/dependency surface, and offline toolchain resolution.

Neither artifact may claim complete sensitive discovery, extraction support,
anonymization, privacy, security, performance, application readiness, or pilot
readiness.

## 14. Failure and rollback boundary

Any failed gate, preflight mismatch, unexpected changed path, unbound behavior,
new dependency, failed test, warning, inventory gap, or generated-artifact
difference stops implementation. Record the failure in the appropriate review
artifact before rollback; do not change an oracle to obtain a pass.

For an uncommitted implementation rollback:

1. Resolve a new, explicit temporary quarantine directory with `mktemp -d` and
   record its path.
2. Move only newly created `rust-toolchain.toml` and
   `crates/outis-core/` into that quarantine so failed evidence remains
   recoverable. Do not recursively delete them.
3. Restore only `Cargo.toml`, `.gitignore`, `src/main.rs`, `Makefile.toml`,
   `release.toml`, and `inventory.md` from the recorded `P1_BASE_COMMIT` using
   path-specific non-destructive Git restoration.
4. After the old manifest and old ignore rule are restored, regenerate the
   legacy ignored `Cargo.lock` offline and require its SHA-256 to equal
   `3b1417200a300593e389a517866b6890e778aac199879d81202492a33fab97ec`.
5. Preserve the pre-test audit, result review, quarantine, and command evidence
   for diagnosis. Confirm no unrelated path changed.

The exact path-specific restoration command must be printed and reviewed
against `P1_BASE_COMMIT` before execution. `git reset --hard`, `git clean`,
broad checkout, glob deletion, and workspace-root deletion are forbidden.

`target/` is ignored build output, not source rollback state. If cleanup is
needed, resolve and verify the package-specific target path while the new
manifest is still active; do not recursively target the repository root.

If the implementation has already been committed, use a normal Git revert of
the exact implementation commit instead of rewriting history. A revert is not
performed without a separate user request.

## 15. Risks and bounded responses

| Risk | Current evidence or consequence | Bound response |
|---|---|---|
| installed `stable` alias drifts | the alias is mutable even though S1-21 observed the approved identity | exact preflight blocks before edits; do not install or update a toolchain |
| whitespace-only delimiting absorbs punctuation | values such as `a@example.com,` become review rather than accepted | R21 locks this conservative behavior; no punctuation-stripping heuristic |
| single-label domains are accepted | S1-20's grammar does not require a dot | A04 locks `a@com`; changing it requires a spec amendment |
| Unicode whitespace behavior is compiler-bound | `char::is_whitespace` is part of the pinned Rust behavior | exact rustc identity is in replay evidence |
| string allocation exists per emitted record | `observed` and accepted equality keys are owned strings | fixed 65,536-record ceiling bounds record count; no speed or memory claim |
| ceiling tests allocate many records | the direct oracle can consume measurable test memory and time | run serially once through the exact test command; diagnose rather than weaken |
| future enum expansion changes public types | MI-01 includes only used variants | later capabilities require their own approved spec and plan |
| generated inventory contains wall-clock text | the existing generator writes `Generated:` | normalize only temporary comparison copies and disclose the sole variable |
| stale root scaffold removals affect old commands | current Makefile and release files describe another package | removal is explicit, reversible, and required by S1-20 |
| old lockfile is ignored before migration | the baseline file is not Git-restorable | recorded hash plus offline regeneration is the explicit rollback oracle |
| malformed hostile text can cause many review records | every `@` span emits until the fixed ceiling | exact typed all-or-nothing limit behavior is directly tested |
| grammar is deliberately narrower than RFC email syntax | quoted, commented, internationalized, and literal forms are reviewed | unsupported forms are retained as `NeedsReview`; no completeness claim |

No benchmark is authorized. Runtime speed, memory, CPU, compile time, and
binary size remain unmeasured by MI-01, so the result review cannot make those
claims.

## 16. Explicit exclusions

MI-01 does not touch or create:

- the macOS app, Xcode project, Swift, Finder integration, menu-bar UI, signing,
  sandboxing, entitlements, or Keychain access;
- `.doc`, `.docx`, PDF, OCR, `.txt`, or Markdown extraction;
- telephone, IBAN, address, matter, person, organization, filename, path, or
  metadata detectors;
- NER source, runtime, dependencies, weights, tokenizer, manifest, acquisition,
  bundle phase, or model evidence;
- entity resolution, review UI, token allocation, vault, database, encryption,
  secret handling, FFI, runtime crate, or publication;
- agent-repository creation, fixtures, acceptance runner, performance harness,
  generated header, package, installer, or distributable; or
- remote services, RAG, embeddings, chat, rendering, synchronization, or Swiss
  verification infrastructure.

The model legal review remains a separate stop gate for later model-specific
work and does not block this dependency-free increment.

## 17. Approval gate

The original plan, P1-01, and P1-02 are explicitly approved. P1-02 changes one
exact physical-line binding after the immutable generator truncated the
approved two-line purpose during pre-test inspection. Implementation remains
blocked until this amended plan and the blocked audit are committed on a clean
baseline and the exact S1-21 preflight passes again.

The amended-plan approval received was:

> I approve the P1-02 full inventory-purpose correction and the amended P1
> MI-01 implementation plan exactly as written. Proceed only after the amended
> plan and blocked pre-test audit are committed on a clean baseline and the
> S1-21 preflight passes again.

Any later modification returns this artifact to implementation planning and
requires a new explicit approval after the change.
