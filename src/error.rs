#[derive(Fail, Debug, PartialEq)]
pub enum DBError {
    #[fail(display = "db_invalid_profile_v2")]
    InvalidProfile,
    #[fail(display = "db_invalid_trust_level")]
    InvalidTrustLevel,
    #[fail(display = "not_applicable")]
    NotApplicable,
}

#[derive(Fail, Debug, PartialEq)]
pub enum ProfileError {
    #[fail(display = "outdated_update")]
    OutdatedUpdate,
    #[fail(display = "publisher_not_allowed_to_create")]
    PublisherNotAllowedToCreate,
    #[fail(display = "publisher_not_allowed_to_update")]
    PublisherNotAllowedToUpdate,
    #[fail(display = "unknown_error")]
    UnknownError,
}

#[derive(Debug, Fail)]
pub enum SecretsError {
    #[fail(display = "invalid sign key source: use 'none', 'file' or 'ssm'")]
    UseNoneFileSsm,
    #[fail(display = "invalid sign key source: use 'none', 'file' or 'ssm'")]
    UseNoneFileSsmWellKnown,
}
