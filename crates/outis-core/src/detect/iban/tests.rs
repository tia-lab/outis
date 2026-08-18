use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, SensitiveCandidateV1,
    SensitiveClassV1, StructuredDiscoveryErrorV1, detect_iban_candidates,
};

const SOURCE_IDENTITY: [u8; 32] = [0xA5; 32];

fn expected_candidate(
    start_byte: usize,
    end_byte: usize,
    observed: &str,
    equality_key: Option<&str>,
    status: CandidateStatusV1,
) -> SensitiveCandidateV1 {
    SensitiveCandidateV1 {
        source_snapshot_sha256: SOURCE_IDENTITY,
        surface: CandidateSurfaceV1::DocumentText,
        path_component_index: None,
        start_byte,
        end_byte,
        sensitive_class: SensitiveClassV1::Iban,
        observed: observed.to_owned(),
        equality_key: equality_key.map(str::to_owned),
        detector_id: "outis.iban.swift_subset",
        detector_version: 1,
        evidence: CandidateEvidenceV1::StructuredGrammar,
        status,
    }
}

fn assert_single(
    input: &str,
    start_byte: usize,
    end_byte: usize,
    equality_key: Option<&str>,
    status: CandidateStatusV1,
) -> Result<(), StructuredDiscoveryErrorV1> {
    let candidates = detect_iban_candidates(SOURCE_IDENTITY, input)?;
    assert!(input.is_char_boundary(start_byte));
    assert!(input.is_char_boundary(end_byte));
    assert_eq!(
        candidates,
        vec![expected_candidate(
            start_byte,
            end_byte,
            &input[start_byte..end_byte],
            equality_key,
            status,
        )]
    );
    Ok(())
}

#[test]
fn registry_compact_and_print_forms_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    let cases = [
        ("CH9300762011623852957", 21, "CH9300762011623852957"),
        ("CH93 0076 2011 6238 5295 7", 26, "CH9300762011623852957"),
        ("DE89370400440532013000", 22, "DE89370400440532013000"),
        ("DE89 3704 0044 0532 0130 00", 27, "DE89370400440532013000"),
        (
            "FR1420041010050500013M02606",
            27,
            "FR1420041010050500013M02606",
        ),
        (
            "FR14 2004 1010 0505 0001 3M02 606",
            33,
            "FR1420041010050500013M02606",
        ),
        (
            "IT60X0542811101000000123456",
            27,
            "IT60X0542811101000000123456",
        ),
        (
            "IT60 X054 2811 1010 0000 0123 456",
            33,
            "IT60X0542811101000000123456",
        ),
    ];
    for (input, end_byte, equality_key) in cases {
        assert_eq!(input.len(), end_byte);
        assert_single(
            input,
            0,
            end_byte,
            Some(equality_key),
            CandidateStatusV1::Accepted,
        )?;
    }
    Ok(())
}

#[test]
fn lowercase_and_unicode_whitespace_normalize_exactly() -> Result<(), StructuredDiscoveryErrorV1> {
    for (input, end_byte, equality_key) in [
        ("ch9300762011623852957", 21, "CH9300762011623852957"),
        ("de89370400440532013000", 22, "DE89370400440532013000"),
        (
            "fr1420041010050500013m02606",
            27,
            "FR1420041010050500013M02606",
        ),
        (
            "it60x0542811101000000123456",
            27,
            "IT60X0542811101000000123456",
        ),
    ] {
        assert_eq!(input.len(), end_byte);
        assert_single(
            input,
            0,
            end_byte,
            Some(equality_key),
            CandidateStatusV1::Accepted,
        )?;
    }
    let thin_space = "CH93\u{2009}0076\u{2009}2011\u{2009}6238\u{2009}5295\u{2009}7";
    assert_eq!(thin_space.len(), 36);
    assert_single(
        thin_space,
        0,
        36,
        Some("CH9300762011623852957"),
        CandidateStatusV1::Accepted,
    )
}

