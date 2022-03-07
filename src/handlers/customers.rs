use crate::model::customer::Customer;
use actix_web::web;
use qstring::QString;
use r2d2_redis::{r2d2, RedisConnectionManager};


use actix_web::{HttpRequest, HttpResponse};
//use std::io::Error;

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::customer::CustomerList;


#[path="../cache.rs"]
mod cache;
pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;

//use cache::R2D2Pool;
use std::convert::Infallible;
use thiserror::Error;
use serde_json::Value;
use serde::{Deserialize, Serialize};

//mod cache;
/*
async fn r2d2_handler(pool: R2D2Pool) ->web::Data<R2D2Pool>  {
    cache::set_str(&pool, "r2d2_hello", "r2d2_world", 60);
//        .map_err(|e| actix_web::error::Error() )?;
    //let value = cache::get_str(&pool, "r2d2_hello").map_err(|e| actix_web::error::ResponseError.error_response()?;
    let value = cache::get_str(&pool, "r2d2_hello")?;
    Ok(value)
}
*/
fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// NOTE: test API without db conneciton;
// fn users_mock() -> Result<User, Error> {
//     let _users_json = r#"{"id":"89251ab3-1cdc-4629-9086-ce022cf6669e","first_name":"Marek","last_name":"Majdak","email":"info@sufrago.com","name":"sufrago","create_at":"2019-12-17T17:58:07.533406","avatar_id":"1cb15088-afd4-4d00-a7fc-d95eae1abefb","phone_no":"+48666666666","company_name":"Sufrago sp z o.o.","vat_id":"PL 9512468001"}"#;

//     let user: User = serde_json::from_str(_users_json)?;

//     Ok(user)
// }



fn get_content_type<'a>(req: &'a HttpRequest) -> Vec<String>{
    let req_headers = req.headers();
    //println!("{:?}", req_headers);     //req.headers().get("content-type")?.to_str().ok()

    let basic_auth_header = req_headers.get("Authorization");
    let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
    let basic_auth: &str = &basic_auth[7..basic_auth.chars().count()];


    let bytes = &base64::decode(&basic_auth).unwrap();
    let bytes = std::str::from_utf8(bytes).clone();
    let bytes = bytes.unwrap().split(":");
    let vec = &bytes.collect::<Vec<&str>>();



    
    //println!("{}",vec[0]);

    let  customer_id=vec[0].to_string();
    let token = vec[1].to_string();
    //println!("{}",&bytes.1);
return vec![customer_id,token]
   // println!("BYTES___{:?}", &bytes);


}


pub async fn get(
    param: web::Path<String>,
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
    redispool:  web::Data<R2D2Pool>,
) -> Result<HttpResponse, HttpResponse> {

    let mut tokens:Vec<String> = get_content_type(&_req);
    let customer_id=param;
    
    //println!("CUSTOMER_ID_FROM_URL: {}",customer_id);
    //let r2d2_pool = cache::connect().expect("can create r2d2 pool");


    let cache_url = format!("cache:{}:/api/customers/{}",  customer_id,customer_id);


//    let value = cache::get_str(&r2d2_pool, &cache_url);
    let value = cache::get_str(&redispool, &cache_url);
println!("Cache_URL__{}",&cache_url);
    
    //ec2-52-59-174-131.eu-central-1.compute.amazonaws.com:8080

    let f = match value {
        Ok(value) => {
        println!("HIT----from_cache");
        let parsedjson: Value = match serde_json::from_str(&value){
            Result::Ok(val) => {val},
            Result::Err(err) => {
                println!("parsing was unsuccessful, cache poisoned");
                //RE-CREATE CACHE, CALL LAMBDA
                serde_json::Value::Null
            }
        };
        println!("parsedjson_from_cache__{:?}",parsedjson);        
        Ok(HttpResponse::Ok().json(parsedjson))},
        Err(error) =>     {
            let mysql_pool = mysql_pool_handler(pool)?;
            Ok(HttpResponse::Ok().json(CustomerList::list(&mysql_pool)))
        },
    };


f

    //println!("{}","CUSTOMERS_GET");
    //println!("{:?}",param);

    //Ok(HttpResponse::Ok().json(CustomerList::list(&mysql_pool)))

/*
    if let value.unwrap(){
        Ok(HttpResponse::Ok().json(value.unwrap()))

    }else{
}
*/
}

// NOTE: testing connection without DB
// pub async fn get(
//     _req: HttpRequest,
// ) -> Result<HttpResponse, HttpResponse> {
//     let mysql_pool = mysql_pool_handler(pool)?;
//     Ok(HttpResponse::Ok().json(UserList::list(&mysql_pool)))
// }


