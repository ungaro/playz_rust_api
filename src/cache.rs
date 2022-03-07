//use crate::{R2D2Error::*, Result, REDIS_CON_STRING};
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::time::Duration;
use thiserror::Error;
use R2D2Error::*;
type Result<T> = std::result::Result<T, Error>;
use r2d2_redis::redis::{Commands, FromRedisValue};

//use r2d2_redis_cluster::{r2d2::Pool, RedisClusterConnectionManager};





//pub type R2D2Pool<T> = r2d2::Pool<T>;
//pub type R2D2Con<T> = r2d2::PooledConnection<T>;

pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;
pub type R2D2Con = r2d2::PooledConnection<RedisConnectionManager>;


const CACHE_POOL_MAX_OPEN: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

    //pub fn init()<T> -> Result<r2d2::Pool<T>> {
    pub fn connect() -> Result<r2d2::Pool<RedisConnectionManager>> {

        println!("connect_dotenv_REDIS_URL__{}",dotenv!("REDIS_URL"));

        let manager = RedisConnectionManager::new(dotenv!("REDIS_URL")).map_err(RedisClientError)?;
        r2d2::Pool::builder()
        .max_size(CACHE_POOL_MAX_OPEN)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .min_idle(Some(CACHE_POOL_MIN_IDLE))
        .build(manager)
        .map_err(|e| RedisPoolError(e).into())
    
    

}

pub fn init() -> r2d2::Pool<RedisConnectionManager> {
    connect().expect("Redis_Error")
}



pub fn get_con(pool: &R2D2Pool) -> Result<R2D2Con> {
    println!("get_con_REDIS");

    pool.get_timeout(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS))
    .map_err(|e| {
        eprintln!("error connecting to redis: {}", e);
        RedisPoolError(e).into()
    })
       
}

pub fn set_str(pool: &R2D2Pool, key: &str, value: &str, ttl_seconds: usize) -> Result<()> {
    let mut con = get_con(&pool)?;
    con.set(key, value).map_err(RedisCMDError)?;
    if ttl_seconds > 0 {
        con.expire(key, ttl_seconds).map_err(RedisCMDError)?;
    }
    Ok(())
}

pub fn get_str(pool: &R2D2Pool, key: &str) -> Result<String> {
    println!("get_str_REDIS");

    let mut con = get_con(&pool)?;
    let value = con.get(key).map_err(RedisCMDError)?;
    FromRedisValue::from_redis_value(&value).map_err(|e| RedisTypeError(e).into())
}


#[derive(Error, Debug)]
pub enum Error {
    #[error("direct redis error: {0}")]
    DirectError(#[from] DirectError),
    #[error("r2d2 error: {0}")]
    R2D2Error(#[from] R2D2Error),
}



#[derive(Error, Debug)]
pub enum R2D2Error {
    #[error("could not get redis connection from pool : {0}")]
    RedisPoolError(r2d2_redis::r2d2::Error),
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(r2d2_redis::redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(r2d2_redis::redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(r2d2_redis::redis::RedisError),
}

#[derive(Error, Debug)]
pub enum DirectError {
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(redis::RedisError),
}