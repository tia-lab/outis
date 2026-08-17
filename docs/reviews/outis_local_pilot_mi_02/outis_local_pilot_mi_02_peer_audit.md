~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot MI-02 Specification Peer Audit

Classification: `PEER_AUDIT_PASSED`
Date: 2026-08-17
Target: `docs/specs/outis_local_pilot_SPEC.md`, S1-22 through S1-22C
Target status inspected: `S1_22C_APPROVED_AUTHOR_PRE_AUDIT_PENDING`
Commit inspected: `f1801ae41ba4acad819c8292641f91c1fd5c963e`
Dirty state: approved documentation changes in progress plus an unrelated,
user-owned `.gitignore` change; no implementation, manifest, lockfile,
toolchain, build, test, inventory, or generated-artifact change
Audit run: separate MI-02 falsifying audit after passed author pre-audit

## Result

No blocking specification finding remains for the exact MI-02 implementation-
planning boundary. S1-22A closes the exhaustive-enum claim, S1-22B closes the
telephone-extension rule, and S1-22C establishes one 10 MiB caller limit. The
structured-detector behavior, failure surface, oracle, trust-zone boundary,
dependency surface, and file allowlist are sufficiently exact for a separate
implementation plan.

This audit does not approve a plan, authorize code, prove runtime behavior,
clear the model legal gate, or establish complete discovery, privacy,
security, anonymization, performance, or pilot readiness.

## Required reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/review_documentation_protocol.md`
- `docs/protocols/task_prompts.md`
- `docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_research_brief.md`
- `docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_spec_pre_audit.md`
- `docs/specs/outis_local_pilot_SPEC.md`
- `docs/architecture/repository_structure.md`
- `ROADMAP.json`
- `docs/roadmaps/outis_local_pilot_file_architecture.json`
- current `outis-core` public types, exports, email detector, seven-test unit
  oracle, component inventory, manifests, lockfile, and toolchain locator
- prior MI-01 pre-audit, peer audit, implementation plan, and result review as
  historical evidence
