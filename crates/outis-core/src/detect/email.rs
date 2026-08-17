use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, EmailDiscoveryErrorV1,
    SensitiveCandidateV1, SensitiveClassV1,
};

use super::MAX_EMAIL_CANDIDATES_PER_SURFACE;

const DETECTOR_ID: &str = "outis.email.ascii";
const DETECTOR_VERSION: u16 = 1;

pub(super) fn detect_email_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, EmailDiscoveryErrorV1> {
    let mut candidates = Vec::new();
    let mut span_start = None;

    for (byte_index, scalar) in text.char_indices() {
        if scalar.is_whitespace() {
            if let Some(start_byte) = span_start.take() {
                emit_candidate(
                    source_snapshot_sha256,
                    text,
                    start_byte,
                    byte_index,
                    &mut candidates,
                )?;
            }
        } else if span_start.is_none() {
            span_start = Some(byte_index);
        }
    }

    if let Some(start_byte) = span_start {
        emit_candidate(
            source_snapshot_sha256,
            text,
            start_byte,
            text.len(),
            &mut candidates,
        )?;
    }

    Ok(candidates)
}

fn emit_candidate(
    source_snapshot_sha256: [u8; 32],
    text: &str,
    start_byte: usize,
    end_byte: usize,
    candidates: &mut Vec<SensitiveCandidateV1>,
) -> Result<(), EmailDiscoveryErrorV1> {
    let observed = &text[start_byte..end_byte];
    if !observed.as_bytes().contains(&b'@') {
        return Ok(());
    }

    if candidates.len() == MAX_EMAIL_CANDIDATES_PER_SURFACE {
        return Err(EmailDiscoveryErrorV1::CandidateLimitExceeded {
            limit: MAX_EMAIL_CANDIDATES_PER_SURFACE,
        });
    }

    let equality_key = accepted_equality_key(observed);
    let status = if equality_key.is_some() {
        CandidateStatusV1::Accepted
    } else {
        CandidateStatusV1::NeedsReview
    };

    candidates.push(SensitiveCandidateV1 {
        source_snapshot_sha256,
        surface: CandidateSurfaceV1::DocumentText,
        path_component_index: None,
        start_byte,
        end_byte,
        sensitive_class: SensitiveClassV1::Email,
        observed: observed.to_owned(),
        equality_key,
        detector_id: DETECTOR_ID,
        detector_version: DETECTOR_VERSION,
        evidence: CandidateEvidenceV1::StructuredGrammar,
        status,
    });

    Ok(())
}

fn accepted_equality_key(observed: &str) -> Option<String> {
    let bytes = observed.as_bytes();
    if bytes.len() > 254 || !bytes.is_ascii() {
        return None;
    }

    let mut at_index = None;
    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b'@' {
            if at_index.is_some() {
                return None;
            }
            at_index = Some(index);
        }
    }
    let at_index = at_index?;

    let local_part = &bytes[..at_index];
    let domain = &bytes[at_index + 1..];
    if !valid_local_part(local_part) || !valid_domain(domain) {
        return None;
    }

    let mut equality_key = String::with_capacity(bytes.len());
    equality_key.push_str(&observed[..at_index]);
    equality_key.push('@');
    for byte in domain {
        equality_key.push(char::from(byte.to_ascii_lowercase()));
    }
    Some(equality_key)
}

fn valid_local_part(local_part: &[u8]) -> bool {
    if local_part.is_empty() || local_part.len() > 64 {
        return false;
    }
    if local_part.first() == Some(&b'.') || local_part.last() == Some(&b'.') {
        return false;
    }
    if local_part.windows(2).any(|pair| pair == b"..") {
        return false;
    }

    local_part
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+/=?^_{|}~-.".contains(byte))
}

fn valid_domain(domain: &[u8]) -> bool {
    if domain.is_empty() {
        return false;
    }

    let mut final_label = None;
    for label in domain.split(|byte| *byte == b'.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if label.first() == Some(&b'-') || label.last() == Some(&b'-') {
            return false;
        }
        if !label
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
        {
            return false;
        }
        final_label = Some(label);
    }

    let Some(final_label) = final_label else {
        return false;
    };
    (2..=63).contains(&final_label.len()) && final_label.iter().all(u8::is_ascii_alphabetic)
}

#[cfg(test)]
mod tests;
