~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis R1 Publication and Platform Contract

Status: R1_SELECTED_FOR_S1_RATIFICATION
Date: 2026-08-17
Classification: research only
Code authorization: none

## Agent Repository

The final directory is named `outis`. Its candidate shape is:

~~~text
outis/
  outis-manifest.json
  <tokenized-source-relative-directories>/
    <tokenized-source-base-name>.md
~~~

Every successfully processed source document has exactly one target `.md`.
The target mirrors its relative directory and base name, except each sensitive
path component is replaced with a stable repository-local path token before
publication. Extension comparison and target-collision checks are
case-insensitive. A collision blocks publication; no file is overwritten and
no undisclosed suffix is added. Outis creates no Git metadata.

The deterministic UTF-8 manifest uses fixed key order, LF endings, no optional
whitespace, and no timestamp. S1 must bind exact bytes and schema. It contains
at least:

- schema and pipeline versions;
- model and detector identities;
- a new non-sensitive opaque export identity unrelated to the vault;
- tokenized relative document paths and SHA-256 values;
- output-tree SHA-256 and class counts;
- `status: complete`.

It contains no plaintext sensitive path component, source metadata, entity,
alias, mapping, confidence, review text, or error detail.

## Staging Validation

Staging is a new same-filesystem sibling named
`.outis-staging-<opaque-job-id>`, mode `0700`. It contains only the candidate
agent tree. Plaintext intermediates and vault files are forbidden.

Before publication, validation must prove:

- every path is an allowed regular file or directory;
- no link, alias, package, mount crossing, extra file, extended-attribute
  payload, resource fork, or forbidden metadata exists;
- manifest counts and hashes match the full staged tree;
- every token matches active grammar and an active private-vault record;
- every synthetic oracle sensitive value, sensitive path component, and
  mapping value is absent from all staged names and bytes;
- source snapshots still match their captured identities and hashes;
- no `needs_review`, blocked, failed, or cancelled state exists.

Known-plaintext scanning proves only absence of declared oracle values. It does
not prove complete discovery.

## Atomic Publication

- First publication renames validated staging to an absent final sibling.
- Replacement requires a valid existing Outis manifest and successful
  `renameatx_np(..., RENAME_SWAP)` behavior.
- Unsupported exchange, cross-filesystem paths, or an unexpected existing
  target block replacement. There is no copy or delete-then-rename fallback.
- After exchange, the old tree occupies the staging name and is removed.
  Cleanup failure is a private warning; the new final tree remains published
  and the old tree cannot be granted to an agent.
- Files receive `F_FULLFSYNC`; directories receive checked `fsync` before
  rename. This is a candidate durability boundary, not a power-loss claim.

Cancellation is observed between bounded operations. During the publication
critical section, Outis completes or rejects the atomic operation before
acknowledging cancellation. Cancellation before publication removes staging
and preserves the old output. Cancellation received after a successful swap is
too late and the job completes.

## macOS Application

S1 candidates:

- deployment target macOS 14.0;
- arm64 only;
- Swift 6 and native SwiftUI/AppKit/Quick Look UI;
- native AppKit, PDFKit, Core Graphics, and Vision extraction owned by the
  application target;
- one application target and one unit-test target;
- App Sandbox enabled;
- user-selected read/write file entitlement enabled;
- no network client, network server, app group, Keychain sharing, automation,
  camera, microphone, location, contacts, or background-service entitlement;
- no persisted security-scoped bookmark;
- no Finder extension;
- one foreground application job.

The main window owns folder selection, start, review, cancellation, terminal
status, and opening the completed export. `MenuBarExtra` mirrors progress and
can activate the window or request cancellation while the app runs. It creates
no background agent and does not survive application exit.

Binary extraction review embeds `QLPreviewView` beside the normalized text.
Quick Look preview fidelity, helper processes, caches, and diagnostics remain
S1 privacy and lifecycle blockers; preview success is not extraction proof.

