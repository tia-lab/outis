use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, EmailDiscoveryErrorV1,
    SensitiveCandidateV1, SensitiveClassV1, detect_email_candidates,
};

const SOURCE_IDENTITY: [u8; 32] = [0xA5; 32];
const MULTIPLE_CANDIDATES: &str = "Préface\na@example.com\tb@EXAMPLE.ORG\nfin";

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
        sensitive_class: SensitiveClassV1::Email,
        observed: observed.to_owned(),
        equality_key: equality_key.map(str::to_owned),
        detector_id: "outis.email.ascii",
        detector_version: 1,
        evidence: CandidateEvidenceV1::StructuredGrammar,
        status,
    }
}

fn assert_single_candidate(
    input: &str,
    end_byte: usize,
    equality_key: Option<&str>,
    status: CandidateStatusV1,
) -> Result<(), EmailDiscoveryErrorV1> {
    let candidates = detect_email_candidates(SOURCE_IDENTITY, input)?;
    assert_eq!(
        candidates,
        vec![expected_candidate(0, end_byte, input, equality_key, status)]
    );
    assert!(input.is_char_boundary(end_byte));
    Ok(())
}

#[test]
fn accepted_ascii_grammar_has_exact_records() -> Result<(), EmailDiscoveryErrorV1> {
    let local_limit = format!("{}@example.com", "a".repeat(64));
    let final_label_limit = format!("a@{}", "b".repeat(63));
    let label_limit = format!("a@{}.com", "b".repeat(63));
    let total_limit = format!(
        "{}@{}.{}.{}",
        "a".repeat(64),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    let cases = [
        ("a@example.com", 13, "a@example.com"),
        ("A.B+tag@Sub.Example.COM", 23, "A.B+tag@sub.example.com"),
        (
            "a!#$%&'*+/=?^_{|}~-b@example.com",
            32,
            "a!#$%&'*+/=?^_{|}~-b@example.com",
        ),
        ("a@com", 5, "a@com"),
        ("a@b.co", 6, "a@b.co"),
    ];

    for (input, end_byte, equality_key) in cases {
        assert_single_candidate(
            input,
            end_byte,
            Some(equality_key),
            CandidateStatusV1::Accepted,
        )?;
    }
    for input in [
        final_label_limit.as_str(),
        local_limit.as_str(),
        label_limit.as_str(),
        total_limit.as_str(),
    ] {
        assert_single_candidate(input, input.len(), Some(input), CandidateStatusV1::Accepted)?;
    }
    Ok(())
}

#[test]
fn invalid_ascii_and_unsupported_forms_need_review() -> Result<(), EmailDiscoveryErrorV1> {
    let cases = [
        (".a@example.com", 14),
        ("a.@example.com", 14),
        ("a..b@example.com", 16),
        ("a@@example.com", 14),
        ("@example.com", 12),
        ("a@", 2),
        ("a@-example.com", 14),
        ("a@example-.com", 14),
        ("a@exam_ple.com", 14),
        ("a@example.12", 12),
        ("a@example.c", 11),
        ("a@exa!mple.com", 14),
        ("\"a\"@example.com", 15),
        ("a(comment)@example.com", 22),
        ("ä@example.com", 14),
        ("a@exämple.com", 14),
        ("a@[127.0.0.1]", 13),
        ("a@.example.com", 14),
        ("a@example..com", 14),
        ("a@example.com.", 14),
        ("a@example.com,", 14),
    ];

    for (input, end_byte) in cases {
        assert_single_candidate(input, end_byte, None, CandidateStatusV1::NeedsReview)?;
    }

    let local_over_limit = format!("{}@example.com", "a".repeat(65));
    let label_over_limit = format!("a@{}.com", "b".repeat(64));
    let total_over_limit = format!(
        "{}@{}.{}.{}",
        "a".repeat(64),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62)
    );
    for input in [
        local_over_limit.as_str(),
        label_over_limit.as_str(),
        total_over_limit.as_str(),
    ] {
        assert_single_candidate(input, input.len(), None, CandidateStatusV1::NeedsReview)?;
    }
    Ok(())
}

#[test]
fn whitespace_unicode_order_and_ranges_are_exact() -> Result<(), EmailDiscoveryErrorV1> {
    let candidates = detect_email_candidates(SOURCE_IDENTITY, MULTIPLE_CANDIDATES)?;
    assert_eq!(MULTIPLE_CANDIDATES.len(), 40);
    assert_eq!(
        candidates,
        vec![
            expected_candidate(
                9,
                22,
                "a@example.com",
                Some("a@example.com"),
                CandidateStatusV1::Accepted,
            ),
            expected_candidate(
                23,
                36,
                "b@EXAMPLE.ORG",
                Some("b@example.org"),
                CandidateStatusV1::Accepted,
            ),
        ]
    );
    for candidate in candidates {
        assert!(MULTIPLE_CANDIDATES.is_char_boundary(candidate.start_byte));
        assert!(MULTIPLE_CANDIDATES.is_char_boundary(candidate.end_byte));
        assert_eq!(
            &MULTIPLE_CANDIDATES[candidate.start_byte..candidate.end_byte],
            candidate.observed
        );
    }
    Ok(())
}

#[test]
fn empty_and_marker_free_inputs_emit_nothing() {
    assert_eq!(detect_email_candidates(SOURCE_IDENTITY, ""), Ok(Vec::new()));
    assert_eq!(
        detect_email_candidates(SOURCE_IDENTITY, "plain text without marker"),
        Ok(Vec::new())
    );
}

#[test]
fn replay_is_exact_across_three_runs() -> Result<(), EmailDiscoveryErrorV1> {
    let first = detect_email_candidates(SOURCE_IDENTITY, MULTIPLE_CANDIDATES)?;
    let second = detect_email_candidates(SOURCE_IDENTITY, MULTIPLE_CANDIDATES)?;
    let third = detect_email_candidates(SOURCE_IDENTITY, MULTIPLE_CANDIDATES)?;
    assert_eq!(first, second);
    assert_eq!(second, third);
    Ok(())
}

#[test]
fn candidate_limit_accepts_exact_ceiling() -> Result<(), EmailDiscoveryErrorV1> {
    let input = "@ ".repeat(65_535) + "@";
    let candidates = detect_email_candidates(SOURCE_IDENTITY, &input)?;
    assert_eq!(input.len(), 131_071);
    assert_eq!(candidates.len(), 65_536);

    for (index, candidate) in candidates.iter().enumerate() {
        let start_byte = index * 2;
        assert_eq!(
            candidate,
            &expected_candidate(
                start_byte,
                start_byte + 1,
                "@",
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
    let input = "@ ".repeat(65_536) + "@";
    assert_eq!(input.len(), 131_073);
    assert_eq!(
        detect_email_candidates(SOURCE_IDENTITY, &input),
        Err(EmailDiscoveryErrorV1::CandidateLimitExceeded { limit: 65_536 })
    );
}
