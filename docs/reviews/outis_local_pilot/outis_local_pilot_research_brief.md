~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local macOS Pilot Research Brief

Slug: outis_local_pilot
Status: R1_COMPLETE_READY_FOR_S1_DRAFT
Date: 2026-08-17
Scope: local macOS source-to-agent-repository funding demo
Code authorization: none

## Status

R1 is complete for S1 drafting. The pilot accepts `.doc`, `.docx`, `.pdf`,
`.txt`, and `.md`, including local OCR for scanned PDF pages, and produces one
`.md` per successfully processed source. The final R1 decision map is in
`outis_local_pilot_r1_decision_closure.md`; exact extraction selection and
evidence are in `outis_local_pilot_r1_2_extraction_evaluation.md`; the NER
selection remains in `outis_local_pilot_ner_evaluation.md`.

No Outis pilot implementation exists. Research does not approve code,
dependencies, generated bindings, model distribution, Xcode configuration, or
security claims. Code remains blocked until the spec, peer audit, and exact
implementation plan are approved.

## Source Materials

Repository sources read:

- `AGENTS.md`, `README.md`, `initial-intake.md`, and `architecture.md`;
- `docs/invariants/core_invariants.md`;
- `docs/architecture/repository_structure.md`;
- every protocol under `docs/protocols/`;
- root `ROADMAP.json` and its JSON file-architecture companion under
  `docs/roadmaps/`;
