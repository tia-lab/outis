~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Core Invariants

Status: active
Scope: entire repository

These invariants are mandatory unless a later approved spec explicitly narrows
or supersedes one. Supersession must be local, justified, and audited.

Universal invariants apply to every Outis phase. Active pilot invariants apply
to the local macOS pilot. Conditional future-service invariants apply only when
a later approved spec adds the named service or route.

## Universal Trust-Zone Invariants

1. The Human Zone contains the authorized user, original source repository,
   trusted local application, and explicitly approved extracted-plaintext
   preprocessing.
2. The AI Zone contains the generated agent-facing repository, agents, and any
   future RAG, embeddings, agent tools, or model-provider routes.
3. The Key Zone contains the private entity graph, token dictionary, sensitive
   mappings, approved secret material, and future rendering authority.
4. AI Zone components and stores must not receive known plaintext sensitive
   values.
5. AI Zone components must not call Key Zone token lookup, decryption, or
   rendering routes.
6. Key Zone functionality must not be exposed as an agent or LLM tool.
7. The original source repository must not be used as the agent workspace.
8. Trust-zone boundaries, allowed callers, allowed copy points, and denied
   routes must be documented before implementation.
9. Trust-zone violations are correctness failures, not operational warnings.
10. Filesystem placement alone is not proof that an unsandboxed same-user
    process cannot access a store.

## Universal Sensitive-Data Invariants

1. Sensitive-data classes must be declared by spec before detection,
   tokenization, or redaction is implemented.
2. Plaintext sensitive values may exist only inside approved Human Zone or Key
   Zone boundaries.
3. Document content must be tokenized or redacted before agent-facing storage,
   embeddings, retrieval, model-provider access, or agent-tool access.
4. Source paths, filenames, metadata, comments, annotations, attachments, logs,
   and error messages must be treated as potential plaintext surfaces.
5. Conversation memory, retrieval logs, agent logs, embeddings, and telemetry
   must not store plaintext sensitive values when those surfaces are in scope.
6. Synthetic data must be clearly marked and must not be confused with real
   sensitive data.
7. Real sensitive data must not be committed to the repository.
8. Reversible replacement is called tokenization or pseudonymization in
   technical contracts.

## Active Pilot Source and Extraction Invariants

1. The source repository is read-only unless a later spec binds a specific
   source mutation.
2. Source enumeration must remain inside the approved source boundary.
3. Symlinks, hard links, Finder aliases, packages, mounts, nested repositories,
   archives, and path traversal require explicit contracts.
4. Supported formats and format variants must be declared by spec.
5. Extraction must identify relevant text, structure, metadata, comments,
   revisions, annotations, forms, attachments, and embedded content according
   to the declared format contract.
6. Non-empty extracted text is not proof of complete extraction.
7. Unsupported, corrupt, encrypted, partial, low-confidence, or failed
   extraction must have explicit blocking or review behavior.
8. A source change during processing must not silently produce a mixed
   snapshot.
9. Plaintext extraction intermediates, caches, temporary files, IPC messages,
   FFI buffers, logs, and crash artifacts must be identified and bounded.
10. Extraction adapters and versions are part of deterministic and correctness
    evidence.
11. The pilot source contract must cover `.doc`, `.docx`, `.pdf`, `.txt`, and
    `.md`; it must not silently narrow the product requirement to text and
    Markdown sources.
12. Text-bearing and image-only PDFs require distinct extraction evidence. An
    image-only page requires an approved local OCR result or an explicit
    blocking outcome.
13. Every successfully processed source document produces one `.md` target at
    the corresponding relative location. The base name is preserved after
    required filename and path-component tokenization.
14. Two sources that resolve to the same target Markdown path are an explicit
    collision and must not overwrite one another.
15. In the funding-demo slice, every `.doc`, `.docx`, and PDF result, including
    OCR, requires explicit review before publication. Confirmation binds the
    exact source and normalized hashes; rejection blocks the complete job.
16. Native extraction and OCR replay identity must include the macOS build,
    framework environment, exact API and configuration, render identity, and
    normalized-document schema. Apple-managed OCR artifacts must not be
    represented as independently pinned.
17. Native extraction text crossing the Swift/Rust boundary is bounded Human
    Zone plaintext. It must not be placed in event JSON, diagnostics, logs, or
    an agent-facing artifact before tokenization.
18. Normalized Markdown from binary formats must not be represented as a
    layout-faithful or extraction-complete replica without a separate proved
    contract.

## Active Pilot Detection and Model Invariants

1. A complete user-authored glossary is not a pilot input requirement.
2. Outis creates and maintains the private entity graph from automatic
   discovery and approved review decisions.
