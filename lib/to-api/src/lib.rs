use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use regex::Regex;
use to_auth;
use to_config::read_config;

#[get("/trustorg")]
async fn app() -> impl Responder {
    return HttpResponse::Ok().body("APP");
}

#[get("/status")]
async fn status() -> impl Responder {
    return HttpResponse::Ok().body("OK");
}

#[post("/api/login")]
async fn login(user: web::Json<to_auth::User>) -> impl Responder {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(&user.email) || user.password.len() < 10 {
        return HttpResponse::BadRequest().body("Invalid");
    }

    return HttpResponse::Ok().body("Ok");
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
