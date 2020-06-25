use crate::error::DBError;
use cis_profile::schema::Display;
use dino_park_trust::Trust;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Clone, DbEnum, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[DieselType = "Trust_type"]
pub enum TrustType {
    Public,
    Authenticated,
    Vouched,
    Ndaed,
    Staff,
}

impl From<Trust> for TrustType {
    fn from(t: Trust) -> Self {
        match t {
            Trust::Staff => TrustType::Staff,
            Trust::Ndaed => TrustType::Ndaed,
            Trust::Vouched => TrustType::Vouched,
            Trust::Authenticated => TrustType::Authenticated,
            Trust::Public => TrustType::Public,
        }
    }
}

impl TryFrom<String> for TrustType {
    type Error = failure::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "staff" => Ok(TrustType::Staff),
            "ndaed" => Ok(TrustType::Ndaed),
            "vouched" => Ok(TrustType::Vouched),
            "authenticated" => Ok(TrustType::Authenticated),
            "public" => Ok(TrustType::Public),
            _ => Err(DBError::InvalidTrustLevel.into()),
        }
    }
}

impl TryFrom<Display> for TrustType {
    type Error = failure::Error;
    fn try_from(d: Display) -> Result<Self, Self::Error> {
        match d {
            Display::Staff => Ok(TrustType::Staff),
            Display::Ndaed => Ok(TrustType::Ndaed),
            Display::Vouched => Ok(TrustType::Vouched),
            Display::Authenticated => Ok(TrustType::Authenticated),
            Display::Public => Ok(TrustType::Public),
            _ => Err(DBError::InvalidTrustLevel.into()),
        }
    }
}
