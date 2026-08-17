~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Agent Contract

Status: active
Scope: repository root and all descendants

Outis is currently a documentation and research project for a local macOS
document-pseudonymization pilot. The intended pilot accepts a user-selected
document folder containing at least `.doc`, `.docx`, `.pdf`, `.txt`, and `.md`
sources, performs extraction and sensitive-entity discovery locally, assigns
stable tokens, and publishes a separate Markdown repository for agent use. The
target mirrors the source-relative tree and base document names after required
path tokenization; every target document has a `.md` extension. The private
entity graph, token dictionary, and secret material remain outside that
agent-facing repository.

Wuthier Terminal is retained only as copyright or historical attribution.
Remote Agent Service, remote Key Service, RAG, embeddings, chat, response
rendering, synchronization, and Swiss-hosted review are not active pilot
surfaces. Rules for those surfaces apply only if a later approved spec adds
them.

This repository is not an experiment dump. Research, review, security-analysis,
test, and benchmark artifacts must be reproducible, evidence-bound, and tied to
an explicit contract.

Stack choices are not approved by this file. Rust, Swift, native macOS
frameworks, SQLite, macOS Keychain, document extractors, OCR engines, model
runtimes, cryptographic algorithms, and service runtimes remain candidates
until approved by specs.

## Active Pilot Boundaries

Human Zone:

- authorized local user;
- original source repository and extracted plaintext inside trusted local
  preprocessing;
- Outis macOS application;
- trusted local extraction, detection, entity resolution, and tokenization;
- plaintext temporary buffers explicitly allowed by spec.

AI Zone:

- generated agent-facing Markdown repository;
- local or remote agents granted access to that repository;
- future retrieval, embedding, agent-tool, or model-provider surfaces if later
  approved.

Key Zone:

- private local vault;
- private entity graph and aliases;
- token dictionary;
- encrypted sensitive values when approved;
- Keychain-held or equivalent secret material when approved;
- future isolated Key Service if later approved.

The generated agent repository and private local vault are different stores.
The private vault must never be created inside, copied into, or exposed through
the agent-facing repository.

## Inderogable Rules

1. Intellectual honesty and mathematical rigor are mandatory.
2. Always prove the assumption.
3. Never lie, tune, or omit evidence to improve a result.
4. Never state an unproved claim as proved.
5. Never use icons or emojis.
6. Clarity over verbosity.
7. Never use hyperbolic or promotional language.
8. Use a grounded and humble tone.
9. Outis measures; it does not predict.
10. Determinism is mandatory when data contracts, token equality, replay, or
    benchmarks depend on it.
11. Performance is time-bounded and evidence-bound.
12. Zero duplication is mandatory unless a spec justifies a prototype copy.
13. Never relax test tolerances before proving the implementation is correct.
14. Use Markdown encoding rather than LaTeX formulas in Markdown files.
15. Build for correctness, privacy, reproducibility, performance, and low
    compile surface.
16. No code change before an approved spec and approved implementation plan.
17. Generated code and generated artifacts must come only from approved
    codegen.
18. Speed claims require run evidence.
19. Compile-time claims require build evidence.
20. The production runtime must remain small. Extractors, model adapters,
    storage adapters, platform integrations, benches, fixtures, and developer
    tools are separate surfaces unless a spec proves otherwise.
21. First spec drafts must close prior-spec, command, artifact, dispatch, FFI,
    entitlement, model-artifact, and test-migration contracts before peer audit.
22. Agent-facing routes and repositories must never receive known plaintext
    sensitive values.
23. The original source repository must never be used as the agent workspace.
24. The agent-facing repository must never contain the private vault, token
    dictionary, decryption keys, Keychain material, or plaintext render values.
25. Document flows must tokenize or redact before AI-facing storage, model
    access, embedding, retrieval, agent-tool access, or agent access.
26. A local detection model that receives plaintext must be explicitly approved
    as trusted preprocessing and must not have network egress, agent tools,
    training, or persistent plaintext logs unless a later spec changes those
    boundaries.
27. Unsupported, corrupt, partially extracted, or uncertain documents must not
    silently enter an agent-facing repository.
