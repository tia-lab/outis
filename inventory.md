```
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.

```

# `outis` - Global Inventory (GENERATED; DO NOT EDIT)

Generated: 2026-08-18T06:04:30Z
Protocol: code-only inventory; docs are excluded from source inventory.

This file is generated from per-component inventories under `crates/*/docs/inventory.md`, `services/*/docs/inventory.md`, and `benchmarks/*/docs/inventory.md`.
Nested crates under `crates/adapters/*` and `crates/schemas/*` are discovered by their `Cargo.toml` files.
Docs, target directories, and vendored dependency directories are excluded from source-file inventory.
If a file purpose is missing in a component inventory, this file will mark it as `INVENTORY GAP`.

## Components

- `crate::outis-core`

---

## `crates/outis-core`

### Source Files

- `crates/outis-core/src/candidate.rs`: MI-01 and MI-02 candidate records, enums, and typed limit errors.
- `crates/outis-core/src/detect.rs`: MI-01 and MI-02 detector module ownership and public entrypoints.
- `crates/outis-core/src/detect/email.rs`: deterministic email scanner, grammar, equality key, and fixed output ceiling.
- `crates/outis-core/src/detect/email/tests.rs`: private MI-01 unit oracle.
- `crates/outis-core/src/detect/iban.rs`: deterministic IBAN scanner, structure, MOD-97, equality key, and fixed output ceiling.
- `crates/outis-core/src/detect/iban/tests.rs`: private MI-02 IBAN unit oracle.
- `crates/outis-core/src/detect/telephone.rs`: deterministic telephone scanner, classification, equality key, and fixed output ceiling.
- `crates/outis-core/src/detect/telephone/tests.rs`: private MI-02 telephone unit oracle.
- `crates/outis-core/src/lib.rs`: intentional public domain API exports.

---