Progress stages are exactly `validating`, `extracting`, `detecting`,
`needs_review`, `tokenizing`, `validating_export`, `publishing`, then one of
`completed`, `cancelled`, `blocked`, or `failed`. Completion is never displayed
as safe, anonymous, or perfectly detected.

Full Xcode is absent from the measured machine. Exact Xcode, SDK, signing,
Hardened Runtime, notarization, nested ONNX Runtime signing, and distribution
remain S1 approval blockers.

## Rust-to-Swift Boundary

Use an in-process Rust `staticlib` imported through a C ABI. Swift does not
enable C++ interop.

Boundary properties:

- ABI version one and a compatibility query;
- opaque engine and job handles;
- pointer-plus-length byte slices;
- versioned UTF-8 JSON request, event, review, and result payloads;
- polling emits an extraction request carrying an opaque request identifier;
- Swift submits typed extraction status/provenance JSON and extracted UTF-8 as
  separate pointer-plus-length input buffers;
- explicit result codes;
- error payloads contain non-sensitive code and stage, never source data,
  paths, spans, entities, or mappings;
- Rust-owned outputs are released only by `outis_buffer_release`;
- no borrowed pointer survives its call;
- every Rust entrypoint catches panic and returns a stable failure;
- no Rust-to-Swift callback and no re-entrant ABI call;
- one Rust job thread;
- NER uses its fixed two intra-op workers only during inference;
- Swift polls every 100 ms while a job is active;
- cancellation sets an atomic flag and returns immediately;
- every terminal job and engine handle requires release.

Version-one symbols:

~~~text
outis_abi_version
outis_engine_create
outis_engine_release
outis_job_start
outis_job_poll
outis_job_submit_extraction
outis_job_submit_review
outis_job_cancel
outis_job_release
outis_buffer_release
~~~

Rust `repr(C)` declarations are the ABI source. `cbindgen` 0.29.4 generates
`generated/ffi/outis.h`. The macOS target owns a small reviewed source
`module.modulemap`. Header regeneration must produce a byte-clean diff;
generated bindings are never edited manually.

## Native Extraction Boundary

Swift performs one AppKit, PDFKit, or Vision extraction operation on a
background task after Rust polling emits a request. Swift then calls
`outis_job_submit_extraction`. Rust copies and validates the request identifier,
status/provenance payload, and text bytes before the call returns.

The submitted text buffer is Human Zone plaintext and is bounded by the R1.2
per-file and aggregate limits. JSON carries no source text or sensitive path.
No pointer survives its call. There are no Rust-to-Swift callbacks, re-entrant
calls, file handles, platform objects, vault handles, or model handles across
the ABI. S1 must define the exact extraction request schema, ownership,
cancellation races, stale or duplicate response behavior, and ABI tests.

The selected native APIs, exact OCR configuration, measured resource behavior,
and fidelity limits are recorded in
`outis_local_pilot_r1_2_extraction_evaluation.md`.

## Publication Probe

A disposable C probe on 2026-08-17 used the local macOS SDK and APFS
`/System/Volumes/Data`. `renameatx_np(RENAME_SWAP)` exchanged two sibling
directories and preserved both marker trees. The SDK and inspected `libc`
candidate declare `RENAME_SWAP`, `VOL_CAP_INT_RENAME_SWAP`, and `F_FULLFSYNC`.

This is one host, one volume, and one happy path. It does not prove Outis
validation, failure recovery, sync durability, crash behavior, or support on a
user-selected volume. The harness was removed after recording.

## Primary Platform Sources

Sources were read on 2026-08-17.

- [Apple: Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)
- [Apple: App Sandbox](https://developer.apple.com/documentation/security/app-sandbox)
- [Apple: MenuBarExtra](https://developer.apple.com/documentation/swiftui/menubarextra)
- [Apple: Quick Look UI](https://developer.apple.com/documentation/quicklookui)
- [Apple: QLPreviewView](https://developer.apple.com/documentation/quicklookui/qlpreviewview)
- [Swift C++ interoperability overview](https://www.swift.org/documentation/cxx-interop/)
- [Mozilla cbindgen](https://github.com/mozilla/cbindgen)
