use crate::helpers::db::get_pool;
use crate::helpers::users::basic_user;
use actix_web::dev::*;
use actix_web::http::header::HeaderMap;
use actix_web::test;
use actix_web::web;
use actix_web::HttpMessage;
use base64::decode;
use base64::encode;
use cis_client::AsyncCisClientTrait;
use cis_profile::schema::Profile;
use dino_park_gate::scope::ScopeAndUser;
use dino_park_trust::AALevel;
use dino_park_trust::GroupsTrust;
use dino_park_trust::Trust;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use dino_park_cis::*;

#[derive(Serialize, Deserialize)]
pub struct Soa {
    pub user_id: String,
    pub scope: Trust,
    pub groups_scope: GroupsTrust,
    pub aa_level: AALevel,
}

impl Soa {
    pub fn new(user_id: &str, scope: Trust, groups_scope: GroupsTrust, aa_level: AALevel) -> Self {
        Soa {
            user_id: user_id.to_owned(),
            scope,
            groups_scope,
            aa_level,
        }
    }

    pub fn creator(mut self) -> Self {
        self.groups_scope = GroupsTrust::Creator;
        self
    }

    pub fn admin(mut self) -> Self {
        self.groups_scope = GroupsTrust::Admin;
        self
    }

    pub fn aal_medium(mut self) -> Self {
        self.aa_level = AALevel::Medium;
        self
    }

    pub fn encode(&self) -> String {
        encode(serde_json::to_vec(self).unwrap())
    }
}

impl From<&Profile> for Soa {
    fn from(p: &Profile) -> Self {
        let scope = if p.staff_information.staff.value == Some(true) {
            Trust::Staff
        } else if p
            .access_information
            .mozilliansorg
            .values
            .as_ref()
            .map(|v| v.0.contains_key("nda"))
            .unwrap_or_default()
        {
            Trust::Ndaed
        } else {
            Trust::Authenticated
        };
        Soa {
            user_id: p.user_id.value.clone().unwrap(),
            scope,
            groups_scope: GroupsTrust::None,
            aa_level: AALevel::Low,
        }
    }
}

impl From<Soa> for ScopeAndUser {
    fn from(soa: Soa) -> Self {
        ScopeAndUser {
            user_id: soa.user_id,
            scope: soa.scope,
            groups_scope: soa.groups_scope,
            aa_level: soa.aa_level,
        }
    }
}

pub fn nobody_soa() -> Soa {
    Soa::new("nobody", Trust::Public, GroupsTrust::None, AALevel::Unknown)
}

fn scope_from_sau_str(sau: &str) -> ScopeAndUser {
    let j = decode(sau).unwrap();
    serde_json::from_slice::<Soa>(&j).unwrap().into()
}

fn scope_from_headers(headers: &HeaderMap) -> ScopeAndUser {
    scope_from_sau_str(headers.get("sau").map(|v| v.to_str().unwrap()).unwrap())
}

pub async fn read_json<B: MessageBody>(res: ServiceResponse<B>) -> Value {
    serde_json::from_slice(test::read_body(res).await.as_ref()).unwrap()
}

pub async fn test_app() -> impl HttpServiceFactory {
    let pool = get_pool();
    web::scope("")
        .data(pool.clone())
        .service(healthz::healthz_app())
        .service(
            web::scope("/cis/api")
                .wrap_fn(|req, srv| {
                    srv.call(req)
                })
                .service(api::change::change_app())
        )
}
