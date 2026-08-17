~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# Outis R1 Token and Vault Contract

Status: R1_SELECTED_FOR_S1_RATIFICATION
Date: 2026-08-17
Classification: research only
Code authorization: none

## Token Grammar and Scope

The exact grammar is:

~~~text
{{<class>.<class>_<sequence>}}
~~~

`<class>` is one active class. `<sequence>` is a zero-padded decimal with a
minimum width of four. Examples are `{{person.person_0001}}` and
`{{iban.iban_0001}}`.

Rules:

- counters are independent per class and begin at one;
- allocation follows resolved candidate order after review;
- an existing repository-local entity reuses its token;
- a new entity receives one greater than the stored class maximum;
- width grows beyond four digits without wrapping;
- token and `(class, equality_key)` uniqueness share one vault transaction;
- tokens expose class and repository-local encounter order, but no source
  value, value hash, path, or repository identity;
- equality exists only inside one vault; cross-repository equality and stable
  cross-vault token values are not promised;
- rotation is outside the first slice;
- missing or conflicting mappings block publication.

Source text matching the reserved active-token grammar blocks the job as a
collision. Replacement runs by descending byte offset after overlap closure.

## Store Selection

The synthetic funding-demo candidate is:

- `rusqlite` 0.40.2 with default features disabled and `bundled` enabled;
- observed `libsqlite3-sys` 0.38.2 and bundled SQLite 3.53.2;
- one connection and one writer owned by the active job;
- no database handle, schema, or file handle exposed through FFI;
- application-container path
  `Application Support/Outis/Vaults/<opaque-repository-id>/outis-vault.sqlite3`;
- parent directories mode `0700` and database and journal mode `0600`;
- no WAL, shared cache, network filesystem, source store, export store, or
  user-selected vault path.

The repository identity is random and private. The vault records repository
identity, source snapshots, entities, aliases, content tokens, private source
path to exported-path mappings, path-component tokens, review decisions,
detector versions, and schema version. R1.2 closes the candidate path-
tokenization contract; S1 owns exact tables and SQL. Extraction provenance also
records the source digest, normalized-document schema, extractor mode and
configuration, macOS build, framework environment, OCR revision and language-
support result when applicable. Plaintext extraction buffers are not persisted
as caches.

## Connection and Transaction Contract

Every connection verifies:

~~~text
foreign_keys = ON
journal_mode = DELETE
synchronous = FULL
temp_store = MEMORY
secure_delete = ON
trusted_schema = OFF
busy_timeout = 0
~~~

Schema creation and every entity, review, or token-allocation unit uses
explicit `BEGIN IMMEDIATE`, `COMMIT`, and error-path rollback. Open checks the
schema version and `quick_check`; corruption blocks the job.

Migration is forward-only, transactional, and limited to an exact spec-bound
prior version. Automatic repair, destructive recreation, and downgrade are
forbidden.

## Retention and Deletion

Retention lasts until the user explicitly deletes the repository from Outis.
The first slice has no backup, export, restore, or selective mapping deletion.

Deletion closes the database, removes its database and sidecars, removes the
containing opaque vault directory, and reports any failure. `secure_delete`
does not prove erasure from SSD media, filesystem snapshots, or backups.

## Threat Boundary

The database contains plaintext sensitive mappings. No vault secret or
Keychain item exists in the funding demo.

App Sandbox placement and file modes reduce accidental access. They do not
protect against the same user, root, malware, disk recovery, backups, or an
agent with broad filesystem authority.

The permitted first-demo claim is structural: the vault is outside source and
agent repositories, and the export contains no vault file or mapping. A
confidential-data pilot requires separately approved encrypted-vault,
Keychain, backup, migration, recovery, deletion, and agent-access contracts.

## Dependency Candidates for S1

| Candidate | Owner | Feature direction |
|---|---|---|
| `rusqlite` 0.40.2 | runtime vault adapter | defaults off; `bundled` |
| `serde` 1.0.229 | core wire/domain types | defaults off; `derive`, `std` |
| `serde_json` 1.0.151 | runtime manifest and FFI wire | defaults off; `std` |
| `unicode-normalization` 0.1.25 | core equality keys | defaults off; `std` |
| `sha2` 0.11.0 | core artifact and tree hashing | defaults off |
| `libc` 0.2.183 | macOS publication calls | defaults off |
| `cbindgen` 0.29.4 | external code-generation tool | default CLI feature |

The inspected `libc` release declares `renameatx_np`, `RENAME_SWAP`,
`VOL_CAP_INT_RENAME_SWAP`, and `F_FULLFSYNC`. These candidates are not Cargo
approvals. S1 must bind alternatives, lock state, licenses, crate ownership,
features, and validation.

## Disposable Storage Probe

Recorded on 2026-08-17:

- host: arm64 macOS 26.5, Rust 1.89;
- clean release dependency build: 14.73 seconds;
- SQLite version: 3.53.2;
- observed PRAGMAs: foreign keys one, DELETE journal, synchronous two, memory
  temporary store, secure delete one, trusted schema zero, zero busy timeout;
- rollback left zero records and commit left one;
- `quick_check` returned `ok`;
- pre-created database and generated rollback journal were mode `0600`;
- the journal was absent after close;
- warm probe operation took 1–3 ms.

This is one happy-path disposable probe. It is not crash-recovery, migration,
deletion, confidential-data, or product-build evidence. The harness was
removed after recording.

## Primary SQLite Sources

Sources were read on 2026-08-17.

- [Temporary files and rollback journals](https://sqlite.org/tempfiles.html)
- [Transactions](https://sqlite.org/lang_transaction.html)
- [Atomic commit](https://sqlite.org/atomiccommit.html)
- [Threading modes](https://sqlite.org/threadsafe.html)
