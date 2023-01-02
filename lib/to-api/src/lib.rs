use actix_web::{
    error, get,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use cookie::time::Duration;
use regex::Regex;
use serde::Serialize;
use to_auth;
use to_config::read_config;

#[get("/trustorg")]
async fn app() -> impl Responder {
    return HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(include_str!("../static/login.html"));
}

#[get("/trustorg/status")]
async fn status() -> impl Responder {
    return HttpResponse::Ok().body("OK");
}

#[derive(Serialize)]
struct LoginRes {
    success: bool,
}

#[post("/trustorg/api/login")]
async fn login(user: web::Json<to_auth::User>) -> HttpResponse {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(&user.email) || user.password.len() < 10 {
        let res = LoginRes { success: false };
        return HttpResponse::Ok().json(res);
    }
    let is_auth = to_auth::local_auth(&user);
    println!("{:?}", is_auth);
    if is_auth {
        let token = to_auth::get_token(&user);
        let cookie_header = cookie::Cookie::build("trustorg_token", token)
            .same_site(cookie::SameSite::Strict)
            .secure(true)
            .path("/")
            .max_age(Duration::seconds(360))
            .http_only(true)
            .finish();
        let res = LoginRes { success: true };
        return HttpResponse::Ok().cookie(cookie_header).json(res);
    } else {
        let res = LoginRes { success: false };
        return HttpResponse::Ok().json(res);
    }
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let config = read_config();
    println!("api server started at: http://127.0.01:{}", config.port);

    HttpServer::new(|| App::new().service(login).service(status).service(app))
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}
