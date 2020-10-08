use crate::error::ProfileError;
use crate::profile::publishers::PUBLISHER_RULES;
use cis_profile::schema::AccessInformationProviderSubObject;
use cis_profile::schema::Profile;
use cis_profile::schema::PublisherAuthority;
use cis_profile::schema::StandardAttributeBoolean;
use cis_profile::schema::StandardAttributeString;
use cis_profile::schema::StandardAttributeValues;

const ALLOWED_UPDATORS: [PublisherAuthority; 1] = [PublisherAuthority::Mozilliansorg];

pub enum Operation {
    Create,
    Update,
    Noop,
}

macro_rules! update {
    ($pf:ident, $uf:ident, $v:ident) => {{
        if $pf.metadata.last_modified > $uf.metadata.last_modified {
            return Err(ProfileError::OutdatedUpdate);
        }
        *$pf = $uf;
        Ok(())
    }};
}

fn update_sas(
    field: &mut StandardAttributeString,
    update: StandardAttributeString,
) -> Result<(), ProfileError> {
    update!(field, update, value)
}
fn update_sab(
    field: &mut StandardAttributeBoolean,
    update: StandardAttributeBoolean,
) -> Result<(), ProfileError> {
    update!(field, update, value)
}

fn update_sav(
    field: &mut StandardAttributeValues,
    update: StandardAttributeValues,
) -> Result<(), ProfileError> {
    update!(field, update, values)
}

fn update_saac(
    field: &mut AccessInformationProviderSubObject,
    update: AccessInformationProviderSubObject,
) -> Result<(), ProfileError> {
    update!(field, update, values)
}

macro_rules! update_allowed_any {
    ($($f:ident).*, $p:ident, $u:ident, $s:ident, $v:ident, $c:ident) => {{
        let op = if $p.$($f).* == $u.$($f).* ||
            !($u.$($f).*.$v.is_some() && ($p.$($f).*.$v != $u.$($f).*.$v ||
                $p.$($f).*.metadata.display != $u.$($f).*.metadata.display ||
                $p.$($f).*.metadata.verified != $u.$($f).*.metadata.verified))
        {
            Ok(Operation::Noop)
        } else {
            if $p.$($f).*.$v.is_none() {
                if !update_allowed!($u.$($f).*, $s.create.$($f).*) {
                    Err(ProfileError::PublisherNotAllowedToCreate)
                } else {
                    Ok(Operation::Create)
                }
            } else {
                if !update_allowed!($u.$($f).*, $s.update.$($f).*) && !(
                    $p.$($f).*.$v == $u.$($f).*.$v &&
                    $p.$($f).*.metadata.display != $u.$($f).*.metadata.display &&
                    ALLOWED_UPDATORS.contains(&$u.$($f).*.signature.publisher.name)
                ) {
                    Err(ProfileError::PublisherNotAllowedToUpdate)
                } else {
                    Ok(Operation::Update)
                }
            }
        };
        match op {
            Ok(Operation::Create) => $c(&mut $p.$($f).*, $u.$($f).*),
            Ok(Operation::Update) => $c(&mut $p.$($f).*, $u.$($f).*),
            Ok(Operation::Noop) => Ok(()),
            Err(e) => Err(e),
        }
    }};
}

macro_rules! update_allowed_sas {
    ($($f:ident).*, $p:ident, $u:ident, $s:ident) => {
        update_allowed_any!($($f).*, $p, $u, $s, value, update_sas)
    };
}

macro_rules! update_allowed_sav {
    ($($f:ident).*, $p:ident, $u:ident, $s:ident) => {
        update_allowed_any!($($f).*, $p, $u, $s, values, update_sav)
    };
}

macro_rules! update_allowed_sab {
    ($($f:ident).*, $p:ident, $u:ident, $s:ident) => {
        update_allowed_any!($($f).*, $p, $u, $s, value, update_sab)
    };
}

macro_rules! update_allowed_saac {
    ($($f:ident).*, $p:ident, $u:ident, $s:ident) => {
        update_allowed_any!($($f).*, $p, $u, $s, values, update_saac)
    };
}

macro_rules! update_allowed {
    ($p:expr, $r:expr) => {
        $r.check(&$p.signature.publisher.name)
    };
}

