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

The repository now contains the validated `MI-01` Rust capability for
deterministic email-candidate discovery from already validated UTF-8 text.
It is not yet the macOS application or the complete document-processing
pipeline.

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
- ROADMAP.json
- docs/roadmaps/outis_local_pilot_file_architecture.json
- the applicable research brief, spec, review, and implementation-plan
  artifacts when they exist

## Current State

R1 and the approved S1 design select exact NER and native macOS extraction
candidates, normalized Markdown behavior, failure gates, and provisional
resource bounds. `MI-01` passed its bound offline validation: seven Rust unit
tests passed for the declared email grammar, with no registry dependency. No
Swift application, Finder integration, extraction pipeline, contextual or
non-email detector, private vault, tokenization path, or agent-facing export
exists. The MI-02 telephone and IBAN specification, audits, and exact
implementation plan are approved. MI-02 code remains blocked until the
approved documentation is committed on a clean baseline and the offline
preflight passes again. Each later implementation increment remains blocked
until its applicable lifecycle gates pass.

Do not treat architectural intent as a proved privacy, security, correctness,
or performance claim. Claims require the evidence chain defined by the
protocols.
