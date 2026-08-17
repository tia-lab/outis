~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Protocol Prompts

These prompts are reusable task starters. They do not override AGENTS.md,
approved specs, or protocols.

## Research Prompt

~~~text
Read AGENTS.md, docs/invariants/core_invariants.md,
docs/protocols/lifecycle_protocol.md, and
docs/protocols/research_protocol.md first.

Task: prepare a research brief for [slug].

Do not write code, change build configuration, select dependencies, generate
bindings, or add model artifacts.

Write docs/reviews/[slug]/[slug]_research_brief.md with the measured object,
source ownership, source formats, extraction risks, target languages,
sensitive-data classes, automatic discovery and entity-resolution candidates,
token and determinism boundaries, Human/AI/Key Zone surfaces, agent-repository
contract, private-vault contract, macOS/Rust/Swift/FFI/model/storage candidates,
evidence table, hypotheses, unknowns, risks, correctness and privacy oracle
candidates, required decisions before spec, and recommended next phase.

Mark remote services and Swiss-hosted review out of scope unless the task
explicitly includes them.
~~~

## Spec Prompt

~~~text
Read AGENTS.md, core_invariants.md, lifecycle_protocol.md,
research_protocol.md, spec_protocol.md, peer_audit_protocol.md, the target
research brief, relevant architecture, prior specs, and relevant source and
upstream contracts.

Task: write docs/specs/[slug]_SPEC.md.

Do not write code.

Follow the mandatory section order exactly. Close source ownership,
enumeration, formats, extraction, normalized documents, sensitive classes,
automatic discovery, local model artifacts, entity resolution, uncertainty,
tokens, trust zones, agent repository, private vault, staging, atomic
publication, macOS application, Finder, sandbox, signing, entitlements,
Keychain, Rust/Swift/FFI, dependencies, generated artifacts, determinism,
failure, recovery, retention, correctness, privacy, benchmarks, tests, code and
build paths, evidence artifacts, and approval.

Mark future Agent Service, Key Service, RAG, chat, rendering, and Swiss-hosted
review out of scope unless explicitly included.
~~~

## Peer Audit Prompt

~~~text
Read AGENTS.md, core_invariants.md, lifecycle_protocol.md, spec_protocol.md,
peer_audit_protocol.md, the research brief, target spec, prior specs, relevant
source, and upstream contracts.

Task: audit docs/specs/[slug]_SPEC.md.

Do not edit the spec in the first pass. Try to falsify source boundaries,
extraction completeness, detection and entity semantics, model provenance and
evaluation, uncertainty, tokens, trust zones, plaintext copies,
agent-repository contents, private-vault isolation, staging and atomic
publication, macOS/Finder/sandbox/signing/entitlement/Keychain behavior,
Rust/Swift/FFI ownership, generated and model artifacts, failure and recovery,
correctness and privacy oracles, benchmark isolation, compile/application
surface, and binding completeness.

Write docs/reviews/[slug]/[slug]_peer_audit.md and classify exactly
PEER_AUDIT_PASSED or BLOCKED.
~~~

## Implementation Plan Prompt

~~~text
Read AGENTS.md, core_invariants.md, lifecycle_protocol.md,
implementation_protocol.md, code_style_protocol.md, codegen_protocol.md when
relevant, testing_benchmark_protocol.md, the approved spec, and peer audit.

Task: write docs/reviews/[slug]/[slug]_implementation_plan.md.

Do not write code.

Bind every Rust, Swift, C, Xcode, build, schema, migration, entitlement,
signing, Finder, sandbox, Keychain, FFI, extraction, model, storage, generated,
test, benchmark, dataset, security-boundary, and evidence file. Bind commands,
expected outputs, risks, cleanup, rollback, and validation order. Preserve
user-owned worktree changes.

End with an explicit approval gate before code.
~~~

## Implementation Prompt

~~~text
Read AGENTS.md, implementation_protocol.md, code_style_protocol.md,
codegen_protocol.md when relevant, testing_benchmark_protocol.md, the approved
spec, peer audit, and approved implementation plan.

Task: implement only the approved plan.

Do not add formats, classes, model behavior, dependencies, configuration,
targets, entitlements, generated artifacts, routes, or storage behavior absent
from the spec. Keep changes minimal. Preserve source, vault, staging, and agent
boundaries. Use explicit errors in Rust and Swift. Do not allow panic,
exception, force unwrap, partial publication, or plaintext leakage across a
security boundary.

After implementation, perform the required pre-test audit and report whether
testing may begin.
~~~

## Test and Benchmark Prompt

~~~text
Read AGENTS.md, testing_benchmark_protocol.md, the approved spec, approved
implementation plan, peer audit, and implemented code and build paths.

Task: run the bound tests, evaluations, and benchmarks.

Do not change expected values, thresholds, tolerances, datasets, or benchmark
scope unless independent evidence and the lifecycle prove the original
contract was wrong.

Record source, extraction, language, sensitive-class, detector, entity, token,
model, vault, platform, signing, entitlement, FFI, storage, dataset, build,
privacy, determinism, publication, recovery, compile-surface, performance,
failure, and interpretation evidence. Prepare the result review.
~~~
