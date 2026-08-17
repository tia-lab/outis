mod email;

use crate::{EmailDiscoveryErrorV1, SensitiveCandidateV1};

pub const MAX_EMAIL_CANDIDATES_PER_SURFACE: usize = 65_536;

pub fn detect_email_candidates(
    source_snapshot_sha256: [u8; 32],
    text: &str,
) -> Result<Vec<SensitiveCandidateV1>, EmailDiscoveryErrorV1> {
    email::detect_email_candidates(source_snapshot_sha256, text)
}
