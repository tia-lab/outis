mod candidate;
mod detect;

pub use candidate::{
    CandidateEvidenceV1, CandidateStatusV1, CandidateSurfaceV1, EmailDiscoveryErrorV1,
    SensitiveCandidateV1, SensitiveClassV1,
};
pub use detect::{MAX_EMAIL_CANDIDATES_PER_SURFACE, detect_email_candidates};
