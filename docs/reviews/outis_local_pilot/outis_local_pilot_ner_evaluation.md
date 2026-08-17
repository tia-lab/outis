~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis Local Pilot R1.1 NER Evaluation

Slug: outis_local_pilot_ner_evaluation
Status: R1_1_SELECTED_FOR_S1_RATIFICATION
Date: 2026-08-17
Classification: research only
Code authorization: none

## Decision

Select the following fixed contextual detector for the first-slice
specification:

- model: `Davlan/bert-base-multilingual-cased-ner-hrl`;
- upstream revision: `e756de7f7b8f64fea0c3d7c3872f1322fab747b1`;
- model artifact: publisher-provided `onnx/model.onnx`;
- tokenizer artifact: publisher-provided `onnx/tokenizer.json`;
- runtime: ONNX Runtime 1.28.0, CPU execution provider;
- target measured here: Apple Silicon macOS, arm64;
- execution: sequential, batch one, two intra-op threads, one inter-op thread;
- Rust binding candidate for the spec: `ort` 2.0.0-rc.13 with only `std`,
  `api-28`, and `load-dynamic` features;
- tokenizer candidate for the spec: `tokenizers` 0.23.1 with default features
  disabled and `fancy-regex` enabled.

This is a research selection for the later spec. It does not approve a Cargo
dependency, model redistribution, application bundle, generated artifact, or
implementation change.

The model is selected because it is the only evaluated candidate that has all
of the following at the same pinned upstream revision:

- declared Italian, German, and French coverage;
- a conventional fixed token-classification contract;
- a publisher-provided ONNX artifact and complete tokenizer artifact;
- a locally exercised native arm64 macOS runtime path;
- stronger organization-span results than the smaller evaluated DistilBERT
  candidate on the declared smoke corpus;
- less runtime and integration uncertainty than the GLiNER candidate.

The selection is not a claim that the model discovers every sensitive value.
It did not emit a complete postal-address span in any of the nine address cases.
`LOC` output may only be location evidence for a separate deterministic address
assembler. It must never be mapped directly to `postal_address`.

## Remaining Selection Conditions

Before the selected candidate may enter implementation, the spec and its
approval chain must close these conditions:

1. legal review of the model license, base-model terms, training-corpus terms,
   attribution, notice, and redistribution obligations;
2. acceptance or amendment of the proposed 752 MB NER payload and 2 GB peak
   NER-process RSS budgets;
3. the existing 30-case smoke corpus may gate only the synthetic funding demo;
   a larger approved corpus covering legal-document genres, negatives,
   mixed-language text, chunk boundaries, and each active class is required
   before a controlled confidential-data pilot;
4. S1 ratification of the deterministic Italian, German, and French
   postal-address assembler in `outis_local_pilot_r1_decision_closure.md`;
5. exact model packaging, code-signing, notarization, and application-bundle
   verification behavior;
6. dependency approval for `ort`, `ort-sys`, `tokenizers`, and their locked
   transitive surface;
7. S1 ratification of the threshold, review, failure, and unsupported-label
   contracts below.

If any condition fails, model use is blocked. There is no fallback detector,
runtime download, remote inference, or silent runtime substitution.

## Measured Object

The measured object is contextual candidate extraction from trusted local
UTF-8 text before tokenization. It is not the complete Outis pipeline.

Included:

- artifact identity and file size;
- tokenizer and output-label inspection;
- Italian, German, and French synthetic smoke cases;
- exact-span and overlap observations;
- confidence-threshold comparison;
- native CPU load, latency, memory, and replay checks;
- temporary Rust binding and tokenizer integration;
- candidate and license-source comparison.

Excluded:

- extraction fidelity;
- deterministic structured detectors;
- address assembly;
- entity or alias resolution;
- token assignment;
- private-vault behavior;
- review UI;
- complete document and repository throughput;
- cross-machine determinism;
- confidential or real legal data;
- Core ML, Metal, GPU, or Neural Engine execution;
- model conversion, quantization, training, or fine-tuning.

## Required Repository Sources

- `AGENTS.md`;
- `docs/invariants/core_invariants.md`;
- `docs/protocols/lifecycle_protocol.md`;
- `docs/protocols/research_protocol.md`;
- `docs/protocols/testing_benchmark_protocol.md`;
- `architecture.md`;
- `docs/architecture/repository_structure.md`;
- `ROADMAP.json`;
- `docs/roadmaps/outis_local_pilot_file_architecture.json`;
- `docs/reviews/outis_local_pilot/outis_local_pilot_research_brief.md`;
- current Git status.

