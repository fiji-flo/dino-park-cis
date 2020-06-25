use crate::db::model::try_from_profile;
use crate::db::model::ProfileEntry;
use crate::db::schema::profiles;
use cis_profile::schema::Profile;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;

const VERSION_CAP: i32 = 256;

fn previous_version(v: i32) -> i32 {
    if v < 0 {
        0
    } else {
        (VERSION_CAP + v - 1) % VERSION_CAP
    }
}

fn next_version(v: i32) -> i32 {
    if v < 0 {
        0
    } else {
        v + 1 % VERSION_CAP
    }
}

pub fn store_profile(
    connection: &PgConnection,
    p: Profile,
    version: i32,
) -> Result<Profile, Error> {
    let i = try_from_profile(p, next_version(version))?;
    let pe = if version == 0 {
        diesel::insert_into(profiles::table)
            .values(i)
            .get_result::<ProfileEntry>(connection)?
    } else {
        diesel::update(profiles::table)
            .filter(profiles::uuid.eq(i.uuid))
            .filter(profiles::version.eq(previous_version(i.version)))
            .set(i)
            .get_result::<ProfileEntry>(connection)?
    };
    serde_json::from_value(pe.profile).map_err(Into::into)
}
