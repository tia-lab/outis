~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot Specification Author Pre-Audit

Status: `PASSED`
Date: 2026-08-17
Scope: `docs/specs/outis_local_pilot_SPEC.md`
Classification: specification author pre-audit; no code
Audit run: S1-21 amendment rerun; supersedes the pre-S1-21 closure conclusion
Commit inspected: `f2e9b7e106f67ce72df9c5ea5364e98fb14f86a5`
Dirty state: documentation migration in progress; existing user-owned
`architecture-public.md` deletion preserved

S1-22C correction note, 2026-08-17: the Section 8 10 MiB per-document
normalized UTF-8 ceiling is authoritative. The stale 16 MiB wording in the
`S1-20 resource failure` row below is superseded. MI-01 accepts already
validated `&str` input and intentionally adds no second size check. This note
does not alter the historical S1-21 audit classification or MI-01 evidence.

## Finding

The S1-01 through S1-21 decisions are recorded and approved. S1-20 supersedes
S1-19's broad pre-model implementation permission with one complete
capability: `MI-01` maps validated UTF-8 text and a source identity to ordered
email `SensitiveCandidateV1` records. Its entrypoint, public data shape,
grammar, output values, changed paths, unit oracle, commands, exclusions, and
claim boundary are exact. A fixed 65,536-record ceiling gives spec-valid hostile
input a typed all-or-nothing failure instead of an unbounded output allocation.

The S1-20 rerun closes all three findings from the first A1 audit. `MI-01` is not an
incomplete horizontal foundation; it creates no application or FFI surface
requiring a model-absent terminal state; and it creates neither production nor
test publication. The complete-pilot architecture remains unchanged.

S1-21 closes A1-04 without changing the compiler. The installed rustup
`stable` alias is now only a locator: exact Rust, Cargo, Clippy, rustfmt,
component, host, and target identities must match before any MI-01 command.
With rustup distribution endpoints redirected to closed loopback port 9 and
Cargo offline, the exact preflight and current-manifest metadata command exited
zero. This proves local toolchain resolution on the inspected host, not an
Outis build.

The initially missing platform prerequisite was also resolved during the same
audit. Xcode 26.6, SDK 26.5, Swift 6.3.3, macOS 26.5, first-launch completion,
and arm64 Swift-to-Rust static linkage were verified. This does not prove an
Outis build.

The amended author pre-audit passes. S1-21 approval is recorded. The repeated
peer audit may begin, but implementation still requires that audit to pass and
an exact `MI-01` implementation plan to be approved.

## Sources Read

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- every active protocol under `docs/protocols/`
- `README.md`, `initial-intake.md`, and `architecture.md`
- `docs/architecture/repository_structure.md`
- `ROADMAP.json`
- `docs/roadmaps/outis_local_pilot_file_architecture.json`
- every current Outis R1 review artifact
- `docs/specs/outis_local_pilot_SPEC.md`
- the last committed `architecture-public.md` as historical evidence only
- current source, manifest, lock, maintenance-build, and release paths
- current Git status and history for specifications

## Evidence

| Check | Command or observation | Result |
|---|---|---|
| Git identity | `git rev-parse HEAD` | Commit above; dirty documentation worktree |
| Mandatory sections | extract `^## [0-9]+\.` and compare with `jot -s, 42 1` | Pass: exactly 1 through 42 in order |
| JSON syntax | `jq empty ROADMAP.json` and file-architecture JSON | Pass |
| Patch whitespace | `git diff --check` | Pass at audit time |
| Prior specs | `git ls-tree -r --name-only HEAD docs/specs` and current `find` | No prior committed spec; one current working spec |
| Historical architecture | `git show HEAD:architecture-public.md` | Read as historical evidence; not restored |
| Old Linux path | repository Markdown/JSON search for the former Linux repository root | No match |
| Full Xcode path | `test -d /Applications/Xcode.app/Contents/Developer` | Pass |
| Active developer directory | `xcode-select -p` | `/Applications/Xcode.app/Contents/Developer` |
| Xcode build tool | `xcodebuild -version` | Xcode 26.6, build `17F113` |
| First-launch state | `xcodebuild -checkFirstLaunchStatus` | Pass |
| SDK | `xcrun --sdk macosx --show-sdk-version` | macOS SDK 26.5 |
| Swift compiler | `xcrun swiftc --version` | Apple Swift 6.3.3 |
| Host operating system | `sw_vers` | macOS 26.5, build `25F71` |
| Rust toolchain | S1-21 `stable`-alias preflight with exact version output | Rust 1.89.0 commit `29483883eed69d5fb4db01964cdf2af4d86e9cb2`; Cargo 1.89.0 commit `c24e1064277fe51ab72011e2612e556ac56addf7`; arm64 Apple host |
| Rust components and target | installed-component and installed-target queries for `stable-aarch64-apple-darwin` | Required cargo, Clippy, rustc, rustfmt, arm64 Rust standard library, and `aarch64-apple-darwin` target present |
| Rust offline resolution | closed-port check; S1-21 preflight with rustup endpoints at `127.0.0.1:9` and Cargo offline; `cargo metadata --locked --offline --no-deps` | Pass: port closed and every command exited zero; no remote toolchain resolution required on inspected host |
| Swift/Rust linkage | disposable `rustc --crate-type staticlib`, `xcrun swiftc`, and execution probe | Pass: arm64 Mach-O ran and returned `OUTIS_SWIFT_RUST_PROBE_OK` |
| Probe cleanup | remove and verify absence of the exact temporary directory | Pass; no probe file remains |
| Model legal review | file-existence check for the Section 39 path | Absent; accepted only as the model-specific stop gate; not required by `MI-01` |
| Changed surfaces | union of staged and unstaged changed paths | Markdown/JSON only, plus preserved Markdown deletion; no code or build configuration changed |
| S1-20 stale language | search active spec, roadmap, file architecture, and repository structure for the rejected broad-increment phrases | Pass: absent |
| S1-20 workspace | inspect Sections 23, 37, and 40 plus both roadmap documents | Pass: one `outis-core` member for `MI-01`; three members only in the complete-pilot architecture |
| S1-20 publication boundary | inspect Sections 12, 36, 37, and 40 | Pass: no production or test publication, job state, app, Swift, or FFI surface in `MI-01` |
| S1-20 path boundary | compare Section 40, the file-architecture `first_pre_model_increment`, and repository-structure summary | Pass: exact core and root migration boundary, including inventory ownership |
| S1-20 resource failure | inspect 16-MiB caller bound, output cardinality, and terminal return | Pass: fixed 65,536-record ceiling; a 65,537th candidate returns one typed error and no partial output |
| S1-21 command migration | search every active Rust-family command in the specification | Pass: exact commands bind `RUSTUP_TOOLCHAIN=stable`; MI-01 commands also use loopback distribution tripwires and Cargo offline mode |

