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

The pilot is documentation and research only. Product behavior is not
implemented.

R1 proposes Rust for the deterministic engine, Swift with native macOS
frameworks for the application and native extraction surface, and bundled
SQLite for the synthetic funding-demo vault. The extraction candidate uses
AppKit for Word, PDFKit for PDF text/rendering, and Vision revision 3 for local
OCR. The proposed vault is plaintext and does not use Keychain; it is not
suitable for confidential data. These remain S1 specification candidates, not
approved implementation choices.

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
- ROADMAP.json
- docs/roadmaps/outis_local_pilot_file_architecture.json
- the applicable research brief, spec, review, and implementation-plan
  artifacts when they exist

## Current State

R1 is complete for S1 drafting. It selects exact NER and native macOS
extraction candidates, normalized Markdown behavior, failure gates, and
provisional resource bounds. The Rust code remains a minimal executable stub.
No Swift application, Finder integration, extraction pipeline, detector,
private vault, or agent-facing export exists. Code remains blocked until an
approved spec, peer audit, and implementation plan exist.

Do not treat architectural intent as a proved privacy, security, correctness,
or performance claim. Claims require the evidence chain defined by the
protocols.
