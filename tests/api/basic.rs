
use crate::helpers::api::*;
use crate::helpers::db::reset;
use crate::helpers::misc::read_json;
use crate::helpers::misc::test_app;
use crate::helpers::users::basic_user;
use actix_web::test;
use actix_web::App;
use failure::Error;
use serde_json::json;

#[actix_rt::test]
async fn basic() -> Result<(), Error> {
    reset()?;
    let app = App::new().service(test_app().await);
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/cis/api/change/v2").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
    assert_eq!(read_json(res).await, json!("Mozilla Change Integration Service Endpoint"));

    let req = test::TestRequest::get().uri("/cis/api/change/v2/version").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
    assert_eq!(read_json(res).await, json!("0.0.1"));
    Ok(())
}