#[test]
fn checksum_structure_and_supported_lengths_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    for (input, end_byte) in [
        ("CH9400762011623852957", 21),
        ("CH93A0762011623852957", 21),
        ("DE89A70400440532013000", 22),
        ("FR14A0041010050500013M02606", 27),
        ("IT6010542811101000000123456", 27),
        ("CH00ABCD", 8),
        ("CH9300762011", 12),
        ("CH9300762011623852957A", 22),
    ] {
        assert_eq!(input.len(), end_byte);
        assert_single(input, 0, end_byte, None, CandidateStatusV1::NeedsReview)?;
    }
    assert_eq!(
        detect_iban_candidates(SOURCE_IDENTITY, "CH00ABC"),
        Ok(Vec::new())
    );
    Ok(())
}

#[test]
fn unsupported_adjacency_punctuation_and_lines_are_exact() -> Result<(), StructuredDiscoveryErrorV1>
{
    let unsupported_34 = format!("GB00{}", "A".repeat(30));
    let unsupported_35 = format!("GB00{}", "A".repeat(31));
    for (input, end_byte) in [
        ("GB00ABCDEFGHIJK", 15),
        (unsupported_34.as_str(), 34),
        (unsupported_35.as_str(), 35),
        ("GB00ABCDEFGHIJK text", 20),
    ] {
        assert_eq!(input.len(), end_byte);
        assert_single(input, 0, end_byte, None, CandidateStatusV1::NeedsReview)?;
    }
    assert_eq!(
        detect_iban_candidates(SOURCE_IDENTITY, "xCH9300762011623852957"),
        Ok(Vec::new())
    );
    assert_single(
        "(CH9300762011623852957)",
        1,
        22,
        Some("CH9300762011623852957"),
        CandidateStatusV1::Accepted,
    )?;
    assert_single(
        "CH9300762011623852957 ordinary",
        0,
        21,
        Some("CH9300762011623852957"),
        CandidateStatusV1::Accepted,
    )?;

    for delimiter in ["\n", "\r\n", "\r", "\u{2028}", "\u{2029}"] {
        let input = format!("CH93 0076{delimiter}2011 6238 5295 7");
        assert_single(&input, 0, 9, None, CandidateStatusV1::NeedsReview)?;
    }
    Ok(())
}

#[test]
fn empty_marker_free_and_replay_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    assert_eq!(detect_iban_candidates(SOURCE_IDENTITY, ""), Ok(Vec::new()));
    assert_eq!(
        detect_iban_candidates(SOURCE_IDENTITY, "plain text without IBAN marker"),
        Ok(Vec::new())
    );

    let input = "CH9300762011623852957|GB00ABCDEFGHIJK";
    let expected = vec![
        expected_candidate(
            0,
            21,
            "CH9300762011623852957",
            Some("CH9300762011623852957"),
            CandidateStatusV1::Accepted,
        ),
        expected_candidate(
            22,
            37,
            "GB00ABCDEFGHIJK",
            None,
            CandidateStatusV1::NeedsReview,
        ),
    ];
    assert_eq!(input.len(), 37);
    for _ in 0..3 {
        assert_eq!(detect_iban_candidates(SOURCE_IDENTITY, input)?, expected);
    }
    Ok(())
}

#[test]
fn candidate_limit_accepts_exact_ceiling() -> Result<(), StructuredDiscoveryErrorV1> {
    let input = "GB00ABCDEFGHIJK|".repeat(65_535) + "GB00ABCDEFGHIJK";
    let candidates = detect_iban_candidates(SOURCE_IDENTITY, &input)?;
    assert_eq!(input.len(), 1_048_575);
    assert_eq!(candidates.len(), 65_536);
    for (index, candidate) in candidates.iter().enumerate() {
        let start_byte = index * 16;
        assert_eq!(
            candidate,
            &expected_candidate(
                start_byte,
                start_byte + 15,
                "GB00ABCDEFGHIJK",
                None,
                CandidateStatusV1::NeedsReview,
            )
        );
        assert!(input.is_char_boundary(candidate.start_byte));
        assert!(input.is_char_boundary(candidate.end_byte));
    }
    Ok(())
}

#[test]
fn candidate_limit_rejects_next_without_partial_output() {
    let input = "GB00ABCDEFGHIJK|".repeat(65_536) + "GB00ABCDEFGHIJK";
    assert_eq!(input.len(), 1_048_591);
    assert_eq!(
        detect_iban_candidates(SOURCE_IDENTITY, &input),
        Err(StructuredDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
    );
}
