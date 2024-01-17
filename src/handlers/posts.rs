use crate::model::customer::Customer;
use crate::model::gameprofile::Game;
use crate::model::gameprofile::Gameprofile;
use crate::model::likeconnection::LikeConnection;
use crate::model::post::Post;
use crate::model::post_asset::PostAsset;

#[path = "../cache.rs"]
mod cache;
//use cache::R2D2Pool;
use r2d2_redis::{r2d2, RedisConnectionManager};
pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;

use qstring::QString;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::{MySqlPooledConnection, MysqlPool};
use crate::model::post::PostList;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};
use diesel::sql_query;
use std::collections::HashMap;
use std::io::Error;

use std::collections::hash_map::Entry;

use diesel::sql_types::*;

use crate::schema::customer::dsl::*;
use crate::schema::post::dsl::*;

use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerInfoObject {
    pub id: u32,
    pub customer: u32,
    pub photo: Option<String>,
    pub address: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub birthday: Option<f64>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub county: Option<u32>,
    pub city: Option<u32>,
    pub createdAt: Option<u64>,
    pub updatedAt: Option<u64>,
    pub refcode: Option<String>,
    pub registerRefCode: Option<String>,
    pub taskGroupId: Option<i32>,
    pub missionLevel: Option<i8>,
    pub missionSubLevel: Option<i8>,
    pub missionLevelString: Option<String>,
}

#[derive(Associations, Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct QueryObject {
    pub code: i8,
    pub cached: bool,
    pub message: String,
    pub messageFriendly: String,
    pub data: Vec<PostObject>,
    pub cursor: String,
}

#[derive(Associations, Queryable, Serialize, Deserialize, Clone, Debug)]
pub struct CustomerObject {
    pub id: u32,
    pub active: Option<u8>,
    pub displayName: String,
    //pub info: String,
    pub authLevel: Option<u16>,
    pub banned: Option<u8>,

    pub apnFollowEnabled: Option<u8>,
    pub apnRepostEnabled: Option<u8>,
    pub apnCommentEnabled: Option<u8>,
    pub apnLikeEnabled: Option<u8>,
    pub apnMessageEnabled: Option<u8>,
    pub apnInfoEnabled: Option<u8>,
    pub apnMentionEnabled: Option<u8>,
    pub apnEncodeEnabled: Option<u8>,
    pub apnMatchEnabled: Option<u8>,
    pub apnPanelInviteEnabled: u8,
    pub apnEnabled: Option<u8>,

    pub followerCount: Option<u32>,
    pub followingCount: Option<u32>,
    pub mentionCount: Option<u32>,
    pub mentionedCount: Option<u32>,
    pub notificationCount: Option<u32>,
    pub postCount: Option<u32>,
    pub likeCount: Option<u32>,
    pub commentCount: Option<u32>,

    pub tagCount: Option<u32>,
    pub lastOnline: Option<u64>,
    pub gsm: Option<String>,
    pub sessionId: Option<String>,
    pub isFamous: Option<u8>,
    pub createdAt: Option<u64>,
    pub updatedAt: Option<u64>,
    pub lastIp: Option<String>,
    pub isVerified: Option<bool>,
    pub anonymUser: Option<i16>,
    pub taskLevel: Option<i16>,
    pub firebaseToken: Option<String>,
    /**/
    pub info: CustomerInfoObject,
    pub profiles: Vec<GameprofileObject>,
}

#[derive(Associations, Queryable, Serialize, Deserialize, QueryableByName, Clone, Debug)]
pub struct GameprofileNormalizedObject {
    #[sql_type = "Unsigned<Integer>"]
    pub id: u32,
    #[sql_type = "Varchar"]
    pub selective_id: String,
    #[sql_type = "Unsigned<Integer>"]
    pub game: u32,

    #[sql_type = "Unsigned<Integer>"]
    pub customer: u32,
    #[sql_type = "Varchar"]
    pub username: String,
    #[sql_type = "Nullable<Varchar>"]
    pub region: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub platform: Option<String>,

    #[sql_type = "Nullable<Varchar>"]
    pub avatar: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub uri: Option<String>,
    #[sql_type = "Unsigned<Bigint>"]
    pub createdAt: u64,
    #[sql_type = "Unsigned<Bigint>"]
    pub updatedAt: u64,
    #[sql_type = "Nullable<Varchar>"]
    pub slug: Option<String>,
    #[sql_type = "Varchar"]
    pub title: String,
    #[sql_type = "Nullable<Varchar>"]
    pub icon: Option<String>,
    #[sql_type = "Nullable<Smallint>"]
    pub order: Option<i16>,
    /**/
}

