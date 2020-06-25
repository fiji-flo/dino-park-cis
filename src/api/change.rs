use actix_web::dev::HttpServiceFactory;
use actix_web::web;
use actix_web::HttpResponse;

const VERSION: &str =  env!("CARGO_PKG_VERSION");

fn change_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn index() -> HttpResponse {
    HttpResponse::Ok().json("Mozilla Change Integration Service Endpoint")
}

fn version() -> HttpResponse {
    HttpResponse::Ok().json(VERSION)
}

pub fn change_app() -> impl HttpServiceFactory {
    web::scope("/change/v2")
        .service(web::resource("/user").route(web::post().to(change_user)))
        .service(web::resource("/version").to(version))
        .service(web::resource("").to(index))
}
