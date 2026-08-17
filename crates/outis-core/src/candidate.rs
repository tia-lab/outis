#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensitiveCandidateV1 {
    pub source_snapshot_sha256: [u8; 32],
    pub surface: CandidateSurfaceV1,
    pub path_component_index: Option<u32>,
    pub start_byte: usize,
    pub end_byte: usize,
    pub sensitive_class: SensitiveClassV1,
    pub observed: String,
    pub equality_key: Option<String>,
    pub detector_id: &'static str,
    pub detector_version: u16,
    pub evidence: CandidateEvidenceV1,
    pub status: CandidateStatusV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateSurfaceV1 {
    DocumentText,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SensitiveClassV1 {
    Email,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateEvidenceV1 {
    StructuredGrammar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CandidateStatusV1 {
    Accepted,
    NeedsReview,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EmailDiscoveryErrorV1 {
    CandidateLimitExceeded { limit: usize },
}
