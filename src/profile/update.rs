use crate::error::ProfileError;
use crate::profile::publishers::IdentityRules;
use crate::profile::publishers::PUBLISHER_RULES;
use cis_profile::schema::Profile;
use cis_profile::schema::StandardAttributeString;
use failure::Error;

fn update_sas(
    field: &mut StandardAttributeString,
    update: StandardAttributeString,
) -> Result<(), ProfileError> {
    if field.metadata.last_modified >= update.metadata.last_modified {
        return Err(ProfileError::OutdatedUpdate);
    }
    Ok(())
}

macro_rules! update_allowed_sas {
    ($p:expr, $r:ident, $u:expr, $s:ident) => {{
        if $u.value.is_some() {
            if $p.value.is_none() {
                if !update_allowed!($p, $s.create.$r) {
                    Err(ProfileError::PublisherNotAllowedToCreate)
                } else {
                    update_sas(&mut $p, $u)
                }
            } else {
                if !update_allowed!($p, $s.update.$r) {
                    Err(ProfileError::PublisherNotAllowedToUpdate)
                } else {
                    update_sas(&mut $p, $u)
                }
            }
        } else {
            Ok(())
        }
    }};
}

macro_rules! update_allowed {
    ($p:expr, $r:expr) => {
        $r.check(&$p.signature.publisher.name)
    };
}
macro_rules! update_allowed_identity {
    ($p:expr, $r:expr, $i:ident) => {
        match $r {
            IdentityRules::Different { $i, .. } => $i.check(&$p.signature.publisher.name),
            IdentityRules::Same(p) => p.check(&$p.signature.publisher.name),
        }
    };
}

pub async fn update(mut p: Profile, u: Profile) -> Result<Profile, ProfileError> {
    let rules = PUBLISHER_RULES.get().await.unwrap().rules;
    update_allowed_sas!(
        p.primary_username,
        primary_username,
        u.primary_username,
        rules
    );
    Ok(p)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use failure::Error;

    #[tokio::test]
    async fn test_rules() -> Result<(), Error> {
        let rules = PUBLISHER_RULES.get().await.unwrap().rules;
        let mut p = Profile::default();
        let mut u = Profile::default();
        u.pronouns.value = Some(String::from("dino"));
        u.pronouns.metadata.last_modified = Utc::now();
        u.user_id.value = Some(String::from("dino"));
        u.user_id.metadata.last_modified = Utc::now();
        assert!(update_allowed!(p.pronouns, rules.update.pronouns));
        assert!(update_allowed_identity!(
            p.identities.github_id_v4,
            rules.update.identities,
            github_id_v4
        ));
        update_allowed_sas!(p.pronouns, pronouns, u.pronouns, rules);
        assert!(update_allowed_sas!(p.user_id, user_id, u.user_id, rules).is_err());
        assert!(!update_allowed!(p.uuid, rules.update.uuid));
        Ok(())
    }
}