#[derive(Associations, Queryable, Serialize, Deserialize, QueryableByName, Clone, Debug)]
pub struct CustomerNormalizedObject {
    #[sql_type = "Unsigned<Integer>"]
    pub id: u32,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub active: Option<u8>,
    #[sql_type = "VarChar"]
    pub displayName: String,
    //pub info: String,
    #[sql_type = "Nullable<Unsigned<Smallint>>"]
    pub authLevel: Option<u16>,

    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub banned: Option<u8>,

    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnFollowEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnRepostEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnCommentEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnLikeEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnMessageEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnInfoEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnMentionEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnEncodeEnabled: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnMatchEnabled: Option<u8>,
    #[sql_type = "Unsigned<Tinyint>"]
    pub apnPanelInviteEnabled: u8,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub apnEnabled: Option<u8>,

    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub followerCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub followingCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub mentionCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub mentionedCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub notificationCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub postCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub likeCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub commentCount: Option<u32>,

    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub tagCount: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Bigint>>"]
    pub lastOnline: Option<u64>,
    #[sql_type = "Nullable<VarChar>"]
    pub gsm: Option<String>,
    #[sql_type = "Nullable<VarChar>"]
    pub sessionId: Option<String>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub isFamous: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Bigint>>"]
    pub createdAt: Option<u64>,
    #[sql_type = "Nullable<Unsigned<Bigint>>"]
    pub updatedAt: Option<u64>,
    #[sql_type = "Nullable<VarChar>"]
    pub lastIp: Option<String>,
    #[sql_type = "Nullable<Bool>"]
    pub isVerified: Option<bool>,
    #[sql_type = "Nullable<Smallint>"]
    pub anonymUser: Option<i16>,
    #[sql_type = "Nullable<Smallint>"]
    pub taskLevel: Option<i16>,
    #[sql_type = " Nullable<Varchar>"]
    pub firebaseToken: Option<String>,

    #[sql_type = "Nullable<Varchar>"]
    pub photo: Option<String>,
    #[sql_type = "Nullable<Varchar>"]
    pub address: Option<String>,

    #[sql_type = "Nullable<Tinytext>"]
    pub name: Option<String>,
    #[sql_type = "Nullable<Tinytext>"]
    pub surname: Option<String>,

    #[sql_type = "Nullable<Double>"]
    pub birthday: Option<f64>,
    #[sql_type = "Nullable<Varchar>"]
    pub gender: Option<String>,

    #[sql_type = "Nullable<Varchar>"]
    pub biography: Option<String>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub county: Option<u32>,
    #[sql_type = "Nullable<Unsigned<Integer>>"]
    pub city: Option<u32>,

    #[sql_type = "Nullable<Char>"]
    pub refcode: Option<String>,

    #[sql_type = "Nullable<Char>"]
    pub registerRefCode: Option<String>,

    #[sql_type = "Nullable<Integer>"]
    pub taskGroupId: Option<i32>,

    #[sql_type = "Nullable<Tinyint>"]
    pub missionLevel: Option<i8>,

