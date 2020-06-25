use crate::error::ProfileError;
use crate::profile::publishers::IdentityRules;
use crate::profile::publishers::PUBLISHER_RULES;
use cis_profile::schema::Profile;
use cis_profile::schema::StandardAttributeString;

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
    ($p:ident, $f:ident, $u:ident, $s:ident) => {
        if $u.$f.value.is_some() {
            if $p.$f.value.is_none() {
                if !update_allowed!($p.$f, $s.create.$f) {
                    Err(ProfileError::PublisherNotAllowedToCreate)
                } else {
                    update_sas(&mut $p.$f, $u.$f)
                }
            } else {
                if !update_allowed!($p.$f, $s.update.$f) {
                    Err(ProfileError::PublisherNotAllowedToUpdate)
                } else {
                    update_sas(&mut $p.$f, $u.$f)
                }
            }
        } else {
            Ok(())
        }
    };
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
    update_allowed_sas!(p, primary_username, u, rules)?;
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
        assert!(update_allowed_sas!(p, pronouns, u, rules).is_ok());
        assert!(update_allowed_sas!(p, user_id, u, rules).is_err());
        assert!(!update_allowed!(p.uuid, rules.update.uuid));
        Ok(())
    }
}