28. Detection quality must be measured. Perfect discovery must not be claimed.
29. Reversible replacement is called tokenization or pseudonymization in
    technical contracts. An interface label may use Anonymize with Outis.
30. If future Agent Service and Key Service routes are approved, Agent Service,
    RAG, model, embedding, and LLM tool surfaces must not call Key Service or
    receive its dictionary, keys, or rendering authority.
31. Applicable documents under docs/protocols are mandatory. Pilot speed,
    prototype status, and iteration pressure are not waivers.
32. Each implementation iteration must be the smallest complete vertical slice
    approved by its spec and implementation plan.
33. Every production file, module, interface, abstraction, configuration item,
    dependency, and non-trivial branch must have a current spec, invariant,
    acceptance-test, or evidence binding. Otherwise omit it.
34. Unsupported or deferred behavior must be rejected explicitly rather than
    implemented speculatively.
35. Minimal implementation must not weaken plaintext exclusion, vault
    isolation, deterministic contracts, typed failure, atomic publication, or
    required evidence.

Violation of any rule requires protocol restart.

## Evidence Discipline

Every non-trivial statement must identify its evidence type:

- Code-read evidence: file and observed behavior.
- Run evidence: command, environment, input, and observed output.
- Build evidence: command, profile, dirty state, timing, and output.
- Storage evidence: store, schema, query, and observed result.
- Security-boundary evidence: caller, callee, trust zone, authorization or
  sandbox control, and observed allowed or denied behavior.
- Benchmark evidence: command, dataset, profile, machine, and result.
- Data-contract evidence: source format, extraction contract, classifier
  contract, tokenization contract, generated artifact, and check command.
- Model-artifact evidence: source, model identity and hash, conversion path,
  runtime configuration, supported inputs, and evaluation result.
- Platform evidence: macOS version, Xcode and Swift toolchains, target,
  entitlements, signing state, and observed behavior.
- External-doc evidence: upstream source and date-sensitive claim.
- Hypothesis: an unproved statement with the reason it is suspected.

No statement may sound certain when it has not been proved.

## Repository Boundaries

Outis owns, when approved by spec:

- local macOS client behavior;
- user-initiated folder selection and one-shot processing;
- local document extraction coordination;
- local sensitive-entity discovery and entity resolution;
- deterministic token assignment and Markdown serialization;
- private local entity-graph and token-dictionary coordination;
- generation and atomic publication of a separate agent-facing repository;
- macOS progress, cancellation, Finder integration, sandbox, and Keychain
  coordination;
- local audit-event emission;
- evidence discipline for privacy, correctness, determinism, compile surface,
  and performance.

Outis does not own unless a later approved spec adds the surface:

- final legal advice or legal correctness of agent output;
- remote Agent Service or Key Service runtimes;
- RAG, embeddings, chat, prompt processing, response rendering, or
  conversation memory;
- remote synchronization or repository watching;
- Swiss-hosted model review;
- organization-wide identity, retention, legal-hold, or authorization policy;
- stack selection before research, spec, and approval.

External schemas, models, services, and deployment manifests are allowed only
when they pass approved data-flow, trust-boundary, generated-artifact, and
evidence contracts.

## Minimal Pilot Implementation Discipline

Outis advances through narrow, working vertical slices. Minimal means the
smallest end-to-end behavior that satisfies the current approved contract. It
does not mean partially implementing a wider contract.

- Prefer an explicit allowlist and typed rejection over partial support.
- Validate external and trust boundaries once, then pass validated types
  through internal code rather than repeating defensive checks at every layer.
- Do not add general frameworks, plugin systems, future-service adapters,
  compatibility layers, optional modes, feature flags, caches, concurrency, or
  extension points before the current slice requires them.
- Do not add an abstraction for hypothetical reuse. A boundary abstraction is
  allowed only when the current spec requires isolation, substitution, FFI,
  or an evidence oracle.
- Implement edge behavior only when it belongs to the supported contract or is
  required to protect data boundaries, determinism, vault integrity, atomic
  publication, cancellation, or recovery. Reject the rest explicitly.