    #[sql_type = "Nullable<Tinyint>"]
    pub missionSubLevel: Option<i8>,
    #[sql_type = "Nullable<Varchar>"]
    pub missionLevelString: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostObject {
    pub id: u32,
    pub text: Option<String>,
    pub survey: Option<u32>,
    pub isReady: Option<u8>,
    pub pinned: Option<u8>,
    pub weight: Option<u32>,
    pub original: Option<u32>,
    pub section: Option<u32>,
    pub createdByPlayz: Option<u8>,
    pub viewCount: Option<u32>,
    pub likeCount: Option<u32>,
    pub reportCount: Option<u32>,
    pub mentionCount: Option<u32>,
    pub tagCount: Option<u32>,
    pub likeRange: Option<u32>,
    pub commentCount: Option<u32>,
    pub createdAt: u64,
    pub updatedAt: u64,
    pub isLiked: Option<bool>,
    pub assets: Vec<PostAsset>,
    pub customer: Option<CustomerObject>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameprofileObject {
    pub id: u32,
    pub selective_id: String,
    pub game: Game,
    pub customer: u32,
    pub username: String,
    pub region: Option<String>,
    pub platform: Option<String>,
    pub avatar: Option<String>,
    pub uri: Option<String>,
    pub createdAt: u64,
    pub updatedAt: u64,
}

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Vec<String> {
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

    let customer_id = vec[0].to_string();
    let token = vec[1].to_string();
    //println!("{}",&bytes.1);
    return vec![customer_id, token];
    // println!("BYTES___{:?}", &bytes);
}

pub async fn populatePosts(lposts: Vec<Post>, pool: web::Data<MysqlPool>) -> Vec<PostObject> {
    let mysql_pool = mysql_pool_handler(pool).unwrap();

    // GET CUSTOMERS
    let mut customer_ids: Vec<String> = vec![];
    let mut post_ids: Vec<String> = vec![];
    let mut joined_ids = vec![];

    for x in 0..lposts.len() {
        //println!("{:?}",lposts[x]);
        let customer_id = &lposts[x].customer;
        post_ids.push(lposts[x].id.to_string());
        customer_ids.push(customer_id.unwrap().to_string());
        joined_ids.push(format!("({},{},'{}')", post_ids[x], customer_ids[x], "P"));
    }

    let customer_ids_str: String = customer_ids.join(",");
    let post_ids_str: String = post_ids.join(",");
    let joined_ids_str: String = joined_ids.join(",");

    //println!("customer_ids_str2_____{:?}",customer_ids_str2);
    //let customer_ids_str: String = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", customer_ids[0], customer_ids[1], customer_ids[2], customer_ids[3], customer_ids[4], customer_ids[5], customer_ids[6], customer_ids[7], customer_ids[8], customer_ids[9], customer_ids[10], customer_ids[11], customer_ids[12], customer_ids[13], customer_ids[14], customer_ids[15], customer_ids[16], customer_ids[17], customer_ids[18], customer_ids[19]);
    //let post_ids_str: String = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", post_ids[0], post_ids[1], post_ids[2], post_ids[3], post_ids[4], post_ids[5], post_ids[6], post_ids[7], post_ids[8], post_ids[9], post_ids[10], post_ids[11], post_ids[12], post_ids[13], post_ids[14], post_ids[15], post_ids[16], post_ids[17], post_ids[18], post_ids[19]);
    //let joined_ids_str: String = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", joined_ids[0], joined_ids[1], joined_ids[2], joined_ids[3], joined_ids[4], joined_ids[5], joined_ids[6], joined_ids[7], joined_ids[8], joined_ids[9], joined_ids[10], joined_ids[11], joined_ids[12], joined_ids[13], joined_ids[14], joined_ids[15], joined_ids[16], joined_ids[17], joined_ids[18], joined_ids[19]);

    let customersql = format!("SELECT 	*, 
	customer.*, 
	customer.id AS id FROM customer LEFT JOIN customerinformation ON  customer.id = customerinformation.customer where customer.id IN ({})",customer_ids_str);

    let lcustomer = diesel::sql_query(&customersql)
        .load::<CustomerNormalizedObject>(&mysql_pool)
        .expect("Error loading posts____");

    let mut customers_hashmap = HashMap::new();
    let mut customersinfo_hashmap = HashMap::new();

    for x in 0..lcustomer.len() {
        customers_hashmap.insert(lcustomer[x].id, &lcustomer[x]);
        customersinfo_hashmap.insert(lcustomer[x].id, &lcustomer[x]);
    }

    let gameprofilesql = format!(
        "SELECT
    gameprofile.*, 
	game.*
    FROM
    gameprofile
    LEFT JOIN
    game
    ON 
        gameprofile.game = game.id
    WHERE
    gameprofile.customer  IN ({})",
        customer_ids_str
    );

    let gameprofiles = diesel::sql_query(&gameprofilesql)
        .load::<GameprofileNormalizedObject>(&mysql_pool)
        .expect("Error loading gameprofiles");

    let mut gameprofiles_hashmap: HashMap<u32, Vec<GameprofileObject>> = HashMap::new();

    for x in 0..gameprofiles.len() {
        let gameprofileobject = GameprofileObject {
            id: gameprofiles[x].id,
            selective_id: gameprofiles[x].selective_id.clone(),
            customer: gameprofiles[x].customer.clone(),
            username: gameprofiles[x].username.clone(),
            region: gameprofiles[x].region.clone(),
            platform: gameprofiles[x].platform.clone(),
            avatar: gameprofiles[x].avatar.clone(),
            uri: gameprofiles[x].uri.clone(),
            createdAt: gameprofiles[x].createdAt,
            updatedAt: gameprofiles[x].updatedAt,
            game: Game {
                id: gameprofiles[x].id,
                slug: gameprofiles[x].slug.clone(),
                title: gameprofiles[x].title.clone(),
                icon: gameprofiles[x].icon.clone(),
                order: gameprofiles[x].order,
                createdAt: gameprofiles[x].createdAt,
                updatedAt: gameprofiles[x].updatedAt,
            },
        };

        gameprofiles_hashmap
            .entry(gameprofiles[x].customer)
            .or_insert_with(Vec::new)
            .push(gameprofileobject);
    }

    let postassetsql = format!("SELECT pa.post, pa.order, pa.asset, a.* FROM post_asset AS pa INNER JOIN asset AS a ON  a.id = pa.asset  WHERE a.isReady = 1 AND pa.post IN ({})",post_ids_str);

    let postassets = diesel::sql_query(&postassetsql)
        .load::<PostAsset>(&mysql_pool)
        .expect("Error loading assets");

    let mut postassets_hashmap = HashMap::new();

    for x in 0..postassets.len() {
        postassets_hashmap.insert(postassets[x].post, &postassets[x]);
    }
    let likedmessages_sql = format!("select *, CONCAT(post,':',who,':',`type`) as id from likeconnection where (post,who,`type`)  in ({})",joined_ids_str);

    let likedmessages = diesel::sql_query(&likedmessages_sql)
        .load::<LikeConnection>(&mysql_pool)
        .expect("Error loading assets");

    let mut likedmessages_index = Vec::new();

    for x in 0..likedmessages.len() {
        likedmessages_index.push(likedmessages[x].post.parse().unwrap());
    }

    let mut output = Vec::new();
    for x in 0..lposts.len() {
        let mut tempcustomerobject: CustomerObject =
            if customers_hashmap.contains_key(&lposts[x].customer.unwrap()) {
                let tco = customers_hashmap
                    .get(&lposts[x].customer.unwrap())
                    .unwrap()
                    .clone()
                    .clone();
                let mut gameprofile_customer = vec![];
                if gameprofiles_hashmap.contains_key(&lposts[x].customer.unwrap()) {
                    gameprofile_customer = gameprofiles_hashmap
                        .get(&lposts[x].customer.unwrap())
                        .unwrap()
                        .clone()
                        .clone();
                }

                CustomerObject {
                    id: tco.id,
                    active: tco.active,
                    displayName: tco.displayName,

                    authLevel: tco.authLevel,
                    banned: tco.banned,

                    apnFollowEnabled: tco.apnFollowEnabled,
                    apnRepostEnabled: tco.apnRepostEnabled,
                    apnCommentEnabled: tco.apnCommentEnabled,
                    apnLikeEnabled: tco.apnLikeEnabled,
                    apnMessageEnabled: tco.apnMessageEnabled,
                    apnInfoEnabled: tco.apnInfoEnabled,
                    apnMentionEnabled: tco.apnMentionEnabled,
                    apnEncodeEnabled: tco.apnEncodeEnabled,
                    apnMatchEnabled: tco.apnMatchEnabled,
                    apnPanelInviteEnabled: tco.apnPanelInviteEnabled,
                    apnEnabled: tco.apnEnabled,

                    followerCount: tco.followerCount,
                    followingCount: tco.followingCount,
                    mentionCount: tco.mentionCount,
                    mentionedCount: tco.mentionedCount,
                    notificationCount: tco.notificationCount,
                    postCount: tco.postCount,
                    likeCount: tco.likeCount,
                    commentCount: tco.commentCount,

                    tagCount: tco.tagCount,
                    lastOnline: tco.lastOnline,
                    gsm: tco.gsm,
                    sessionId: tco.sessionId,
                    isFamous: tco.isFamous,
                    createdAt: tco.createdAt,
                    updatedAt: tco.updatedAt,
                    lastIp: tco.lastIp,
                    isVerified: tco.isVerified,
                    anonymUser: tco.anonymUser,
                    taskLevel: tco.taskLevel,
                    firebaseToken: tco.firebaseToken,

                    info: CustomerInfoObject {
                        id: tco.id,

                        customer: tco.id,
                        photo: tco.photo,
                        address: tco.address,
                        name: tco.name,
                        surname: tco.surname,

                        birthday: tco.birthday,
                        gender: tco.gender,
                        biography: tco.biography,
                        county: tco.county,
                        city: tco.city,

                        createdAt: tco.createdAt,
                        updatedAt: tco.updatedAt,

                        refcode: tco.refcode,
                        registerRefCode: tco.registerRefCode,
                        taskGroupId: tco.taskGroupId,

                        missionLevel: tco.missionLevel,
                        missionSubLevel: tco.missionSubLevel,
                        missionLevelString: tco.missionLevelString,
                    },
                    profiles: gameprofile_customer,
                }
            } else {
                let tco = customers_hashmap
                    .get(&lposts[x].customer.unwrap())
                    .unwrap()
                    .clone()
                    .clone();
                let mut gameprofile_customer = vec![];
                if gameprofiles_hashmap.contains_key(&lposts[x].customer.unwrap()) {
                    gameprofile_customer = gameprofiles_hashmap
                        .get(&lposts[x].customer.unwrap())
                        .unwrap()
                        .clone()
                        .clone();
                }
                CustomerObject {
                    id: tco.id,
                    active: tco.active,
                    displayName: tco.displayName,
                    authLevel: tco.authLevel,

                    banned: tco.banned,

                    apnFollowEnabled: tco.apnFollowEnabled,
                    apnRepostEnabled: tco.apnRepostEnabled,
                    apnCommentEnabled: tco.apnCommentEnabled,
                    apnLikeEnabled: tco.apnLikeEnabled,
                    apnMessageEnabled: tco.apnMessageEnabled,
                    apnInfoEnabled: tco.apnInfoEnabled,
                    apnMentionEnabled: tco.apnMentionEnabled,
                    apnEncodeEnabled: tco.apnEncodeEnabled,
                    apnMatchEnabled: tco.apnMatchEnabled,
                    apnPanelInviteEnabled: tco.apnPanelInviteEnabled,
                    apnEnabled: tco.apnEnabled,

                    followerCount: tco.followerCount,
                    followingCount: tco.followingCount,
                    mentionCount: tco.mentionCount,
                    mentionedCount: tco.mentionedCount,
                    notificationCount: tco.notificationCount,
                    postCount: tco.postCount,
                    likeCount: tco.likeCount,
                    commentCount: tco.commentCount,

                    tagCount: tco.tagCount,
                    lastOnline: tco.lastOnline,
                    gsm: tco.gsm,
                    sessionId: tco.sessionId,
                    isFamous: tco.isFamous,
                    createdAt: tco.createdAt,
                    updatedAt: tco.updatedAt,
                    lastIp: tco.lastIp,
                    isVerified: tco.isVerified,
                    anonymUser: tco.anonymUser,
                    taskLevel: tco.taskLevel,
                    firebaseToken: tco.firebaseToken,

                    info: CustomerInfoObject {
                        id: tco.id,
                        customer: tco.id,
                        photo: tco.photo,
                        address: tco.address,
                        name: tco.name,
                        surname: tco.surname,
                        birthday: tco.birthday,
                        gender: tco.gender,
                        biography: tco.biography,
                        county: tco.county,
                        city: tco.city,
                        createdAt: tco.createdAt,
                        updatedAt: tco.updatedAt,
                        refcode: tco.refcode,
                        registerRefCode: tco.registerRefCode,
                        taskGroupId: tco.taskGroupId,
                        missionLevel: tco.missionLevel,
                        missionSubLevel: tco.missionSubLevel,
                        missionLevelString: tco.missionLevelString,
                    },
                    profiles: gameprofile_customer,
                }
            };

        output.push(PostObject {
            id: lposts[x].id,
            text: lposts[x].text.clone(),
            survey: lposts[x].survey,

            isReady: lposts[x].isReady,
            pinned: lposts[x].pinned,
            weight: lposts[x].weight,
            original: lposts[x].original,
            section: lposts[x].section,
            createdByPlayz: lposts[x].createdByPlayz,
            viewCount: lposts[x].viewCount,
            likeCount: lposts[x].likeCount,
            reportCount: lposts[x].reportCount,
            mentionCount: lposts[x].mentionCount,
            tagCount: lposts[x].tagCount,
            likeRange: lposts[x].likeRange,
            commentCount: lposts[x].commentCount,
            createdAt: lposts[x].createdAt,
            updatedAt: lposts[x].updatedAt,
            isLiked: Some(false),
            assets: vec![],
            //customer: Some(CustomerObject::new()),
            customer: Some(tempcustomerobject),
        });

        if postassets_hashmap.contains_key(&lposts[x].id) {
            output[x].assets.push(
                postassets_hashmap
                    .get(&lposts[x].id)
                    .unwrap()
                    .clone()
                    .clone(),
            );
        }

        if likedmessages_index.contains(&lposts[x].id) {
            output[x].isLiked = Some(true);
        } else {
            output[x].isLiked = Some(false);
        }
    }

    output
}

pub async fn get(
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
    redispool: web::Data<R2D2Pool>,
) -> Result<HttpResponse, HttpResponse> {
    let mut tokens = vec![];
    tokens = get_content_type(&_req);

    let query_str = _req.query_string();
    let qs = QString::from(query_str);
    let section_id = qs.get("section").unwrap();
    let mut cursor = qs.get("cursor").unwrap_or("0");

    let customer_id = &tokens[0];

    let cache_url = format!(
        "cache:{}:/api/posts/bySection?section={}",
        customer_id, section_id
    );

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
            // println!("parsedjson_from_cache__{:?}",parsedjson);
            Ok(HttpResponse::Ok().json(parsedjson))
        }
        Err(error) => {
            let mysql_pool = mysql_pool_handler(pool.clone())?;

            let limit = "20";

            let sql = format!(
                "SELECT p.* FROM post as p
                    WHERE
                        p.createdAt < NOW() 
                    AND p.isReady = 1
                    AND p.customer NOT IN (
                        SELECT DISTINCT blocked FROM blockconnection WHERE blocker = {}
                    )
                    AND p.customer NOT IN (
                        SELECT DISTINCT blocker FROM blockconnection WHERE blocked = {}
                    )
            AND p.section = {}
            GROUP BY p.id
            ORDER BY p.id DESC LIMIT {}, {}",
                customer_id, customer_id, section_id, cursor, limit
            );

            let mut lposts = diesel::sql_query(&sql)
                .load::<Post>(&mysql_pool)
                .expect("Error loading posts____");

            let output = populatePosts(lposts, pool);

            //println!("{:?}",output);

            let output2 = QueryObject {
                code: 0,
                cached: true,
                message: "".to_string(),
                messageFriendly: "".to_string(),
                data: output.await,
                cursor: (cursor.parse::<u32>().unwrap() + 1).to_string(),
            };

            Ok(HttpResponse::Ok().json(output2))
        }
    };

    f
}

pub async fn customers_posts(
    param: web::Path<String>,
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool.clone())?;
    let mut tokens = vec![];
    tokens = get_content_type(&_req);

