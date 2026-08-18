# Outis

Outis is a proposed local macOS document-pseudonymization application. The
pilot is intended to transform a user-selected document folder containing at
least `.doc`, `.docx`, `.pdf`, `.txt`, and `.md` files into a separate
Markdown-only repository for agent use while keeping the private entity graph
and token dictionary outside the agent-facing repository.

For each source document, the target mirrors the relative folder structure and
base document name, changes the extension to `.md`, and tokenizes sensitive
path components. The generated repository is named `outis` and is physically
separate from the original source folder.

The repository now contains the validated `MI-01` and `MI-02` Rust
capabilities for deterministic email, telephone, and IBAN candidate discovery
from already validated UTF-8 text. It is not yet the macOS application or the
complete document-processing pipeline.

S1 approves Rust for the deterministic engine and Swift with native macOS
frameworks for the planned application and native extraction surface. The
approved design uses AppKit for Word, PDFKit for PDF text, Core Graphics for
fixed PDF rendering, and Vision revision 3 for local OCR. Those application,
extraction, vault, tokenization, and publication surfaces remain unimplemented.
The specified synthetic-demo vault is plaintext, does not use Keychain, and is
not approved for confidential data.

Remote services, RAG, embeddings, chat, response rendering, synchronization,
and Swiss-hosted model review are outside the pilot. A controlled Swiss-hosted
second-pass review model is a deferred direction only.

## Required Reading

- AGENTS.md
- initial-intake.md
- architecture.md
- docs/invariants/core_invariants.md
- docs/protocols/lifecycle_protocol.md
- task-specific protocol files under `docs/protocols/`
- docs/architecture/repository_structure.md
- docs/reviews/outis_local_pilot/outis_local_pilot_research_brief.md
- docs/reviews/outis_local_pilot/outis_local_pilot_r1_decision_closure.md
- docs/reviews/outis_local_pilot/outis_local_pilot_ner_evaluation.md
- docs/reviews/outis_local_pilot/outis_local_pilot_r1_2_extraction_evaluation.md
- docs/reviews/outis_local_pilot/outis_local_pilot_result_review.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_research_brief.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_spec_pre_audit.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_peer_audit.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_implementation_plan.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_pre_test_audit.md
- docs/reviews/outis_local_pilot_mi_02/outis_local_pilot_mi_02_result_review.md
- docs/reviews/outis_toolchain_s1_21a/outis_toolchain_s1_21a_research_brief.md
- docs/reviews/outis_toolchain_s1_21a/outis_toolchain_s1_21a_spec_pre_audit.md
- docs/reviews/outis_toolchain_s1_21a/outis_toolchain_s1_21a_peer_audit.md
- docs/reviews/outis_toolchain_s1_21a/outis_toolchain_s1_21a_implementation_plan.md
- docs/reviews/outis_toolchain_s1_21a/outis_toolchain_s1_21a_result_review.md
- docs/reviews/outis_local_pilot_mi_03/outis_local_pilot_mi_03_research_brief.md
- docs/reviews/outis_local_pilot_mi_03/outis_local_pilot_mi_03_spec_pre_audit.md
- docs/reviews/outis_local_pilot_mi_03/outis_local_pilot_mi_03_peer_audit.md
- ROADMAP.json
- docs/roadmaps/outis_local_pilot_file_architecture.json
- the applicable research brief, spec, review, and implementation-plan
  artifacts when they exist

## Current State

R1 and the approved S1 design select exact NER and native macOS extraction
candidates, normalized Markdown behavior, failure gates, and provisional
resource bounds. `MI-01` and `MI-02` passed their bound offline validations.
The current Rust library has 21 passing synthetic grammar tests: seven email,
seven telephone, and seven IBAN. It has no registry dependency. MI-02 is
committed at `7faf40e`; its result review classifies
`MI_02_VALIDATION_PASSED`.

S1-21A also passed its bounded tooling validation. The repository toolchain
now declares the already installed matching Rust 1.89.0 rust-analyzer
component alongside Clippy and rustfmt, preventing the inspected VS Code
extension from selecting its incompatible newer bundled server. This changes
no product code, dependency, compiler identity, or Cargo graph.

No Swift application, Finder integration, extraction pipeline, contextual
detector, private vault, tokenization path, or agent-facing export exists.
S1-23 through S1-23B are approved as the extraction-only MI-03 specification
boundary, but they authorize no code. S1-23A closes the dependency, fixture-
probe, and Vision-oracle findings; S1-23B closes the identity and canonical-
serialization finding. The corrected author pre-audit is now blocked on the
missing exact failure-code and competing-condition precedence matrix.
S1-23C approval and passed author and separate peer reruns are required before
an exact implementation plan can be written. Model-specific work additionally
remains blocked by the qualified legal-review gate.

Do not treat architectural intent as a proved privacy, security, correctness,
or performance claim. Claims require the evidence chain defined by the
protocols.
