#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

pub mod cache;
pub mod db;
pub mod handlers;
pub mod model;
pub mod schema;

use actix_web::middleware::Logger;
//use actix_web::{web, App, HttpServer};
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::{HttpRequest, HttpResponse};

use db::connect;
use cache::init as cache_init;
use std::sync::Arc;
use r2d2_redis::{r2d2, RedisConnectionManager};



async fn board()  -> Result<HttpResponse, HttpResponse> {
    const DATA:&str = r#"{"code":0,"cached":true,"message":"","messageFriendly":"","data":[{"id":1,"title":"Leauge Of Legends","header":"https://cdn.playzsocial.com/files/static/pre-lol.png","game":2,"index_0":null,"index_1":[18],"index_2":[35],"index_3":[12,13,14,15,16,136],"index_4":[2,28,29,30,31,32],"index_5":[17]},{"id":2,"title":"PUBG","header":"https://cdn.playzsocial.com/files/static/pre-pubg.png","game":3,"index_0":null,"index_1":[19],"index_2":[53,59,60,63],"index_3":[109,110,112,113,94,95,98,99,101,102,105,106],"index_4":[33,34,41,42],"index_5":null},{"id":5,"title":"Fortnite BattleRoyale","header":"https://cdn.playzsocial.com/files/static/pre-fortnite.png","game":7,"index_0":null,"index_1":[131],"index_2":[117,118],"index_3":[121,122,125,126,129,130],"index_4":[134,135],"index_5":null},{"id":6,"title":"Counter Strike: Global Offensive","header":"https://cdn.playzsocial.com/files/static/pre-fortnite.png?version=1","game":1,"index_0":null,"index_1":[142],"index_2":[144],"index_3":[146,147,148],"index_4":[149,150],"index_5":null}]}"#;
    Ok(HttpResponse::Ok().body(DATA))
}

async fn health()  -> Result<HttpResponse, HttpResponse> {
    const HEALTH:&str = r#"{"result":"OK"}"#;
    Ok(HttpResponse::Ok().body(HEALTH))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // let sys = actix::System::new("user_sync");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .data(connect())
        .data(cache_init())
        .service(web::resource("/customers").route(web::get().to(handlers::customers::get)))
        .service(web::resource("/api/posts/bySection").route(web::get().to(handlers::posts::get)))
        .service(web::resource("/api/customers/{id}/followings/posts").route(web::get().to(handlers::posts::customers_followings_posts)))
        .service(web::resource("/api/customers/{id}/posts").route(web::get().to(handlers::posts::customers_posts)))
        .service(web::resource("/api/customers/{id}").route(web::get().to(handlers::customers::get)))
        .service(web::resource("/api/board/schema").route(web::get().to(board)))
        .service(web::resource("/api/health").route(web::get().to(health)))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await




    //println!("Started http server: 127.0.0.1:8088");
    //let _ = sys.run();
}


/*

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // let sys = actix::System::new("user_sync");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(connect())
            .service(web::resource("/").route(web::get().to(handlers::customers::get)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .await

    //println!("Started http server: 127.0.0.1:8088");
    //let _ = sys.run();
}
*/