- [SWIFT IBAN Registry Release 102, June 2026](https://www.swift.com/swift-resource/9606/download)
- [ITU-T E.164, February 2026](https://www.itu.int/rec/T-REC-E.164-202602-I/en)

## Closed author findings

| Finding | Approved resolution | Audit result |
|---|---|---|
| S1-22-PA-01 exhaustive public enum expansion | S1-22A declares intentional exhaustive-enum API evolution and requires all seven MI-01 expectations to remain unchanged | Closed |
| S1-22-PA-02 contradictory extension classification | S1-22B consumes one maximal non-empty ASCII digit run; 1–6 and 7+ both produce one complete `NeedsReview` record without an equality key | Closed |
| S1-22-PA-03 two caller limits | S1-22C makes Section 8's 10 MiB per-document normalized UTF-8 ceiling authoritative and adds explicit historical correction notes | Closed |

## Falsification matrix

| Lens | Challenge | Result |
|---|---|---|
| Author closure | Check exact 42-section order, approvals, prior-spec disposition, bindings, and absence of deferred design | Pass: author pre-audit is `PASSED`; all applicable checklist items are closed |
| Measured object | Try to interpret MI-02 as extraction, combined discovery, application, tokenization, vault, or publication work | Pass: exactly two independent pure transformations over validated `DocumentText` |
| Input ceiling | Search Sections 8, 24, and 40 and historical MI-01 artifacts for competing active limits | Pass: 10 MiB is authoritative; historical 16 MiB wording is explicitly superseded; no duplicate detector check |
| Public API | Compare proposed additions with current `candidate.rs`, `detect.rs`, and `lib.rs` | Pass as a design binding: existing email API remains; additions and intentional exhaustive-enum evolution are exact |
| MI-01 regression | Search current Rust consumers and count the private email oracle | Pass: no exhaustive class match; seven existing tests must retain exact expectations and rerun |
| Telephone start and line boundary | Challenge starts inside words, cross-line spans, non-ASCII digits, unformatted national identifiers, and unsupported digit counts | Pass as a design binding: starts, separators, five logical-line endings, 8–15 digit bound, and explicit omissions are exact |
| Telephone classification | Challenge accepted country codes, national forms, unsupported codes, direct trailing text, punctuation, and equality keys | Pass as a design binding: only leading-plus 33/39/41/49 subset can be accepted; other telephone-like forms require review |
| Telephone extension | Challenge whitespace, every cue, overlapping `ext`/`ext.` forms, absent digits, 1, 6, 7+, and maximality | Pass as a design binding: a cue qualifies only with its following digit run; the complete maximal run and cue are retained in one review record |
| IBAN start and stopping | Challenge prefixes, supported underlength/overlength, whitespace after exact supported length, unsupported countries, adjacent prose, and cross-line input | Pass as a design binding: starts, supported exact stop, contiguous overlength, unsupported maximal scan, and minimum review lengths are explicit |
| IBAN acceptance | Challenge country structure, lowercase normalization, check digits, MOD-97, equality, and unsupported countries | Pass as a design binding: only exact CH/DE/FR/IT structure plus MOD-97 remainder one can be accepted |
| Standards evidence | Compare country lengths and structures with SWIFT Release 102 and telephone digit ceiling with current E.164 | Pass: cited primary sources support the bounded tables; Outis-specific scanner policy is not attributed to the standards |
| Candidate records | Challenge source identity, surface, path index, byte alignment, observed slice, detector evidence, status, equality, order, and overlap | Pass as a design binding: every field and ascending non-nested class-specific output are fixed |
| Resource failure | Challenge unbounded candidate output and partial success | Pass as a design binding: independent 65,536 ceilings; the 65,537th record returns one typed error and no partial vector |
| Determinism | Challenge locale, clock, randomness, environment, threads, model, platform, and replay | Pass as a design binding: these inputs are absent; Rust 1.89 whitespace behavior and three exact replays are bound |
| Trust zones and plaintext | Try to route text or candidates to AI Zone, Key Zone, I/O, logs, persistence, network, model, or filesystem | Pass: MI-02 remains a pure Human-Zone transformation with none of those surfaces |
| File boundary | Compare Section 40, repository structure, and file-architecture JSON | Pass: nine allowed product/generated paths align; no fixture, app, FFI, model, vault, export, or publication path |
| Dependencies and compile surface | Inspect workspace metadata, manifests, and lockfile; search proposed boundary for dependency changes | Pass: one local library package, no dependency or feature, no allowed manifest or lockfile change |
| Generated artifact | Challenge root inventory ownership and generator mutation | Pass: only root `inventory.md` is regenerated by the existing immutable approved generator |
| Oracle strength | Try to pass without boundaries, ranges, records, replays, ceilings, or MI-01 regression | Pass as a design binding: the required case families and every record field are bound; exact literals remain a plan obligation |
| Claim safety | Search for perfect detection, verified account ownership, security, privacy, performance, or readiness inference | Pass: all are explicitly excluded; precision and recall are limited to the bound grammar table |
| Lifecycle | Try to begin code from specification approval or this audit | Pass: a separate exact plan, explicit approval, committed clean baseline, and repeated offline preflight remain mandatory |

“Pass as a design binding” means that the approved specification is exact
enough to plan and later test. It does not prove unimplemented behavior.

## Toolchain and repository evidence

The audit reran the S1-21 identity and offline-resolution checks with the
installed `stable` alias, rustup distribution endpoints redirected to closed
loopback port 9, and Cargo offline. Observed identities were:

- Rust 1.89.0, commit
  `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, host
  `aarch64-apple-darwin`;
- Cargo 1.89.0, commit
  `c24e1064277fe51ab72011e2612e556ac56addf7`;
- Clippy 0.1.89 and rustfmt 1.8.0-stable from the required Rust commit;
- required arm64 components and `aarch64-apple-darwin` target present; and
- `cargo metadata --locked --offline --no-deps --format-version 1` exited zero
  with one local `outis-core` library and no dependency or feature.

This is local command-resolution and metadata evidence, not an MI-02 build or
test result. Current source still contains only MI-01, with seven email tests.

## Preserved limitations and risks

- The telephone subset does not validate national numbering plans, assignment,
  reachability, or all telephone formats.
- Unformatted national digit strings remain an intentional false-negative
  boundary.
- Supported-country IBAN whitespace and unsupported-country maximal scanning
  retain declared false-positive and span-expansion risks.
- MOD-97 success proves neither account existence nor ownership.
- Synthetic grammar tests cannot prove complete discovery or suitability for
  confidential data.
- The later implementation plan must bind exact literal inputs, records,
  commands, expected outputs, line budgets, changed-path proof, risks, and
  path-specific rollback. A missing binding restarts planning.

## Authorization boundary

The approved specification may proceed to a separate MI-02 implementation
plan. That plan requires explicit user approval before any source, test,
inventory, generated artifact, dependency, manifest, lockfile, toolchain, or
build change. Model-specific work remains independently blocked by the
qualified legal-clearance gate.

## Classification

`PEER_AUDIT_PASSED`
