use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::web::to;
use actix_web::{
    error, get, guard, middleware, route, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};

use std::fs::*;
use std::{env, io, io::Read};

use sha256::digest;

async fn not_found() -> impl Responder {
    "404"
}

#[actix_web::get("login/{username}:{password}")]
async fn login(req: HttpRequest) -> HttpResponse {
    let username = req.match_info().get("username").unwrap_or("");
    let password = req.match_info().get("password").unwrap_or("");

    println!("{}", req.connection_info().peer_addr().unwrap());

    if file_exists(("users/".to_string() + username + ".txt").as_str()) {
        let mut file = File::open("users/".to_string() + username + ".txt").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let grabbed_password = (&contents.split('\n').collect::<Vec<&str>>()[1][10..]).to_string();

        if digest(password) == grabbed_password {
            return HttpResponse::Ok().body("ok");
        }

        return HttpResponse::Ok().body("Wrong password");
    }

    HttpResponse::Ok().body("User does not exist")
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

    HttpServer::new(|| App::new().service(login).default_service(to(not_found)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
