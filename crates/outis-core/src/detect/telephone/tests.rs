use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, SensitiveCandidateV1,
    SensitiveClassV1, StructuredDiscoveryErrorV1, detect_telephone_candidates,
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
        sensitive_class: SensitiveClassV1::TelephoneNumber,
        observed: observed.to_owned(),
        equality_key: equality_key.map(str::to_owned),
        detector_id: "outis.telephone.e164_subset",
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
    let candidates = detect_telephone_candidates(SOURCE_IDENTITY, input)?;
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
fn accepted_country_subset_and_ranges_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    let cases = [
        ("+33 1 23 45 67 89", 0, 17, "+33123456789"),
        ("+39.02.12345678", 0, 15, "+390212345678"),
        ("+41 (44) 668 18 00", 0, 18, "+41446681800"),
        ("+49-30-12345678", 0, 15, "+493012345678"),
        (
            "+41\u{00a0}44\u{00a0}668\u{00a0}18\u{00a0}00",
            0,
            20,
            "+41446681800",
        ),
        ("+41123456", 0, 9, "+41123456"),
        ("+411234567890123", 0, 16, "+411234567890123"),
        ("Call: +41 44 668 18 00,", 6, 22, "+41446681800"),
    ];
    for (input, start_byte, end_byte, equality_key) in cases {
        assert_single(
            input,
            start_byte,
            end_byte,
            Some(equality_key),
            CandidateStatusV1::Accepted,
        )?;
    }
    Ok(())
}

#[test]
fn review_rejection_and_boundaries_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    for (input, end_byte) in [
        ("079 123 45 67", 13),
        ("+44 20 7946 0958", 16),
        ("+41 44 668 18 00abc", 16),
        ("+41 44 668 18 00 ext.", 16),
        ("+41 44 668 18 00extensionist 4", 16),
    ] {
        assert_single(input, 0, end_byte, None, CandidateStatusV1::NeedsReview)?;
    }

    for input in [
        "+4112345",
        "+4112345678901234",
        "0791234567",
        "+٤١ ٤٤ ٦٦٨ ١٨ ٠٠",
        "é+41 44 668 18 00",
    ] {
        assert_eq!(
            detect_telephone_candidates(SOURCE_IDENTITY, input),
            Ok(Vec::new())
        );
    }

    for input in [
        "+41 44 668 18 00 ",
        "+41 44 668 18 00.",
        "+41 44 668 18 00-",
        "+41 44 668 18 00(",
    ] {
        assert_single(
            input,
            0,
            16,
            Some("+41446681800"),
            CandidateStatusV1::Accepted,
        )?;
    }
    Ok(())
}

#[test]
fn extension_cues_and_digit_runs_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    let cases = [
        ("+41 44 668 18 00\u{00a0}x\u{00a0}1", 22),
        ("+41 44 668 18 00 EXT 12", 23),
        ("+41 44 668 18 00 ext. 123", 25),
        ("+41 44 668 18 00 extension 4", 28),
        ("+41 44 668 18 00 interno 5", 26),
        ("+41 44 668 18 00 int. 6", 23),
        ("+41 44 668 18 00 durchwahl 7", 28),
        ("+41 44 668 18 00 dw 8", 21),
        ("+41 44 668 18 00 poste 9", 24),
        ("+41 44 668 18 00 ext 123456", 27),
        ("+41 44 668 18 00 ext. 1234567", 29),
    ];
    for (input, end_byte) in cases {
        assert_eq!(input.len(), end_byte);
        assert_single(input, 0, end_byte, None, CandidateStatusV1::NeedsReview)?;
    }
    Ok(())
}

#[test]
fn unicode_lines_empty_and_marker_free_are_exact() -> Result<(), StructuredDiscoveryErrorV1> {
    assert_single(
        "é +41 44 668 18 00",
        3,
        19,
        Some("+41446681800"),
        CandidateStatusV1::Accepted,
    )?;
    assert_eq!(
        detect_telephone_candidates(SOURCE_IDENTITY, ""),
        Ok(Vec::new())
    );
    assert_eq!(
        detect_telephone_candidates(SOURCE_IDENTITY, "plain text without telephone marker"),
        Ok(Vec::new())
    );

    for (delimiter, byte_length) in [
        ("\n", 16),
        ("\r\n", 17),
        ("\r", 16),
        ("\u{2028}", 18),
        ("\u{2029}", 18),
    ] {
        let input = format!("+41 44 668{delimiter}18 00");
        assert_eq!(input.len(), byte_length);
        assert_eq!(
            detect_telephone_candidates(SOURCE_IDENTITY, &input),
            Ok(Vec::new())
        );
    }
    Ok(())
}

#[test]
fn replay_is_exact_across_three_runs() -> Result<(), StructuredDiscoveryErrorV1> {
    let input = "+41 44 668 18 00 | 079 123 45 67";
    let expected = vec![
        expected_candidate(
            0,
            16,
            "+41 44 668 18 00",
            Some("+41446681800"),
            CandidateStatusV1::Accepted,
        ),
        expected_candidate(
            19,
            32,
            "079 123 45 67",
            None,
            CandidateStatusV1::NeedsReview,
        ),
    ];
    assert_eq!(input.len(), 32);
    for _ in 0..3 {
        assert_eq!(
            detect_telephone_candidates(SOURCE_IDENTITY, input)?,
            expected
        );
    }
    Ok(())
}

#[test]
fn candidate_limit_accepts_exact_ceiling() -> Result<(), StructuredDiscoveryErrorV1> {
    let input = "+41123456 ".repeat(65_535) + "+41123456";
    let candidates = detect_telephone_candidates(SOURCE_IDENTITY, &input)?;
    assert_eq!(input.len(), 655_359);
    assert_eq!(candidates.len(), 65_536);
    for (index, candidate) in candidates.iter().enumerate() {
        let start_byte = index * 10;
        assert_eq!(
            candidate,
            &expected_candidate(
                start_byte,
                start_byte + 9,
                "+41123456",
                Some("+41123456"),
                CandidateStatusV1::Accepted,
            )
        );
        assert!(input.is_char_boundary(candidate.start_byte));
        assert!(input.is_char_boundary(candidate.end_byte));
    }
    Ok(())
}

#[test]
fn candidate_limit_rejects_next_without_partial_output() {
    let input = "+41123456 ".repeat(65_536) + "+41123456";
    assert_eq!(input.len(), 655_369);
    assert_eq!(
        detect_telephone_candidates(SOURCE_IDENTITY, &input),
        Err(StructuredDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
    );
}
