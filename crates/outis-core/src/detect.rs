mod email;
mod iban;
mod telephone;

use crate::{EmailDiscoveryErrorV1, SensitiveCandidateV1, StructuredDiscoveryErrorV1};

pub const MAX_EMAIL_CANDIDATES_PER_SURFACE: usize = 65_536;
pub const MAX_STRUCTURED_CANDIDATES_PER_SURFACE: usize = 65_536;

pub fn detect_email_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, EmailDiscoveryErrorV1> {
    email::detect_email_candidates(source_snapshot_sha256, text)
}

pub fn detect_telephone_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1> {
    telephone::detect_telephone_candidates(source_snapshot_sha256, text)
}

pub fn detect_iban_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, StructuredDiscoveryErrorV1> {
    iban::detect_iban_candidates(source_snapshot_sha256, text)
}

pub(super) fn scalar_at(text: &str, byte_index: usize) -> Option<(char, usize)> {
    text[byte_index..]
        .chars()
        .next()
        .map(|scalar| (scalar, scalar.len_utf8()))
}

pub(super) fn is_logical_line_break(scalar: char) -> bool {
    matches!(scalar, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}