    let query_str = _req.query_string(); // "name=ferret"
    let qs = QString::from(query_str);
    let mut cursor = qs.get("cursor").unwrap_or("0"); // "ferret"

    let customer_id = &tokens[0];
    let limit = "20";

    let sql = format!("select * from post where isReady = true and  customer = {} and updatedAt < NOW()  order by id desc LIMIT {}, {}",customer_id,cursor,limit);

    let mut lposts = diesel::sql_query(&sql)
        .load::<Post>(&mysql_pool)
        .expect("Error loading posts____");

    let output = populatePosts(lposts, pool);

    let output2 = QueryObject {
        code: 0,
        cached: true,
        message: "".to_string(),
        messageFriendly: "".to_string(),
        data: output.await,
        cursor: (cursor.parse::<u32>().unwrap() + 1).to_string(),
    };

    Ok(HttpResponse::Ok().json(output2))
}

pub async fn customers_followings_posts(
    param: web::Path<String>,
    _req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool.clone())?;
    let mut tokens = vec![];
    tokens = get_content_type(&_req);

    let query_str = _req.query_string(); // "name=ferret"
    let qs = QString::from(query_str);
    //let section_id = qs.get("section").unwrap(); // "ferret"
    let mut cursor = qs.get("cursor").unwrap_or("0"); // "ferret"

    let customer_id = &tokens[0];
    //let section_id = section_id;
    let limit = "20";

    let sql = format!(
        "SELECT  * FROM post AS p
        INNER JOIN userconnection AS uc ON uc.with = p.customer
     WHERE uc.who = {} AND p.isReady = 1
     AND p.createdAt < NOW()
     GROUP BY p.id
     ORDER BY p.id DESC LIMIT {}, {}",
        customer_id, cursor, limit
    );

    let mut lposts = diesel::sql_query(&sql)
        .load::<Post>(&mysql_pool)
        .expect("Error loading posts____");

    let output = populatePosts(lposts, pool);

    //println!("{:?}",output);

    let output2 = QueryObject {
        code: 0,
        cached: true,
        message: "".to_string(),
        messageFriendly: "".to_string(),
        data: output.await,
        cursor: (cursor.parse::<u32>().unwrap() + 1).to_string(),
    };

    Ok(HttpResponse::Ok().json(output2))
}
