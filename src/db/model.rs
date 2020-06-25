use crate::error::DBError;
use crate::db::schema::*;
use crate::db::types::*;
use cis_profile::schema::Profile;
use failure::Error;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

const NDA: [&str; 2] = ["nda", "contingentworkernda"];

#[derive(Identifiable, Insertable, Queryable, PartialEq, Debug, AsChangeset, Serialize)]
#[table_name = "profiles"]
#[primary_key(uuid)]
pub struct ProfileEntry {
    pub uuid: Uuid,
    pub user_id: String,
    pub primary_email: String,
    pub primary_username: String,
    pub active: bool,
    pub trust: TrustType,
    pub version: i32,
    pub profile: Value,
}

fn trust_from(p: &Profile) -> TrustType {
    if let Some(true) = p.staff_information.staff.value {
        return TrustType::Staff;
    }
    if let Some(ref groups) = p.access_information.mozilliansorg.values {
        if NDA.iter().any(|k| groups.0.contains_key(*k)) {
            return TrustType::Ndaed;
        }
    }
    TrustType::Authenticated
}

pub fn try_from_profile(p: Profile, version: i32) -> Result<ProfileEntry, Error> {
    match (
        p.uuid.value.clone(),
        p.user_id.value.clone(),
        p.primary_email.value.clone(),
        p.primary_username.value.clone(),
        p.active.value.clone(),
    ) {
        (Some(uuid), Some(user_id), Some(primary_email), Some(primary_username), Some(active)) => {
            let trust = trust_from(&p);
            let uuid = Uuid::parse_str(&uuid)?;
            Ok(ProfileEntry {
                uuid,
                user_id,
                primary_email,
                primary_username,
                active,
                trust,
                version,
                profile: serde_json::to_value(p)?,
            })
        }
        _ => Err(DBError::InvalidProfile.into()),
    }
}
