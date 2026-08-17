~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# PROTOCOL: Outis Reviews and Documentation

Version: 1.1
Status: active
Scope: docs/reviews/**, architecture, intake, invariants, protocols, and README

## Purpose

Review and architecture documents are due-diligence artifacts. They preserve
evidence, limitations, scope, and decisions so future work can continue without
guessing.

## Required Reads

- AGENTS.md;
- docs/invariants/core_invariants.md;
- docs/protocols/lifecycle_protocol.md;
- target architecture, research brief, spec, peer audit, and implementation
  plan when they exist;
- test, evaluation, benchmark, storage, model, platform, generated, and
  security-boundary artifacts when writing result reviews;
- relevant code and build paths when writing technical reviews.

## Review Documents

Per slug:

- docs/reviews/[slug]/[slug]_research_brief.md
- docs/reviews/[slug]/[slug]_peer_audit.md
- docs/reviews/[slug]/[slug]_implementation_plan.md
- docs/reviews/[slug]/[slug]_result_review.md

Optional:

- [slug]_technical_review.md
- [slug]_security_boundary_review.md
- [slug]_extraction_review.md
- [slug]_model_evaluation_review.md
- [slug]_failure_review.md
- [slug]_benchmark_review.md
- [slug]_compile_surface_review.md

## Source-of-Truth Chain

1. Approved intake and active invariants
2. Research brief
3. Spec
4. Peer audit
5. Implementation plan
6. Code, build configuration, generated artifacts, and model artifacts
7. Tests, evaluations, and benchmarks
8. Result review

Later artifacts may not claim more than earlier contracts and recorded evidence
support.

## Status Discipline

Documents must distinguish:

- active approved scope;
- candidate architecture;
- unapproved research direction;
- hypothesis;
- deferred future work;
- implemented behavior;
- proved behavior under recorded evidence;
- known limitation.

Future remote services must not be described as active pilot behavior.
Processed or completed must not be described as perfect detection or proved
safety.

## Writing Rules

- Findings first when reviewing.
- Use impersonal due-diligence style.
- Separate facts, decisions, evidence, hypotheses, and interpretation.
- Include evidence limitations and missing categories.
- Do not include conversation trace.
- Do not use promotional language.
- Use tokenization or pseudonymization for reversible mappings.
- Keep future architecture in one bounded location and reference it elsewhere.
- Identify user-owned dirty-worktree changes and preserve them.

## Result Review Minimum

Include:

- slug and status;
- source artifacts and commands;
- git commit and dirty state;
- source owner, source snapshot, formats, languages, and input sizes;
- Human, AI, and Key Zone surfaces;
- sensitive-data classes;
- extraction adapter and normalized-document identity;
- detector rules, model identity, runtime, configuration, and dataset when
  relevant;
- token, vault, schema, storage, platform, signing, entitlement, FFI, generated,
  and build identities when relevant;
- extraction correctness;
- detection and entity-resolution results;
- privacy and known-plaintext exclusion result;
- vault and agent-access boundary result;
- determinism result;
- publication, cancellation, persistence, and recovery result;
- compile and application-surface result;
- benchmark result;
- failures and unstable runs;
- interpretation limits;
- recommendation:
  - continue;
  - redesign;
  - abandon;
  - promote to product-spec discussion.

Unavailable evidence is reported as unproved, not passed.
