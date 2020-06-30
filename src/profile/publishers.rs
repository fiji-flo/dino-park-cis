use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use cis_profile::schema::PublisherAuthority;
use failure::Error;
use futures::FutureExt;
use futures::TryFutureExt;
use headers::CacheControl;
use headers::HeaderMapExt;
use lazy_static::lazy_static;
use serde::de;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use shared_expiry_get::Expiry;
use shared_expiry_get::ExpiryFut;
use shared_expiry_get::ExpiryGetError;
use shared_expiry_get::Provider;
use shared_expiry_get::RemoteStore;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

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

    #[serde(deserialize_with = "string_or_struct")]
    pub identities: IdentityRules,
    pub access_information: AccessInformationRules,
    #[serde(deserialize_with = "string_or_struct")]
    pub staff_information: StaffInformationRules,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdentityRules {
    pub github_id_v3: Publishers,
    pub github_id_v4: Publishers,
    pub github_primary_email: Publishers,
    pub mozilliansorg_id: Publishers,
    pub bugzilla_mozilla_org_id: Publishers,
    pub bugzilla_mozilla_org_primary_email: Publishers,
    pub mozilla_ldap_id: Publishers,
    pub mozilla_ldap_primary_email: Publishers,
    pub mozilla_posix_id: Publishers,
    pub google_oauth2_id: Publishers,
    pub google_primary_email: Publishers,
    pub firefox_accounts_id: Publishers,
    pub firefox_accounts_primary_email: Publishers,
    pub custom_1_primary_email: Publishers,
    pub custom_2_primary_email: Publishers,
    pub custom_3_primary_email: Publishers,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessInformationRules {
    pub access_provider: Publishers,
    pub ldap: Publishers,
    pub hris: Publishers,
    pub mozilliansorg: Publishers,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StaffInformationRules {
    pub manager: Publishers,
    pub director: Publishers,
    pub staff: Publishers,
    pub title: Publishers,
    pub team: Publishers,
    pub cost_center: Publishers,
    pub worker_type: Publishers,
    pub wpr_desk_number: Publishers,
    pub office_location: Publishers,
}

impl FromStr for IdentityRules {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let publishers: Publishers = serde_json::from_value(s.into())?;
        Ok(IdentityRules {
            github_id_v3: publishers.clone(),
            github_id_v4: publishers.clone(),
            github_primary_email: publishers.clone(),
            mozilliansorg_id: publishers.clone(),
            bugzilla_mozilla_org_id: publishers.clone(),
            bugzilla_mozilla_org_primary_email: publishers.clone(),
            mozilla_ldap_id: publishers.clone(),
            mozilla_ldap_primary_email: publishers.clone(),
            mozilla_posix_id: publishers.clone(),
            google_oauth2_id: publishers.clone(),
            google_primary_email: publishers.clone(),
            firefox_accounts_id: publishers.clone(),
            firefox_accounts_primary_email: publishers.clone(),
            custom_1_primary_email: publishers.clone(),
            custom_2_primary_email: publishers.clone(),
            custom_3_primary_email: publishers,
        })
    }
}

impl FromStr for AccessInformationRules {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let publishers: Publishers = serde_json::from_value(s.into())?;
        Ok(AccessInformationRules {
            access_provider: publishers.clone(),
            ldap: publishers.clone(),
            hris: publishers.clone(),
            mozilliansorg: publishers,
        })
    }
}

impl FromStr for StaffInformationRules {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let publishers: Publishers = serde_json::from_value(s.into())?;
        Ok(StaffInformationRules {
            manager: publishers.clone(),
            director: publishers.clone(),
            staff: publishers.clone(),
            title: publishers.clone(),
            team: publishers.clone(),
            cost_center: publishers.clone(),
            worker_type: publishers.clone(),
            wpr_desk_number: publishers.clone(),
            office_location: publishers,
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Error>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Error>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
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
        let rules =
            serde_json::from_str::<PublisherRules>(include_str!("../../tests/data/rules.json"));
        assert!(rules.is_ok());
    }
}