3. Detection must combine only signals approved by spec.
4. Entity, class, span, alias, and relationship decisions must preserve
   provenance.
5. Overlap, conflict, same-string/different-entity, and
   different-string/same-entity behavior must be explicit.
6. Uncertain detection must not silently become an accepted non-sensitive
   decision.
7. Detection must be evaluated for false negatives, false positives,
   unresolved cases, and extraction failures on declared datasets, languages,
   formats, versions, and thresholds.
8. Perfect detection must not be claimed.
9. An on-device model is optional until approved by spec.
10. An approved local model that receives plaintext is trusted preprocessing,
    not an agent or model-provider route.
11. A plaintext local model must have explicit network, tool, training,
    persistence, logging, cache, failure, and cancellation contracts.
12. Model source, identity, hash, conversion path, runtime, configuration,
    supported inputs, and evaluation dataset must be recorded.
13. Model output is an untrusted candidate set and must pass deterministic
    validation and resolution before tokenization or publication.

## Tokenization and Redaction Invariants

1. Token grammar, namespace, escaping, class semantics, stability, and scope
   must be defined by spec.
2. Token equality semantics must be deterministic when retrieval, reasoning,
   audit, or replay depends on equality.
3. Repository scope must prevent unapproved cross-repository correlation.
4. Token dictionaries must be isolated from agent-facing routes and stores.
5. Collision detection and failure behavior must be explicit.
6. Missing, malformed, forged, stale, revoked, and token-like source-text
   behavior must be explicit.
7. Token rotation, entity merge, entity split, deletion, and migration behavior
   must be explicit when supported.
8. Redaction or tokenization must preserve enough non-sensitive context for the
   approved use case and must not invent facts.
9. Tokenization and redaction tests must cover false-positive and false-negative
   effects when the feature is in scope.

## Active Pilot Storage and Vault Invariants

1. The agent-facing repository and private vault are separate stores.
2. The private vault must not be inside or copied into the agent-facing
   repository.
3. The private vault must not be inside the original source repository.
4. The agent-facing repository must not store the private entity graph, token
   dictionary, sensitive mappings, decryption keys, Keychain material, or
   plaintext render values.
5. The private-vault schema must identify repository, entity, alias, token,
   source, extraction, detector, model, normalization, schema, run, and audit
   versions when those records are in scope.
6. Database engine choice is not approved by these invariants.
7. Secret-protection mechanism choice is not approved by these invariants.
8. Storage durability, transactions, crash consistency, journals, temporary
   files, migrations, backup, restore, retention, deletion, recovery, and audit
   semantics must be spec-bound.
9. Corrupt, partial, stale, wrong-repository, version-mismatched, and
   unauthorized records must have explicit failure behavior.
10. Allowed vault callers and tested denied callers must be specified.

## Active Pilot Agent-Repository Invariants

1. The generated agent repository is an AI Zone store.
2. It must be physically separate from the original source repository.
3. It may contain only approved tokenized or redacted Markdown, tokenized or
   independently approved relative names, a bound manifest, and approved
   non-sensitive metadata.
4. It must not copy original source binaries.
5. Source paths and filenames must be tokenized when sensitive. Unchanged path
   publication requires independent classification and approval. Document
   metadata is excluded unless independently approved.
6. Every source item must have a deterministic processed, ignored,
   review-required, blocked, or failed outcome under the source contract.
7. Unsupported or incompletely extracted content must not be represented as
   successfully processed.
8. Publication must validate the complete staged artifact.
9. Cancellation or failure must not expose a partial repository.
10. Failed publication must leave the last valid output unchanged.
11. Atomic-publication and recovery semantics must be spec-bound and tested.
12. A known-plaintext exclusion scan proves only the declared known-value
    oracle; it does not prove complete discovery.

## Active Pilot macOS and Route Invariants

1. Outis is the trusted local client surface.
2. User-initiated one-shot processing is the pilot event model.
3. Continuous watching, synchronization, remote processing, and rendering are
   outside the pilot.
4. Finder integration is a dispatch surface and must not receive direct vault
   or rendering access.
5. Folder access, persisted access, sandbox, signing, entitlements, app groups,
   Keychain access, application lifecycle, retry, offline, degraded, and
   cancellation behavior must be spec-bound when relevant.
6. The source, staging, destination, application-container, and vault locations
   must be validated before processing or publication.
7. No route may silently downgrade extraction, detection, tokenization,
   storage, authorization, or publication failure.

## Minimal Pilot-Surface Invariants

1. Each iteration implements the smallest complete vertical slice accepted by
   the approved spec and implementation plan.
2. Minimal scope is achieved through an explicit allowlist and typed rejection
   of unsupported behavior, not partial handling of a wider contract.