- Avoid unnecessary allocation, copying, persistence, and compile surface.
  Optimize measured bottlenecks; do not make performance claims without run
  evidence.
- Keep tests proportional to the slice while covering its accepted path,
  explicit rejection path, and applicable privacy and failure boundaries.

## Mandatory Execution Flow

Every non-trivial task follows this sequence without skips.

### 1. Intake

No code.

- Restate the goal.
- Classify the task.
- List required reads, inputs, datasets, configurations, trust zones,
  sensitive-data classes, and unknowns.
- State stop/go risks.
- Ask only blocking questions.

### 2. Context Read

No code.

- Read this file.
- Read docs/invariants/core_invariants.md.
- Read the lifecycle and task-specific protocols.
- Read relevant architecture, specs, reviews, implementation plans, code, model
  contracts, source-data contracts, and upstream documentation.

### 3. Research Brief

No code.

Write docs/reviews/[slug]/[slug]_research_brief.md with:

- measured object;
- candidate approach;
- source-data and extraction contracts;
- trust zones and sensitive-data classes;
- model and generated-artifact surfaces;
- unknowns and risks;
- available and required evidence;
- correctness and privacy oracle candidates;
- required decisions before spec.

### 4. Spec

No code.

The spec is the source of truth. If a design decision is not in the spec, it
must not appear in code.

Every applicable spec must define:

- goal, non-goals, and measured object;
- source formats and extraction contract;
- sensitive-data classification and uncertainty contract;
- tokenization and redaction contract;
- local model and model-artifact contract;
- Human, AI, and Key Zone boundaries;
- agent-repository publication and access contract;
- private-vault, secret, storage, retention, and recovery contract;
- macOS application, Finder, sandbox, entitlement, Keychain, and cancellation
  contracts;
- Rust, Swift, FFI, generated-artifact, crate, target, and dependency
  boundaries;
- determinism, failure, compile-surface, and performance budgets;
- correctness and privacy proof plan;
- exact code, test, benchmark, and artifact bindings;
- implementation-plan requirement and approval status;
- conditional future-service contracts only when those services are in scope.

Before the first peer audit, the spec must pass the pre-audit closure gate in
docs/protocols/spec_protocol.md.

### 5. Peer Audit

No code.

Write docs/reviews/[slug]/[slug]_peer_audit.md and classify exactly:

- PEER_AUDIT_PASSED
- BLOCKED

The audit must try to falsify the spec.

### 6. Implementation Plan and Approval

No code.

Write docs/reviews/[slug]/[slug]_implementation_plan.md. Bind every file,
generated artifact, model artifact, dependency, Xcode target, entitlement, FFI
surface, test, benchmark, evidence path, validation command, expected output,
risk, and rollback boundary.

The plan must be explicitly approved before any code change.

### 7. Implementation

- Implement only the approved spec and plan.
- Implement the smallest complete approved vertical slice.
- Keep changes minimal and bounded; omit unbound scaffolding and future-facing
  code.
- Reject unsupported inputs and behavior explicitly at the narrowest approved
  boundary.
- Preserve deterministic behavior and explicit failure surfaces.
- Do not hide allocations or plaintext copies.
- Do not use unchecked failure in runtime, persistence, model, platform,
  codegen, measurement, or security-boundary logic.
- Do not add configuration without a defined default, bounds, and operator
  meaning.

### 8. Validation

- Run the narrowest correctness checks first.
- Validate extraction before detection claims.
- Validate privacy and trust boundaries before security claims.
- Validate deterministic replay before performance claims.
- Validate vault isolation and agent-export contents.
- Validate model identity and evaluation before model-quality claims.
- Validate build and compile surfaces before compile-time claims.
- Record commands, environment, inputs, build profile, trust zones, and output.
- If validation fails, diagnose without weakening expectations.

### 9. Review and Report

- Update evidence and review artifacts.
- State what changed.
- State which invariants are proved.
- State which claims remain unproved.
- Do not claim readiness beyond the completed evidence chain.

## Stop Gates

Stop before code if any applicable gate fails:

