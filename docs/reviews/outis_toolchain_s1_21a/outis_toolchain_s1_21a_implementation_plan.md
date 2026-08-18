~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis S1-21A Implementation Plan

Status: `APPROVED_BY_USER_SCOPE_DIRECTIVE`
Classification: tooling implementation plan
Date: 2026-08-18
Specification: approved S1-21A
Author pre-audit: `PASSED`
Peer audit: `PEER_AUDIT_PASSED`

## 1. Goal and authorization

Add the already installed Rust 1.89.0 rust-analyzer component to the repository
toolchain declaration. The user's S1-21A approval explicitly authorizes this
exact documentation and toolchain correction plus metadata, check, and
rust-analyzer validation.

No other implementation change is permitted.

## 2. Exact implementation path

| Path | Action | Responsibility |
|---|---|---|
| `rust-toolchain.toml` | modify one line | declare matching rust-analyzer while retaining Clippy and rustfmt |

No file is created, removed, generated, installed, downloaded, or renamed by
the implementation. Review documents are lifecycle evidence, not product or
toolchain implementation paths.

## 3. Exact initial and final bytes

Initial SHA-256:

~~~text
5d660b0669d5123f6528cdaa959c51a202abda7bf8ef373f8ad7047391ef03f3
~~~

Initial component line:

~~~text
components = ["clippy", "rustfmt"]
~~~

Final file:

~~~toml
[toolchain]
channel = "stable"
components = ["clippy", "rust-analyzer", "rustfmt"]
profile = "minimal"
targets = ["aarch64-apple-darwin"]
~~~

Expected final SHA-256:

~~~text
1e76173c772a44718b16da77383a05ab04c01baf148f820580aaf8a92d60b765
~~~

Any other byte blocks.

## 4. Preserved surfaces

These paths remain byte-identical to their state immediately before the
toolchain edit:

- `Cargo.toml`;
- `Cargo.lock`;
- `crates/outis-core/Cargo.toml`;
- all Rust source except the pre-existing user-owned `telephone.rs` diff;
- `.vscode/`, including all user-owned settings;
- `.gitignore`, inventory files, generator, targets, and build configuration;
  and
- every model, fixture, application, Swift, FFI, vault, and export surface.

The current hashes of the first three preserved paths are:

| Path | SHA-256 |
|---|---|
| `Cargo.toml` | `c5be7f8ead089b9c4d03b6dbaceb67bfa1610c3e093f4c5f9da56b63b9a7d26a` |
| `Cargo.lock` | `ba6b438e2751e14a2299aa53232e83e21767a08a8e54f5d20cee3575220fc163` |
| `crates/outis-core/Cargo.toml` | `f592e6d4c4a04d78ba6e1a3b4c1006b4ab49cd22ac53e3475bca8eb65680f8c7` |

## 5. Ordered implementation

1. Verify the initial `rust-toolchain.toml` hash.
2. Verify the matching rust-analyzer component is already installed.
3. Change only the component-array line with `apply_patch`.
4. Verify the complete final file and expected SHA-256.
5. Run the ordered validation below with unreachable rustup distribution
   endpoints and Cargo offline.
6. Record the result review. Do not edit user-owned changes.

## 6. Validation and expected results

All Rust commands use:

~~~text
RUSTUP_TOOLCHAIN=stable
RUSTUP_DIST_SERVER=http://127.0.0.1:9
RUSTUP_UPDATE_ROOT=http://127.0.0.1:9/rustup
CARGO_NET_OFFLINE=true
~~~

Ordered checks:

1. `rustc --version --verbose` reports release `1.89.0`, full commit
   `29483883eed69d5fb4db01964cdf2af4d86e9cb2`, and arm64 Apple host.
2. `cargo --version --verbose` reports release `1.89.0`, full commit
   `c24e1064277fe51ab72011e2612e556ac56addf7`, and arm64 Apple host.
3. `cargo clippy --version` reports Clippy `0.1.89` at the required commit.
4. `rustfmt --version` reports `1.8.0-stable` at the required commit.
5. `rust-analyzer --version` reports `1.89.0`, commit `29483883`, date
   2025-08-04.
6. The installed component list includes cargo, Clippy, rust-analyzer, rustc,
   rustfmt, and the arm64 standard library; the installed target list includes
   `aarch64-apple-darwin`.
7. `rustup which rust-analyzer --toolchain stable-aarch64-apple-darwin`
   resolves the installed matching component, not the VS Code extension
   server.
8. `cargo metadata --locked --offline --format-version 1 --no-deps` exits
   zero.
9. `cargo check --workspace --locked --offline --all-targets` exits zero.
10. Preserved manifest and lock hashes remain exact.
11. `git diff --check` and both roadmap JSON parses pass.
12. The implementation diff for `rust-toolchain.toml` is exactly the one-line
    component addition.

No test-count, performance, privacy, security, or editor-restart claim is made.

## 7. Failure and rollback

Any missing component, identity mismatch, attempted network resolution,
unexpected file diff, Cargo failure, malformed JSON, or hash mismatch blocks.
No expectation is weakened and no toolchain is installed or updated.

Rollback is limited to restoring `rust-toolchain.toml` to the recorded initial
bytes. It must not touch `.vscode/`, `telephone.rs`, documentation from the
active MI-03 work, Cargo files, source, or generated artifacts. No rollback is
planned unless validation fails.

## 8. Risks

- A future mutation of the `stable` alias is detected by the exact identity
  preflight; the declaration itself does not pin the alias.
- Another machine without the component will block under offline tripwires;
  this plan authorizes no installation.
- VS Code requires a manual rust-analyzer restart or window reload after the
  file changes. Terminal validation cannot prove that user-interface action.

## 9. Approval record

The user's 2026-08-18 S1-21A directive approves the exact component addition,
preservation requirements, and validation above. No broader tooling or product
change is authorized.