pub async fn update(mut p: Profile, u: Profile) -> Result<Profile, ProfileError> {
    let rules = PUBLISHER_RULES.get().await.unwrap().rules;
    update_allowed_sas!(uuid, p, u, rules)?;
    update_allowed_sas!(user_id, p, u, rules)?;
    update_allowed_sas!(primary_username, p, u, rules)?;
    update_allowed_sas!(login_method, p, u, rules)?;
    update_allowed_sab!(active, p, u, rules)?;
    update_allowed_sas!(last_modified, p, u, rules)?;
    update_allowed_sas!(created, p, u, rules)?;
    update_allowed_sav!(usernames, p, u, rules)?;
    update_allowed_sas!(pronouns, p, u, rules)?;
    update_allowed_sas!(first_name, p, u, rules)?;
    update_allowed_sas!(last_name, p, u, rules)?;
    update_allowed_sas!(alternative_name, p, u, rules)?;
    update_allowed_sas!(primary_email, p, u, rules)?;
    update_allowed_sav!(ssh_public_keys, p, u, rules)?;
    update_allowed_sav!(pgp_public_keys, p, u, rules)?;
    update_allowed_sas!(fun_title, p, u, rules)?;
    update_allowed_sas!(description, p, u, rules)?;
    update_allowed_sas!(location, p, u, rules)?;
    update_allowed_sas!(timezone, p, u, rules)?;
    update_allowed_sav!(languages, p, u, rules)?;
    update_allowed_sav!(tags, p, u, rules)?;
    update_allowed_sas!(picture, p, u, rules)?;
    update_allowed_sav!(uris, p, u, rules)?;
    update_allowed_sav!(phone_numbers, p, u, rules)?;

    update_allowed_sas!(identities.github_id_v3, p, u, rules)?;
    update_allowed_sas!(identities.github_id_v4, p, u, rules)?;
    update_allowed_sas!(identities.github_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.mozilliansorg_id, p, u, rules)?;
    update_allowed_sas!(identities.bugzilla_mozilla_org_id, p, u, rules)?;
    update_allowed_sas!(identities.bugzilla_mozilla_org_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.mozilla_ldap_id, p, u, rules)?;
    update_allowed_sas!(identities.mozilla_ldap_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.mozilla_posix_id, p, u, rules)?;
    update_allowed_sas!(identities.google_oauth2_id, p, u, rules)?;
    update_allowed_sas!(identities.google_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.firefox_accounts_id, p, u, rules)?;
    update_allowed_sas!(identities.firefox_accounts_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.custom_1_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.custom_2_primary_email, p, u, rules)?;
    update_allowed_sas!(identities.custom_3_primary_email, p, u, rules)?;

    update_allowed_saac!(access_information.access_provider, p, u, rules)?;
    update_allowed_saac!(access_information.ldap, p, u, rules)?;
    update_allowed_saac!(access_information.hris, p, u, rules)?;
    update_allowed_saac!(access_information.mozilliansorg, p, u, rules)?;

    update_allowed_sab!(staff_information.manager, p, u, rules)?;
    update_allowed_sab!(staff_information.director, p, u, rules)?;
    update_allowed_sab!(staff_information.staff, p, u, rules)?;
    update_allowed_sas!(staff_information.title, p, u, rules)?;
    update_allowed_sas!(staff_information.team, p, u, rules)?;
    update_allowed_sas!(staff_information.cost_center, p, u, rules)?;
    update_allowed_sas!(staff_information.worker_type, p, u, rules)?;
    update_allowed_sas!(staff_information.wpr_desk_number, p, u, rules)?;
    update_allowed_sas!(staff_information.office_location, p, u, rules)?;
    Ok(p)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::profile::publishers::PublisherRules;
    use chrono::Utc;
    use cis_profile::schema::Display;
    use failure::Error;

    #[tokio::test]
    async fn test_rules() -> Result<(), Error> {
        let rules: PublisherRules =
            serde_json::from_str(include_str!("../../tests/data/rules.json"))?;
        let mut p = Profile::default();
        let mut u = Profile::default();
        u.pronouns.value = Some(String::from("dino"));
        u.pronouns.metadata.last_modified = Utc::now();
        u.user_id.value = Some(String::from("dino"));
        u.user_id.metadata.last_modified = Utc::now();
        u.identities.github_id_v3.value = Some(String::from("dino"));
        u.identities.github_id_v3.metadata.last_modified = Utc::now();
        assert!(update_allowed!(p.pronouns, rules.update.pronouns));
        assert!(update_allowed_sas!(identities.github_id_v4, p, u, rules).is_ok());
        assert!(
            update_allowed_any!(identities.github_id_v3, p, u, rules, value, update_sas).is_ok()
        );
        assert!(update_allowed_sas!(pronouns, p, u, rules).is_ok());
        assert!(update_allowed_sas!(user_id, p, u, rules).is_err());
        assert!(!update_allowed!(p.uuid, rules.update.uuid));
        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> Result<(), Error> {
        let p = Profile::default();
        let mut u = Profile::default();
        u.pronouns.value = Some(String::from("dino"));
        u.pronouns.metadata.last_modified = Utc::now();
        u.identities.github_id_v3.value = Some(String::from("dino"));
        u.identities.github_id_v3.metadata.last_modified = Utc::now();
        let p = update(p, u).await?;
        assert_eq!(p.pronouns.value, Some(String::from("dino")));
        Ok(())
    }

    #[tokio::test]
    async fn test_updators() -> Result<(), Error> {
        let mut o = Profile::default();
        o.primary_email.value = Some(String::from("dino@dino.dino"));
        o.primary_email.metadata.last_modified = Utc::now();
        o.primary_email.metadata.display = Some(Display::Staff);
        o.primary_email.signature.publisher.name = PublisherAuthority::Ldap;
        let p = o.clone();
        let u = o.clone();
        assert!(update(p, u).await.is_ok());
        let p = o.clone();
        let mut u = o.clone();
        u.primary_email.metadata.display = Some(Display::Public);
        u.primary_email.signature.publisher.name = PublisherAuthority::Ldap;
        assert!(update(p, u).await.is_err());
        let p = o.clone();
        let mut u = o.clone();
        u.primary_email.metadata.display = Some(Display::Public);
        u.primary_email.signature.publisher.name = PublisherAuthority::Mozilliansorg;
        update(p, u).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), Error> {
        let mut o = Profile::default();
        o.primary_email.value = Some(String::from("dino@dino.dino"));
        o.primary_email.metadata.last_modified = Utc::now();
        o.primary_email.metadata.display = Some(Display::Staff);
        o.primary_email.signature.publisher.name = PublisherAuthority::Ldap;
        let p = o.clone();
        let u = o.clone();
        let p = update(p, u.clone()).await?;
        assert_eq!(p, u);
        let p = o.clone();
        let mut u = o.clone();
        o.primary_email.metadata.last_modified = Utc::now();
        u.primary_email.value = Some(String::from("mc@dino.dino"));
        u.primary_email.signature.publisher.name = PublisherAuthority::AccessProvider;
        let p = update(p, u).await?;
        assert_eq!(p.primary_email.value, Some(String::from("mc@dino.dino")));
        Ok(())
    }
}
