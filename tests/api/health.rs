use crate::helpers::api::*;
use crate::helpers::db::reset;
use crate::helpers::misc::nobody_soa;
use crate::helpers::misc::read_json;
use crate::helpers::misc::test_app;
use crate::helpers::misc::Soa;
use crate::helpers::users::basic_user;
use actix_web::test;
use actix_web::App;
use failure::Error;
use serde_json::json;

#[actix_rt::test]
async fn healthz() -> Result<(), Error> {
    reset()?;
    let app = App::new().service(test_app().await);
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/healthz").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
    Ok(())
}