## External Primary Sources

Sources were read on 2026-08-17.

- [selected mBERT model card](https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl);
- [selected mBERT pinned tree](https://huggingface.co/Davlan/bert-base-multilingual-cased-ner-hrl/tree/e756de7f7b8f64fea0c3d7c3872f1322fab747b1/onnx);
- [DistilBERT model card](https://huggingface.co/Davlan/distilbert-base-multilingual-cased-ner-hrl);
- [XLM-R model card](https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl);
- [GLiNER multilingual model card](https://huggingface.co/urchade/gliner_multi-v2.1);
- [GLiNER ONNX export documentation](https://github.com/urchade/GLiNER/blob/main/docs/convert_to_onnx.md);
- [WikiNEuRal model card](https://huggingface.co/Babelscape/wikineural-multilingual-ner);
- [ONNX Runtime 1.28.0 release](https://github.com/microsoft/onnxruntime/releases/tag/v1.28.0);
- [ONNX Runtime C API guidance](https://onnxruntime.ai/docs/get-started/with-c.html);
- [ONNX Runtime Core ML provider](https://onnxruntime.ai/docs/execution-providers/CoreML-ExecutionProvider.html);
- [`ort` 2.0.0-rc.13 release](https://github.com/pykeio/ort/releases/tag/v2.0.0-rc.13);
- [`tokenizers` repository](https://github.com/huggingface/tokenizers);
- [`tokenizers` encoding documentation](https://huggingface.co/docs/tokenizers/main/api/encoding).

External documentation is evidence of published contracts, not proof of Outis
behavior or legal clearance.

## Candidate Funnel

| Candidate | Revision | Declared classes | Principal artifact | License metadata | R1.1 result |
|---|---|---|---:|---|---|
| Davlan multilingual mBERT | `e756de7f7b8f64fea0c3d7c3872f1322fab747b1` | PER, ORG, LOC; config also contains DATE | ONNX, 709,345,293 bytes | AFL-3.0 | selected for spec review |
| Davlan multilingual DistilBERT | `d421f57d5b1d36b375408588669e9340f9b11a89` | PER, ORG, LOC; config also contains DATE | safetensors, 538,976,358 bytes | AFL-3.0 | rejected for first slice |
| GLiNER multi v2.1 | `443d26d654e0324125a96bebd8e796c14ff2efe6` | caller-declared open labels | safetensors, 1,155,830,112 bytes | Apache-2.0 | rejected for first slice |

Pre-screened exclusions:

- Davlan XLM-R was excluded before local inference: its safetensors artifact is
  1,109,868,164 bytes, it exposes the same fixed classes as the selected model,
  and the publisher tree has no ONNX artifact.
- WikiNEuRal was excluded because its model card declares
  CC-BY-NC-SA-4.0, which is incompatible with the intended commercial product
  direction without different rights.

## Selected Artifact Contract

### Model and tokenizer

| File | Bytes | SHA-256 |
|---|---:|---|
| `onnx/model.onnx` | 709,345,293 | `6c018415dc8129b358e9d629543c17481ad067ad02f9a6b8750473f161f9c5bd` |
| `onnx/tokenizer.json` | 2,919,362 | `bf1b59b7b11c95f194f51708d918eea378e09d05f84c0e1656dc5180e8117088` |

Observed model properties:

- ONNX IR version 6;
- default ONNX domain opset 11;
- producer `pytorch` 2.0.1;
- dynamic batch and sequence dimensions;
- inputs `input_ids`, `attention_mask`, and `token_type_ids`, all `int64`;
- output `logits` with shape `[batch, sequence, 9]`, `float32`;
- BERT hidden size 768, 12 layers, vocabulary size 119,547;
- maximum sequence length 512 including special tokens.

Required startup behavior:

1. resolve the artifacts only from the signed application bundle;
2. compute and compare every approved SHA-256 before session creation;
3. validate input names, input types, output name, output type, and label count;
4. block the job on any mismatch;
5. do not download, convert, repair, or replace an artifact at runtime.

### Runtime

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `onnxruntime-osx-arm64-1.28.0.tgz` | 32,396,562 | `1268b359718099bde2cedb55787f182a130067bc4f31e8c88478c445b850d3d8` |
| `libonnxruntime.1.28.0.dylib` | 39,312,136 | `dc19bbcb2f5c9fb3c68b4f9248aa0a35065ff702c5dbeae75eac54a74da97b6d` |

The official archive reports ONNX Runtime commit
`da9b5e364c465de65c49d91e696cd6485270757f`. The dylib is arm64 and
linker-signed ad hoc; the application packaging plan must define final nested
code signing and verification.

The first slice must select only `CPUExecutionProvider`. Core ML is deferred
because provider graph partitioning, compiled-model caching, first-load cost,
numeric parity, determinism, signing, and cancellation have not been measured.

### Rust integration candidates

The temporary probe used:

- `ort = 2.0.0-rc.13`, default features disabled, features `std`, `api-28`,
  and `load-dynamic`;
- `tokenizers = 0.23.1`, default features disabled, feature `fancy-regex`;
- an explicit application-bundle dylib path passed to `ort::init_from`;
- no `download-binaries`, HTTP, Hugging Face Hub, Core ML, GPU, or training
  feature.

`ort` is still a release candidate. This is a dependency risk, not a hidden
approval. A spec may accept it only with the exact lock, feature set, dynamic
library path, error contract, and rollback behavior.

## Input and Chunking Contract Candidate

Input is valid UTF-8 normalized-document text from an approved extractor. Outis
must not apply locale-sensitive lowercasing. The pinned tokenizer defines:

- BERT normalizer with clean-text enabled;
- Chinese-character handling enabled;
- lowercase disabled;
- accent stripping left at tokenizer default for the cased model;
- BERT pre-tokenization;
- WordPiece with `##` continuation prefix;
- `[CLS]` id 101 and `[SEP]` id 102.

Proposed deterministic windowing:

1. tokenize the entire normalized document without special tokens while
   retaining original UTF-8 byte offsets;
2. take 510 content tokens per window;
3. use 64 content tokens of overlap;
4. add `[CLS]` and `[SEP]` to each inference window;
5. process windows in ascending source-byte order, batch size one;
6. deduplicate exact `(model_label, start_byte, end_byte)` candidates and keep
   the maximum minimum-token confidence;
7. never auto-merge non-identical overlaps;
8. mark a candidate touching a non-terminal window boundary `needs_review`;
9. treat an invalid UTF-8 boundary, inverted span, out-of-range span, invalid
   BIO sequence, or conflicting label as an explicit failure or review event.

Rust `tokenizers::Tokenizer::encode` returns original byte offsets. The probe
confirmed that `Élodie` occupies bytes `[0, 7)` while the corresponding tokens
preserved valid byte boundaries. The canonical Outis span coordinate is a
half-open UTF-8 byte range. Character or UTF-16 offsets are UI projections and
must not become persistence or equality keys.

The 510/64 policy is exact enough for a spec draft, but its boundary quality is
not measured. The larger corpus must include entities starting and ending on
every overlap boundary before implementation approval.

## Label, Confidence, and Review Contract Candidate

The publisher model card describes PER, ORG, and LOC. The pinned config also
contains BIO DATE labels. That discrepancy is not silently accepted.

| Model label | Outis handling | Automatic tokenization |
|---|---|---|
| `B-PER`, `I-PER` | untrusted `person` candidate | forbidden |
| `B-ORG`, `I-ORG` | untrusted `organization` candidate | forbidden |
| `B-LOC`, `I-LOC` | location component evidence for deterministic postal-address assembly | forbidden |
| `B-DATE`, `I-DATE` | unsupported model label; record non-plaintext evidence and mark `needs_review` | forbidden |
| `O` | no contextual candidate | not applicable |
| any other label | artifact-contract failure; block job | forbidden |

Entity confidence is the minimum softmax probability across the tokens in the
assembled BIO span.

| Class or evidence | Emit threshold | Decision |
|---|---:|---|
| person | `>= 0.50` | always `needs_review` unless a later approved oracle permits deterministic acceptance |
| organization | `>= 0.50` | always `needs_review` unless a later approved oracle permits deterministic acceptance |
| LOC address component | `>= 0.50` and overlaps an independently detected address structure | always `needs_review` |
| any supported label | `< 0.50` | do not promote from NER alone; no safety inference is permitted |

No confidence value means safe, correct, or complete. The 0.50 threshold is
selected to preserve more candidate evidence in the first-slice review flow;
it is not calibrated probability and does not prove recall. Increasing the
threshold to 0.90 removed correct organization spans in this smoke corpus.

## Runtime Behavior Candidate

- one immutable inference session per active job and never more than one
  session at a time;
- CPU provider only;
- sequential execution;
- batch size one;
- two intra-op threads and one inter-op thread;
- one inference call at a time; document-level inference is not parallel;
- no user-selectable thread count, automatic tuning, or alternate performance
  mode in the first slice;
- graph optimization level `ORT_ENABLE_ALL`;
- inference ordered by document order and byte-offset window order;
- cancellation checked before and after every window;
- no network access;
- no runtime model discovery or download;
- no persistent input, output, logits, or plaintext cache;
- ONNX Runtime log level `ERROR` with errors mapped to non-plaintext Outis
  codes;
- no source text, tokens, spans, filenames, or paths in model logs;
- input and output buffers released after the containing document or job
  boundary defined by the spec;
- release the model session at the terminal job boundary so the model thread
  pool and model memory are not intentionally retained while Outis is idle;
- no Core ML compiled-model cache;
- model or runtime upgrades require a new revision, hashes, evaluation,
  approved spec amendment, and signed application version;
- rollback is to a prior approved signed application version, never an
  automatic artifact downgrade.

## Synthetic Smoke Corpus

All values are synthetic. The corpus contains ten cases per language, six
person spans, six organization spans, and three postal-address spans per
language. Gold strings are exact substrings of the displayed NFC text.

The SHA-256 of Python `json.dumps(cases, ensure_ascii=False, sort_keys=True)` is
`161a30b43d3e07756986a59b5f99cfa486c1446be5c9d5a6d06c82541c99dbf3`.

| ID | Text | Person gold | Organization gold | Postal-address gold |
|---|---|---|---|---|
| it-01 | Giulia Bianchi lavora per Alpina Tecnologie SA a Lugano. | Giulia Bianchi | Alpina Tecnologie SA | — |
| it-02 | Inviare il contratto a Marco De Santis, Via delle Rose 14, 6900 Lugano. | Marco De Santis | — | Via delle Rose 14, 6900 Lugano |
| it-03 | La società Futura Energia S.p.A. ha sede in Corso Milano 22, 10121 Torino. | — | Futura Energia S.p.A. | Corso Milano 22, 10121 Torino |
| it-04 | L'avvocata Anna Rossi incontrerà Banca Esempio SA a Ginevra. | Anna Rossi | Banca Esempio SA | — |
| it-05 | Il fascicolo riguarda Jean-Luc Moretti e Studio Legale Verdi. | Jean-Luc Moretti | Studio Legale Verdi | — |
| it-06 | Il pagamento deve essere effettuato entro trenta giorni. | — | — | — |
| it-07 | L'udienza oppone D'Amico Consulting SNC a Paolo D'Angelo. | Paolo D'Angelo | D'Amico Consulting SNC | — |
| it-08 | Contattare Müller & Figli Srl presso Piazza della Repubblica 3, 20121 Milano. | — | Müller & Figli Srl | Piazza della Repubblica 3, 20121 Milano |
| it-09 | Il comune ha trasmesso la risposta lunedì mattina. | — | — | — |
| it-10 | La testimone María-José Fernández vive a Bellinzona. | María-José Fernández | — | — |
| de-01 | Clara Meier arbeitet bei Alpenblick Beratung AG in Zürich. | Clara Meier | Alpenblick Beratung AG | — |
| de-02 | Senden Sie den Vertrag an Lukas von Bergen, Bahnhofstrasse 18, 8001 Zürich. | Lukas von Bergen | — | Bahnhofstrasse 18, 8001 Zürich |
| de-03 | Die Beispiel Maschinen GmbH hat ihren Sitz an der Hauptstrasse 7, 3011 Bern. | — | Beispiel Maschinen GmbH | Hauptstrasse 7, 3011 Bern |
| de-04 | Rechtsanwältin Sabine Keller trifft die Musterbank AG in Genf. | Sabine Keller | Musterbank AG | — |
| de-05 | Die Akte betrifft Hans-Peter Müller und Kanzlei Seeblick. | Hans-Peter Müller | Kanzlei Seeblick | — |
| de-06 | Die Zahlung ist innerhalb von dreißig Tagen fällig. | — | — | — |
| de-07 | Die Klage von Dr. Özlem Yılmaz richtet sich gegen Müller & Söhne KG. | Özlem Yılmaz | Müller & Söhne KG | — |
| de-08 | Kontaktieren Sie Côte d'Or Beratung GmbH, Marktgasse 4, 4051 Basel. | — | Côte d'Or Beratung GmbH | Marktgasse 4, 4051 Basel |
| de-09 | Die Abteilung hat den Bericht am Montag versandt. | — | — | — |
| de-10 | Zeugin Anne-Marie de Luca wohnt in Bellinzona. | Anne-Marie de Luca | — | — |
| fr-01 | Élodie Martin travaille chez Conseil Alpin SA à Genève. | Élodie Martin | Conseil Alpin SA | — |
| fr-02 | Envoyez le contrat à Marc de la Fontaine, Rue du Rhône 12, 1204 Genève. | Marc de la Fontaine | — | Rue du Rhône 12, 1204 Genève |
| fr-03 | La société Énergie Exemple SA a son siège au Quai du Mont-Blanc 5, 1201 Genève. | — | Énergie Exemple SA | Quai du Mont-Blanc 5, 1201 Genève |
| fr-04 | L'avocate Claire Dubois rencontrera Banque Modèle SA à Lausanne. | Claire Dubois | Banque Modèle SA | — |
| fr-05 | Le dossier concerne Jean-Pierre Rossi et Étude Juridique Léman. | Jean-Pierre Rossi | Étude Juridique Léman | — |
| fr-06 | Le paiement doit être effectué dans un délai de trente jours. | — | — | — |
| fr-07 | La demande de Maître Chloé d'Aubigné vise Müller & Associés Sàrl. | Chloé d'Aubigné | Müller & Associés Sàrl | — |
| fr-08 | Écrivez à Atelier Côte d'Azur SA, Avenue de la Gare 9, 1003 Lausanne. | — | Atelier Côte d'Azur SA | Avenue de la Gare 9, 1003 Lausanne |
| fr-09 | La direction a envoyé le rapport lundi matin. | — | — | — |
| fr-10 | Le témoin François-Xavier de Luca réside à Bellinzone. | François-Xavier de Luca | — | — |

This is a smoke corpus, not an acceptance or privacy oracle. It is small,
synthetic, template-like, and not representative of legal-document frequency,
format, length, ambiguity, or class prevalence.

## Evaluation Method

For person and organization, exact precision and exact recall require identical
gold and predicted source spans. Overlap recall counts a gold span when any
same-class predicted span overlaps it.

For postal addresses:

- exact address recall requires one model span equal to the full gold address;
- location-evidence overlap records whether any `LOC` span overlaps the address;
- general `LOC` predictions cannot be assigned address precision because cities
  outside postal addresses are valid model locations but not active Outis
  postal-address spans.

The full-corpus run used Python 3.13.3, `onnxruntime` 1.28.0, `tokenizers`
0.23.1, NumPy 2.5.2, and `psutil` 7.2.2 only as a temporary evaluation
harness. The selected product path remains Rust. The harness was outside the
repository and is not a product artifact.

Temporary evidence hashes:

| Object | SHA-256 |
|---|---|
| selected-model evaluation script | `2320ba3387efbaaee62f604a3810fad4f8b897592237170f55951de36e905f9b` |
| DistilBERT comparison script | `0e1bd91ad6fcfabcfc4106edc20283cf453f975c748feef0cf823228c470e1c3` |
| native C++ probe source | `a2992267e466222147ff67ad88b4c08fa357f16d93638547d2fb1386df415dac` |
| Rust probe source | `be8a979878b65105580fbcaf31fee944aa0cbc41af34089f4623ae508bd0c7bc` |
| Rust probe lock | `635bafd9e0941b4f17be16f0c32087b98df91da4e5b7457720d993e9a373b9d3` |

Because the temporary harnesses are not retained, this is bounded run evidence,
not a durable benchmark artifact. The approved implementation plan must create
a repository-owned synthetic evaluation surface before performance or quality
can be an acceptance gate.

## Quality Results

At the selected 0.50 emit threshold:

| Language | Class | Gold | Predicted | Exact precision | Exact recall | Overlap recall |
|---|---|---:|---:|---:|---:|---:|
| Italian | person | 6 | 6 | 1.000 | 1.000 | 1.000 |
| Italian | organization | 6 | 7 | 0.857 | 1.000 | 1.000 |
| German | person | 6 | 7 | 0.714 | 0.833 | 1.000 |
| German | organization | 6 | 7 | 0.857 | 1.000 | 1.000 |
| French | person | 6 | 6 | 1.000 | 1.000 | 1.000 |
| French | organization | 6 | 5 | 1.000 | 0.833 | 0.833 |

Postal-address observations:

| Language | Gold addresses | Exact complete spans | Addresses with overlapping LOC evidence | Total LOC predictions |
|---|---:|---:|---:|---:|
| Italian | 3 | 0 | 3 | 10 |
| German | 3 | 0 | 3 | 9 |
| French | 3 | 0 | 3 | 9 |

Aggregate threshold comparison:

| Threshold | Person exact P/R | Person overlap recall | Organization exact P/R | Organization overlap recall |
|---:|---:|---:|---:|---:|
| 0.50 | 0.895 / 0.944 | 1.000 | 0.895 / 0.944 | 0.944 |
| 0.75 | 0.944 / 0.944 | 1.000 | 0.941 / 0.889 | 0.889 |
| 0.90 | 0.944 / 0.944 | 1.000 | 1.000 / 0.833 | 0.833 |

Observed failures include:

- `Hans-Peter Müller` split into two PER spans with the hyphen classified ORG;
- `Étude Juridique Léman` missed as an organization;
- `società` emitted as an ORG false positive;
- `comune` emitted as a LOC false positive;
- no complete street, number, postcode, and city address span;
- high-confidence outputs were still wrong or incomplete.

These observations are reasons for deterministic validation and review. They
are not evidence that unobserved cases will behave similarly.

## Runtime and Build Results

Measured host:

- Mac model `Mac16,7`, Apple M4 Pro;
- 25,769,803,776 bytes physical memory;
- macOS 26.5 build 25F71, arm64;
- Apple clang 16.0.0;
- Rust 1.89.0;
- Swift 6.0.3;
- full Xcode unavailable; only Command Line Tools were active.

Preliminary native C++ probe, official ONNX Runtime dylib, CPU, one thread,
batch one, 100 repeats per length:

| Sequence tokens | First run | Warm mean | Bitwise-identical logits |
|---:|---:|---:|---|
| 16 | 10.52 ms | 8.18 ms | yes, 100 repeats |
| 128 | 34.59 ms | 33.79 ms | yes, 100 repeats |
| 256 | 69.71 ms | 67.45 ms | yes, 100 repeats |
| 510 | 158.76 ms | 154.96 ms | yes, 100 repeats |

Across fresh native probe processes, model-session load was 225–863 ms after
download and filesystem caching. Observed RSS increase after session load was
approximately 0.95–1.46 GB. The variation is material; neither value is a
portable memory guarantee.

Selected model, tokenizer, and dylib total 751,576,791 bytes, or 716.76 MiB,
before notices, manifest, app code, Swift assets, or other dependencies.

Temporary Rust integration result:

- exact model and dylib loaded successfully;
- accented UTF-8 byte offsets were valid and matched the original string;
- the sample produced the expected PER, ORG, and LOC token labels;
- model load 862.27 ms and one 14-token run 10.48 ms;
- clean release build 15.76 s on the measured host;
- 93 registry packages in Cargo metadata, 88 normal dependency tree lines;
- 219 MB clean Cargo target directory;
- 4,769,056-byte release probe binary.

The Rust integration evidence shows feasibility. It also shows that the
tokenizer and runtime bindings are not a small dependency surface.

### Thread-count benchmark addendum

R1.1 compared one, two, and four ONNX Runtime intra-op threads. Every
configuration kept one inter-op thread, sequential execution, CPU provider,
batch one, graph optimization `ORT_ENABLE_ALL`, and identical model inputs.
The count configures ONNX Runtime worker threads; it does not reserve or pin
physical Apple CPU cores.

Method:

- UTC interval: 2026-08-17 10:12:07–10:18:50;
- operator: Codex under user direction;
- Git commit: `f2e9b7e106f67ce72df9c5ea5364e98fb14f86a5` with the documented
  documentation-only dirty worktree;
- host: the same `Mac16,7` M4 Pro, 25,769,803,776-byte RAM, macOS 26.5
  build 25F71, arm64;
- runtime: the selected ONNX Runtime 1.28.0 dylib and selected model hash;
- native probe: Apple clang 16.0.0, `-O3 -DNDEBUG`, arm64;
- balanced fresh-process order: `1,2,4`, then `4,1,2`, then `2,4,1`;
- each process and sequence length: one excluded baseline run, ten excluded
  warm-up runs, then 50 measured runs;
- aggregate: three processes and 150 measured samples for each thread count
  and sequence length;
- payload: identical deterministic synthetic token IDs at 16, 128, 256, and
  510 tokens;
- percentiles: nearest-rank over all 150 measured latencies;
- `pmset -g therm` reported no recorded thermal, performance, or CPU-power
  warning before and after each balanced round.

Aggregate native results:

| Tokens | 1 thread mean / p95 | 2 threads mean / p95 | 4 threads mean / p95 | 2-thread speedup | 4-thread speedup |
|---:|---:|---:|---:|---:|---:|
| 16 | 8.68 / 9.46 ms | 6.34 / 7.29 ms | 5.96 / 7.85 ms | 1.37x | 1.46x |
| 128 | 34.73 / 36.31 ms | 20.42 / 21.64 ms | 19.80 / 25.10 ms | 1.70x | 1.75x |
| 256 | 71.17 / 75.99 ms | 40.16 / 42.47 ms | 36.11 / 40.92 ms | 1.77x | 1.97x |
| 510 | 161.75 / 170.18 ms | 89.54 / 94.03 ms | 79.15 / 88.44 ms | 1.81x | 2.04x |

At 510 tokens:

| Intra-op threads | CPU-time / wall-time | CPU ms per window | Effective source tokens/s with 510/64 overlap |
|---:|---:|---:|---:|
| 1 | 0.99 | 160.65 | 2,757 |
| 2 | 1.99 | 178.35 | 4,981 |
| 4 | 3.92 | 310.70 | 5,634 |

Two threads reduce mean 510-token latency by 44.6% from one thread. Four
threads reduce it by a further 11.6%, but consume 74.2% more total CPU time per
window than two threads. The four-thread p95 improvement over two threads at
510 tokens is 5.9%, and its 16- and 128-token p95 results are worse. Two threads
are therefore selected as the smallest measured configuration with a material
latency and throughput gain.

Median process-session load was 478, 592, and 517 ms for one, two, and four
threads. Median peak RSS was 1.44, 1.30, and 1.46 GB respectively. The process
and filesystem-cache variation is too large to claim a memory or load-time
advantage for any thread count.

All 150 measured outputs per sequence and configuration were bitwise identical
to their process baseline. Across all nine fresh processes and all three thread
counts, each sequence length produced one raw-logit SHA-256:

| Tokens | Raw-logit SHA-256 |
|---:|---|
| 16 | `df3a0629ef25394a39f914f34061380c4c097f154313399ed98813a3a404fb40` |
| 128 | `ef909bc9ee80dd13442c35a7137f690de8592f86aaf9bb92e37c81d249a8c339` |
| 256 | `6f760cbde8fb397a7946ebaa57cd9c04220fe9e7c94de2be2df5360c883968ab` |
| 510 | `656f62b97870a5c2b8988e342cebf2c41c992a8bc0d0958e4deadf8914a1d68f` |

Temporary evidence identities:

- benchmark source SHA-256:
  `a7dc106982f0785a5e2eb017d321b7ca30041cc0aca5f4857df8302034268aad`;
- compiled probe SHA-256:
  `c4de4985cb53ff905cf6cea33ff23e80722ca392b7cd7071130fc3f6be6f0d26`;
- sorted nine-report manifest SHA-256:
  `f472712bad2105aca30455dbd7b280be65e30ff1cf04e4194b33d8319077d7be`;
- sorted 36-logit-file manifest SHA-256:
  `de65e0f53304471b8c1024d4d55bc74525163d9ded0d0a3d86ecd4f223ece1b2`.

The temporary source, reports, and binaries are not retained. This is bounded
research evidence, not a durable repository benchmark. Energy, idle-session
CPU behavior, app integration, other Apple Silicon models, and complete-job
throughput remain unmeasured.

## Proposed NER Budgets for the Spec

| Measure | Proposed first-slice budget | Evidence state |
|---|---:|---|
| bundled model + tokenizer + runtime dylib | at most 760,000,000 bytes | 751,576,791 observed |
| complete signed application | at most 850,000,000 bytes | unmeasured |
| NER-process peak RSS | at most 2,000,000,000 bytes | 0.95–1.46 GB load increase observed |
| warm 510-token CPU window | p95 at most 125 ms on the approved reference Mac | 94.03 ms p95 with selected two-thread configuration |
| model session load | at most 2,000 ms on the approved reference Mac | 225–863 ms observed with cached local files |
| cancellation observation | before and after every window | selected two-thread maximum was 96.32 ms in 150 measured 510-token runs |
| runtime network requests | zero | selected feature set has no runtime download feature; app not built |

The spec must either approve or amend these budgets. Only the exact measured
host has evidence; support for other Macs remains unproved.

## Determinism Boundary

Five complete 30-case Python replays produced one output hash:
`5d861060d2d396165e33b2c68519d46a7f1b4b0acefe2441811430094c0160c9`.
The native probe produced bitwise-identical logits for 100 repeats at every
tested sequence length. The thread-count addendum also produced one raw-logit
hash per sequence length across 1, 2, and 4 threads and nine fresh processes.

This supports the recorded same-host, same-artifact CPU replays, including the
nine thread-count processes. It does not prove bitwise equality across:

- unmeasured process or host environments;
- macOS or ONNX Runtime versions;
- different Apple Silicon generations;
- compiler or optimization changes;
- Core ML or other execution providers;
- chunking, overlap, extraction, entity resolution, or full repository output.

Outis determinism therefore binds the model revision, artifact hashes,
tokenizer hash, runtime version and dylib hash, CPU provider, thread counts,
optimization level, chunking policy, label mapping, confidence calculation,
overlap policy, and canonical byte offsets.

## Rejected Candidates

### Davlan multilingual DistilBERT

The smaller artifact is attractive, and its person results were comparable.
It was rejected for the first slice because:

- the publisher tree contains no ONNX artifact;
- using ONNX would require an Outis-controlled conversion and new parity
  evidence;
- direct safetensors use would require a different runtime and a classification
  head integration;
- its exact organization results on this corpus were lower in all three
  languages;
- the temporary Transformers run emitted a tokenizer warning, so that path is
  not an acceptable production reference.

This does not prove the model is generally worse. It closes only the first-slice
integration decision.

### GLiNER multi v2.1

GLiNER can accept open labels such as postal address and its Apache-2.0 metadata
is attractive. It was rejected for the first slice because:

- the pinned safetensors artifact is 1,155,830,112 bytes;
- the publisher model repository contains no ready ONNX artifact;
- the official project documents a conversion step and a larger custom
  preprocessing/postprocessing contract;
- no exact Italian, German, and French legal-domain results were available for
  the pinned artifact in the reviewed model card;
- evaluating and packaging it would add conversion and runtime work before the
  minimal local loop exists.

GLiNER may be reconsidered in a later lifecycle increment after the first slice
provides a durable corpus and benchmark harness.

## Risks and Unknowns

- The selected model was trained on dated news corpora, not Swiss legal
  documents; transfer quality is unproved.
- Model-card license metadata does not by itself clear training-data or weight
  redistribution rights.
- The model config and card disagree about DATE labels.
- Confidence is not calibrated for Outis classes.
- Postal-address completion remains outside the NER model; R1 defines a narrow
  reviewed assembler candidate, but no implementation or oracle exists.
- Mixed-language, code-switching, OCR noise, decomposed Unicode, long tokens,
  Markdown syntax, and chunk boundaries are not evaluated.
- The smoke corpus prevalence is artificial and its precision values do not
  predict field precision.
- The runtime and tokenizer binding are release/version sensitive.
- `ort` is a release candidate and exposes a large unsafe FFI boundary through
  `ort-sys`.
- The selected payload is large for a simple utility.
- Memory observations varied substantially across fresh processes.
- Full Xcode, application sandbox, signing, notarization, bundle loading, and
  cancellation are untested.
- Model errors can leave plaintext in the agent export unless review and
  publication gates catch them.

## R1.1 Exit Classification

`R1.1_MODEL_SELECTED_FOR_SPEC_REVIEW`

The undefined-model blocker is closed: one exact model, tokenizer, runtime,
provider, binding candidate, input policy, label mapping, threshold policy,
artifact identity, and provisional budget set now exists.

This R1.1 selection remains valid research input. R1.2 subsequently selected
the required extraction path and the broader R1 phase is now closed for S1
drafting in `outis_local_pilot_r1_decision_closure.md`. Code remains blocked
because no approved pilot spec, peer-audit pass, implementation plan,
dependency approval, model legal clearance, generated binding, full-Xcode
identity, or product artifact exists.
