~~~
WUTHIER TERMINAL PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 WUTHIER TERMINAL. All Rights Reserved.
~~~

# `outis-core`

- `src/lib.rs`: intentional public domain API exports.
- `src/candidate.rs`: MI-01 and MI-02 candidate records, enums, and typed limit errors.
- `src/detect.rs`: MI-01 and MI-02 detector module ownership and public entrypoints.
- `src/detect/email.rs`: deterministic email scanner, grammar, equality key, and fixed output ceiling.
- `src/detect/email/tests.rs`: private MI-01 unit oracle.
- `src/detect/telephone.rs`: deterministic telephone scanner, classification, equality key, and fixed output ceiling.
- `src/detect/telephone/tests.rs`: private MI-02 telephone unit oracle.
- `src/detect/iban.rs`: deterministic IBAN scanner, structure, MOD-97, equality key, and fixed output ceiling.
- `src/detect/iban/tests.rs`: private MI-02 IBAN unit oracle.
