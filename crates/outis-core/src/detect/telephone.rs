use std::matches;

use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, SensitiveCandidateV1,
    SensitiveClassV1, StructuredDiscoveryErrorV1,
};

use super::{MAX_STRUCTURED_CANDIDATES_PER_SURFACE, is_logical_line_break, scalar_at};

const DETECTOR_ID: &str = "outis.telephone.e164_subset";
const DETECTOR_VERSION: u16 = 1;
const EXTENSION_CUES: [&str; 9] = [
    "extension",
    "durchwahl",
    "interno",
    "poste",
    "ext.",
    "int.",
    "ext",
    "dw",
    "x",
];

pub(super) fn detect_telephone_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1> {
    let mut candidates = Vec::new();
    let mut byte_index = 0;
    let mut previous = None;

    while let Some((scalar, width)) = scalar_at(text, byte_index) {
        if is_start(scalar) {
            let resume = if valid_predecessor(previous) {
                scan_at(source_snapshot_sha256, text, byte_index, &mut candidates)?
            } else {
                scan_body(text, byte_index).0
            };
            if resume > byte_index {
                previous = text[..resume].chars().next_back();
                byte_index = resume;
                continue;
            }
        }
        previous = Some(scalar);
        byte_index += width;
    }

    Ok(candidates)
}

fn scan_at(
    source_snapshot_sha256: [u8; 32],
    text: &str,
    start_byte: usize,
    candidates: &mut Vec<SensitiveCandidateV1>,
) -> Result<usize, StructuredDiscoveryErrorV1> {
    let leading_plus = text.as_bytes().get(start_byte) == Some(&b'+');
    let (body_end, digit_count, formatted) = scan_body(text, start_byte);
    let base_end = trimmed_base_end(text, start_byte, body_end);
    if !(8..=15).contains(&digit_count) || (!leading_plus && !formatted) {
        return Ok(body_end);
    }

    let extension_end = extension_end(text, body_end);
    let trailing_ascii_alphanumeric =
        scalar_at(text, body_end).is_some_and(|(scalar, _)| scalar.is_ascii_alphanumeric());
    let accepted = extension_end.is_none()
        && !trailing_ascii_alphanumeric
        && leading_plus
        && has_supported_country_code(&text[start_byte..base_end]);
    let end_byte = extension_end.unwrap_or(base_end);
    let equality_key = accepted.then(|| equality_key(&text[start_byte..base_end]));
    let status = if accepted {
        CandidateStatusV1::Accepted
    } else {
        CandidateStatusV1::NeedsReview
    };

    emit_candidate(
        source_snapshot_sha256,
        text,
        start_byte,
        end_byte,
        equality_key,
        status,
        candidates,
    )?;
    Ok(extension_end.unwrap_or(body_end))
}

fn scan_body(text: &str, start_byte: usize) -> (usize, usize, bool) {
    let mut byte_index = start_byte;
    let mut digit_count = 0;
    let mut formatted = false;
    while let Some((scalar, width)) = scalar_at(text, byte_index) {
        if is_logical_line_break(scalar) || !is_body_scalar(scalar, byte_index == start_byte) {
            break;
        }
        digit_count += usize::from(scalar.is_ascii_digit());
        formatted |= is_format_scalar(scalar);
        byte_index += width;
    }
    (byte_index, digit_count, formatted)
}

fn is_start(scalar: char) -> bool {
    scalar == '+' || scalar.is_ascii_digit()
}

fn valid_predecessor(previous: Option<char>) -> bool {
    previous.is_none_or(|scalar| {
        scalar.is_whitespace()
            || matches!(
                scalar,
                '(' | '[' | '{' | '<' | '\'' | '"' | ':' | ';' | ',' | '='
            )
    })
}

fn is_body_scalar(scalar: char, at_start: bool) -> bool {
    scalar.is_ascii_digit() || is_format_scalar(scalar) || (at_start && scalar == '+')
}

fn is_format_scalar(scalar: char) -> bool {
    matches!(scalar, ' ' | '\u{00a0}' | '.' | '-' | '(' | ')')
}

fn trimmed_base_end(text: &str, start_byte: usize, mut end_byte: usize) -> usize {
    loop {
        let Some(scalar) = text[start_byte..end_byte].chars().next_back() else {
            return end_byte;
        };
        if !matches!(scalar, ' ' | '\u{00a0}' | '.' | '-' | '(') {
            return end_byte;
        }
        end_byte -= scalar.len_utf8();
    }
}

fn extension_end(text: &str, body_end: usize) -> Option<usize> {
    let cue_start = skip_extension_space(text, body_end);
    for cue in EXTENSION_CUES {
        let cue_end = cue_start.checked_add(cue.len())?;
        let Some(observed_cue) = text.get(cue_start..cue_end) else {
            continue;
        };
        if !observed_cue.eq_ignore_ascii_case(cue) {
            continue;
        }
        if scalar_at(text, cue_end).is_some_and(|(scalar, _)| scalar.is_ascii_alphabetic()) {
            continue;
        }

        let digit_start = skip_extension_space(text, cue_end);
        let mut digit_end = digit_start;
        while let Some((scalar, width)) = scalar_at(text, digit_end) {
            if !scalar.is_ascii_digit() {
                break;
            }
            digit_end += width;
        }
        if digit_end > digit_start {
            return Some(digit_end);
        }
    }
    None
}

fn skip_extension_space(text: &str, mut byte_index: usize) -> usize {
    while let Some((scalar, width)) = scalar_at(text, byte_index) {
        if !matches!(scalar, ' ' | '\u{00a0}') {
            break;
        }
        byte_index += width;
    }
    byte_index
}

fn has_supported_country_code(base: &str) -> bool {
    let mut digits = base.chars().filter(char::is_ascii_digit);
    let first = digits.next();
    let second = digits.next();
    matches!(
        (first, second),
        (Some('3'), Some('3' | '9')) | (Some('4'), Some('1' | '9'))
    )
}

fn equality_key(base: &str) -> String {
    let mut key = String::with_capacity(base.len());
    key.push('+');
    key.extend(base.chars().filter(char::is_ascii_digit));
    key
}

fn emit_candidate(
    source_snapshot_sha256: [u8; 32],
    text: &str,
    start_byte: usize,
    end_byte: usize,
    equality_key: Option<String>,
    status: CandidateStatusV1,
    candidates: &mut Vec<SensitiveCandidateV1>,
) -> Result<(), StructuredDiscoveryErrorV1> {
    if candidates.len() == MAX_STRUCTURED_CANDIDATES_PER_SURFACE {
        return Err(StructuredDiscoveryErrorV1::CandidateLimitExceeded {
            limit: MAX_STRUCTURED_CANDIDATES_PER_SURFACE,
        });
    }
    candidates.push(SensitiveCandidateV1 {
        source_snapshot_sha256,
        surface: CandidateSurfaceV1::DocumentText,
        path_component_index: None,
        start_byte,
        end_byte,
        sensitive_class: SensitiveClassV1::TelephoneNumber,
        observed: text[start_byte..end_byte].to_owned(),
        equality_key,
        detector_id: DETECTOR_ID,
        detector_version: DETECTOR_VERSION,
        evidence: CandidateEvidenceV1::StructuredGrammar,
        status,
    });
    Ok(())
}

#[cfg(test)]
mod tests;