3. Every production file, module, target, interface, abstraction,
   configuration item, dependency, and non-trivial branch must bind to a
   current requirement, invariant, acceptance test, or evidence need.
4. General frameworks, plugin systems, compatibility layers, future-service
   adapters, optional modes, caches, concurrency, and extension points are
   prohibited until the current approved slice requires them.
5. Validation occurs at external and trust boundaries. Validated types should
   carry established internal invariants so checks are not duplicated without
   a contract reason.
6. Edge behavior is implemented only when it is inside the supported contract
   or protects privacy boundaries, determinism, vault integrity, atomic
   publication, cancellation, or recovery. Other behavior fails explicitly.
7. Minimality does not waive correctness, plaintext exclusion, vault
   isolation, deterministic replay, typed failure, atomic publication, or
   evidence requirements.
8. Production paths avoid unnecessary allocation, copying, persistence,
   dependencies, and compile surface. Optimization and performance claims
   require measurement of the approved path.

## Code Boundary Invariants

1. Runtime code must stay small and focused on approved local-client behavior.
2. macOS UI, Finder, extraction, model, database, vault, FFI, future service,
   benchmarks, and fixtures are separate surfaces unless a spec proves
   otherwise.
3. Dependency-heavy behavior must not enter the default runtime surface without
   a spec.
4. Generated files and model artifacts are not edited by hand.
5. Generated code and artifacts must come only from approved codegen.
6. Public Rust, Swift, C, IPC, storage, and model interfaces must be intentional
   and spec-bound.
7. Configuration settings must be spec-bound with defaults, bounds, and
   operator meaning.
8. Security-sensitive and data-boundary code must use explicit error handling.
9. FFI and IPC contracts must define ownership, encoding, offsets, errors,
   cancellation, concurrency, and cleanup.
10. Benchmarks, fixtures, and synthetic-data generators must not become
    production runtime requirements without approval.

## Testing and Benchmark Invariants

1. Privacy and trust-boundary claims require run evidence.
2. Performance claims require run evidence.
3. Compile-time claims require build evidence.
4. Correctness, extraction, privacy, and determinism must pass before speed
   claims.
5. Evaluation and benchmark datasets must identify whether they are synthetic,
   pseudonymized, or real approved data.
6. Real sensitive data must not be used in tests or benchmarks unless a spec
   defines storage, access, cleanup, and audit controls.
7. Detection reports must identify dataset, language, format, model, rules,
   versions, thresholds, and unresolved or failed cases.
8. Baselines must use identical logical payloads and correctness/privacy
   oracles.
9. Warm-cache and cold-cache results must be separated when storage or models
   are involved.
10. Failed and unstable runs are evidence and must not be hidden.
11. Tolerances and expected values must not be relaxed before correctness is
    proved.
12. Agent-boundary tests must include vault, source, staging, temporary-file,
    filename, metadata, and log exposure.
13. Deterministic replay must bind every stateful, platform, extraction, model,
    normalization, token, and serialization input.

## Conditional Future-Service Invariants

These rules remain dormant until a later approved spec adds the corresponding
surface:

1. Agent Service, RAG, embeddings, agent tools, and model providers receive
   tokenized or redacted inputs only.
2. Agent Service and AI-facing stores must not store plaintext sensitive
   values, token dictionaries, decryption keys, or rendering authority.
3. Agent Service, RAG, embedding, LLM, and agent-tool routes must not call Key
   Service.
4. Agent Service and Key Service credentials, storage, network, and audit
   boundaries must remain separate.
5. User prompts must be tokenized before AI-facing model or tool access.
6. AI responses remain tokenized until an approved rendering boundary.
7. Conversation memory, retrieval logs, embeddings, and telemetry store
   tokenized or redacted content only.
8. Human-visible plaintext rendering requires approved authorization, audit,
   missing-token, stale-token, wrong-tenant, wrong-matter, and denial behavior.
9. A future Swiss-hosted detector receiving plaintext is trusted preprocessing
   only under a separate approved route, storage, destruction, network, model,
   authorization, and audit contract.

## Documentation Invariants

1. Specs are the source of truth.
2. Research briefs, peer audits, implementation plans, and result reviews are
   separate artifacts.
3. Result reviews record evidence and limitations.
4. Documentation separates facts, decisions, hypotheses, and deferred work.
5. Conversation trace is not documentation.
6. Public claims must not exceed recorded evidence.
7. Future architecture must not be presented as active pilot scope.
8. A first spec draft must close prior-spec conflicts, command surfaces,
   generated and model-artifact ownership, dispatch and FFI paths, exact code
   bindings, and test migration.
9. Peer audit is not a substitute for author-side spec completeness.