- required reads are incomplete;
- measured object is unclear;
- source ownership or source-format contract is missing;
- extraction completeness and failure behavior are unclear;
- sensitive-data classes or uncertainty semantics are missing;
- token format, equality, scope, collision, missing-token, or rotation behavior
  is unclear;
- trust zones or allowed plaintext copy points are unclear;
- agent-repository content or access contract is missing;
- private-vault isolation, secret ownership, storage, recovery, deletion, or
  audit behavior is missing;
- local model boundary, identity, supported inputs, evaluation, or failure
  contract is missing when a model is in scope;
- macOS sandbox, security-scoped access, Finder dispatch, entitlement, signing,
  or cancellation behavior is missing when relevant;
- Rust, Swift, FFI, crate, target, dependency, or generated-artifact ownership
  is missing;
- spec is missing, incomplete, or unapproved;
- peer audit is missing or blocked;
- implementation plan is missing or unapproved;
- correctness or privacy oracle is missing;
- benchmark methodology is missing for performance claims;
- compile-surface budget is missing for generated or dependency-heavy work;
- a dependency choice is justified by preference rather than evidence.

For future services, also stop if Agent Service plaintext exclusion, Key Service
isolation, route authorization, remote storage, or rendering behavior is
unclear.

## Mandatory Protocol Reads

Before any non-trivial task, read:

- docs/protocols/lifecycle_protocol.md
- docs/invariants/core_invariants.md

Then read the applicable task protocol:

- research: docs/protocols/research_protocol.md
- spec authoring: docs/protocols/spec_protocol.md and
  docs/protocols/peer_audit_protocol.md
- peer audit: docs/protocols/peer_audit_protocol.md
- implementation planning or coding:
  docs/protocols/implementation_protocol.md and
  docs/protocols/code_style_protocol.md
- generated work: docs/protocols/codegen_protocol.md
- testing or benchmarking: docs/protocols/testing_benchmark_protocol.md
- reviews and reports: docs/protocols/review_documentation_protocol.md
- reusable prompts: docs/protocols/task_prompts.md

If these reads are incomplete, work must stop.

## Privacy and Trusted-Boundary Rules

Privacy-preserving, secure, isolated, trusted-client, or safe-to-use claims
require evidence that:

- the original source repository and extracted plaintext stayed inside the
  approved Human Zone boundary;
- extraction behavior and unsupported content were identified;
- declared sensitive classes were evaluated before publication;
- known plaintext values, source filenames, metadata, temporary files, and
  vault records were excluded from the agent repository;
- tokenization was deterministic where equality, audit, or replay required it;
- the private vault and secret material were unreachable from the tested agent
  boundary;
- uncertain and failed documents did not silently publish;
- partial publication did not replace the last valid output;
- test and benchmark data were synthetic or explicitly approved;
- model, extraction, and configuration versions were recorded;
- future Agent Service, RAG, embeddings, logs, LLMs, and agent tools receive
  tokenized or redacted content only when those surfaces are approved;
- future Key Service and rendering routes remain unreachable from Agent
  Service and model-tool surfaces.

Detection evaluation may prove results only for the recorded dataset, languages,
document formats, model, rules, and thresholds. It must not be generalized to
perfect detection.

## Configuration Hygiene

- Do not add configuration unless necessary.
- Every setting requires a spec binding, default, bounds, and operator meaning.
- Prefer checked configuration objects and explicit application settings.
- Environment variables require explicit justification.

## Dependency and Artifact Hygiene

Every new dependency or model artifact requires a spec binding stating:

- why it is needed;
- which crate, Xcode target, adapter, or service owns it;
- alternatives considered;
- selected version or artifact identity;
- source and integrity information;
- correctness, privacy, security, performance, licensing, and compile-surface
  risks;
- validation commands and expected evidence.

## Documentation Style

- Use concise Markdown.
- Use Markdown encoding for formulas and pseudocode.
- Avoid promotional language.
- Separate proved facts, decisions, hypotheses, and deferred work.
- Use review documents as due-diligence artifacts, not conversation traces.
- Keep future architecture in one bounded section and reference it elsewhere.