## Pre-Audit Closure Matrix

| Protocol requirement | Result | Binding or reason |
|---|---|---|
| Exact mandatory section order | Pass | 42 sections in required order |
| Prior approved specs searched and disposed | Pass | No prior committed spec; historical architecture is not a spec |
| Exact command and UI surfaces | Pass as a document binding | Sections 6, 21, and 35 through 38 |
| One owner for source, staging, agent repository, vault, temporary, model, and evidence paths | Pass as a document binding | Sections 6, 7, 12, 17 through 20, 35, 37, and 38 |
| Supported and unsupported format behavior | Pass as a document binding | Sections 7 through 9 |
| Generated, model, schema, fixture, and evidence ownership and commands | Pass as a document binding | Sections 19, 26, and 35 through 38 |
| Artifact and trust-zone compatibility | Pass as a document binding | Sections 17 through 20 and 38 |
| Rust, Swift, C, Xcode, adapter, storage, and dispatch paths listed | Pass as a document binding | Section 37 and its incorporated file architecture |
| Sandbox, signing, entitlements, app groups, Keychain, and persisted access explicit | Pass as a document binding | Section 23; no app group, Keychain item, or persistent bookmark |
| FFI ownership, encoding, offsets, errors, threading, cancellation, and cleanup | Pass as a document binding | Section 24 |
| Existing behavior and test migration explicit | Pass as a document binding | Current repository has a Hello World stub and no Rust tests; Sections 37 and 40 remove the stub and bind the first unit oracle |
| No design deferred to implementation plan | Pass | Section 42 declares no open design decision; plan may only bind execution |
| Privacy, extraction, detection, vault, replay, publication, and recovery proof commands | Pass as a document binding | Sections 33 through 38 |
| Compile-surface and application-size evidence commands | Pass as a document binding | Sections 31, 35 through 38 |
| Future services explicitly out of scope | Pass | Sections 4 and 27 |
| Smallest complete measured path | Pass as a specification decision | `MI-01` is one dependency-free core transformation with exact input, output, return, and oracle; no app or side-effect surface |
| Necessity binding for every production surface | Pass as a document binding | Section 40 and incorporated file architecture bind only five core source files, one component inventory, and necessary root migration paths |
| Deferred behavior rejected without scaffolding | Pass | Sections 4, 22, 27, and 37 |
| Model provenance and redistribution clearance | Pass as a conditional gate | Qualified review remains mandatory before every model-specific surface and complete-pilot acceptance |
| Selected full platform/toolchain evidence | Pass | Exact Xcode, SDK, Swift, Rust and Cargo identities, required Rust components and target, host OS, offline resolution, and disposable linkage probe observed |

“Pass as a document binding” means that the specification text is explicit. It
does not mean the unimplemented behavior has passed a runtime, build, privacy,
correctness, or performance test.

## Preserved Limitations

- No fixture, model artifact, generated header, migration, Xcode target, or
  evidence directory exists.
- The current root Cargo package and Hello World source remain unchanged until
  an approved implementation plan authorizes migration.
- The plaintext SQLite vault is restricted to synthetic funding-demo data.
- Model quality is bounded to the recorded synthetic corpus and is not perfect
  discovery evidence.
- No confidential-data, anonymity, security, privacy, or performance readiness
  claim follows from specification closure.

## Conditional Model Gate

A qualified human must author or approve
`outis_local_pilot_model_legal_review.md` with classification `CLEARED` before
any model-specific dependency, source, test, fixture, manifest, acquisition,
artifact, Xcode phase, bundle content, execution, distribution, or
complete-pilot acceptance. A changed model decision requires an S1 amendment
and another author pre-audit.

If the legal review or later platform evidence changes an approved model,
dependency, platform, command, or artifact contract, S1 must be amended and
explicitly approved before the audit is rerun.

## Authorization

`PASSED`. This document authorizes repeating the separate peer audit against
approved S1-21. It authorizes no code, dependency, build configuration,
generated artifact, model acquisition, fixture generation, or implementation
plan.