- current reviews under `docs/reviews/outis_local_pilot/`;
- `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `inventory.md`, and current Git
  status.

Historical evidence:

- `git show HEAD:architecture-public.md` was read during the documentation
  migration;
- the file remains deleted in the user-owned worktree and is not active
  architecture.

Primary platform, storage, format, FFI, model, and runtime sources are linked
from the R1 contracts, R1.2 extraction evaluation, and NER evaluation. External
documentation supports candidate contracts; it does not prove Outis behavior.

## Measured Object

The eventual measured object is one complete user-initiated job:

~~~text
authorized source snapshot containing .doc, .docx, .pdf, .txt, and .md
  -> local format extraction and OCR where required
  -> deterministic and contextual discovery
  -> explicit review of uncertainty
  -> repository-local entity resolution
  -> private stable token assignment
  -> validated tokenized Markdown staging tree
  -> atomic publication of a separate Markdown repository named outis
~~~

Measurement includes cancellation, failure, cleanup, deterministic replay,
last-valid-output preservation, vault separation, and declared known-plaintext
scanning. It excludes remote services, RAG, embeddings, chat, rendering,
synchronization, and Swiss-hosted review.

## Candidate Approach

~~~text
Human Zone
  authorized user
  read-only source folder
  sandboxed Outis application
  local extraction, detection, review, and tokenization

Key Zone
  application-container private SQLite vault
  private entity graph, aliases, review decisions, and token mappings

AI Zone
  separate generated repository named outis
  deterministic manifest and tokenized Markdown only
~~~

Candidate responsibility split:

- Rust: deterministic domain engine, filesystem and strict UTF-8 runtime, NER
  adapter, private vault adapter, publication, and narrow C ABI;
- Swift and native macOS frameworks: folder selection, security-scoped access,
  AppKit Word import, PDFKit PDF inspection/text, Core Graphics page rendering,
  Vision OCR, application and menu-bar UI, review, progress, cancellation,
  signing, and sandbox integration;
- SQL: exact private-vault schema after S1 approval.

The first slice proposes three Rust crates, one macOS application target, one
macOS test target, one generated C header, and no Finder extension. Exact paths
remain the roadmap file-architecture candidate until P1 ratifies them.

## Source Data and Owner

Funding-demo input is synthetic and owned by the Outis project. The user
selecting any later source must have authority to read it, create a
pseudonymized derivative, and provide that derivative to the intended agent.
Outis cannot infer legal authority, retention duties, or legal-hold status.

The selected source is a normal local directory. Required source types are
legacy Word `.doc`, Open XML Word `.docx`, text-bearing and scanned `.pdf`,
UTF-8 `.txt`, and UTF-8 `.md`. Links, aliases, packages, mount crossings,
nested repositories, mutation, corrupt input, and unsupported content cannot
be silently ignored. R1.2 selected candidate per-format limits and explicit
review, blocking, failure, and cancellation behavior; S1 must ratify them.
Sensitive source path components remain private and require tokenized exported
names.

## Formats and Languages

Required funding-demo source formats:

- legacy Word `.doc`;
- Open XML Word `.docx`;
- text-bearing PDF;
- scanned or image-only PDF through local OCR;
- UTF-8 `.txt`;
- UTF-8 `.md`.

Every successfully processed source produces one Markdown file. R1.2 selects
strict Rust UTF-8 decoding; AppKit Word import; PDFKit page inspection and
text; Core Graphics rendering; and Vision revision 3 accurate OCR at fixed 200
DPI with ordered Italian, German, and French languages, automatic language
detection, and language correction disabled. Binary-format output always
requires review. Spreadsheets, presentations, email containers, archives, and
standalone images are deferred.

Required content languages are Italian, German, and French. The deterministic
structured grammar is intentionally narrow; contextual coverage uses the
selected multilingual NER candidate. English and Swiss-German claims are
deferred.

## Affected Trust Zones

Human Zone sensitive surfaces:

- source folder and security-scoped URL;
- in-process extraction, normalized text, NER buffers, candidates, review, and
  token replacement;
- private error presentation;
- candidate staging contains tokenized output only, not plaintext
  intermediates.

Key Zone surfaces:

- unencrypted funding-demo SQLite database and rollback journal;
- entities, aliases, provenance, review decisions, token mappings, schema, and
  detector versions;
- application-container paths and deletion behavior.

AI Zone surfaces:

- `outis-manifest.json`;
- one `.md` per source under a tokenized mirror of its relative source path;
- no plaintext sensitive path components, mappings, vault, secrets, logs, or
  partial output represented as complete.

An agent with same-user broad filesystem access can still reach source or vault
paths. Repository separation alone is not operating-system access control.

## Sensitive-Data Classes

Active classes are:

- person;
- organization;
- postal address;
- email address;
- telephone number;
- IBAN;
- matter identifier.

Context-sensitive passages, government identifiers, credentials, identifying
dates or events, and unlisted structured identifiers are unsupported. This is
a declared coverage limit, not evidence that those values are safe.

## Automatic Entity Discovery

The active discovery order is:

1. bounded deterministic detectors for email, telephone, IBAN, postal-address
   structure, and cue-bound matter identifiers;
2. pinned offline NER evidence for person, organization, and address-location
   components;
3. deterministic overlap and conflict classification;
4. exact same-class normalized equality across the source repository;
5. explicit user review of all NER, address, matter, conflict, and plausible-
   but-unsupported candidates;
6. deterministic token allocation only after review closure.

The private entity graph is produced automatically. A user-authored glossary is
not required. Fuzzy alias merging, surname matching, legal-suffix removal, and
confidence-based automatic acceptance are forbidden in the first slice.

## Determinism Boundary

Byte-identical output is required only when source bytes and paths, initial
vault, review decisions, pipeline versions, dependency versions, model and
runtime artifacts, detector configuration, architecture, taxonomy, and
manifest schema are identical.

Token equality is repository-local. No cross-repository, cross-machine,
cross-version, cross-operating-system-build, clean-vault, or changed-review
equality is promised. R1.1 found bitwise-identical model logits and R1.2 found
stable native extraction output in their bounded probes; complete export replay
is not yet measured. OCR identity includes the macOS build, Vision revision and
configuration, supported-language result, render, and observation ordering.

## Agent-Repository Contract

The candidate tree is:

~~~text
outis/
  outis-manifest.json
  <tokenized-source-relative-directories>/
    <tokenized-source-base-name>.md
~~~

Every successfully processed source maps to one `.md`. The source-relative
tree and base name are preserved after sensitive path components are replaced
with stable repository-local path tokens. A case-insensitive target collision
blocks publication; no source is overwritten or silently renamed.

The manifest has deterministic ordering and no timestamp. Publication occurs
only from a validated same-filesystem sibling staging directory. First publish
uses rename; replacement requires `RENAME_SWAP`; no copy or destructive
fallback is permitted. An unresolved, blocked, failed, cancelled, mutated, or
partially extracted job cannot publish.

Validation covers an exact path allowlist, manifest and file hashes, token-to-
vault correspondence, source snapshot recheck, and the declared synthetic
known-plaintext oracle. Passing this scan does not prove complete discovery or
anonymity.

## Private-Vault Contract

R1 selects bundled SQLite for S1 review:

- `rusqlite` 0.40.2, default features disabled, `bundled` only;
- `libsqlite3-sys` 0.38.2 and observed bundled SQLite 3.53.2;
- one connection and writer;
- rollback-journal mode, full synchronization, foreign keys, memory temporary
  storage, secure delete, untrusted schema, and no busy wait;
- application-container parent mode `0700`, database and journal mode `0600`;
- explicit immediate transactions, rollback, version check, and quick check;
- no WAL, network filesystem, backup, restore, automatic repair, or downgrade.

This vault is plaintext and synthetic-demo-only. Keychain is not included
because a Keychain secret would not encrypt the SQLite mappings by itself.
Encrypted storage and Keychain protection are required research before a real
confidential-data pilot.

## Model Contract

The selected candidate is
`Davlan/bert-base-multilingual-cased-ner-hrl` revision
`e756de7f7b8f64fea0c3d7c3872f1322fab747b1`, publisher ONNX and tokenizer
artifacts, ONNX Runtime 1.28 CPU, sequential batch one, two intra-op threads,
one inter-op thread, and one inference call.

It emits untrusted person and organization candidates and location-component
evidence. Every promoted NER span requires review. It never emits a complete
postal address directly. Artifact mismatch, unknown labels, or runtime failure
blocks the job; there is no download, fallback model, or remote path.

Commercial redistribution, full legal-document quality, signed-app packaging,
cross-machine replay, and complete-job resource behavior remain unproved.

## macOS and FFI Contract

The S1 candidate is arm64 macOS 14 or later, Swift 6, App Sandbox, only the
user-selected read/write file entitlement, ephemeral security-scoped access,
one application job, and a `MenuBarExtra`. Network, app-group, Keychain-
sharing, persistent-bookmark, Finder-extension, daemon, and background-service
surfaces are absent.

The in-process Rust `staticlib` exposes a version-one C ABI with opaque handles,
pointer-length UTF-8 JSON payloads, explicit result codes, Rust-owned buffers,
polling, cancellation, and mandatory release. There are no callbacks and no C++
interop. Polling emits native extraction requests and Swift returns status and
provenance plus a separate bounded UTF-8 buffer through the candidate
`outis_job_submit_extraction` entrypoint. Rust panics cannot unwind across the
boundary.

`cbindgen` 0.29.4 is the header-generator candidate. The generated header is
checked in and reproduced by command; the module map is reviewed source. Full
Xcode, SDK, signing, Hardened Runtime, notarization, dylib embedding, and final
build integration remain unmeasured.

## Correctness and Privacy Oracle Candidates

The approved implementation plan must bind repository-owned synthetic fixtures
for:

- per-format extraction text, structure, page or block provenance, Unicode
  offsets, OCR coverage, corrupt input, metadata, and mutation;
- per-language structured positives and negatives;
- the 30-case NER regression smoke corpus;
- exact entity reuse, separation, conflict, merge, and token allocation;
- manifest bytes and complete output tree;
- known plaintext, filenames, paths, mappings, vault files, and sidecars;
- first publication, replacement, cancellation, cleanup, corrupt prior output,
  unsupported exchange, sync failure, and source mutation;
- deterministic replay under the declared complete identity.

Structured accepted-subset tests require exact oracle precision and recall.
NER results remain reported per language and class, not converted into a
perfect-discovery gate. Every unresolved candidate blocks publication.

## Evidence Table

| Evidence type | Available evidence | Limitation |
|---|---|---|
| Code read | Current Rust package is a dependency-free Hello World scaffold. | No pilot behavior exists. |
| Run | macOS 26.5, arm64, Rust 1.89, Swift 6.0.3 observed on one M4 Pro. | One machine; no Outis app run. |
| Build | R1.1 temporary NER integration and R1 SQLite probe built; full Xcode is absent. | No Swift app, signing, bundle, or complete product build. |
| Storage | Bundled SQLite 3.53.2 probe observed exact PRAGMAs, rollback, commit, `quick_check`, and `0600` database/journal. | Happy-path disposable probe; no crash recovery or confidential data. |
| Security boundary | Zone and export contracts are documented. | No sandboxed Outis or denied agent-access run; no security claim. |
| Benchmark | R1.1 measured 1/2/4 threads; fixed two-thread 510-token mean/p95 was 89.54/94.03 ms. | Temporary harness, one host, synthetic token payload. |
| Data and extraction | Native Word/PDF/OCR and strict UTF-8 candidates were selected after synthetic fidelity, failure, replay, latency, and memory probes. | No Outis implementation or repository fixture; small corpus, binary completeness, cold start, and cross-OS-build replay remain unproved. |
| Model | Revision, hashes, labels, tokenizer, runtime, thresholds, memory, payload, and smoke results recorded in R1.1. | Small corpus; legal, packaging, genre, and cross-machine limits remain. |
| Platform | Apple documentation and local SDK declarations support the sandbox, security-scope, menu, exchange, and sync candidates. | No Outis entitlement, signing, or lifecycle test. |
| Publication | Disposable APFS sibling-directory `RENAME_SWAP` probe succeeded. | One volume and happy path; no crash or sync injection. |
| External documentation | Apple, Microsoft, Unicode, SWIFT, ITU, RFC, SQLite, Swift, model/runtime, and extraction-alternative primary sources were read. | Published contracts are not proof of implemented behavior. |

## Hypotheses

1. The selected Rust/Swift/C split can keep deterministic policy out of UI code
   with a tolerable compile surface. No product build proves this.
2. The structured gate plus reviewed NER is useful for a synthetic trilingual
   demo. The small corpus does not prove legal-document coverage.
3. Exact entity equality avoids hidden false merges while preserving repeated
   tokens. The repository-owned oracle does not yet exist.
4. Same-filesystem staging and exchange preserve an old-or-new visible tree.
   Crash and power-loss behavior is unproved.
5. A separate plaintext vault is sufficient to demonstrate the graph and
   export contract on synthetic data. It is not sufficient for confidential
   data.
6. Native extraction keeps the first app smaller than bundled office, Java, or
   OCR runtimes while providing usable trilingual synthetic output. Real legal
   document quality and signed-app resources remain unproved.

## Unknowns and Risks

- model redistribution and training-data obligations need qualified legal
  review;
- the measured machine lacks full Xcode and a selected signing identity;
- the complete dependency lock, compile surface, app size, energy, and full-job
  latency are unknown;
- the NER smoke corpus is small and the structured/address/matter oracles are
  not yet repository-owned;
- address and matter cue lists miss valid forms;
- unsupported sensitive classes and model false negatives can enter an export;
- a same-user agent with broad filesystem access can bypass repository
  separation;
- plaintext may be copied by OS swap, diagnostics, indexing, snapshots, backup,
  security tools, or crash handling;
- source metadata, aliases, packages, resource forks, extended attributes, and
  mutation checks are not implemented;
- Word/PDF layout, notes, revisions, embedded content, mixed image/text pages,
  real scan quality, rotation, handwriting, and reading order remain unproved;
- Vision OCR is OS-managed and cannot be independently pinned; a macOS build
  change requires regression evidence;
- native extraction cold start, energy, and signed sandboxed Swift-app resource
  behavior are unmeasured;
- embedded Quick Look review fidelity, helper-process access, caches, and
  diagnostics are unmeasured;
- SQLite crash recovery, migration, deletion, and SSD erasure are not proved;
- filesystem exchange, sync, cancellation, and cleanup failure paths are not
  injected;
- completed processing may be misunderstood as safe or anonymous.

## Required Decisions in S1

S1 must ratify the R1 candidates and bind:

1. exact schemas for source, spans, review, entity, token, vault, manifest, FFI,
   job state, and failure;
2. exact crates, Swift targets, source files, generated header, model files,
   fixtures, evidence paths, and commands;
3. exact dependency versions, features, licenses, ownership, alternatives, and
   lock state;
4. exact Xcode, SDK, signing, entitlement, sandbox, embedded-runtime, and
   distribution settings;
5. exact SQLite schema and migration, retention, deletion, and recovery SQL;
6. exact acceptance commands, metrics, budgets, and expected outputs;
7. model legal-review disposition or an explicit implementation block;
8. pre-audit closure and approval status.

## Recommended Next Phase

Draft the minimal `outis_local_pilot_SPEC.md` from the closed R1 contracts.
Do not create or modify product code, dependencies, build configuration,
generated bindings, model artifacts, or fixtures until S1 passes peer audit
and an exact implementation plan is approved.
