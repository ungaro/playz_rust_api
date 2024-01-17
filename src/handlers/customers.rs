use crate::model::customer::Customer;
use actix_web::web;
use qstring::QString;
use r2d2_redis::{r2d2, RedisConnectionManager};

use actix_web::{HttpRequest, HttpResponse};

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::customer::CustomerList;

#[path = "../cache.rs"]
mod cache;
pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::Infallible;
use thiserror::Error;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Vec<String> {
    let req_headers = req.headers();
    let basic_auth_header = req_headers.get("Authorization");
    let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
    let basic_auth: &str = &basic_auth[7..basic_auth.chars().count()];

    let bytes = &base64::decode(&basic_auth).unwrap();
    let bytes = std::str::from_utf8(bytes).clone();
    let bytes = bytes.unwrap().split(":");
    let vec = &bytes.collect::<Vec<&str>>();


    let customer_id = vec[0].to_string();
    let token = vec[1].to_string();
    return vec![customer_id, token];
}

pub async fn get(
    param: web::Path<String>,
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
    redispool: web::Data<R2D2Pool>,
) -> Result<HttpResponse, HttpResponse> {
    let mut tokens: Vec<String> = get_content_type(&_req);
    let customer_id = param;

    let cache_url = format!("cache:{}:/api/customers/{}", customer_id, customer_id);

    let value = cache::get_str(&redispool, &cache_url);
    println!("Cache_URL__{}", &cache_url);

    let f = match value {
        Ok(value) => {
            println!("HIT----from_cache");
            let parsedjson: Value = match serde_json::from_str(&value) {
                Result::Ok(val) => val,
                Result::Err(err) => {
                    println!("parsing was unsuccessful, cache poisoned");
                    //RE-CREATE CACHE, CALL LAMBDA
                    serde_json::Value::Null
                }
            };
            println!("parsedjson_from_cache__{:?}", parsedjson);
            Ok(HttpResponse::Ok().json(parsedjson))
        }
        Err(error) => {
            let mysql_pool = mysql_pool_handler(pool)?;
            Ok(HttpResponse::Ok().json(CustomerList::list(&mysql_pool)))
        }
    };

    f
}
