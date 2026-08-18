use crate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, SensitiveCandidateV1,
    SensitiveClassV1, StructuredDiscoveryErrorV1,
};

use super::{MAX_STRUCTURED_CANDIDATES_PER_SURFACE, is_logical_line_break, scalar_at};

const DETECTOR_ID: &str = "outis.iban.swift_subset";
const DETECTOR_VERSION: u16 = 1;

#[derive(Clone, Copy)]
enum Country {
    Ch,
    De,
    Fr,
    It,
}

impl Country {
    fn from_prefix(first: u8, second: u8) -> Option<Self> {
        match (first.to_ascii_uppercase(), second.to_ascii_uppercase()) {
            (b'C', b'H') => Some(Self::Ch),
            (b'D', b'E') => Some(Self::De),
            (b'F', b'R') => Some(Self::Fr),
            (b'I', b'T') => Some(Self::It),
            _ => None,
        }
    }

    fn length(self) -> usize {
        match self {
            Self::Ch => 21,
            Self::De => 22,
            Self::Fr | Self::It => 27,
        }
    }
}

pub(super) fn detect_iban_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1> {
    let mut candidates = Vec::new();
    let mut byte_index = 0;
    let mut previous = None;

    while let Some((scalar, width)) = scalar_at(text, byte_index) {
        if candidate_start(text, byte_index, previous) {
            let resume = scan_at(source_snapshot_sha256, text, byte_index, &mut candidates)?;
            previous = text[..resume].chars().next_back();
            byte_index = resume;
            continue;
        }
        previous = Some(scalar);
        byte_index += width;
    }

    Ok(candidates)
}

fn candidate_start(text: &str, start_byte: usize, previous: Option<char>) -> bool {
    if previous.is_some_and(|scalar| scalar.is_ascii_alphanumeric()) || start_byte + 4 > text.len()
    {
        return false;
    }
    let prefix = &text.as_bytes()[start_byte..start_byte + 4];
    prefix[0].is_ascii_alphabetic()
        && prefix[1].is_ascii_alphabetic()
        && prefix[2].is_ascii_digit()
        && prefix[3].is_ascii_digit()
}

fn scan_at(
    source_snapshot_sha256: [u8; 32],
    text: &str,
    start_byte: usize,
    candidates: &mut Vec<SensitiveCandidateV1>,
) -> Result<usize, StructuredDiscoveryErrorV1> {
    let bytes = text.as_bytes();
    let country = Country::from_prefix(bytes[start_byte], bytes[start_byte + 1]);
    let exact_length = country.map(Country::length);
    let mut normalized = [0_u8; 27];
    let mut normalized_count = 0;
    let mut byte_index = start_byte;
    let mut last_alphanumeric_end = start_byte;

    while let Some((scalar, width)) = scalar_at(text, byte_index) {
        if is_logical_line_break(scalar) {
            break;
        }
        if scalar.is_ascii_alphanumeric() {
            if normalized_count < normalized.len() {
                normalized[normalized_count] = (scalar as u8).to_ascii_uppercase();
            }
            normalized_count += 1;
            byte_index += width;
            last_alphanumeric_end = byte_index;
            if exact_length == Some(normalized_count) {
                break;
            }
        } else if scalar.is_whitespace() {
            byte_index += width;
        } else {
            break;
        }
    }

    if exact_length == Some(normalized_count) {
        let mut overlength_end = byte_index;
        while let Some((scalar, width)) = scalar_at(text, overlength_end) {
            if !scalar.is_ascii_alphanumeric() {
                break;
            }
            overlength_end += width;
        }
        if overlength_end > byte_index {
            emit_candidate(
                source_snapshot_sha256,
                text,
                start_byte,
                overlength_end,
                None,
                CandidateStatusV1::NeedsReview,
                candidates,
            )?;
            return Ok(overlength_end);
        }

        let Some(country) = country else {
            return Ok(byte_index);
        };
        let accepted = valid_structure(country, &normalized[..normalized_count])
            && mod_97_is_one(&normalized[..normalized_count]);
        let equality_key = accepted.then(|| normalized_key(&normalized[..normalized_count]));
        let status = if accepted {
            CandidateStatusV1::Accepted
        } else {
            CandidateStatusV1::NeedsReview
        };
        emit_candidate(
            source_snapshot_sha256,
            text,
            start_byte,
            last_alphanumeric_end,
            equality_key,
            status,
            candidates,
        )?;
        return Ok(byte_index);
    }

    let minimum = if country.is_some() { 8 } else { 15 };
    if normalized_count >= minimum {
        emit_candidate(
            source_snapshot_sha256,
            text,
            start_byte,
            last_alphanumeric_end,
            None,
            CandidateStatusV1::NeedsReview,
            candidates,
        )?;
    }
    Ok(byte_index)
}

fn valid_structure(country: Country, value: &[u8]) -> bool {
    match country {
        Country::Ch => value[4..9].iter().all(u8::is_ascii_digit),
        Country::De => value[4..22].iter().all(u8::is_ascii_digit),
        Country::Fr => {
            value[4..14].iter().all(u8::is_ascii_digit)
                && value[25..27].iter().all(u8::is_ascii_digit)
        }
        Country::It => {
            value[4].is_ascii_alphabetic() && value[5..15].iter().all(u8::is_ascii_digit)
        }
    }
}

fn mod_97_is_one(value: &[u8]) -> bool {
    let mut remainder = 0_u32;
    for byte in value[4..].iter().chain(value[..4].iter()) {
        if byte.is_ascii_digit() {
            remainder = (remainder * 10 + u32::from(byte - b'0')) % 97;
        } else {
            remainder = (remainder * 100 + u32::from(byte - b'A' + 10)) % 97;
        }
    }
    remainder == 1
}

fn normalized_key(value: &[u8]) -> String {
    value.iter().map(|byte| char::from(*byte)).collect()
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
        sensitive_class: SensitiveClassV1::Iban,
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
