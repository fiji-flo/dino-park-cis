use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use cis_profile::schema::PublisherAuthority;
use futures::FutureExt;
use futures::TryFutureExt;
use headers::CacheControl;
use headers::HeaderMapExt;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use shared_expiry_get::Expiry;
use shared_expiry_get::ExpiryFut;
use shared_expiry_get::ExpiryGetError;
use shared_expiry_get::Provider;
use shared_expiry_get::RemoteStore;

lazy_static! {
    pub static ref PUBLISHER_RULES: RemoteStore<RemotePublisherRules, RemotePublisherRulesProvider> =
        RemoteStore::new(RemotePublisherRulesProvider {
            url: String::from("https://auth.mozilla.com/.well-known/mozilla-iam-publisher-rules"),
        });
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Publishers {
    Many(Vec<PublisherAuthority>),
    One(Publisher),
}

impl Publishers {
    pub fn check(&self, publisher: &PublisherAuthority) -> bool {
        match self {
            Self::Many(v) => v.contains(publisher),
            Self::One(p) => *p == *publisher,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Publisher {
    #[serde(rename = "")]
    None,
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "mozilliansorg")]
    Mozilliansorg,
    #[serde(rename = "hris")]
    Hris,
    #[serde(rename = "cis")]
    Cis,
    #[serde(rename = "access_provider")]
    AccessProvider,
}

impl PartialEq<PublisherAuthority> for Publisher {
    fn eq(&self, other: &PublisherAuthority) -> bool {
        match (self, other) {
            (Publisher::Ldap, PublisherAuthority::Ldap)
            | (Publisher::Mozilliansorg, PublisherAuthority::Mozilliansorg)
            | (Publisher::Hris, PublisherAuthority::Hris)
            | (Publisher::Cis, PublisherAuthority::Cis)
            | (Publisher::AccessProvider, PublisherAuthority::AccessProvider) => true,
            _ => false,
        }
    }
}

impl From<Publisher> for Vec<PublisherAuthority> {
    fn from(p: Publisher) -> Self {
        match p {
            Publisher::None => vec![],
            Publisher::Ldap => vec![PublisherAuthority::Ldap],
            Publisher::Mozilliansorg => vec![PublisherAuthority::Mozilliansorg],
            Publisher::Hris => vec![PublisherAuthority::Hris],
            Publisher::Cis => vec![PublisherAuthority::Cis],
            Publisher::AccessProvider => vec![PublisherAuthority::AccessProvider],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublisherRules {
    pub create: Rules,
    pub update: Rules,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rules {
    pub uuid: Publishers,
    pub user_id: Publishers,
    pub primary_username: Publishers,
    pub login_method: Publishers,
    pub active: Publishers,
    pub last_modified: Publishers,
    pub created: Publishers,
    pub usernames: Publishers,
    pub pronouns: Publishers,
    pub first_name: Publishers,
    pub last_name: Publishers,
    pub alternative_name: Publishers,
    pub primary_email: Publishers,
    pub ssh_public_keys: Publishers,
    pub pgp_public_keys: Publishers,
    pub fun_title: Publishers,
    pub description: Publishers,
    pub location: Publishers,
    pub timezone: Publishers,
    pub languages: Publishers,
    pub tags: Publishers,
    pub picture: Publishers,
    pub uris: Publishers,
    pub phone_numbers: Publishers,

    pub identities: IdentityRules,
    pub access_information: AccessInformationRules,
    pub staff_information: StaffInformationRules,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum IdentityRules {
    Different {
        github_id_v3: Publishers,
        github_id_v4: Publishers,
        github_primary_email: Publishers,
        mozilliansorg_id: Publishers,
        bugzilla_mozilla_org_id: Publishers,
        bugzilla_mozilla_org_primary_email: Publishers,
        mozilla_ldap_id: Publishers,
        mozilla_ldap_primary_email: Publishers,
        mozilla_posix_id: Publishers,
        google_oauth2_id: Publishers,
        google_primary_email: Publishers,
        firefox_accounts_id: Publishers,
        firefox_accounts_primary_email: Publishers,
        custom_1_primary_email: Publishers,
        custom_2_primary_email: Publishers,
        custom_3_primary_email: Publishers,
    },
    Same(Publishers),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AccessInformationRules {
    Different {
        access_provider: Publishers,
        ldap: Publishers,
        hris: Publishers,
        mozilliansorg: Publishers,
    },
    Same(Publishers),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum StaffInformationRules {
    Different {
        manager: Publishers,
        director: Publishers,
        staff: Publishers,
        title: Publishers,
        team: Publishers,
        cost_center: Publishers,
        worker_type: Publishers,
        wpr_desk_number: Publishers,
        office_location: Publishers,
    },
    Same(Publishers),
}

pub struct RemotePublisherRulesProvider {
    url: String,
}

#[derive(Clone, Debug)]
pub struct RemotePublisherRules {
    pub rules: PublisherRules,
    pub valid_till: DateTime<Utc>,
}

impl Expiry for RemotePublisherRules {
    fn valid(&self) -> bool {
        Utc::now() < self.valid_till
    }
}

impl Provider<RemotePublisherRules> for RemotePublisherRulesProvider {
    fn update(&self) -> ExpiryFut<RemotePublisherRules> {
        reqwest::get(reqwest::Url::parse(&self.url).unwrap())
            .map_ok(move |res| {
                let headers = res.headers();
                let cc: Option<CacheControl> = headers.typed_get();
                let max_age = cc
                    .and_then(|cc| cc.max_age())
                    .and_then(|max_age| Duration::from_std(max_age).ok())
                    .unwrap_or_else(|| Duration::hours(24));

                (res, max_age)
            })
            .and_then(move |(res, max_age)| {
                res.json::<PublisherRules>()
                    .map_ok(move |rules| RemotePublisherRules {
                        rules,
                        valid_till: Utc::now() + max_age,
                    })
            })
            .map_err(|e| ExpiryGetError::UpdateFailed(e.to_string()))
            .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_remote() {
        let rules = PUBLISHER_RULES.get().await;
        assert!(rules.is_ok());
    }
}
