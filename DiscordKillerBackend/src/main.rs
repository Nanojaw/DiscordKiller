use actix_web::http::header::ContentType;
use actix_web::http::{header, Method, StatusCode};
use actix_web::web::to;
use actix_web::{
    error, get, guard, middleware, route, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};

use std::fs::*;
use std::{env, io, io::Read};

use actix_web::{http, web::Json, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone)]
struct UserSendToClient {
    pub username: String,
    pub servers: Vec<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct UserSendToServer {
    pub username: String,
    pub password: String,
}

async fn not_found() -> impl Responder {
    "404"
}

#[actix_web::get("login/{username}:{password}")]
async fn login(req: HttpRequest) -> Result<HttpResponse> {
    let username = req.match_info().get("username").unwrap_or("none");
    let password = req.match_info().get("password").unwrap_or("none");

    if file_exists(("users/".to_string() + username + ".json").as_str()) {
        let mut file = File::open("users/".to_string() + username + ".json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let user_to_client: UserSendToClient = serde_json::from_str(&contents)?;

        if password == user_to_client.password {
            let resp = HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(contents);

            return Ok(resp);
        }

        return Ok(HttpResponse::Ok().body("Wrong password"));
    }

    return Ok(HttpResponse::Ok().body("User does not exist"));
}

fn dir_exists(path: &str) -> bool {
    // Check if the directory exists
    metadata(path).is_ok()
}
fn file_exists(path: &str) -> bool {
    // Check if the file exists
    metadata(path).is_ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    if !dir_exists("users") {
        create_dir("users").expect("Failed to create directory");
    }

    HttpServer::new(move || App::new().service(login).default_service(to(not_found)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
