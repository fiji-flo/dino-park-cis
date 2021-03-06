use crate::helpers::misc::Soa;
use actix_http::Request;
use actix_web::dev::*;
use actix_web::test;
use serde::Serialize;

pub async fn get<S, B, E>(mut app: &mut S, endpoint: &str, scope: &Soa) -> S::Response
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::get()
        .header("sau", scope.encode())
        .uri(endpoint)
        .to_request();
    test::call_service(&mut app, req).await
}

pub async fn delete<S, B, E>(mut app: &mut S, endpoint: &str, scope: &Soa) -> S::Response
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::delete()
        .header("sau", scope.encode())
        .uri(endpoint)
        .to_request();
    test::call_service(&mut app, req).await
}

pub async fn post<S, B, E>(
    mut app: &mut S,
    endpoint: &str,
    json: impl Serialize,
    scope: &Soa,
) -> S::Response
where
    S: Service<Request = Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let req = test::TestRequest::post()
        .header("sau", scope.encode())
        .uri(endpoint)
        .set_json(&json)
        .to_request();
    test::call_service(&mut app, req).await
}
