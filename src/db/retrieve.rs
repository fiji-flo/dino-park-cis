use crate::profile::display::DisplayFilter;
use crate::db::model::ProfileEntry;
use crate::db::schema::profiles;
use cis_profile::schema::Profile;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;
use uuid::Uuid;
use diesel::pg::expression::dsl::any;

pub fn retrieve_profile(
    connection: &PgConnection,
    uuid: Uuid,
    filter: DisplayFilter,
) -> Result<Profile, Error> {
    let pe = profiles::table
            .filter(profiles::uuid.eq(uuid))
            .filter(profiles::active.eq(any(filter.filter())))
            .first::<ProfileEntry>(connection)?;
    serde_json::from_value(pe.profile).map_err(Into::into)
}
