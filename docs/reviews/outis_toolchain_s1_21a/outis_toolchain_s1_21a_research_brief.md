~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis S1-21A Rust-Analyzer Toolchain Research Brief

Status: `COMPLETE_APPROVED_DIRECTION`
Classification: tooling research; no product code
Date: 2026-08-18

## Decision

The user approved S1-21A on 2026-08-18. The amendment adds the already
installed matching `rust-analyzer` component to `rust-toolchain.toml` while
preserving the `stable` locator, exact Rust 1.89.0 commit, Cargo identity,
Clippy, rustfmt, minimal profile, arm64 target, and offline preflight.

## Measured object

The measured object is editor language-server selection for this workspace.
The installed VS Code extension contains standalone rust-analyzer
`0.3.3016` from 2026-08-16. The workspace selects Rust 1.89.0 from 2025-08-04.
The extension's local selection logic uses `rustup which rust-analyzer` for a
workspace toolchain file only when that file declares the `rust-analyzer`
component. The current file declares only Clippy and rustfmt, so the extension
attempts its newer bundled server and reports the compatibility warning.

The matching installed component resolves to:

~~~text
/Users/tia/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rust-analyzer
rust-analyzer 1.89.0 (29483883 2025-08-04)
~~~

The absolute path is run evidence only. It is not written to repository
configuration.

## Candidate correction

Replace only the toolchain component array:

~~~text
components = ["clippy", "rustfmt"]
~~~

with:

~~~text
components = ["clippy", "rust-analyzer", "rustfmt"]
~~~

No component is removed. No toolchain is installed or updated. No registry,
dependency, manifest, lockfile, source, generated artifact, model, application,
or runtime setting changes.

## Zones, data, and artifacts

This change is developer tooling only. It processes repository Rust source and
Cargo metadata. It does not read a source-document repository, sensitive data,
an agent repository, or a private vault. Human, AI, and Key Zone product flows
are unchanged. No dataset, model artifact, generated code, or sensitive-data
class is involved.

## Evidence

| Question | Evidence | Result |
|---|---|---|
| Does the current Cargo workspace exist? | local path inspection | root and `crates/outis-core/Cargo.toml` exist |
| Is the reported missing-manifest error current? | timestamp and current commands | no; the log is from 2026-08-17 and current offline metadata/check pass |
| Is matching rust-analyzer installed? | `rustup component list`, `rustup which`, version command | yes, matching Rust 1.89.0 commit |
| Why is the extension selecting its bundled server? | code read of installed extension `0.3.3016` | its workspace toolchain detection requires a declared rust-analyzer component |
| Is network access needed for the correction? | installed-component evidence | no installation or update is required |

## Risks and limits

- `stable` remains a mutable alias; every implementation entry still requires
  the exact S1-21 identity preflight.
- Declaring a missing component on another machine may cause rustup to seek an
  installation. Outis validation requires the component to be installed
  before commands and uses unreachable distribution endpoints during proof.
- Command-line resolution proves the matching server is available. Restarting
  the graphical editor remains a user action and is not claimed by terminal
  evidence.
- The current worktree contains user-owned `.vscode/` and
  `crates/outis-core/src/detect/telephone.rs` changes. They must be preserved.

## Correctness oracle

The correction passes only if:

- `rust-toolchain.toml` has the exact approved bytes and SHA-256;
- exact rustc, Cargo, Clippy, rustfmt, and rust-analyzer identities match;
- all required components and the arm64 target are installed under loopback
  rustup endpoints;
- `rustup which rust-analyzer` selects the stable-toolchain component;
- locked offline Cargo metadata and workspace all-target check pass; and
- no unapproved implementation path changes.

## Recommendation

Record S1-21A as a narrow amendment, audit it, bind the one-file
implementation plan, apply the one-line component-array correction, and run
the exact offline oracle. Preserve all user-owned changes.
